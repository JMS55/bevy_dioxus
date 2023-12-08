use bevy::{
    ecs::{
        entity::Entity,
        reflect::{AppTypeRegistry, ReflectComponent},
        system::{Command, Commands},
        world::Mut,
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    prelude::World,
    reflect::Reflect,
    utils::HashMap,
};
use dioxus_core::{BorrowedAttributeValue, ElementId, Mutation, Mutations};
use std::sync::Arc;

pub fn apply_mutations(
    mutations: Mutations,
    element_id_to_bevy_ui_entity: &mut HashMap<ElementId, Entity>,
    templates: &mut HashMap<String, ()>,
    root_entity: Entity,
    commands: &mut Commands,
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
                let mut parent = commands.entity(map[&id]);
                for _ in 0..m {
                    parent.add_child(stack.pop().unwrap());
                }
            }
            Mutation::AssignId { path, id } => todo!(),
            Mutation::CreatePlaceholder { id } => {
                map.insert(id, commands.spawn(()).id());
            }
            Mutation::CreateTextNode { .. } => {
                unreachable!("Should not be used by bevy_dioxus elements");
            }
            Mutation::HydrateText { .. } => {
                unreachable!("Should not be used by bevy_dioxus elements");
            }
            Mutation::LoadTemplate { name, index, id } => todo!(),
            Mutation::ReplaceWith { id, m } => todo!(),
            Mutation::ReplacePlaceholder { path, m } => todo!(),
            Mutation::InsertAfter { id, m } => todo!(),
            Mutation::InsertBefore { id, m } => todo!(),
            Mutation::SetAttribute {
                name,
                value,
                id,
                ns: _,
            } => commands.add(SetReflectedComponent {
                entity: map[&id],
                component_type_path: name.to_owned(),
                component_value: match value {
                    BorrowedAttributeValue::Any(value) => Some(Arc::clone(
                        value
                            .as_any()
                            .downcast_ref::<Arc<dyn Reflect>>()
                            .expect(&format!(
                            "Encountered an attribute with name {name} that did not impl Reflect"
                        )),
                    )),
                    BorrowedAttributeValue::None => None,
                    _ => unreachable!("Should not be used by bevy_dioxus elements"),
                },
            }),
            Mutation::SetText { .. } => unreachable!("Should not be used by bevy_dioxus elements"),
            Mutation::NewEventListener { name, id } => todo!(),
            Mutation::RemoveEventListener { name, id } => todo!(),
            Mutation::Remove { id } => {
                commands
                    .entity(map.remove(&id).unwrap())
                    .despawn_recursive();
            }
            Mutation::PushRoot { id } => stack.push(map[&id]),
        }
    }
}

struct SetReflectedComponent {
    entity: Entity,
    component_type_path: String,
    component_value: Option<Arc<dyn Reflect>>,
}

impl Command for SetReflectedComponent {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world: &mut World, type_registry: Mut<AppTypeRegistry>| {
            let type_registry = type_registry.read();
            let reflected_component = type_registry
                .get_with_type_path(&self.component_type_path)
                .expect(&format!(
                    "Encountered an attribute with name {} that was not registered for reflection",
                    self.component_type_path
                ))
                .data::<ReflectComponent>()
                .expect(&format!(
                    "Encountered an attribute with name {} that did not reflect Component",
                    self.component_type_path
                ));

            let entity_mut = &mut world.entity_mut(self.entity);
            match self.component_value {
                Some(value) => reflected_component.insert(entity_mut, &*value),
                None => reflected_component.remove(entity_mut),
            }
        });
    }
}
