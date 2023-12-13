use bevy::{
    app::{App, Startup},
    core::DebugName,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{entity::Entity, query::Without, system::Commands},
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
                dioxus_ui_root: DioxusUiRoot::new(Editor),
                node_bundle: NodeBundle::default(),
            });
            commands.spawn(Camera2dBundle::default());
        })
        .run();
}

#[component]
fn Editor(cx: Scope) -> Element {
    let selected_entity = use_state(cx, || Option::<Entity>::None);

    render! {
        SceneTree { selected_entity: selected_entity }
        EntityInspector { selected_entity: selected_entity }
    }
}

#[component]
fn SceneTree<'a>(cx: Scope, selected_entity: &'a UseState<Option<Entity>>) -> Element {
    let entities = use_query_filtered::<(Entity, DebugName), Without<Node>>(cx);
    let entities = entities.query();
    let mut entities = entities.into_iter().collect::<Vec<_>>();
    entities.sort_by_key(|(entity, _)| *entity);

    render! {
        div {
            flex_direction: "column",
            if entities.is_empty() {
                rsx! { "No entities exist" }
            } else {
                rsx! {
                    for (entity, name) in entities {
                        div {
                            onclick: move |_| selected_entity.set(Some(entity)),
                            background_color: if Some(entity) == ***selected_entity { INDIGO_600 } else { NEUTRAL_800 },
                            format!("{name:?}")
                        }
                    }
                }
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
