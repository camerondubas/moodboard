use bevy::render::primitives::Aabb;

use crate::prelude::*;

pub(super) fn get_anchor(position: Vec2) -> shapes::RectangleOrigin {
    match (position.x, position.y) {
        (x, y) if x > 0.0 && y > 0.0 => shapes::RectangleOrigin::BottomLeft,
        (x, y) if x < 0.0 && y > 0.0 => shapes::RectangleOrigin::BottomRight,
        (x, y) if x > 0.0 && y < 0.0 => shapes::RectangleOrigin::TopLeft,
        (x, y) if x < 0.0 && y < 0.0 => shapes::RectangleOrigin::TopRight,
        _ => shapes::RectangleOrigin::Center,
    }
}

pub(super) fn get_surrounding_rect(query: Vec<(&GlobalTransform, &Aabb)>) -> Option<Rect> {
    let mut rect_option: Option<Rect> = None;
    for (transform, aabb) in query {
        let rect =
            Rect::from_center_half_size(transform.translation().xy(), aabb.half_extents.xy());

        rect_option = Some(rect_option.map_or(rect, |r| r.union(rect)));
    }

    rect_option
}
