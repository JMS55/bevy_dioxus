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

    render! {
        node {
            onclick: move |_| selected_entity.set(None),
            flex_direction: "column",
            if entities.is_empty() {
                rsx! { "No entities exist" }
            } else {
                rsx! {
                    for (entity, name) in entities {
                        Button {
                            onclick: move |event: Event<PointerButton>| if *event.data == PointerButton::Primary {
                                if Some(entity) == ***selected_entity {
                                    selected_entity.set(None);
                                } else {
                                    selected_entity.set(Some(entity));
                                }
                                event.stop_propagation();
                            },
                            base_color: if Some(entity) == ***selected_entity { Some(VIOLET_700) } else { None },
                            click_color: if Some(entity) == ***selected_entity { Some(VIOLET_400) } else { None },
                            hover_color: if Some(entity) == ***selected_entity { Some(VIOLET_500) } else { None },
                            match name.name {
                                Some(name) => format!("{name}"),
                                _ => format!("Entity ({:?})", name.entity)
                            }
                        }
                    }
                }
            }
            Button {
                onclick: move |event: Event<PointerButton>| if *event.data == PointerButton::Primary {
                    spawn_entity();
                    event.stop_propagation();
                },
                text { text: "Spawn Entity", text_size: "18" }
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
                node {
                    padding: "8",
                    "Select an entity to view its components"
                }
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

#[allow(non_snake_case)]
fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let clicked = use_state(cx, || false);
    let hovered = use_state(cx, || false);
    let background_color = if **clicked {
        cx.props.click_color.unwrap_or(NEUTRAL_500)
    } else if **hovered {
        cx.props.hover_color.unwrap_or(NEUTRAL_600)
    } else {
        cx.props.base_color.unwrap_or(NEUTRAL_800)
    };

    render! {
        node {
            onclick: move |event| cx.props.onclick.call(event),
            onclick_down: |event| if *event.data == PointerButton::Primary { clicked.set(true) },
            onclick_up: |event| if *event.data == PointerButton::Primary { clicked.set(false) },
            onmouse_enter: |_| hovered.set(true),
            onmouse_exit: |_| { hovered.set(false); clicked.set(false) },
            padding: "8",
            background_color: background_color,
            &cx.props.children
        }
    }
}

#[derive(Props)]
struct ButtonProps<'a> {
    onclick: EventHandler<'a, Event<PointerButton>>,
    base_color: Option<&'a str>,
    click_color: Option<&'a str>,
    hover_color: Option<&'a str>,
    children: Element<'a>,
}
