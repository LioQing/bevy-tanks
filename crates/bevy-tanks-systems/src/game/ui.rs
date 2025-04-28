use bevy::prelude::*;
use bevy_tanks_data::*;

use crate::common::ui::button;

pub fn setup_loading(mut commands: Commands, typography: Res<Typography>) {
    commands
        .spawn((
            Name::new("Loading UI"),
            StateScoped(GameState::Loading),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|children| {
            children.spawn((
                Text::new("Loading..."),
                typography.subtitle.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

pub fn setup_paused(mut commands: Commands, typography: Res<Typography>) {
    commands
        .spawn((
            Name::new("Pause UI"),
            StateScoped(GameState::Paused),
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
                Text::new("Paused"),
                typography.subtitle.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
            children.spawn(Node {
                height: Val::Px(32.0),
                ..default()
            });
            button::spawn(children, &typography, "Resume", PausedUiButtonEvent::Resume);
            children.spawn(Node {
                height: Val::Px(12.0),
                ..default()
            });
            button::spawn(
                children,
                &typography,
                "Main Menu",
                PausedUiButtonEvent::MainMenu,
            );
        });
}

pub fn update_paused(
    mut game_state: ResMut<NextState<GameState>>,
    buttons: Res<ButtonInput<KeyCode>>,
) {
    if buttons.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Playing);
    }
}

pub fn observe_paused_button(
    trigger: Trigger<PausedUiButtonEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    match trigger.event() {
        PausedUiButtonEvent::Resume => {
            game_state.set(GameState::Playing);
        }
        PausedUiButtonEvent::MainMenu => {
            app_state.set(AppState::MainMenu);
        }
    }
}

pub fn update_pause_when_playing(
    mut game_state: ResMut<NextState<GameState>>,
    buttons: Res<ButtonInput<KeyCode>>,
) {
    if buttons.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Paused);
    }
}

pub fn setup_game_over(
    mut commands: Commands,
    typography: Res<Typography>,
    tank: Single<&TankController, With<TankAlive>>,
) {
    commands
        .spawn((
            Name::new("Game Over UI"),
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
