use bevy::prelude::*;

use crate::hold::Held;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemCounterResource>()
            .add_systems(Update, increment_item_counter);
    }
}

#[derive(Resource, Default)]
pub struct ItemCounterResource(pub ItemCounter);

#[derive(Default)]
pub struct ItemCounter {
    count: f32,
}

impl ItemCounter {
    pub fn get_count(&self) -> f32 {
        self.count
    }

    pub fn increment(&mut self) {
        self.count += 1.0;
    }
}

#[derive(Component)]
pub struct Item;

fn increment_item_counter(
    mut item_counter: ResMut<ItemCounterResource>,
    mut query: Query<&mut Transform, Or<(Added<Item>, Added<Held>)>>,
) {
    for mut transform in query.iter_mut() {
        item_counter.0.increment();
        transform.translation.z = item_counter.0.get_count();
    }
}
