use crate::{
    events::{insert_event_listener, remove_event_listener},
    parse_attributes::set_attribute,
};
use bevy::{
    ecs::{entity::Entity, system::Command, world::World},
    hierarchy::{BuildWorldChildren, Children, DespawnRecursive, Parent},
    prelude::default,
    render::color::Color,
    text::{Text, TextLayoutInfo, TextStyle},
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
                    BorrowedAttributeValue::None => todo!("Remove the attribute"),
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
            Mutation::NewEventListener { name, id } => {
                insert_event_listener(name, world.entity_mut(element_id_to_bevy_ui_entity[&id]));
            }
            Mutation::RemoveEventListener { name, id } => {
                remove_event_listener(name, world.entity_mut(element_id_to_bevy_ui_entity[&id]));
            }
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
