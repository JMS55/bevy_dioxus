use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventWriter, Events, ManualEventReader},
        system::{Local, Query, Resource},
        world::World,
    },
    hierarchy::Parent,
    prelude::EntityWorldMut,
    ui::RelativeCursorPosition,
    utils::EntityHashSet,
};
use bevy_mod_picking::events::{Click, Down, Out, Over, Pointer, Up};
use dioxus::core::ScopeState;
use std::{any::Any, mem, rc::Rc};

// TODO: Other events
pub mod events {
    use bevy_mod_picking::pointer::PointerButton;

    super::impl_event! [
        ();
        onmouse_over
        onmouse_out
        onmouse_enter
        onmouse_exit
    ];

    super::impl_event! [
        PointerButton;
        onclick
        onclick_down
        onclick_up
    ];
}

#[derive(Resource, Default)]
pub struct EventReaders {
    click: ManualEventReader<Pointer<Click>>,
    click_down: ManualEventReader<Pointer<Down>>,
    click_up: ManualEventReader<Pointer<Up>>,
    mouse_over: ManualEventReader<Pointer<Over>>,
    mouse_out: ManualEventReader<Pointer<Out>>,
    mouse_enter: ManualEventReader<MouseEnter>,
    mouse_exit: ManualEventReader<MouseExit>,
}

impl EventReaders {
    pub fn read_events(
        &mut self,
        click: &Events<Pointer<Click>>,
        click_down: &Events<Pointer<Down>>,
        click_up: &Events<Pointer<Up>>,
        mouse_over: &Events<Pointer<Over>>,
        mouse_out: &Events<Pointer<Out>>,
        mouse_enter: &Events<MouseEnter>,
        mouse_exit: &Events<MouseExit>,
    ) -> Vec<(Entity, &'static str, Rc<dyn Any>, bool)> {
        let mut events: Vec<(Entity, &'static str, Rc<dyn Any>, bool)> = Vec::new();
        for event in self.click.read(click) {
            events.push((event.target, "click", Rc::new(event.button), true));
        }
        for event in self.click_down.read(click_down) {
            events.push((event.target, "click_down", Rc::new(event.button), true));
        }
        for event in self.click_up.read(click_up) {
            events.push((event.target, "click_up", Rc::new(event.button), true));
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

pub fn insert_event_listener(name: &str, mut entity: EntityWorldMut<'_>) {
    match name {
        "click" => entity.insert(HasClickEventListener),
        "click_down" => entity.insert(HasClickDownEventListener),
        "click_up" => entity.insert(HasClickUpEventListener),
        "mouse_over" => &mut entity,
        "mouse_out" => &mut entity,
        "mouse_enter" => entity.insert((
            HasMouseEnterEventListener,
            RelativeCursorPosition::default(),
        )),
        "mouse_exit" => {
            entity.insert((HasMouseExitEventListener, RelativeCursorPosition::default()))
        }
        _ => panic!("Encountered unsupported bevy_dioxus event `{name}`."),
    };
}

pub fn remove_event_listener(name: &str, mut entity: EntityWorldMut<'_>) {
    match name {
        "click" => entity.remove::<HasClickEventListener>(),
        "click_down" => entity.remove::<HasClickDownEventListener>(),
        "click_up" => entity.remove::<HasClickUpEventListener>(),
        "mouse_over" => &mut entity,
        "mouse_out" => &mut entity,
        "mouse_enter" => {
            entity.remove::<HasMouseEnterEventListener>();
            if !entity.contains::<HasMouseExitEventListener>() {
                entity.remove::<RelativeCursorPosition>();
            }
            &mut entity
        }
        "mouse_exit" => {
            entity.remove::<HasMouseExitEventListener>();
            if !entity.contains::<HasMouseEnterEventListener>() {
                entity.remove::<RelativeCursorPosition>();
            }
            &mut entity
        }
        _ => unreachable!(),
    };
}

#[derive(Component)]
pub struct HasClickEventListener;

#[derive(Component)]
pub struct HasClickDownEventListener;

#[derive(Component)]
pub struct HasClickUpEventListener;

#[derive(Component)]
pub struct HasMouseEnterEventListener;

#[derive(Component)]
pub struct HasMouseExitEventListener;

// ----------------------------------------------------------------------------

pub fn bubble_event(event_name: &str, target_entity: &mut Entity, world: &World) {
    match event_name {
        "click" => bubble_event_helper::<HasClickEventListener>(target_entity, world),
        "click_down" => bubble_event_helper::<HasClickDownEventListener>(target_entity, world),
        "click_up" => bubble_event_helper::<HasClickUpEventListener>(target_entity, world),
        _ => unreachable!(),
    };
}

fn bubble_event_helper<T: Component>(target_entity: &mut Entity, world: &World) {
    while !world.entity(*target_entity).contains::<T>() {
        *target_entity = match world.entity(*target_entity).get::<Parent>() {
            Some(parent) => parent.get(),
            None => return,
        };
    }
}

// ----------------------------------------------------------------------------

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
