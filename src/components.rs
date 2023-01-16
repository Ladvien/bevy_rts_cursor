use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct CursorReflector;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct SelectionHighlighter;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Selected;
