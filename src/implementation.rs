use super::DioxusUiRoot;
use bevy::{
    ecs::{
        entity::Entity,
        system::{CommandQueue, Commands, Resource},
        world::World,
    },
    prelude::{Deref, DerefMut},
    utils::synccell::SyncCell,
};
use dioxus_core::{Element, Mutations, Scope, ScopeState, VirtualDom};
use std::{cell::RefCell, mem::transmute, rc::Rc};

pub fn tick_dioxus_ui(world: &mut World) {
    let apply_mutations = |mutations: Mutations, root_entity: Entity| {
        todo!("Modify bevy_ui entities based on mutations");
    };

    let mut command_queue = CommandQueue::default();
    let ecs_context = unsafe {
        EcsContext {
            world_read_only: transmute(&world),
            commands: Rc::new(RefCell::new(Commands::new(
                transmute(&mut command_queue),
                transmute(&world),
            ))),
        }
    };

    for (root_entity, mut dioxus_ui_root) in
        world.query::<(Entity, &mut DioxusUiRoot)>().iter_mut(world)
    {
        dioxus_ui_root
            .virtual_dom
            .get()
            .base_scope()
            .provide_context(ecs_context.clone());

        if !dioxus_ui_root.initial_build {
            apply_mutations(dioxus_ui_root.virtual_dom.get().rebuild(), root_entity);
            dioxus_ui_root.initial_build = true;
        }

        // TODO: Handle events from winit
        // dioxus_ui_root
        //     .virtual_dom
        //     .get()
        //     .handle_event(todo!(), todo!(), todo!(), todo!());

        apply_mutations(
            dioxus_ui_root.virtual_dom.get().render_immediate(),
            root_entity,
        );
    }

    command_queue.apply(world);
}

#[derive(Clone)]
struct EcsContext {
    world_read_only: &'static World,
    commands: Rc<RefCell<Commands<'static, 'static>>>,
}

pub fn use_world<'a>(cx: &'a ScopeState) -> &'a World {
    cx.consume_context::<EcsContext>()
        .expect("Must be used from a dioxus component within a DioxusUiRoot bevy component")
        .world_read_only
}

pub fn use_res<'a, T: Resource>(cx: &'a ScopeState) -> &'a T {
    cx.consume_context::<EcsContext>()
        .expect("Must be used from a dioxus component within a DioxusUiRoot bevy component")
        .world_read_only
        .resource()
}

// TODO
// pub fn use_query<'a, Q: ReadOnlyWorldQuery, F: ReadOnlyWorldQuery>(
//     cx: &'a ScopeState,
// ) -> QueryIter<'a, '_, Q, F> {
//     let world = &mut cx
//         .consume_context::<EcsContext>()
//         .expect("Must be used from a dioxus component within a DioxusUiRoot bevy component")
//         .world_read_only;
//     world.query_filtered::<Q, F>().iter(&world)
// }

pub fn use_commands<'a>(cx: &'a ScopeState) -> Rc<RefCell<Commands<'a, 'a>>> {
    unsafe {
        transmute(Rc::clone(
            &cx.consume_context::<EcsContext>()
                .expect("Must be used from a dioxus component within a DioxusUiRoot bevy component")
                .commands,
        ))
    }
}

#[derive(Deref, DerefMut)]
pub struct VirtualDomUnsafe(pub SyncCell<VirtualDom>);
unsafe impl Send for VirtualDomUnsafe {}

impl VirtualDomUnsafe {
    pub fn new(root_component: fn(Scope) -> Element) -> Self {
        Self(SyncCell::new(VirtualDom::new(root_component)))
    }
}
