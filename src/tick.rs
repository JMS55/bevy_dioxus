use crate::{
    apply_mutations::apply_mutations, deferred_system::DeferredSystemRegistry, DioxusUiRoot,
};
use bevy::{
    ecs::{
        entity::Entity,
        system::{CommandQueue, Commands},
        world::{Mut, World},
    },
    prelude::{Deref, DerefMut},
    utils::synccell::SyncCell,
};
use dioxus::core::{Element, Scope, ScopeState, VirtualDom};
use std::{mem, sync::Arc};

pub fn tick_dioxus_ui(world: &mut World) {
    let world_ptr: *mut World = world;
    let world_cell = world.as_unsafe_world_cell();
    let mut command_queue = CommandQueue::default();
    let mut commands = Commands::new_from_entities(&mut command_queue, world_cell.entities());

    for (root_entity, mut dioxus_ui_root) in unsafe {
        world_cell
            .world_mut()
            .query::<(Entity, &mut DioxusUiRoot)>()
            .iter_mut(world_cell.world_mut())
    } {
        let DioxusUiRoot {
            virtual_dom,
            hierarchy,
            element_id_to_bevy_ui_entity,
            templates,
            needs_rebuild,
        } = &mut *dioxus_ui_root;
        let virtual_dom = virtual_dom.get();

        virtual_dom
            .base_scope()
            .provide_context(EcsContext { world: world_ptr });

        // TODO: Handle events from winit
        // virtual_dom.handle_event(todo!(), todo!(), todo!(), todo!());

        if *needs_rebuild {
            apply_mutations(
                virtual_dom.rebuild(),
                hierarchy,
                element_id_to_bevy_ui_entity,
                templates,
                root_entity,
                &mut commands,
            );
            *needs_rebuild = false;
        }

        apply_mutations(
            virtual_dom.render_immediate(),
            hierarchy,
            element_id_to_bevy_ui_entity,
            templates,
            root_entity,
            &mut commands,
        );
    }

    command_queue.apply(world);

    for system_id in mem::take(&mut *world.resource_mut::<DeferredSystemRegistry>().run_queue) {
        let _ = world.run_system(system_id);
    }

    world.resource_scope(|world, mut system_registry: Mut<DeferredSystemRegistry>| {
        system_registry.ref_counts.retain(|system_id, ref_count| {
            let cleanup = Arc::strong_count(ref_count) == 1;
            if cleanup {
                world.remove_system(*system_id).unwrap();
            }
            !cleanup
        });
    });
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
