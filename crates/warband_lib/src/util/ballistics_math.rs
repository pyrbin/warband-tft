use glam::Vec3;
use std::f32;

pub fn solve_ballistic_arc(
    proj_pos: Vec3,
    proj_speed: f32,
    target: Vec3,
    gravity: f32,
) -> Option<(Vec3, Vec3)> {
    assert!(
        proj_pos != target && proj_speed > 0.0 && gravity > 0.0,
        "solve_ballistic_arc called with invalid data"
    );

    let diff = target - proj_pos;
    let diff_xz = Vec3::new(diff.x, 0.0, diff.z);
    let ground_dist = diff_xz.length();

    let speed2 = proj_speed * proj_speed;
    let speed4 = speed2 * speed2;
    let y = diff.y;
    let x = ground_dist;
    let gx = gravity * x;

    let root = speed4 - gravity * (gravity * x * x + 2.0 * y * speed2);
    if root < 0.0 {
        return None;
    }

    let root = root.sqrt();

    let low_ang = (speed2 - root).atan2(gx);
    let high_ang = (speed2 + root).atan2(gx);

    let num_solutions = if low_ang != high_ang { 2 } else { 1 };

    let ground_dir = diff_xz.normalize();

    let s0 = ground_dir * low_ang.cos() * proj_speed + Vec3::Y * low_ang.sin() * proj_speed;

    let s1 = if num_solutions > 1 {
        ground_dir * high_ang.cos() * proj_speed + Vec3::Y * high_ang.sin() * proj_speed
    } else {
        s0
    };

    Some((s0, s1))
}
