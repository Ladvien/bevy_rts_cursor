use bevy::prelude::*;

const BOUNDING_BOX_COLOR: Color = Color::rgba(0.0, 1.0, 0.0, 0.33);
const SELECTED_AREA_BOX_COLOR: Color = Color::rgba(1.0, 1.0, 0.0, 0.33);
const THICKNESS_OF_SELECTION_LINES: f32 = 0.05;
const AFTER_SELECTION_BLINK_DURATION: f32 = 0.08;
const SELECTED_LINE_THICKNESS: f32 = 0.1;

#[derive(Debug, Clone, Resource, Reflect)]
pub struct Bounds2D {
    pub min_x: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_z: f32,
}

#[derive(Debug, Clone, Resource, Reflect)]
pub struct Aesthetics {
    pub bounding_box_color: Color,
    pub selected_area_box_color: Color,
    pub line_thickness: f32,
    pub selected_line_thickness: f32,
}

impl Default for Aesthetics {
    fn default() -> Self {
        Self {
            bounding_box_color: BOUNDING_BOX_COLOR,
            selected_area_box_color: SELECTED_AREA_BOX_COLOR,
            line_thickness: SELECTED_LINE_THICKNESS,
            selected_line_thickness: THICKNESS_OF_SELECTION_LINES,
        }
    }
}

pub struct CursorPlugin {
    pub bounds: Bounds2D,
    pub aesthetics: Aesthetics,
}
