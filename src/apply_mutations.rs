use bevy::{
    ecs::{entity::Entity, system::Commands},
    hierarchy::BuildChildren,
    prelude::default,
    text::{Text, TextStyle},
    ui::node_bundles::TextBundle,
    utils::HashMap,
};
use dioxus::core::{ElementId, Mutation, Mutations, Template, TemplateNode};
use smallvec::SmallVec;

pub fn apply_mutations(
    mutations: Mutations,
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
                for _ in 0..m {
                    parent.add_child(stack.pop().unwrap());
                }
            }
            Mutation::AssignId { path, id } => todo!(),
            Mutation::CreatePlaceholder { id } => todo!(),
            Mutation::CreateTextNode { value, id } => todo!(),
            Mutation::HydrateText { path, value, id } => todo!(),
            Mutation::LoadTemplate { name, index, id } => {
                let entity = templates[name].roots[index].spawn(commands);
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
    roots: SmallVec<[BevyTemplateNode; 4]>,
}

enum BevyTemplateNode {
    Text(Text),
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
                namespace,
                attrs,
                children,
            } => todo!(),
            TemplateNode::Text { text } => {
                Self::Text(Text::from_section(*text, TextStyle::default()))
            }
            TemplateNode::Dynamic { id } => todo!(),
            TemplateNode::DynamicText { id } => todo!(),
        }
    }

    fn spawn(&self, commands: &mut Commands) -> Entity {
        match self {
            Self::Text(text) => commands.spawn(TextBundle {
                text: text.clone(),
                ..default()
            }),
        }
        .id()
    }
}
