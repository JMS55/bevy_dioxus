use bevy::{
    ecs::{
        entity::Entity,
        event::{Event, EventWriter, Events, ManualEventReader},
        system::{Local, Query, Resource},
    },
    ui::RelativeCursorPosition,
    utils::EntityHashSet,
};
use bevy_mod_picking::events::{Click, Out, Over, Pointer};
use dioxus::core::ScopeState;
use std::{any::Any, mem, rc::Rc};

// TODO: Other events

#[derive(Resource, Default)]
pub struct EventReaders {
    click: ManualEventReader<Pointer<Click>>,
    mouse_over: ManualEventReader<Pointer<Over>>,
    mouse_out: ManualEventReader<Pointer<Out>>,
    mouse_enter: ManualEventReader<MouseEnter>,
    mouse_exit: ManualEventReader<MouseExit>,
}

impl EventReaders {
    pub fn get_dioxus_events(
        &mut self,
        click: &Events<Pointer<Click>>,
        mouse_over: &Events<Pointer<Over>>,
        mouse_out: &Events<Pointer<Out>>,
        mouse_enter: &Events<MouseEnter>,
        mouse_exit: &Events<MouseExit>,
    ) -> Vec<(Entity, &'static str, Rc<dyn Any>, bool)> {
        let mut events: Vec<(Entity, &'static str, Rc<dyn Any>, bool)> = Vec::new();
        for event in self.click.read(click) {
            events.push((event.target, "click", Rc::new(()), true));
        }
        for event in self.mouse_over.read(mouse_over) {
            events.push((event.target, "mouse_over", Rc::new(()), false));
        }
        for event in self.mouse_out.read(mouse_out) {
            events.push((event.target, "mouse_out", Rc::new(()), false));
        }
        for event in self.mouse_enter.read(mouse_enter) {
            events.push((event.target, "mouse_enter", Rc::new(()), false));
        }
        for event in self.mouse_exit.read(mouse_exit) {
            events.push((event.target, "mouse_exit", Rc::new(()), false));
        }
        events
    }
}

pub fn is_supported_event(event: &str) -> bool {
    match event {
        "click" => true,
        "mouse_over" => true,
        "mouse_out" => true,
        "mouse_enter" => true,
        "mouse_exit" => true,
        _ => false,
    }
}

pub mod events {
    super::impl_event! [
        ();
        onclick
        onmouse_over
        onmouse_out
        onmouse_enter
        onmouse_exit
    ];
}

pub fn generate_mouse_enter_leave_events(
    entities: Query<(Entity, &RelativeCursorPosition)>,
    mut previous_over: Local<EntityHashSet<Entity>>,
    mut over: Local<EntityHashSet<Entity>>,
    mut enter: EventWriter<MouseEnter>,
    mut leave: EventWriter<MouseExit>,
) {
    mem::swap::<EntityHashSet<Entity>>(&mut previous_over, &mut over);

    over.clear();
    for (entity, relative_cursor_position) in &entities {
        if relative_cursor_position.mouse_over() {
            over.insert(entity);
        }
    }

    enter.send_batch(
        over.iter()
            .copied()
            .filter(|entity| !previous_over.contains(entity))
            .map(|target| MouseEnter { target }),
    );

    leave.send_batch(
        previous_over
            .iter()
            .copied()
            .filter(|entity| !over.contains(entity))
            .map(|target| MouseExit { target }),
    );
}

#[derive(Event)]
pub struct MouseEnter {
    target: Entity,
}

#[derive(Event)]
pub struct MouseExit {
    target: Entity,
}

// ----------------------------------------------------------------------------

pub trait EventReturn<P>: Sized {
    fn spawn(self, _cx: &ScopeState) {}
}

impl EventReturn<()> for () {}

macro_rules! impl_event {
    (
        $data:ty;
        $(
            $( #[$attr:meta] )*
            $name:ident
        )*
    ) => {
        $(
            $( #[$attr] )*
            #[inline]
            pub fn $name<'a, E: crate::events::EventReturn<T>, T>(_cx: &'a dioxus::core::ScopeState, mut _f: impl FnMut(dioxus::core::Event<$data>) -> E + 'a) -> dioxus::core::Attribute<'a> {
                dioxus::core::Attribute::new(
                    stringify!($name),
                    _cx.listener(move |e: dioxus::core::Event<$data>| {
                        _f(e).spawn(_cx);
                    }),
                    None,
                    false,
                )
            }
        )*
    };

}
pub(self) use impl_event;
