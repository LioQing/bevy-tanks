use bevy::prelude::*;
use bevy_tanks_data::*;

pub fn handle_game_over_enter(
    mut commands: Commands,
    typography: Res<Typography>,
    button_ui_prefab: Res<ButtonUiPrefab>,
    game_over_ui_prefab: Res<GameOverUiPrefab>,
    tank: Single<&TankController, With<TankAlive>>,
) {
    game_over_ui_prefab.spawn(&mut commands, &button_ui_prefab, &typography, tank.label);
}

pub fn update_interaction<E: Event + Clone>(
    mut evw: EventWriter<E>,
    mut interaction_q: Query<
        (
            &Interaction,
            &EventEmitter<E>,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, event, mut background_color, mut border_color) in interaction_q.iter_mut() {
        let EventEmitter(event) = event;

        match *interaction {
            Interaction::Pressed => {
                *background_color = ButtonUiPrefab::BACKGROUND_PRESSED_COLOR.into();
                *border_color = ButtonUiPrefab::BORDER_PRESSED_COLOR.into();

                evw.send(event.clone());
            }
            Interaction::Hovered => {
                *background_color = ButtonUiPrefab::BACKGROUND_HOVER_COLOR.into();
                *border_color = ButtonUiPrefab::BORDER_HOVER_COLOR.into();
            }
            Interaction::None => {
                *background_color = ButtonUiPrefab::BACKGROUND_COLOR.into();
                *border_color = ButtonUiPrefab::BORDER_COLOR.into();
            }
        }
    }
}

pub fn handle_game_over_button_event(
    mut evr: EventReader<GameOverUiButtonEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut ext_evw: EventWriter<AppExit>,
) {
    for event in evr.read() {
        match event {
            GameOverUiButtonEvent::PlayAgain => {
                game_state.set(GameState::Playing);
            }
            GameOverUiButtonEvent::Quit => {
                ext_evw.send(AppExit::Success);
            }
        }
    }
}
