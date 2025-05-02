use bevy::prelude::*;
use bevy_tanks_data::*;

use crate::common::ui::button;

pub fn plugin(app: &mut App) {
    app.add_state_scoped_event::<HowToPlayMenuUiButtonEvent>(MenuState::HowToPlay)
        .add_observer(observe_button)
        .add_systems(OnEnter(MenuState::HowToPlay), setup)
        .add_systems(
            Update,
            button::update_interaction::<HowToPlayMenuUiButtonEvent>,
        );
}

fn setup(mut commands: Commands, typography: Res<Typography>) {
    commands
        .spawn((
            Name::new("How To Play Menu UI"),
            StateScoped(MenuState::HowToPlay),
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                Text::new("How To Play"),
                typography.subtitle.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
            children.spawn(Node {
                height: Val::Px(32.0),
                ..default()
            });
            children.spawn((
                Text::new(
                    "Player 1:\n\
                    - WASD to move\n\
                    - Space to fire\n\
                    \n\
                    Player 2 (Only in Multiplayer):\n\
                    - Arrow keys to move\n\
                    - Enter to fire",
                ),
                typography.body.clone(),
                TextLayout::new_with_justify(JustifyText::Left),
            ));
            children.spawn(Node {
                height: Val::Px(32.0),
                ..default()
            });
            button::spawn(
                children,
                &typography,
                "Back",
                HowToPlayMenuUiButtonEvent::Ok,
            );
        });
}

fn observe_button(
    trigger: Trigger<HowToPlayMenuUiButtonEvent>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    match trigger.event() {
        HowToPlayMenuUiButtonEvent::Ok => {
            menu_state.set(MenuState::Main);
        }
    }
}
