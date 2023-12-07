use crate::{
    apply_mutations::apply_mutations, deferred_system::DeferredSystemRunQueue, DioxusUiRoot,
};
use bevy::{
    ecs::{entity::Entity, world::World},
    prelude::{Deref, DerefMut},
    utils::synccell::SyncCell,
};
use dioxus_core::{Element, Scope, ScopeState, VirtualDom};
use std::mem;

// TODO: This is not sound. Can't borrow the world while iterating over DioxusUiRoots.
pub fn tick_dioxus_ui(world: &mut World) {
    let world_ptr: *mut World = world;
    let world_cell = world.as_unsafe_world_cell();

    for (root_entity, mut dioxus_ui_root) in unsafe {
        world_cell
            .world_mut()
            .query::<(Entity, &mut DioxusUiRoot)>()
            .iter_mut(world_cell.world_mut())
    } {
        let DioxusUiRoot {
            virtual_dom,
            needs_rebuild,
            element_id_to_bevy_ui_entity,
            templates,
        } = &mut *dioxus_ui_root;
        let virtual_dom = virtual_dom.get();

        virtual_dom
            .base_scope()
            .provide_context(EcsContext { world: world_ptr });

        if *needs_rebuild {
            apply_mutations(
                virtual_dom.rebuild(),
                element_id_to_bevy_ui_entity,
                templates,
                root_entity,
                unsafe { world_cell.world_mut() },
            );
            *needs_rebuild = false;
        }

        // TODO: Handle events from winit
        // virtual_dom.handle_event(todo!(), todo!(), todo!(), todo!());

        apply_mutations(
            virtual_dom.render_immediate(),
            element_id_to_bevy_ui_entity,
            templates,
            root_entity,
            unsafe { world_cell.world_mut() },
        );
    }

    for system_id in mem::take(&mut *world.resource_mut::<DeferredSystemRunQueue>().0) {
        world.run_system(system_id).unwrap();
    }
}

#[derive(Clone)]
pub(crate) struct EcsContext {
    world: *mut World,
}

impl EcsContext {
    pub fn get_world(cx: &ScopeState) -> &mut World {
        unsafe {
            &mut *cx
                .consume_context::<EcsContext>()
                .expect("Must be used from a dioxus component within a DioxusUiRoot bevy component")
                .world
        }
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
