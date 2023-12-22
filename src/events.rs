use bevy::ecs::{
    entity::Entity,
    event::{Events, ManualEventReader},
    system::Resource,
};
use bevy_mod_picking::events::{Click, Out, Over, Pointer};
use dioxus::core::ScopeState;
use std::{any::Any, rc::Rc};

// TODO: Other events

#[derive(Resource, Default)]
pub struct EventReaders {
    click: ManualEventReader<Pointer<Click>>,
    mouse_enter: ManualEventReader<Pointer<Over>>,
    mouse_exit: ManualEventReader<Pointer<Out>>,
}

impl EventReaders {
    pub fn get_dioxus_events(
        &mut self,
        click: &Events<Pointer<Click>>,
        mouse_enter: &Events<Pointer<Over>>,
        mouse_exit: &Events<Pointer<Out>>,
    ) -> Vec<(Entity, &'static str, Rc<dyn Any>)> {
        let mut events: Vec<(Entity, &'static str, Rc<dyn Any>)> = Vec::new();
        for event in self.click.read(click) {
            events.push((event.target, "click", Rc::new(())));
        }
        for event in self.mouse_enter.read(mouse_enter) {
            events.push((event.target, "mouse_enter", Rc::new(())));
        }
        for event in self.mouse_exit.read(mouse_exit) {
            events.push((event.target, "mouse_exit", Rc::new(())));
        }
        events
    }
}

pub fn is_supported_event(event: &str) -> bool {
    match event {
        "click" => true,
        "mouse_enter" => true,
        "mouse_exit" => true,
        _ => false,
    }
}

pub mod events {
    super::impl_event! [
        ();
        onclick
        onmouse_enter
        onmouse_exit
    ];
}

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
