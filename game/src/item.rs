#![allow(clippy::type_complexity)]
use crate::{
    prelude::*,
    select::components::{Selectable, Selected, SelectionBox},
};
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemCounter>()
            .add_systems(Update, (increment_item_counter, delete_item));
    }
}

#[derive(Resource, Default)]
pub struct ItemCounter {
    count: f32,
}

#[derive(Component)]
pub struct Deletable;

impl ItemCounter {
    pub fn count(&self) -> f32 {
        self.count
    }

    fn increment(&mut self) {
        self.count += 1.0;
    }
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub selectable: Selectable,
    pub fill: Fill,
    pub stroke: Stroke,
    pub shape: ShapeBundle,
    pub deletable: Deletable,
}

impl Default for ItemBundle {
    fn default() -> Self {
        Self {
            item: Item,
            selectable: Selectable,
            deletable: Deletable,
            fill: Fill::color(Palette::WHITE),
            stroke: Stroke::new(Palette::BLACK.with_a(0.), 0.),
            shape: ShapeBundle {
                ..Default::default()
            },
        }
    }
}

#[derive(Component)]
pub struct Item;

fn increment_item_counter(
    mut item_counter: ResMut<ItemCounter>,
    mut query: Query<(Entity, &mut Transform), Or<(Added<Item>, Added<Selected>)>>,
    selected_query: Query<Entity, &Selected>,
    selection_box_query: Query<&SelectionBox>,
) {
    let selected_count = selected_query.iter().count();
    for (entity, mut transform) in &mut query {
        let is_selected = selected_query.get(entity).is_ok();
        let is_selecting = !selection_box_query.is_empty();

        if (is_selected && selected_count > 1) || is_selecting {
            // Only increment if there is only one selected item
            // This indicates that the item was just selected, and should
            // be brought to the front
            continue;
        }

        item_counter.increment();
        transform.translation.z = item_counter.count();
    }
}

fn delete_item(
    mut commands: Commands,
    query: Query<(Entity, &Deletable), With<Item>>,
    selected_query: Query<Entity, With<Selected>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Back) {
        for entity in selected_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
