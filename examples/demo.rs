use bevy::{
    app::{App, Startup},
    core::{DebugName, Name},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{entity::Entity, query::Without, system::Commands, world::World},
    ui::{node_bundles::NodeBundle, Node},
    DefaultPlugins,
};
use bevy_dioxus::{
    bevy_mod_picking::DefaultPickingPlugins, colors::*, dioxus::prelude::*, hooks::*,
    DioxusUiBundle, DioxusUiPlugin, DioxusUiRoot,
};

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
        div {
            width: "100vw",
            height: "100vh",
            justify_content: "space-between",
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

    render! {
        div {
            onclick: move |_| selected_entity.set(None),
            flex_direction: "column",
            if entities.is_empty() {
                rsx! { "No entities exist" }
            } else {
                rsx! {
                    for (entity, name) in entities {
                        div {
                            onclick: move |_| selected_entity.set(Some(entity)),
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
            div {
                onclick: move |_| spawn_entity(),
                padding: "8",
                background_color: NEUTRAL_800,
                "Spawn Entity"
            }
        }
    }
}

#[component]
fn EntityInspector<'a>(cx: Scope, selected_entity: &'a UseState<Option<Entity>>) -> Element {
    render! {
        if selected_entity.is_none() {
            "Select an entity to view its components"
        } else {
            "TODO: Component widgets"
        }
    }
}
