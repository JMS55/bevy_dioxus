use crate::{
    apply_mutations::apply_mutations,
    deferred_system::DeferredSystemRegistry,
    events::EventReaders,
    hooks::{EcsContext, EcsSubscriptions},
    DioxusUiRoot,
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
use dioxus::core::{Element, Scope, VirtualDom};
use std::{mem, rc::Rc, sync::Arc};

pub fn tick_dioxus_ui(world: &mut World) {
    let world_ptr: *mut World = world;
    let world_cell = world.as_unsafe_world_cell();
    let ecs_subscriptions = unsafe { world_cell.get_resource::<EcsSubscriptions>().unwrap() };
    let events = unsafe {
        world_cell
            .get_resource_mut::<EventReaders>()
            .unwrap()
            .get_dioxus_events(world_cell.get_resource().unwrap())
    };
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
            parent_to_children,
            children_to_parent,
            element_id_to_bevy_ui_entity,
            bevy_ui_entity_to_element_id,
            templates,
            needs_rebuild,
        } = &mut *dioxus_ui_root;
        let virtual_dom = virtual_dom.get();

        let schedule_update = virtual_dom.base_scope().schedule_update_any();
        for scope_id in &*ecs_subscriptions.world_and_queries {
            schedule_update(*scope_id);
        }
        for (resource_id, scope_ids) in &*ecs_subscriptions.resources {
            if unsafe { world_cell.world() }.is_resource_changed_by_id(*resource_id) {
                for scope_id in scope_ids {
                    schedule_update(*scope_id);
                }
            }
        }

        virtual_dom
            .base_scope()
            .provide_context(EcsContext { world: world_ptr });

        for (mut target, name, data) in &events {
            let mut target_element_id = bevy_ui_entity_to_element_id.get(&target);
            while target_element_id.is_none() {
                target = children_to_parent[&target];
                target_element_id = bevy_ui_entity_to_element_id.get(&target);
            }
            virtual_dom.handle_event(name, Rc::clone(data), *target_element_id.unwrap(), true);
        }

        if *needs_rebuild {
            apply_mutations(
                virtual_dom.rebuild(),
                parent_to_children,
                children_to_parent,
                element_id_to_bevy_ui_entity,
                bevy_ui_entity_to_element_id,
                templates,
                root_entity,
                &mut commands,
            );
            *needs_rebuild = false;
        }

        apply_mutations(
            virtual_dom.render_immediate(),
            parent_to_children,
            children_to_parent,
            element_id_to_bevy_ui_entity,
            bevy_ui_entity_to_element_id,
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

#[derive(Deref, DerefMut)]
pub struct VirtualDomUnsafe(pub SyncCell<VirtualDom>);
unsafe impl Send for VirtualDomUnsafe {}

impl VirtualDomUnsafe {
    pub fn new(root_component: fn(Scope) -> Element) -> Self {
        Self(SyncCell::new(VirtualDom::new(root_component)))
    }
}
