use crate::events::is_supported_event;
use bevy::{
    ecs::{entity::Entity, system::Commands},
    hierarchy::BuildChildren,
    prelude::default,
    text::{Text, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        *,
    },
    utils::{EntityHashMap, HashMap},
};
use dioxus::core::{ElementId, Mutation, Mutations, Template, TemplateAttribute, TemplateNode};

pub fn apply_mutations(
    mutations: Mutations,
    hierarchy: &mut HashMap<(Entity, u8), Entity>,
    element_id_to_bevy_ui_entity: &mut HashMap<ElementId, Entity>,
    bevy_ui_entity_to_element_id: &mut EntityHashMap<Entity, ElementId>,
    templates: &mut HashMap<String, BevyTemplate>,
    root_entity: Entity,
    commands: &mut Commands,
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
                let mut parent = commands.entity(element_id_to_bevy_ui_entity[&id]);
                let parent_existing_child_count =
                    hierarchy.keys().filter(|(p, _)| *p == parent.id()).count();
                for (i, child) in stack.drain((stack.len() - m)..).enumerate() {
                    parent.add_child(child);
                    hierarchy.insert(
                        (parent.id(), (parent_existing_child_count + i + 1) as u8),
                        child,
                    );
                }
            }
            Mutation::AssignId { path, id } => todo!(),
            Mutation::CreatePlaceholder { id } => todo!(),
            Mutation::CreateTextNode { value, id } => {
                let entity = BevyTemplateNode::from_dioxus(&TemplateNode::Text { text: value })
                    .spawn(commands, hierarchy);
                element_id_to_bevy_ui_entity.insert(id, entity);
                bevy_ui_entity_to_element_id.insert(entity, id);
                stack.push(entity);
            }
            Mutation::HydrateText { path, value, id } => {
                let mut entity = *stack.last().unwrap();
                for index in path {
                    entity = hierarchy[&(entity, *index)];
                }
                commands
                    .entity(entity)
                    .insert(Text::from_section(value, TextStyle::default()));
                element_id_to_bevy_ui_entity.insert(id, entity);
                bevy_ui_entity_to_element_id.insert(entity, id);
            }
            Mutation::LoadTemplate { name, index, id } => {
                let entity = templates[name].roots[index].spawn(commands, hierarchy);
                element_id_to_bevy_ui_entity.insert(id, entity);
                bevy_ui_entity_to_element_id.insert(entity, id);
                stack.push(entity);
            }
            Mutation::ReplaceWith { id, m } => todo!(),
            Mutation::ReplacePlaceholder { path, m } => todo!(),
            Mutation::InsertAfter { id, m } => todo!(),
            Mutation::InsertBefore { id, m } => todo!(),
            Mutation::SetAttribute {
                name,
                value,
                id,
                ns,
            } => todo!(),
            Mutation::SetText { value, id } => {
                commands
                    .entity(element_id_to_bevy_ui_entity[&id])
                    .insert(Text::from_section(value, TextStyle::default()));
            }
            Mutation::NewEventListener { name, id: _ } => {
                if !is_supported_event(name) {
                    panic!("Encountered unsupported bevy_dioxus event `{name}`.");
                }
            }
            Mutation::RemoveEventListener { .. } => {}
            Mutation::Remove { id } => todo!(),
            Mutation::PushRoot { id } => stack.push(element_id_to_bevy_ui_entity[&id]),
        }
    }
}

pub struct BevyTemplate {
    roots: Box<[BevyTemplateNode]>,
}

enum BevyTemplateNode {
    Node { style: Style, children: Box<[Self]> },
    TextNode(Text),
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
                tag,
                namespace: _,
                attrs,
                children,
            } => {
                if *tag != "div" {
                    panic!(
                        "Encountered unsupported bevy_dioxus tag `{tag}`. Only `div` is supported."
                    );
                }
                Self::Node {
                    style: parse_style_attributes(attrs),
                    children: children.iter().map(Self::from_dioxus).collect(),
                }
            }
            TemplateNode::Text { text } => {
                Self::TextNode(Text::from_section(*text, TextStyle::default()))
            }
            TemplateNode::Dynamic { id: _ } => Self::Node {
                style: Style::default(),
                children: Box::new([]),
            },
            TemplateNode::DynamicText { id: _ } => {
                Self::TextNode(Text::from_section("", TextStyle::default()))
            }
        }
    }

    fn spawn(
        &self,
        commands: &mut Commands,
        hierarchy: &mut HashMap<(Entity, u8), Entity>,
    ) -> Entity {
        match self {
            BevyTemplateNode::Node { style, children } => {
                // TODO: Can probably use with_children() instead
                let children = children
                    .iter()
                    .map(|child| child.spawn(commands, hierarchy))
                    .collect::<Box<[_]>>();
                let parent = commands
                    .spawn(NodeBundle {
                        style: style.clone(),
                        ..default()
                    })
                    .push_children(&children)
                    .id();
                for (i, child) in children.iter().enumerate() {
                    hierarchy.insert((parent, i as u8), *child);
                }
                parent
            }
            Self::TextNode(text) => commands
                .spawn(TextBundle {
                    text: text.clone(),
                    ..default()
                })
                .id(),
        }
    }
}

fn parse_style_attributes(attributes: &[TemplateAttribute]) -> Style {
    let mut style = Style::default();
    for attribute in attributes {
        if let TemplateAttribute::Static {
            name,
            value,
            namespace: _,
        } = attribute
        {
            // TODO: The rest of Style
            match (*name, *value) {
                ("display", "flex") => style.display = Display::Flex,
                ("display", "grid") => style.display = Display::Grid,
                ("display", "none") => style.display = Display::None,
                ("position", "relative") => style.position_type = PositionType::Relative,
                ("position", "absolute") => style.position_type = PositionType::Absolute,
                _ => panic!("Encountered unsupported bevy_dioxus attribute `{name}: {value}`."),
            }
        }
    }
    style
}
