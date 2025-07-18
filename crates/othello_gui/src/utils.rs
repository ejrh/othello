use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::prelude::{EventWriter, KeyCode, Res};

pub fn close_on_esc(
    input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}
