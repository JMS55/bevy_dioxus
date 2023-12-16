use crate::{
    apply_mutations::{apply_mutations, BevyTemplate},
    deferred_system::DeferredSystemRegistry,
    events::EventReaders,
    hooks::{EcsContext, EcsSubscriptions},
    DioxusUiRoot,
};
use bevy::{
    ecs::{
        entity::Entity,
        world::{Mut, World},
    },
    hierarchy::Parent,
    prelude::{Deref, DerefMut},
    utils::{hashbrown::HashMap, synccell::SyncCell, EntityHashMap},
};
use dioxus::core::{Element, ElementId, Scope, VirtualDom};
use std::{any::Any, mem, rc::Rc, sync::Arc};

pub fn tick_dioxus_ui(world: &mut World) {
    run_deferred_systems(world);

    let world_ptr: *mut World = world;
    let world_cell = world.as_unsafe_world_cell();

    let ui_events = unsafe {
        world_cell
            .get_resource_mut::<EventReaders>()
            .unwrap()
            .get_dioxus_events(world_cell.get_resource().unwrap())
    };

    for (root_entity, mut dioxus_ui_root) in unsafe {
        world_cell
            .world_mut()
            .query::<(Entity, &mut DioxusUiRoot)>()
            .iter_mut(world_cell.world_mut())
    } {
        let DioxusUiRoot {
            virtual_dom,
            element_id_to_bevy_ui_entity,
            bevy_ui_entity_to_element_id,
            templates,
            needs_rebuild,
        } = &mut *dioxus_ui_root;
        let virtual_dom = virtual_dom.get();

        dispatch_ui_events(
            &ui_events,
            bevy_ui_entity_to_element_id,
            virtual_dom,
            unsafe { world_cell.world() },
        );

        schedule_ui_renders_from_ecs_subscriptions(
            virtual_dom,
            unsafe { world_cell.get_resource::<EcsSubscriptions>().unwrap() },
            unsafe { world_cell.world() },
        );

        render_ui(
            virtual_dom,
            needs_rebuild,
            element_id_to_bevy_ui_entity,
            bevy_ui_entity_to_element_id,
            templates,
            root_entity,
            unsafe { world_cell.world_mut() },
            world_ptr,
        );
    }
}

fn run_deferred_systems(world: &mut World) {
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

fn schedule_ui_renders_from_ecs_subscriptions(
    virtual_dom: &mut VirtualDom,
    ecs_subscriptions: &EcsSubscriptions,
    world: &World,
) {
    let schedule_update = virtual_dom.base_scope().schedule_update_any();
    for scope_id in &*ecs_subscriptions.world_and_queries {
        schedule_update(*scope_id);
    }
    for (resource_id, scope_ids) in &*ecs_subscriptions.resources {
        if world.is_resource_changed_by_id(*resource_id) {
            for scope_id in scope_ids {
                schedule_update(*scope_id);
            }
        }
    }
}

fn dispatch_ui_events(
    events: &Vec<(Entity, &str, Rc<dyn Any>)>,
    bevy_ui_entity_to_element_id: &mut EntityHashMap<Entity, ElementId>,
    virtual_dom: &mut VirtualDom,
    world: &World,
) {
    for (mut target, name, data) in events {
        let mut target_element_id = bevy_ui_entity_to_element_id.get(&target);
        while target_element_id.is_none() {
            target = world.entity(target).get::<Parent>().unwrap().get();
            target_element_id = bevy_ui_entity_to_element_id.get(&target);
        }
        virtual_dom.handle_event(name, Rc::clone(data), *target_element_id.unwrap(), true);
    }
}

fn render_ui(
    virtual_dom: &mut VirtualDom,
    needs_rebuild: &mut bool,
    element_id_to_bevy_ui_entity: &mut HashMap<ElementId, Entity>,
    bevy_ui_entity_to_element_id: &mut EntityHashMap<Entity, ElementId>,
    templates: &mut HashMap<String, BevyTemplate>,
    root_entity: Entity,
    world: &mut World,
    world_ptr: *mut World,
) {
    virtual_dom
        .base_scope()
        .provide_context(EcsContext { world: world_ptr });

    if *needs_rebuild {
        apply_mutations(
            virtual_dom.rebuild(),
            element_id_to_bevy_ui_entity,
            bevy_ui_entity_to_element_id,
            templates,
            root_entity,
            world,
        );
        *needs_rebuild = false;
    }

    apply_mutations(
        virtual_dom.render_immediate(),
        element_id_to_bevy_ui_entity,
        bevy_ui_entity_to_element_id,
        templates,
        root_entity,
        world,
    );
}

#[derive(Deref, DerefMut)]
pub struct VirtualDomUnsafe(pub SyncCell<VirtualDom>);
unsafe impl Send for VirtualDomUnsafe {}

impl VirtualDomUnsafe {
    pub fn new(root_component: fn(Scope) -> Element) -> Self {
        Self(SyncCell::new(VirtualDom::new(root_component)))
    }
}
