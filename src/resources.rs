use bevy::prelude::*;
use std::collections::HashSet;

const BOUNDING_BOX_COLOR: Color = Color::rgba(0.0, 1.0, 0.0, 0.33);
const SELECTED_AREA_BOX_COLOR: Color = Color::rgba(1.0, 1.0, 0.0, 0.33);
const THICKNESS_OF_SELECTION_LINES: f32 = 0.05;
const SELECTED_LINE_THICKNESS: f32 = 0.1;

#[derive(Resource, Default, Debug, Clone)]
pub struct Cursor {
    pub entity: Option<Entity>,
    pub location: Vec3,
    pub pressed_location: Vec3,
    pub selection: Selection,
    pub xyz1: Vec3,
    pub xyz2: Vec3,
    pub cursor_settings: CursorSettings,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct Selection {
    pub entity: Option<Entity>,
    pub selected_units: HashSet<Entity>,
    pub just_selected: bool,
}

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

#[derive(Resource, Debug, Clone)]
pub struct CursorPlugin {
    pub bounds: Bounds2D,
    pub aesthetics: Aesthetics,
    pub y_inclusion_limit: f32,
    pub torus_offset: f32,
}

pub type CursorSettings = CursorPlugin;
