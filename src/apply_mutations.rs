use bevy::{
    ecs::{entity::Entity, system::Commands},
    hierarchy::BuildChildren,
    prelude::default,
    text::{Text, TextStyle},
    ui::node_bundles::{NodeBundle, TextBundle},
    utils::HashMap,
};
use dioxus::core::{ElementId, Mutation, Mutations, Template, TemplateNode};

pub fn apply_mutations(
    mutations: Mutations,
    hierarchy: &mut HashMap<(Entity, u8), Entity>,
    element_id_to_bevy_ui_entity: &mut HashMap<ElementId, Entity>,
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

    let map = element_id_to_bevy_ui_entity;
    map.insert(ElementId(0), root_entity);
    let mut stack = vec![root_entity];

    for edit in mutations.edits {
        match edit {
            Mutation::AppendChildren { id, m } => {
                let mut parent = commands.entity(map[&id]);
                let parent_existing_children_count =
                    hierarchy.keys().filter(|(p, _)| *p == parent.id()).count();
                for i in 1..=m {
                    let child = stack.pop().unwrap();
                    parent.add_child(child);
                    hierarchy.insert(
                        (parent.id(), (parent_existing_children_count + i) as u8),
                        child,
                    );
                }
            }
            Mutation::AssignId { path, id } => todo!(),
            Mutation::CreatePlaceholder { id } => todo!(),
            Mutation::CreateTextNode { value, id } => {
                let entity = BevyTemplateNode::from_dioxus(&TemplateNode::Text { text: value })
                    .spawn(commands, hierarchy);
                map.insert(id, entity);
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
                map.insert(id, entity);
            }
            Mutation::LoadTemplate { name, index, id } => {
                let entity = templates[name].roots[index].spawn(commands, hierarchy);
                map.insert(id, entity);
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
            Mutation::SetText { value, id } => todo!(),
            Mutation::NewEventListener { name, id } => todo!(),
            Mutation::RemoveEventListener { name, id } => todo!(),
            Mutation::Remove { id } => todo!(),
            Mutation::PushRoot { id } => todo!(),
        }
    }
}

pub struct BevyTemplate {
    roots: Box<[BevyTemplateNode]>,
}

enum BevyTemplateNode {
    Node { children: Box<[Self]> },
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
                attrs: _,
                children,
            } => {
                if *tag != "div" {
                    panic!(
                        "Encountered unsupported bevy_dioxus tag `{tag}`. Only `div` is supported."
                    );
                }
                Self::Node {
                    children: children.iter().map(Self::from_dioxus).collect(),
                }
            }
            TemplateNode::Text { text } => {
                Self::TextNode(Text::from_section(*text, TextStyle::default()))
            }
            TemplateNode::Dynamic { id: _ } => Self::Node {
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
            BevyTemplateNode::Node { children } => {
                // TODO: Can probably use with_children() instead
                let children = children
                    .iter()
                    .map(|child| child.spawn(commands, hierarchy))
                    .collect::<Box<[_]>>();
                let parent = commands
                    .spawn(NodeBundle::default())
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
