use avian3d::{
    parry::{
        na::{Const, OPoint, Unit, Vector3},
        query::IntersectResult,
        shape::{Polyline, TriMesh, TypedShape},
    },
    prelude::Collider,
};
use dashmap::DashMap;
use hexx::Hex;

use crate::prelude::*;

use super::Footprint;

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct Occupied(Arc<OccupiedBoard>);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct OccupiedBoard(DashMap<Hex, HashSet<Entity>>);

impl OccupiedBoard {
    pub fn cells(&self) -> impl Iterator<Item = Hex> + '_ {
        self.iter().map(|v| *v.key())
    }
}

pub(super) fn splat(
    footprints_changed: Query<Entity, Changed<Footprint>>,
    footprints: Query<(Entity, &Footprint)>,
    obstacles: ResMut<Occupied>,
) {
    if footprints_changed.is_empty() {
        return;
    }

    obstacles.clear();

    for (entity, footprint) in &footprints {
        for hex in footprint.cells().copied() {
            obstacles.entry(hex).or_default().insert(entity);
        }
    }
}

/// ref: https://github.com/vleue/vleue_navigator/blob/main/src/obstacles/mod.rs
pub trait GetPolygon: Component + Clone {
    /// Get the polygon of the obstacle in the local space of the mesh.
    fn get_polygon(
        &self,
        obstacle: &GlobalTransform,
        board: &Transform,
        up: (Dir3, f32),
    ) -> Vec<Vec2>;
}

impl GetPolygon for Collider {
    fn get_polygon(
        &self,
        obstacle: &GlobalTransform,
        board: &Transform,
        up: (Dir3, f32),
    ) -> Vec<Vec2> {
        self.shape_scaled()
            .as_typed_shape()
            .get_polygon(obstacle, board, up)
    }
}

trait InnerGetPolygon {
    fn get_polygon(
        &self,
        obstacle: &GlobalTransform,
        board: &Transform,
        up: (Dir3, f32),
    ) -> Vec<Vec2>;
}

impl<'a> InnerGetPolygon for TypedShape<'a> {
    fn get_polygon(
        &self,
        obstacle: &GlobalTransform,
        board: &Transform,
        (up, shift): (Dir3, f32),
    ) -> Vec<Vec2> {
        const RESOLUTION: u32 = 32;

        let mut transform = obstacle.compute_transform();
        transform.scale = Vec3::ONE;
        let world_to_board = board.compute_affine().inverse();

        let to_board =
            |p: OPoint<f32, Const<3>>| world_to_board.transform_point(vec3(p.x, p.y, p.z)).xz();

        let intersection_to_board = |intersection: IntersectResult<Polyline>| match intersection {
            IntersectResult::Intersect(i) => i.segments().map(|s| s.a).map(to_board).collect(),
            IntersectResult::Negative => vec![],
            IntersectResult::Positive => vec![],
        };

        let d =
            (-up.x * board.translation.x - up.y * board.translation.y - up.z * board.translation.z)
                / (up.x.powi(2) + up.y.powi(2) + up.z.powi(2)).sqrt();
        let shift: f32 = shift - d;

        // #FB_NOTE: not sure why but using 0.0 causes error in parry
        const MIN_SHIFT: f32 = 0.01;
        let shift = if shift < 0.0 {
            shift.min(-MIN_SHIFT)
        } else {
            shift.max(MIN_SHIFT)
        };

        let to_world = |p: &OPoint<f32, Const<3>>| transform.transform_point(vec3(p.x, p.y, p.z));
        let up_axis = Unit::new_normalize(Vector3::new(up.x, up.y, up.z));
        let trimesh_to_world = |vertices: Vec<OPoint<f32, Const<3>>>| {
            vertices
                .iter()
                .map(to_world)
                .map(|v| v.into())
                .collect::<Vec<OPoint<f32, Const<3>>>>()
        };
        match self {
            TypedShape::Cuboid(collider) => {
                let (vertices, indices) = collider.to_trimesh();
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::Ball(collider) => {
                let (vertices, indices) = collider.to_trimesh(RESOLUTION, RESOLUTION);
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::Capsule(collider) => {
                let (vertices, indices) = collider.to_trimesh(RESOLUTION, RESOLUTION);
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::TriMesh(collider) => {
                vec![intersection_to_board(
                    collider.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::HeightField(collider) => {
                let (vertices, indices) = collider.to_trimesh();
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::Compound(collider) => collider
                .shapes()
                .iter()
                .map(|(_iso, shape)| {
                    shape
                        .as_typed_shape()
                        .get_polygon(obstacle, board, (up, shift))
                })
                .collect(),
            TypedShape::ConvexPolyhedron(collider) => {
                let (vertices, indices) = collider.to_trimesh();
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::Cylinder(collider) => {
                let (vertices, indices) = collider.to_trimesh(RESOLUTION);
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::Cone(collider) => {
                let (vertices, indices) = collider.to_trimesh(RESOLUTION);
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::RoundCuboid(collider) => {
                let (vertices, indices) = collider.inner_shape.to_trimesh();
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::RoundCylinder(collider) => {
                let (vertices, indices) = collider.inner_shape.to_trimesh(RESOLUTION);
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::RoundCone(collider) => {
                let (vertices, indices) = collider.inner_shape.to_trimesh(RESOLUTION);
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            TypedShape::RoundConvexPolyhedron(collider) => {
                let (vertices, indices) = collider.inner_shape.to_trimesh();
                let trimesh = TriMesh::new(trimesh_to_world(vertices), indices);
                vec![intersection_to_board(
                    trimesh.intersection_with_local_plane(&up_axis, shift, f32::EPSILON),
                )]
            }
            _ => {
                warn!("Collider not supported for obstacle generation");
                vec![]
            }
        }
        .into_iter()
        .flatten()
        .collect()
    }
}
