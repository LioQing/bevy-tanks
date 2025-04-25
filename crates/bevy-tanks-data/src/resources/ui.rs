use bevy::prelude::*;

use crate::{EventEmitter, GameOverUiButtonEvent, GameState, TankLabel};

use super::Typography;

#[derive(Debug, Default, Resource)]
pub struct GameOverUiPrefab;

impl GameOverUiPrefab {
    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        button_ui_prefab: &ButtonUiPrefab,
        typography: &Typography,
        winner: TankLabel,
    ) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn((
            Name::new("Game Over UI"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            StateScoped(GameState::Over),
        ));

        entity_commands.with_children(|children| {
            children.spawn((
                Text::new("Game Over"),
                typography.subtitle.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
            children.spawn((
                Text::new(format!("{winner} won!")),
                typography.title.clone(),
                TextColor(match winner {
                    TankLabel::Red => Color::srgb(1.0, 0.0, 0.0),
                    TankLabel::Blue => Color::srgb(0.0, 0.5, 1.0),
                }),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
            children.spawn(Node {
                height: Val::Px(32.0),
                ..default()
            });

            children
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|children| {
                    button_ui_prefab
                        .spawn(children, typography, "Play Again")
                        .insert(EventEmitter(GameOverUiButtonEvent::PlayAgain));
                    button_ui_prefab
                        .spawn(children, typography, "Quit")
                        .insert(EventEmitter(GameOverUiButtonEvent::Quit));
                });
        });

        entity_commands
    }
}

#[derive(Debug, Default, Resource)]
pub struct ButtonUiPrefab;

impl ButtonUiPrefab {
    pub const BORDER_COLOR: Color = Color::WHITE;
    pub const BORDER_HOVER_COLOR: Color = Color::WHITE;
    pub const BORDER_PRESSED_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
    pub const BACKGROUND_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BACKGROUND_HOVER_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
    pub const BACKGROUND_PRESSED_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

    pub fn spawn<'a>(
        &self,
        commands: &'a mut ChildBuilder,
        typography: &Typography,
        text: &str,
    ) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn((
            Name::new(format!("{text} Button")),
            Button,
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Self::BORDER_COLOR),
            BorderRadius::MAX,
            BackgroundColor(Self::BACKGROUND_COLOR),
        ));

        entity_commands.with_children(|children| {
            children.spawn((
                Text::new(text),
                typography.body.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });

        entity_commands
    }
}
