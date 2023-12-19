use bevy::ecs::{
    entity::Entity,
    event::{Events, ManualEventReader},
    system::Resource,
};
use bevy_mod_picking::events::{Click, Pointer};
use dioxus::core::ScopeState;
use std::{any::Any, rc::Rc};

// TODO: Other events

#[derive(Resource, Default)]
pub struct EventReaders {
    clicks: ManualEventReader<Pointer<Click>>,
}

impl EventReaders {
    pub fn get_dioxus_events(
        &mut self,
        clicks: &Events<Pointer<Click>>,
    ) -> Vec<(Entity, &'static str, Rc<dyn Any>)> {
        let mut events: Vec<(Entity, &'static str, Rc<dyn Any>)> = Vec::new();

        for event in self.clicks.read(clicks) {
            events.push((event.target, "click", Rc::new(())));
        }

        events
    }
}

pub fn is_supported_event(event: &str) -> bool {
    event == "click"
}

pub mod events {
    super::impl_event! [
        ();
        onclick
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
