use crate::{
    apply_mutations::apply_mutations,
    deferred_system::DeferredSystemRunQueue,
    ecs_hooks::EcsContext,
    events::{bubble_event, EventReaders},
    DioxusUiRoot, UiContext, UiRoot,
};
use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        world::{Mut, World},
    },
    utils::HashMap,
};
use std::{any::Any, mem, rc::Rc};

pub fn tick_dioxus_ui(world: &mut World) {
    run_deferred_systems(world);

    let ui_events = world.resource_scope(|world, mut event_readers: Mut<EventReaders>| {
        event_readers.read_events(
            world.resource(),
            world.resource(),
            world.resource(),
            world.resource(),
            world.resource(),
            world.resource(),
            world.resource(),
        )
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
    for mut system in mem::take(&mut *world.resource_mut::<DeferredSystemRunQueue>().run_queue) {
        system.initialize(world);
        let _ = system.run((), world);
    }
}

fn dispatch_ui_events(
    events: &Vec<(Entity, &str, Rc<dyn Any>, bool)>,
    ui_root: &mut UiRoot,
    world: &World,
) {
    for (mut target, name, data, bubbles) in events {
        if *bubbles {
            bubble_event(name, &mut target, world);
        }
        if let Some(target_element_id) = ui_root.bevy_ui_entity_to_element_id.get(&target) {
            ui_root
                .virtual_dom
                .handle_event(name, Rc::clone(data), *target_element_id, *bubbles);
        }
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
        let mutations = ui_root.virtual_dom.rebuild();
        world.resource_scope(|world, asset_server: Mut<AssetServer>| {
            apply_mutations(
                mutations,
                &mut ui_root.element_id_to_bevy_ui_entity,
                &mut ui_root.bevy_ui_entity_to_element_id,
                &mut ui_root.templates,
                root_entity,
                world,
                &asset_server,
            );
        });
        ui_root.needs_rebuild = false;
    }

    let mutations = ui_root.virtual_dom.render_immediate();
    world.resource_scope(|world, asset_server: Mut<AssetServer>| {
        apply_mutations(
            mutations,
            &mut ui_root.element_id_to_bevy_ui_entity,
            &mut ui_root.bevy_ui_entity_to_element_id,
            &mut ui_root.templates,
            root_entity,
            world,
            &asset_server,
        );
    });
}
