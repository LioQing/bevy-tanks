use bevy::prelude::*;
use bevy_tanks_data::*;

use crate::common::ui::button;

pub fn setup_game_over(
    mut commands: Commands,
    typography: Res<Typography>,
    tank: Single<&TankController, With<TankAlive>>,
) {
    commands
        .spawn((
            Name::new("Game Over UI"),
            StateScoped(AppState::Game),
            StateScoped(GameState::Over),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                Text::new("Game Over"),
                typography.subtitle.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
            children.spawn((
                Text::new(format!("{} won!", tank.label)),
                typography.title.clone(),
                TextColor(match tank.label {
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
                    button::spawn(
                        children,
                        &typography,
                        "Play Again",
                        GameOverUiButtonEvent::PlayAgain,
                    );
                    button::spawn(
                        children,
                        &typography,
                        "Main Menu",
                        GameOverUiButtonEvent::MainMenu,
                    );
                });
        });
}

pub fn observe_game_over_button(
    trigger: Trigger<GameOverUiButtonEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    match trigger.event() {
        GameOverUiButtonEvent::PlayAgain => {
            game_state.set(GameState::Playing);
        }
        GameOverUiButtonEvent::MainMenu => {
            app_state.set(AppState::MainMenu);
        }
    }
}
