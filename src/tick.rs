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
        query::With,
        world::{unsafe_world_cell::UnsafeWorldCell, Mut, World},
    },
    hierarchy::Parent,
    prelude::{Deref, DerefMut},
    utils::synccell::SyncCell,
};
use dioxus::core::{Element, Scope, VirtualDom};
use std::{any::Any, mem, rc::Rc, sync::Arc};

pub fn tick_dioxus_ui(world: &mut World) {
    run_deferred_systems(world);

    let ui_events = world.resource_scope(|world, mut event_readers: Mut<EventReaders>| {
        event_readers.get_dioxus_events(world.resource())
    });
    let root_entities: Vec<Entity> = world
        .query_filtered::<Entity, With<DioxusUiRoot>>()
        .iter(world)
        .collect();

    for root_entity in root_entities {
        dispatch_ui_events(&ui_events, root_entity, world.as_unsafe_world_cell());

        schedule_ui_renders_from_ecs_subscriptions(root_entity, world);

        render_ui(root_entity, world);
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

fn dispatch_ui_events(
    events: &Vec<(Entity, &str, Rc<dyn Any>)>,
    root_entity: Entity,
    world_cell: UnsafeWorldCell,
) {
    let mut ui_root = unsafe {
        world_cell
            .get_entity(root_entity)
            .unwrap()
            .get_mut::<DioxusUiRoot>()
            .unwrap()
    };

    let get_parent = |entity| unsafe {
        world_cell
            .get_entity(entity)
            .unwrap()
            .get::<Parent>()
            .unwrap()
            .get()
    };

    for (mut target, name, data) in events {
        let mut target_element_id = ui_root.bevy_ui_entity_to_element_id.get(&target).copied();
        while target_element_id.is_none() {
            target = get_parent(target);
            target_element_id = ui_root.bevy_ui_entity_to_element_id.get(&target).copied();
        }

        ui_root.virtual_dom.get().handle_event(
            name,
            Rc::clone(data),
            target_element_id.unwrap(),
            true,
        );
    }
}

fn schedule_ui_renders_from_ecs_subscriptions(root_entity: Entity, world: &mut World) {
    let schedule_update = world
        .get_mut::<DioxusUiRoot>(root_entity)
        .unwrap()
        .virtual_dom
        .get()
        .base_scope()
        .schedule_update_any();

    let ecs_subscriptions = world.resource::<EcsSubscriptions>();

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

fn render_ui(root_entity: Entity, world: &mut World) {
    virtual_dom
        .base_scope()
        .provide_context(EcsContext { world });

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
