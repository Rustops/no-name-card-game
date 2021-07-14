use amethyst::{
    core::SystemDesc,
    ecs::{System, SystemData, Write},
    shred::World,
    shrev::{EventChannel, ReaderId},
    ui::UiEvent,
};

/// This shows how to handle UI events. This is the same as in the 'ui' example.
pub struct UiEventHandlerSystem {
    reader_id: ReaderId<UiEvent>,
}

impl UiEventHandlerSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'s> System<'s> for UiEventHandlerSystem {
    type SystemData = Write<'s, EventChannel<UiEvent>>;

    fn run(&mut self, events: Self::SystemData) {
        // Reader id was just initialized above if empty
        for ev in events.read(&mut self.reader_id) {
            log::info!("[SYSTEM] You just interacted with an ui element: {:?}", ev);
        }
    }
}

pub struct UiEventHandlerSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, UiEventHandlerSystem> for UiEventHandlerSystemDesc {
    fn build(self, world: &mut World) -> UiEventHandlerSystem {
        let mut event_channel = <Write<EventChannel<UiEvent>>>::fetch(world);
        let reader_id = event_channel.register_reader();

        UiEventHandlerSystem::new(reader_id)
    }
}
