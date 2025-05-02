use bevy::prelude::*;
use bevy_tanks_data::*;

use crate::common::ui::button;

pub fn plugin(app: &mut App) {
    app.add_state_scoped_event::<MainMenuUiButtonEvent>(AppState::MainMenu)
        .add_observer(observe_button)
        .add_systems(OnEnter(AppState::MainMenu), setup)
        .add_systems(Update, button::update_interaction::<MainMenuUiButtonEvent>);
}

fn setup(mut commands: Commands, typography: Res<Typography>) {
    commands.spawn((Camera2d::default(), StateScoped(AppState::MainMenu)));

    commands
        .spawn((
            Name::new("Main Menu UI"),
            StateScoped(AppState::MainMenu),
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
                Text::new("Bevy Tanks"),
                typography.title.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
            children.spawn(Node {
                height: Val::Px(64.0),
                ..default()
            });
            button::spawn_with_node(
                children,
                &typography,
                "Play Single Player",
                MainMenuUiButtonEvent::PlaySinglePlayer,
                Node {
                    width: Val::Px(250.0),
                    ..button::default_node()
                },
            );
            children.spawn(Node {
                height: Val::Px(12.0),
                ..default()
            });
            button::spawn_with_node(
                children,
                &typography,
                "Play Multiplayer",
                MainMenuUiButtonEvent::PlayMultiplayer,
                Node {
                    width: Val::Px(250.0),
                    ..button::default_node()
                },
            );
            if !cfg!(target_arch = "wasm32") {
                children.spawn(Node {
                    height: Val::Px(12.0),
                    ..default()
                });
                button::spawn_with_node(
                    children,
                    &typography,
                    "Quit",
                    MainMenuUiButtonEvent::Quit,
                    Node {
                        width: Val::Px(250.0),
                        ..button::default_node()
                    },
                );
            }
        });
}

fn observe_button(
    trigger: Trigger<MainMenuUiButtonEvent>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit_evw: EventWriter<AppExit>,
) {
    match trigger.event() {
        MainMenuUiButtonEvent::PlaySinglePlayer => {
            app_state.set(AppState::Game);
            commands.insert_resource(GameMode { multiplayer: false });
        }
        MainMenuUiButtonEvent::PlayMultiplayer => {
            app_state.set(AppState::Game);
            commands.insert_resource(GameMode { multiplayer: true });
        }
        MainMenuUiButtonEvent::Quit => {
            exit_evw.send(AppExit::Success);
        }
    }
}
