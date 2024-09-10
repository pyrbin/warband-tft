use crate::prelude::*;

pub(crate) fn plugin<T: Component + Clone + GetTypeRegistration + TypePath + FromReflect>(
    app: &mut App,
) {
    app_register_types!(Previous<T>);
    app.add_systems(Last, propagate_previous_changed::<T>);
}

#[derive(Component, Default, Deref, DerefMut, Reflect, From)]
pub struct Previous<T: Component + Clone>(T);

impl<T: Component + Clone> Previous<T> {
    #[allow(unused)]
    pub fn get(&self) -> &T {
        &self.0
    }
}

pub(crate) fn propagate_previous_changed<T: Component + Clone>(
    mut commands: Commands,
    mut values: Query<(Entity, Option<&mut Previous<T>>, &T), Changed<T>>,
) {
    for (entity, mut previous_value, current_value) in values.iter_mut() {
        if let Some(previous_value) = &mut previous_value {
            previous_value.0 = current_value.clone();
        } else {
            commands
                .entity(entity)
                .insert(Previous(current_value.clone()));
        }
    }
}
