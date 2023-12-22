use bevy::{
    app::{App, Startup},
    core::{DebugName, Name},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{entity::Entity, query::Without, system::Commands, world::World},
    ui::{node_bundles::NodeBundle, Node},
    DefaultPlugins,
};
use bevy_dioxus::{colors::*, prelude::*};
use bevy_mod_picking::DefaultPickingPlugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DioxusUiPlugin, DefaultPickingPlugins))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(DioxusUiBundle {
                dioxus_ui_root: DioxusUiRoot(Editor),
                node_bundle: NodeBundle::default(),
            });
            commands.spawn((Camera2dBundle::default(), Name::new("Camera")));
        })
        .run();
}

#[component]
fn Editor(cx: Scope) -> Element {
    // TODO: When selected entity is despawned, need to reset this to None
    let selected_entity = use_state(cx, || Option::<Entity>::None);

    render! {
        node {
            width: "100vw",
            height: "100vh",
            justify_content: "space_between",
            SceneTree { selected_entity: selected_entity }
            EntityInspector { selected_entity: selected_entity }
        }
    }
}

#[component]
fn SceneTree<'a>(cx: Scope, selected_entity: &'a UseState<Option<Entity>>) -> Element {
    let entities = use_query_filtered::<(Entity, DebugName), Without<Node>>(cx);
    let entities = entities.query();
    let mut entities = entities.into_iter().collect::<Vec<_>>();
    entities.sort_by_key(|(entity, _)| *entity);

    let spawn_entity = use_system(cx, |world: &mut World| {
        world.spawn_empty();
    });

    let (spawn_entity_hovered, enter, exit) = use_hover(cx);

    render! {
        node {
            onclick: move |_| selected_entity.set(None),
            flex_direction: "column",
            if entities.is_empty() {
                rsx! { "No entities exist" }
            } else {
                rsx! {
                    for (entity, name) in entities {
                        node {
                            onclick: move |event| {
                                if Some(entity) == ***selected_entity {
                                    selected_entity.set(None);
                                } else {
                                    selected_entity.set(Some(entity));
                                }
                                event.stop_propagation();
                            },
                            padding: "8",
                            background_color: if Some(entity) == ***selected_entity { INDIGO_600 } else { NEUTRAL_800 },
                            match name.name {
                                Some(name) => format!("{name}"),
                                _ => format!("Entity ({:?})", name.entity)
                            }
                        }
                    }
                }
            }
            node {
                padding: "8",
                background_color: if spawn_entity_hovered { NEUTRAL_600 } else { NEUTRAL_800 },
                text {
                    onclick: move |_| spawn_entity(),
                    onmouse_enter: enter,
                    onmouse_exit: exit,
                    text: "Spawn Entity",
                    text_size: "18"
                }
            }
        }
    }
}

#[component]
fn EntityInspector<'a>(cx: Scope, selected_entity: &'a UseState<Option<Entity>>) -> Element {
    let world = use_world(cx);

    let components = if let Some(selected_entity) = selected_entity.get() {
        let entity_ref = world.get_entity(*selected_entity).unwrap();
        let archetype = entity_ref.archetype();
        let mut components = archetype
            .components()
            .map(|component_id| {
                let info = world.components().get_info(component_id).unwrap();
                let name = info.name();

                (name, component_id, info.type_id(), info.layout().size())
            })
            .collect::<Vec<_>>();
        components.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));
        components
    } else {
        vec![]
    };

    render! {
        if selected_entity.is_none() {
            rsx! {
                "Select an entity to view its components"
            }
        } else {
            rsx! {
                node {
                    flex_direction: "column",
                    for (name, _component_id, _type_id, _size) in components {
                        node {
                            padding: "8",
                            background_color: NEUTRAL_800,
                            "Component: {name}"
                        }
                    }
                }
            }
        }
    }
}
