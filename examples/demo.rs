use bevy::{prelude::*, reflect::TypeInfo};
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
fn Editor() -> Element {
    // TODO: When selected entity is despawned, need to reset this to None
    let selected_entity = use_signal_sync(|| Option::<Entity>::None);

    rsx! {
        node {
            width: "100vw",
            height: "100vh",
            justify_content: "space_between",
            SceneTree { selected_entity }
            EntityInspector { selected_entity }
        }
    }
}

#[component]
fn SceneTree(selected_entity: Signal<Option<Entity>, SyncStorage>) -> Element {
    let mut entities = use_query_filtered::<(Entity, DebugName), Without<Node>>();
    let entities = entities.query();
    let mut entities = entities.into_iter().collect::<Vec<_>>();
    entities.sort_by_key(|(entity, _)| *entity);

    let system_scheduler = use_system_scheduler();

    rsx! {
        node {
            onclick: move |_| selected_entity.set(None),
            flex_direction: "column",
            if entities.is_empty() {
                "No entities exist"
            } else {
                for (entity, name) in entities {
                    Button {
                        onclick: move |event: DioxusEvent<PointerButton>| if *event.data == PointerButton::Primary {
                            if Some(entity) == selected_entity() {
                                selected_entity.set(None);
                            } else {
                                selected_entity.set(Some(entity));
                            }
                            event.stop_propagation();
                        },
                        base_color: if Some(entity) == selected_entity() { Some(VIOLET_700.to_owned()) } else { None },
                        click_color: if Some(entity) == selected_entity() { Some(VIOLET_400.to_owned()) } else { None },
                        hover_color: if Some(entity) == selected_entity() { Some(VIOLET_500.to_owned()) } else { None },
                        match name.name {
                            Some(name) => format!("{name}"),
                            _ => format!("Entity ({:?})", name.entity)
                        }
                    }
                }
            }
            Button {
                onclick: move |event: DioxusEvent<PointerButton>| if *event.data == PointerButton::Primary {
                    system_scheduler.schedule(move |world: &mut World| {
                        let new_entity = world.spawn_empty();
                        selected_entity.set(Some(new_entity.id()));
                    });
                    event.stop_propagation();
                },
                text { text: "Spawn Entity", text_size: "18" }
            }
        }
    }
}

#[component]
fn EntityInspector(selected_entity: ReadOnlySignal<Option<Entity>, SyncStorage>) -> Element {
    let world = use_world();
    let type_registry = use_resource::<AppTypeRegistry>().read();
    let components = selected_entity()
        .map(|selected_entity| {
            let entity_ref = world.get_entity(selected_entity).unwrap();
            let mut components = entity_ref
                .archetype()
                .components()
                .map(|component_id| {
                    let component_info = world.components().get_info(component_id).unwrap();
                    let type_info = component_info
                        .type_id()
                        .and_then(|type_id| type_registry.get_type_info(type_id));
                    let (_, name) = component_info.name().rsplit_once("::").unwrap();
                    let (crate_name, _) = component_info.name().split_once("::").unwrap();
                    (name, crate_name, type_info)
                })
                .collect::<Vec<_>>();
            components.sort_by_key(|(name, _, _)| *name);
            components
        })
        .unwrap_or_default();

    rsx! {
        if selected_entity().is_none() {
            node {
                margin: "8",
                "Select an entity to view its components"
            }
        } else {
            node {
                flex_direction: "column",
                margin: "8",
                text { text: "Entity Inspector", text_size: "24" }
                for (name, crate_name, type_info) in components {
                    node {
                        flex_direction: "column",
                        margin_bottom: "6",
                        node {
                            column_gap: "6",
                            align_items: "baseline",
                            text { text: name, text_size: "18" }
                            text { text: crate_name, text_size: "14", text_color: NEUTRAL_400 }
                        }
                        if let Some(type_info) = type_info {
                            { component_inspector(type_info) }
                        }
                    }
                }
            }
        }
    }
}

// TODO: Ideally this would be a component
fn component_inspector<'a>(type_info: &'a TypeInfo) -> Element {
    rsx! {
        match type_info {
            TypeInfo::Struct(info) => rsx! {
                for field in info.iter() {
                    { format!("{}: {}", field.name(), field.type_path()) }
                }
            },
            TypeInfo::TupleStruct(_) => rsx! { "TODO" },
            TypeInfo::Tuple(_) => rsx! { "TODO" },
            TypeInfo::List(_) => rsx! { "TODO" },
            TypeInfo::Array(_) => rsx! { "TODO" },
            TypeInfo::Map(_) => rsx! { "TODO" },
            TypeInfo::Enum(_) => rsx! { "TODO" },
            TypeInfo::Value(_) => rsx! { "TODO" },
        }
    }
}

#[allow(non_snake_case)]
fn Button(props: ButtonProps) -> Element {
    let mut clicked = use_signal(|| false);
    let mut hovered = use_signal(|| false);
    let background_color = if clicked() {
        props.click_color.unwrap_or(NEUTRAL_500.to_owned())
    } else if hovered() {
        props.hover_color.unwrap_or(NEUTRAL_600.to_owned())
    } else {
        props.base_color.unwrap_or(NEUTRAL_800.to_owned())
    };

    rsx! {
        node {
            onclick: move |event| props.onclick.call(event),
            onclick_down: move |event| if *event.data == PointerButton::Primary { clicked.set(true) },
            onclick_up: move |event| if *event.data == PointerButton::Primary { clicked.set(false) },
            onmouse_enter: move |_| hovered.set(true),
            onmouse_exit: move |_| { hovered.set(false); clicked.set(false) },
            padding: "8",
            background_color,
            { &props.children }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct ButtonProps {
    onclick: EventHandler<DioxusEvent<PointerButton>>,
    base_color: Option<String>,
    click_color: Option<String>,
    hover_color: Option<String>,
    children: Element,
}
