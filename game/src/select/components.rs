use crate::prelude::*;

#[derive(Component)]
pub(crate) struct SelectionBox {
    pub start_position: Vec2,
    pub end_position: Vec2,
}

impl SelectionBox {
    pub fn update(&mut self, position: Vec2) {
        self.end_position = position;
    }

    pub fn rect(&self) -> Rect {
        Rect::from_corners(self.start_position, self.end_position)
    }
}

#[derive(Component)]
pub(crate) struct Selectable;

#[derive(Component, Default)]
pub(super) struct SelectedRect {
    rect: Rect,
    initial_rect: Rect,
}

impl SelectedRect {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            initial_rect: rect,
        }
    }

    pub fn move_to(&mut self, position: Vec2) {
        self.rect = Rect::from_center_size(position, self.rect.size());
    }

    pub fn update(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn commit(&mut self) {
        self.initial_rect = self.rect;
    }

    pub fn contains(&self, position: Vec2) -> bool {
        self.rect.contains(position)
    }

    pub fn initial_rect(&self) -> Rect {
        self.initial_rect
    }

    pub fn initial_point(&self) -> Vec2 {
        self.initial_rect.center()
    }
}

#[derive(Component)]
pub(crate) struct Selected {
    pub start_position: Vec2,
}

impl Selected {
    pub fn new(start_position: Vec2) -> Self {
        Self { start_position }
    }
}
