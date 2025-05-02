use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct Gamepads(pub Vec<Entity>);

#[derive(Debug, Default, Resource)]
pub struct AiNavQueue(pub VecDeque<(Entity, Vec3)>);

#[derive(Debug, Resource)]
pub struct Typography {
    pub title: TextFont,
    pub subtitle: TextFont,
    pub body: TextFont,
}

impl FromWorld for Typography {
    fn from_world(world: &mut World) -> Self {
        Typography {
            title: TextFont {
                font: world.load_asset("fonts/Quicksand-Bold.ttf"),
                font_size: 64.0,
                ..default()
            },
            subtitle: TextFont {
                font: world.load_asset("fonts/Quicksand-Regular.ttf"),
                font_size: 32.0,
                ..default()
            },
            body: TextFont {
                font: world.load_asset("fonts/Quicksand-Regular.ttf"),
                font_size: 24.0,
                ..default()
            },
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct GameMode {
    pub multiplayer: bool,
}
