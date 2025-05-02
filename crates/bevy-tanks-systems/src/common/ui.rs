use bevy::prelude::*;
use bevy_tanks_data::*;

pub mod button {
    use super::*;

    pub const BORDER_COLOR: Color = Color::WHITE;
    pub const BORDER_HOVER_COLOR: Color = Color::WHITE;
    pub const BORDER_PRESSED_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
    pub const BACKGROUND_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BACKGROUND_HOVER_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
    pub const BACKGROUND_PRESSED_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

    pub fn default_node() -> Node {
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        }
    }

    pub fn spawn<'a>(
        commands: &'a mut ChildBuilder,
        typography: &Typography,
        text: &str,
        event: impl Event,
    ) -> EntityCommands<'a> {
        spawn_with_node(commands, typography, text, event, default_node())
    }

    pub fn spawn_with_node<'a>(
        commands: &'a mut ChildBuilder,
        typography: &Typography,
        text: &str,
        event: impl Event,
        node: Node,
    ) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn((
            Name::new(format!("{text} Button")),
            Button,
            EventEmitter(event),
            node,
            BorderColor(BORDER_COLOR),
            BorderRadius::MAX,
            BackgroundColor(BACKGROUND_COLOR),
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

    pub fn update_interaction<E: Event + Clone>(
        mut commands: Commands,
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
        for (interaction, event, mut background_color, mut border_color) in interaction_q.iter_mut()
        {
            let EventEmitter(event) = event;

            match *interaction {
                Interaction::Pressed => {
                    *background_color = BACKGROUND_PRESSED_COLOR.into();
                    *border_color = BORDER_PRESSED_COLOR.into();

                    commands.trigger(event.clone());
                }
                Interaction::Hovered => {
                    *background_color = BACKGROUND_HOVER_COLOR.into();
                    *border_color = BORDER_HOVER_COLOR.into();
                }
                Interaction::None => {
                    *background_color = BACKGROUND_COLOR.into();
                    *border_color = BORDER_COLOR.into();
                }
            }
        }
    }
}
