use amethyst::{
    core::{
        shrev::{EventChannel, ReaderId},
        EventReader,
    },
    derive::EventReader,
    ecs::{Read, SystemData, World},
    input::{BindingTypes, InputEvent, StringBindings},
    ui::UiEvent,
    winit::Event,
};

use super::connection_event::ConnectionEvent;

/// The enum holding the different types of event that can be received in a `State` in the
/// `handle_event` method.
#[derive(Debug, Clone, EventReader)]
#[reader(ExtendedStateEventReader)]
pub enum ExtendedStateEvent<T = StringBindings>
where
    T: BindingTypes + Clone,
{
    /// Events sent by the winit window.
    Window(Event),
    /// Events sent by the ui system.
    Ui(UiEvent),
    /// Events sent by the input system.
    Input(InputEvent<T>),
    /// Events about connection
    Connection(ConnectionEvent),
}
