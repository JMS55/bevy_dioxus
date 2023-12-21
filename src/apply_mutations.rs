use crate::events::is_supported_event;
use bevy::{
    ecs::{entity::Entity, system::Command, world::World},
    hierarchy::{BuildWorldChildren, Children, DespawnRecursive, Parent},
    prelude::default,
    render::color::Color,
    text::{Text, TextAlignment, TextLayoutInfo, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        widget::TextFlags,
        *,
    },
    utils::{EntityHashMap, HashMap},
};
use dioxus::core::{
    BorrowedAttributeValue, ElementId, Mutation, Mutations, Template, TemplateAttribute,
    TemplateNode,
};

pub fn apply_mutations(
    mutations: Mutations,
    element_id_to_bevy_ui_entity: &mut HashMap<ElementId, Entity>,
    bevy_ui_entity_to_element_id: &mut EntityHashMap<Entity, ElementId>,
    templates: &mut HashMap<String, BevyTemplate>,
    root_entity: Entity,
    world: &mut World,
) {
    for new_template in mutations.templates {
        templates.insert(
            new_template.name.to_owned(),
            BevyTemplate::from_dioxus(&new_template),
        );
    }

    element_id_to_bevy_ui_entity.insert(ElementId(0), root_entity);
    bevy_ui_entity_to_element_id.insert(root_entity, ElementId(0));
    let mut stack = vec![root_entity];

    for edit in mutations.edits {
        match edit {
            Mutation::AppendChildren { id, m } => {
                let mut parent = world.entity_mut(element_id_to_bevy_ui_entity[&id]);
                for child in stack.drain((stack.len() - m)..) {
                    parent.add_child(child);
                }
            }
            Mutation::AssignId { path, id } => {
                let mut entity = *stack.last().unwrap();
                for index in path {
                    entity = world.entity(entity).get::<Children>().unwrap()[*index as usize];
                }
                element_id_to_bevy_ui_entity.insert(id, entity);
                bevy_ui_entity_to_element_id.insert(entity, id);
            }
            Mutation::CreatePlaceholder { id } => {
                let entity = world.spawn(NodeBundle::default()).id();
                element_id_to_bevy_ui_entity.insert(id, entity);
                bevy_ui_entity_to_element_id.insert(entity, id);
                stack.push(entity);
            }
            Mutation::CreateTextNode { value, id } => {
                let entity =
                    BevyTemplateNode::from_dioxus(&TemplateNode::Text { text: value }).spawn(world);
                element_id_to_bevy_ui_entity.insert(id, entity);
                bevy_ui_entity_to_element_id.insert(entity, id);
                stack.push(entity);
            }
            Mutation::HydrateText { path, value, id } => {
                let mut entity = *stack.last().unwrap();
                for index in path {
                    entity = world.entity(entity).get::<Children>().unwrap()[*index as usize];
                }
                world.entity_mut(entity).insert((
                    Text::from_section(value, TextStyle::default()),
                    TextLayoutInfo::default(),
                    TextFlags::default(),
                    ContentSize::default(),
                ));
                element_id_to_bevy_ui_entity.insert(id, entity);
                bevy_ui_entity_to_element_id.insert(entity, id);
            }
            Mutation::LoadTemplate { name, index, id } => {
                let entity = templates[name].roots[index].spawn(world);
                element_id_to_bevy_ui_entity.insert(id, entity);
                bevy_ui_entity_to_element_id.insert(entity, id);
                stack.push(entity);
            }
            Mutation::ReplaceWith { id, m } => {
                let existing = element_id_to_bevy_ui_entity[&id];
                let existing_parent = world.entity(existing).get::<Parent>().unwrap().get();
                let mut existing_parent = world.entity_mut(existing_parent);

                let existing_index = existing_parent
                    .get::<Children>()
                    .unwrap()
                    .iter()
                    .position(|child| *child == existing)
                    .unwrap();
                existing_parent.insert_children(existing_index, &stack.split_off(stack.len() - m));

                DespawnRecursive { entity: existing }.apply(world);
                // TODO: We're not removing child entities from the element maps
                if let Some(existing_element_id) = bevy_ui_entity_to_element_id.remove(&existing) {
                    element_id_to_bevy_ui_entity.remove(&existing_element_id);
                }
            }
            Mutation::ReplacePlaceholder { path, m } => {
                let mut existing = stack[stack.len() - m - 1];
                for index in path {
                    existing = world.entity(existing).get::<Children>().unwrap()[*index as usize];
                }
                let existing_parent = world.entity(existing).get::<Parent>().unwrap().get();
                let mut existing_parent = world.entity_mut(existing_parent);

                let existing_index = existing_parent
                    .get::<Children>()
                    .unwrap()
                    .iter()
                    .position(|child| *child == existing)
                    .unwrap();
                existing_parent.insert_children(existing_index, &stack.split_off(stack.len() - m));

                DespawnRecursive { entity: existing }.apply(world);
                // TODO: We're not removing child entities from the element maps
                if let Some(existing_element_id) = bevy_ui_entity_to_element_id.remove(&existing) {
                    element_id_to_bevy_ui_entity.remove(&existing_element_id);
                }
            }
            Mutation::InsertAfter { id, m } => {
                let entity = element_id_to_bevy_ui_entity[&id];
                let parent = world.entity(entity).get::<Parent>().unwrap().get();
                let mut parent = world.entity_mut(parent);
                let index = parent
                    .get::<Children>()
                    .unwrap()
                    .iter()
                    .position(|child| *child == entity)
                    .unwrap();
                parent.insert_children(index + 1, &stack.split_off(stack.len() - m));
            }
            Mutation::InsertBefore { id, m } => {
                let existing = element_id_to_bevy_ui_entity[&id];
                let parent = world.entity(existing).get::<Parent>().unwrap().get();
                let mut parent = world.entity_mut(parent);
                let index = parent
                    .get::<Children>()
                    .unwrap()
                    .iter()
                    .position(|child| *child == existing)
                    .unwrap();
                parent.insert_children(index, &stack.split_off(stack.len() - m));
            }
            Mutation::SetAttribute {
                name,
                value,
                id,
                ns: _,
            } => {
                let value = match value {
                    BorrowedAttributeValue::Text(value) => value,
                    value => {
                        panic!("Encountered unsupported bevy_dioxus attribute `{name}: {value:?}`.")
                    }
                };

                let (mut style, mut background_color, mut text) = world
                    .query::<(&mut Style, &mut BackgroundColor, Option<&mut Text>)>()
                    .get_mut(world, element_id_to_bevy_ui_entity[&id])
                    .unwrap();

                set_attribute(
                    name,
                    value,
                    &mut style,
                    &mut background_color,
                    text.as_deref_mut(),
                );
            }
            Mutation::SetText { value, id } => {
                world
                    .entity_mut(element_id_to_bevy_ui_entity[&id])
                    .insert(Text::from_section(value, TextStyle::default()));
            }
            Mutation::NewEventListener { name, id: _ } => {
                if !is_supported_event(name) {
                    panic!("Encountered unsupported bevy_dioxus event `{name}`.");
                }
            }
            Mutation::RemoveEventListener { .. } => {}
            Mutation::Remove { id } => {
                let entity = element_id_to_bevy_ui_entity[&id];
                DespawnRecursive { entity }.apply(world);
                // TODO: We're not removing child entities from the element maps
                if let Some(existing_element_id) = bevy_ui_entity_to_element_id.remove(&entity) {
                    element_id_to_bevy_ui_entity.remove(&existing_element_id);
                }
            }
            Mutation::PushRoot { id } => stack.push(element_id_to_bevy_ui_entity[&id]),
        }
    }
}

pub struct BevyTemplate {
    roots: Box<[BevyTemplateNode]>,
}

enum BevyTemplateNode {
    Node {
        style: (Style, BackgroundColor),
        children: Box<[Self]>,
    },
    TextNode {
        text: Text,
        style: (Style, BackgroundColor),
        children: Box<[Self]>,
    },
    IntrinsicTextNode(Text),
}

impl BevyTemplate {
    fn from_dioxus(template: &Template) -> Self {
        Self {
            roots: template
                .roots
                .iter()
                .map(BevyTemplateNode::from_dioxus)
                .collect(),
        }
    }
}

impl BevyTemplateNode {
    fn from_dioxus(node: &TemplateNode) -> Self {
        match node {
            TemplateNode::Element {
                tag: "node",
                namespace: Some("bevy_ui"),
                attrs,
                children,
            } => {
                let (style, background_color, _) = parse_template_attributes(attrs);
                Self::Node {
                    style: (style, background_color),
                    children: children.iter().map(Self::from_dioxus).collect(),
                }
            }
            TemplateNode::Element {
                tag: "text",
                namespace: Some("bevy_ui"),
                attrs,
                children,
            } => {
                let (style, background_color, text) = parse_template_attributes(attrs);
                Self::TextNode {
                    text,
                    style: (style, background_color),
                    children: children.iter().map(Self::from_dioxus).collect(),
                }
            }
            TemplateNode::Text { text } => {
                Self::IntrinsicTextNode(Text::from_section(*text, TextStyle::default()))
            }
            TemplateNode::Dynamic { id: _ } => Self::Node {
                style: (Style::default(), Color::NONE.into()),
                children: Box::new([]),
            },
            TemplateNode::DynamicText { id: _ } => {
                Self::IntrinsicTextNode(Text::from_section("", TextStyle::default()))
            }
            TemplateNode::Element {
                tag,
                namespace: None,
                ..
            } => {
                panic!("Encountered unsupported bevy_dioxus tag `{tag}`.")
            }
            TemplateNode::Element {
                tag,
                namespace: Some(namespace),
                ..
            } => {
                panic!("Encountered unsupported bevy_dioxus tag `{namespace}::{tag}`.")
            }
        }
    }

    fn spawn(&self, world: &mut World) -> Entity {
        match self {
            BevyTemplateNode::Node {
                style: (style, background_color),
                children,
            } => {
                let children = children
                    .iter()
                    .map(|child| child.spawn(world))
                    .collect::<Box<[_]>>();
                world
                    .spawn(NodeBundle {
                        style: style.clone(),
                        background_color: background_color.clone(),
                        ..default()
                    })
                    .push_children(&children)
                    .id()
            }
            BevyTemplateNode::TextNode {
                text,
                style: (style, background_color),
                children,
            } => {
                let children = children
                    .iter()
                    .map(|child| child.spawn(world))
                    .collect::<Box<[_]>>();
                world
                    .spawn(TextBundle {
                        text: text.clone(),
                        style: style.clone(),
                        background_color: background_color.clone(),
                        ..default()
                    })
                    .push_children(&children)
                    .id()
            }
            Self::IntrinsicTextNode(text) => world
                .spawn(TextBundle {
                    text: text.clone(),
                    ..default()
                })
                .id(),
        }
    }
}

fn parse_template_attributes(attributes: &[TemplateAttribute]) -> (Style, BackgroundColor, Text) {
    let mut style = Style::default();
    let mut background_color = Color::NONE.into();
    let mut text = Text::from_section("", TextStyle::default());
    for attribute in attributes {
        if let TemplateAttribute::Static {
            name,
            value,
            namespace: _,
        } = attribute
        {
            set_attribute(
                name,
                value,
                &mut style,
                &mut background_color,
                Some(&mut text),
            );
        }
    }
    (style, background_color, text)
}

fn set_attribute(
    name: &str,
    value: &str,
    style: &mut Style,
    background_color: &mut BackgroundColor,
    text: Option<&mut Text>,
) {
    match (name, value) {
        ("display", "flex") => style.display = Display::Flex,
        ("display", "grid") => style.display = Display::Grid,
        ("display", "none") => style.display = Display::None,
        ("position", "relative") => style.position_type = PositionType::Relative,
        ("position", "absolute") => style.position_type = PositionType::Absolute,
        ("overflow", "visible") => style.overflow = Overflow::visible(),
        ("overflow", "clip") => style.overflow = Overflow::clip(),
        ("overflow_x", "visible") => style.overflow.x = OverflowAxis::Visible,
        ("overflow_x", "clip") => style.overflow.x = OverflowAxis::Clip,
        ("overflow_y", "visible") => style.overflow.y = OverflowAxis::Visible,
        ("overflow_y", "clip") => style.overflow.y = OverflowAxis::Clip,
        ("left", val) => style.left = parse_val(val),
        ("right", val) => style.right = parse_val(val),
        ("top", val) => style.top = parse_val(val),
        ("bottom", val) => style.bottom = parse_val(val),
        ("width", val) => style.width = parse_val(val),
        ("height", val) => style.height = parse_val(val),
        ("min_width", val) => style.min_width = parse_val(val),
        ("min_height", val) => style.min_height = parse_val(val),
        ("max_width", val) => style.max_width = parse_val(val),
        ("max_height", val) => style.max_height = parse_val(val),
        ("aspect_ratio", "none") => style.aspect_ratio = None,
        ("aspect_ratio", float) => {
            style.aspect_ratio = Some(
                float
                    .parse::<f32>()
                    .unwrap_or_else(|val| panic!("Encountered invalid bevy_dioxus f32 `{val}`.")),
            );
        }
        ("align_items", "default") => style.align_items = AlignItems::Default,
        ("align_items", "start") => style.align_items = AlignItems::Start,
        ("align_items", "end") => style.align_items = AlignItems::End,
        ("align_items", "flex_start") => style.align_items = AlignItems::FlexStart,
        ("align_items", "flex_end") => style.align_items = AlignItems::FlexEnd,
        ("align_items", "center") => style.align_items = AlignItems::Center,
        ("align_items", "baseline") => style.align_items = AlignItems::Baseline,
        ("align_items", "stretch") => style.align_items = AlignItems::Stretch,
        // TODO: The rest of the attributes from here on out
        ("flex_direction", "column") => style.flex_direction = FlexDirection::Column,
        ("background_color", hex) => {
            background_color.0 = Color::hex(hex).expect(&format!(
                "Encountered invalid bevy_dioxus Color hex `{hex}`."
            ));
        }
        ("padding", val) => style.padding = UiRect::all(parse_val(val)),
        ("justify_content", "space_between") => {
            style.justify_content = JustifyContent::SpaceBetween;
        }
        ("align_content", "space_between") => style.align_content = AlignContent::SpaceBetween,
        ("text", new_text) if text.is_some() => text.unwrap().sections[0] = new_text.into(),
        ("text_direction", "inherit") if text.is_some() => style.direction = Direction::Inherit,
        ("text_direction", "left_to_right") if text.is_some() => {
            style.direction = Direction::LeftToRight;
        }
        ("text_direction", "right_to_left") if text.is_some() => {
            style.direction = Direction::RightToLeft;
        }
        ("text_alignment", "left") if text.is_some() => {
            text.unwrap().alignment = TextAlignment::Left;
        }
        ("text_alignment", "center") if text.is_some() => {
            text.unwrap().alignment = TextAlignment::Center;
        }
        ("text_alignment", "right") if text.is_some() => {
            text.unwrap().alignment = TextAlignment::Right;
        }
        ("text_size", val) if text.is_some() => {
            text.unwrap().sections[0].style.font_size = val
                .parse::<f32>()
                .unwrap_or_else(|val| panic!("Encountered invalid bevy_dioxus f32 `{val}`."));
        }
        ("text_color", hex) if text.is_some() => {
            text.unwrap().sections[0].style.color = Color::hex(hex).expect(&format!(
                "Encountered invalid bevy_dioxus Color hex `{hex}`."
            ));
        }
        _ => panic!("Encountered unsupported bevy_dioxus attribute `{name}: {value}`."),
    }
}

fn parse_val(val: &str) -> Val {
    if let Ok(val) = val.parse::<f32>() {
        return Val::Px(val);
    }
    if let Some((val, "")) = val.split_once("px") {
        if let Ok(val) = val.parse::<f32>() {
            return Val::Px(val);
        }
    }
    if let Some((val, "")) = val.split_once("vw") {
        if let Ok(val) = val.parse::<f32>() {
            return Val::Vw(val);
        }
    }
    if let Some((val, "")) = val.split_once("vh") {
        if let Ok(val) = val.parse::<f32>() {
            return Val::Vh(val);
        }
    }
    panic!("Encountered invalid bevy_dioxus Val `{val}`.");
}
