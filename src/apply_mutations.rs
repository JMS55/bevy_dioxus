use crate::bsn::Bsn;
use bevy::{
    ecs::{entity::Entity, world::World},
    hierarchy::{BuildWorldChildren, DespawnRecursiveExt},
    text::Text,
    ui::node_bundles::TextBundle,
    utils::HashMap,
};
use dioxus_core::{ElementId, Mutation, Mutations};

pub fn apply_mutations(
    mutations: Mutations,
    element_id_to_bevy_ui_entity: &mut HashMap<ElementId, Entity>,
    templates: &mut HashMap<String, Bsn>,
    root_entity: Entity,
    world: &mut World,
) {
    for new_template in mutations.templates {
        templates.insert(new_template.name.to_owned(), todo!());
    }

    let map = element_id_to_bevy_ui_entity;
    map.insert(ElementId(0), root_entity);

    let mut stack = vec![root_entity];
    for edit in mutations.edits {
        match edit {
            Mutation::AppendChildren { id, m } => {
                let mut parent = world.entity_mut(map[&id]);
                for _ in 0..m {
                    parent.add_child(stack.pop().unwrap());
                }
            }
            Mutation::AssignId { path, id } => todo!(),
            Mutation::CreatePlaceholder { id } => {
                map.insert(id, world.spawn(()).id());
            }
            Mutation::CreateTextNode { value, id } => {
                map.insert(id, world.spawn(TextBundle::from(value)).id());
            }
            Mutation::HydrateText { path, value, id } => todo!(),
            Mutation::LoadTemplate { name, index, id } => todo!(),
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
                world
                    .entity_mut(map[&id])
                    .get_mut::<Text>()
                    .unwrap()
                    .sections[0]
                    .value = value.to_owned();
            }
            Mutation::NewEventListener { name, id } => todo!(),
            Mutation::RemoveEventListener { name, id } => todo!(),
            Mutation::Remove { id } => {
                world
                    .entity_mut(map.remove(&id).unwrap())
                    .despawn_recursive();
            }
            Mutation::PushRoot { id } => stack.push(map[&id]),
        }
    }
}
