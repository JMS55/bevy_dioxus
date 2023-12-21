use crate::{
    apply_mutations::apply_mutations, deferred_system::DeferredSystemRegistry,
    events::EventReaders, hooks::EcsContext, DioxusUiRoot, UiContext, UiRoot,
};
use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        world::{Mut, World},
    },
    hierarchy::Parent,
    utils::HashMap,
};
use std::{any::Any, mem, rc::Rc, sync::Arc};

pub fn tick_dioxus_ui(world: &mut World) {
    run_deferred_systems(world);

    let ui_events = world.resource_scope(|world, mut event_readers: Mut<EventReaders>| {
        event_readers.get_dioxus_events(world.resource())
    });

    let root_entities: HashMap<Entity, DioxusUiRoot> = world
        .query::<(Entity, &DioxusUiRoot)>()
        .iter(world)
        .map(|(entity, ui_root)| (entity, *ui_root))
        .collect();
    let mut ui_roots = mem::take(&mut world.non_send_resource_mut::<UiContext>().roots);

    for (root_entity, dioxus_ui_root) in root_entities {
        let mut ui_root = ui_roots
            .remove(&(root_entity, dioxus_ui_root))
            .unwrap_or_else(|| UiRoot::new(dioxus_ui_root));

        dispatch_ui_events(&ui_events, &mut ui_root, world);

        schedule_ui_renders_from_ecs_subscriptions(&mut ui_root, world);

        render_ui(root_entity, &mut ui_root, world);

        world
            .non_send_resource_mut::<UiContext>()
            .roots
            .insert((root_entity, dioxus_ui_root), ui_root);
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
    ui_root: &mut UiRoot,
    world: &World,
) {
    for (mut target, name, data) in events {
        let mut target_element_id = ui_root.bevy_ui_entity_to_element_id.get(&target).copied();
        while target_element_id.is_none() || world.entity(target).contains::<IntrinsicTextNode>() {
            target = world.entity(target).get::<Parent>().unwrap().get();
            target_element_id = ui_root.bevy_ui_entity_to_element_id.get(&target).copied();
        }

        ui_root
            .virtual_dom
            .handle_event(name, Rc::clone(data), target_element_id.unwrap(), true);
    }
}

fn schedule_ui_renders_from_ecs_subscriptions(ui_root: &mut UiRoot, world: &World) {
    let ecs_subscriptions = &world.non_send_resource::<UiContext>().subscriptions;

    for scope_id in &*ecs_subscriptions.world_and_queries {
        ui_root.virtual_dom.mark_dirty(*scope_id);
    }

    for (resource_id, scope_ids) in &*ecs_subscriptions.resources {
        if world.is_resource_changed_by_id(*resource_id) {
            for scope_id in scope_ids {
                ui_root.virtual_dom.mark_dirty(*scope_id);
            }
        }
    }
}

fn render_ui(root_entity: Entity, ui_root: &mut UiRoot, world: &mut World) {
    ui_root
        .virtual_dom
        .base_scope()
        .provide_context(EcsContext { world });

    #[cfg(feature = "hot_reload")]
    crate::hot_reload::update_templates(world, &mut ui_root.virtual_dom);

    if ui_root.needs_rebuild {
        apply_mutations(
            ui_root.virtual_dom.rebuild(),
            &mut ui_root.element_id_to_bevy_ui_entity,
            &mut ui_root.bevy_ui_entity_to_element_id,
            &mut ui_root.templates,
            root_entity,
            world,
        );
        ui_root.needs_rebuild = false;
    }

    apply_mutations(
        ui_root.virtual_dom.render_immediate(),
        &mut ui_root.element_id_to_bevy_ui_entity,
        &mut ui_root.bevy_ui_entity_to_element_id,
        &mut ui_root.templates,
        root_entity,
        world,
    );
}

#[derive(Component)]
pub struct IntrinsicTextNode;
