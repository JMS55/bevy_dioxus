use crate::{
    events::{insert_event_listener, remove_event_listener},
    parse_attributes::set_attribute,
};
use bevy::{
    asset::AssetServer,
    ecs::{
        entity::{Entity, EntityHashMap},
        system::Command,
        world::World,
    },
    hierarchy::{BuildWorldChildren, Children, DespawnRecursive, Parent},
    prelude::default,
    render::{color::Color, view::Visibility},
    text::{Text, TextLayoutInfo, TextStyle},
    transform::components::Transform,
    ui::{
        node_bundles::{ImageBundle, NodeBundle, TextBundle},
        widget::TextFlags,
        *,
    },
    utils::HashMap,
};
use dioxus::dioxus_core::{
    AttributeValue, ElementId, Template, TemplateAttribute, TemplateNode, WriteMutations,
};

pub struct MutationApplier<'a> {
    element_id_to_bevy_ui_entity: &'a mut HashMap<ElementId, Entity>,
    bevy_ui_entity_to_element_id: &'a mut EntityHashMap<ElementId>,
    templates: &'a mut HashMap<String, BevyTemplate>,
    world: &'a mut World,
    asset_server: &'a AssetServer,
    stack: Vec<Entity>,
}

impl<'a> MutationApplier<'a> {
    pub fn new(
        element_id_to_bevy_ui_entity: &'a mut HashMap<ElementId, Entity>,
        bevy_ui_entity_to_element_id: &'a mut EntityHashMap<ElementId>,
        templates: &'a mut HashMap<String, BevyTemplate>,
        root_entity: Entity,
        world: &'a mut World,
        asset_server: &'a AssetServer,
    ) -> Self {
        element_id_to_bevy_ui_entity.insert(ElementId(0), root_entity);
        bevy_ui_entity_to_element_id.insert(root_entity, ElementId(0));

        Self {
            element_id_to_bevy_ui_entity,
            bevy_ui_entity_to_element_id,
            templates,
            world,
            asset_server,
            stack: vec![root_entity],
        }
    }
}

impl<'a> WriteMutations for MutationApplier<'a> {
    fn register_template(&mut self, template: Template) {
        self.templates.insert(
            template.name.to_owned(),
            BevyTemplate::from_dioxus(&template, self.asset_server),
        );
    }

    fn append_children(&mut self, id: ElementId, m: usize) {
        let mut parent = self
            .world
            .entity_mut(self.element_id_to_bevy_ui_entity[&id]);
        for child in self.stack.drain((self.stack.len() - m)..) {
            parent.add_child(child);
        }
    }

    fn assign_node_id(&mut self, path: &'static [u8], id: ElementId) {
        let mut entity = *self.stack.last().unwrap();
        for index in path {
            entity = self.world.entity(entity).get::<Children>().unwrap()[*index as usize];
        }
        self.element_id_to_bevy_ui_entity.insert(id, entity);
        self.bevy_ui_entity_to_element_id.insert(entity, id);
    }

    fn create_placeholder(&mut self, id: ElementId) {
        let entity = self.world.spawn(NodeBundle::default()).id();
        self.element_id_to_bevy_ui_entity.insert(id, entity);
        self.bevy_ui_entity_to_element_id.insert(entity, id);
        self.stack.push(entity);
    }

    fn create_text_node(&mut self, value: &str, id: ElementId) {
        let entity =
            BevyTemplateNode::IntrinsicTextNode(Text::from_section(value, TextStyle::default()))
                .spawn(self.world);
        self.element_id_to_bevy_ui_entity.insert(id, entity);
        self.bevy_ui_entity_to_element_id.insert(entity, id);
        self.stack.push(entity);
    }

    fn hydrate_text_node(&mut self, path: &'static [u8], value: &str, id: ElementId) {
        let mut entity = *self.stack.last().unwrap();
        for index in path {
            entity = self.world.entity(entity).get::<Children>().unwrap()[*index as usize];
        }
        self.world.entity_mut(entity).insert((
            Text::from_section(value, TextStyle::default()),
            TextLayoutInfo::default(),
            TextFlags::default(),
            ContentSize::default(),
        ));
        self.element_id_to_bevy_ui_entity.insert(id, entity);
        self.bevy_ui_entity_to_element_id.insert(entity, id);
    }

    fn load_template(&mut self, name: &'static str, index: usize, id: ElementId) {
        let entity = self.templates[name].roots[index].spawn(self.world);
        self.element_id_to_bevy_ui_entity.insert(id, entity);
        self.bevy_ui_entity_to_element_id.insert(entity, id);
        self.stack.push(entity);
    }

    fn replace_node_with(&mut self, id: ElementId, m: usize) {
        let existing = self.element_id_to_bevy_ui_entity[&id];
        let existing_parent = self.world.entity(existing).get::<Parent>().unwrap().get();
        let mut existing_parent = self.world.entity_mut(existing_parent);

        let existing_index = existing_parent
            .get::<Children>()
            .unwrap()
            .iter()
            .position(|child| *child == existing)
            .unwrap();
        existing_parent
            .insert_children(existing_index, &self.stack.split_off(self.stack.len() - m));

        DespawnRecursive { entity: existing }.apply(self.world);
        // TODO: We're not removing child entities from the element maps
        if let Some(existing_element_id) = self.bevy_ui_entity_to_element_id.remove(&existing) {
            self.element_id_to_bevy_ui_entity
                .remove(&existing_element_id);
        }
    }

    fn replace_placeholder_with_nodes(&mut self, path: &'static [u8], m: usize) {
        let mut existing = self.stack[self.stack.len() - m - 1];
        for index in path {
            existing = self.world.entity(existing).get::<Children>().unwrap()[*index as usize];
        }
        let existing_parent = self.world.entity(existing).get::<Parent>().unwrap().get();
        let mut existing_parent = self.world.entity_mut(existing_parent);

        let existing_index = existing_parent
            .get::<Children>()
            .unwrap()
            .iter()
            .position(|child| *child == existing)
            .unwrap();
        existing_parent
            .insert_children(existing_index, &self.stack.split_off(self.stack.len() - m));

        DespawnRecursive { entity: existing }.apply(self.world);
        // TODO: We're not removing child entities from the element maps
        if let Some(existing_element_id) = self.bevy_ui_entity_to_element_id.remove(&existing) {
            self.element_id_to_bevy_ui_entity
                .remove(&existing_element_id);
        }
    }

    fn insert_nodes_after(&mut self, id: ElementId, m: usize) {
        let entity = self.element_id_to_bevy_ui_entity[&id];
        let parent = self.world.entity(entity).get::<Parent>().unwrap().get();
        let mut parent = self.world.entity_mut(parent);
        let index = parent
            .get::<Children>()
            .unwrap()
            .iter()
            .position(|child| *child == entity)
            .unwrap();
        parent.insert_children(index + 1, &self.stack.split_off(self.stack.len() - m));
    }

    fn insert_nodes_before(&mut self, id: ElementId, m: usize) {
        let existing = self.element_id_to_bevy_ui_entity[&id];
        let parent = self.world.entity(existing).get::<Parent>().unwrap().get();
        let mut parent = self.world.entity_mut(parent);
        let index = parent
            .get::<Children>()
            .unwrap()
            .iter()
            .position(|child| *child == existing)
            .unwrap();
        parent.insert_children(index, &self.stack.split_off(self.stack.len() - m));
    }

    fn set_attribute(
        &mut self,
        name: &'static str,
        _ns: Option<&'static str>,
        value: &AttributeValue,
        id: ElementId,
    ) {
        let value = match value {
            AttributeValue::Text(value) => value,
            AttributeValue::None => todo!("Remove the attribute"),
            value => {
                panic!("Encountered unsupported bevy_dioxus attribute `{name}: {value:?}`.")
            }
        };

        let (
            mut style,
            mut border_color,
            mut outline,
            mut background_color,
            mut transform,
            mut visibility,
            mut z_index,
            mut text,
            mut image,
        ) = self
            .world
            .query::<(
                &mut Style,
                &mut BorderColor,
                &mut Outline,
                &mut BackgroundColor,
                &mut Transform,
                &mut Visibility,
                &mut ZIndex,
                Option<&mut Text>,
                Option<&mut UiImage>,
            )>()
            .get_mut(self.world, self.element_id_to_bevy_ui_entity[&id])
            .unwrap();

        set_attribute(
            name,
            &value,
            &mut style,
            &mut border_color,
            &mut outline,
            &mut background_color,
            &mut transform,
            &mut visibility,
            &mut z_index,
            text.as_deref_mut(),
            image.as_deref_mut(),
            self.asset_server,
        );
    }

    fn set_node_text(&mut self, value: &str, id: ElementId) {
        self.world
            .entity_mut(self.element_id_to_bevy_ui_entity[&id])
            .insert(Text::from_section(value, TextStyle::default()));
    }

    fn create_event_listener(&mut self, name: &'static str, id: ElementId) {
        insert_event_listener(
            &name,
            self.world
                .entity_mut(self.element_id_to_bevy_ui_entity[&id]),
        );
    }

    fn remove_event_listener(&mut self, name: &'static str, id: ElementId) {
        remove_event_listener(
            &name,
            self.world
                .entity_mut(self.element_id_to_bevy_ui_entity[&id]),
        );
    }

    fn remove_node(&mut self, id: ElementId) {
        let entity = self.element_id_to_bevy_ui_entity[&id];
        DespawnRecursive { entity }.apply(self.world);
        // TODO: We're not removing child entities from the element maps
        if let Some(existing_element_id) = self.bevy_ui_entity_to_element_id.remove(&entity) {
            self.element_id_to_bevy_ui_entity
                .remove(&existing_element_id);
        }
    }

    fn push_root(&mut self, id: ElementId) {
        self.stack.push(self.element_id_to_bevy_ui_entity[&id]);
    }
}

pub struct BevyTemplate {
    roots: Box<[BevyTemplateNode]>,
}

enum BevyTemplateNode {
    Node {
        style: StyleComponents,
        children: Box<[Self]>,
    },
    TextNode {
        text: Text,
        style: StyleComponents,
        children: Box<[Self]>,
    },
    ImageNode {
        image: UiImage,
        style: StyleComponents,
        children: Box<[Self]>,
    },
    IntrinsicTextNode(Text),
}

impl BevyTemplate {
    fn from_dioxus(template: &Template, asset_server: &AssetServer) -> Self {
        Self {
            roots: template
                .roots
                .iter()
                .map(|node| BevyTemplateNode::from_dioxus(node, asset_server))
                .collect(),
        }
    }
}

impl BevyTemplateNode {
    fn from_dioxus(node: &TemplateNode, asset_server: &AssetServer) -> Self {
        match node {
            TemplateNode::Element {
                tag: "node",
                namespace: Some("bevy_ui"),
                attrs,
                children,
            } => {
                let (style, _, _) = parse_template_attributes(attrs, Color::NONE, asset_server);
                Self::Node {
                    style,
                    children: children
                        .iter()
                        .map(|node| Self::from_dioxus(node, asset_server))
                        .collect(),
                }
            }
            TemplateNode::Element {
                tag: "text",
                namespace: Some("bevy_ui"),
                attrs,
                children,
            } => {
                let (style, text, _) = parse_template_attributes(attrs, Color::NONE, asset_server);
                Self::TextNode {
                    text,
                    style,
                    children: children
                        .iter()
                        .map(|node| Self::from_dioxus(node, asset_server))
                        .collect(),
                }
            }
            TemplateNode::Element {
                tag: "image",
                namespace: Some("bevy_ui"),
                attrs,
                children,
            } => {
                let (style, _, image) =
                    parse_template_attributes(attrs, Color::WHITE, asset_server);
                Self::ImageNode {
                    image,
                    style,
                    children: children
                        .iter()
                        .map(|node| Self::from_dioxus(node, asset_server))
                        .collect(),
                }
            }
            TemplateNode::Text { text } => {
                Self::IntrinsicTextNode(Text::from_section(*text, TextStyle::default()))
            }
            TemplateNode::Dynamic { id: _ } => Self::Node {
                style: StyleComponents::default(),
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
            BevyTemplateNode::Node { style, children } => {
                let children = children
                    .iter()
                    .map(|child| child.spawn(world))
                    .collect::<Box<[_]>>();
                world
                    .spawn((
                        NodeBundle {
                            style: style.style.clone(),
                            border_color: style.border_color,
                            background_color: style.background_color,
                            transform: style.transform,
                            visibility: style.visibility,
                            z_index: style.z_index,
                            ..default()
                        },
                        style.outline,
                    ))
                    .push_children(&children)
                    .id()
            }
            BevyTemplateNode::TextNode {
                text,
                style,
                children,
            } => {
                let children = children
                    .iter()
                    .map(|child| child.spawn(world))
                    .collect::<Box<[_]>>();
                world
                    .spawn(NodeBundle {
                        border_color: style.border_color,
                        ..default()
                    })
                    .insert((
                        TextBundle {
                            text: text.clone(),
                            style: style.style.clone(),
                            background_color: style.background_color,
                            transform: style.transform,
                            visibility: style.visibility,
                            z_index: style.z_index,
                            ..default()
                        },
                        style.outline,
                    ))
                    .push_children(&children)
                    .id()
            }
            BevyTemplateNode::ImageNode {
                image,
                style,
                children,
            } => {
                let children = children
                    .iter()
                    .map(|child| child.spawn(world))
                    .collect::<Box<[_]>>();
                world
                    .spawn(NodeBundle {
                        border_color: style.border_color,
                        ..default()
                    })
                    .insert((
                        ImageBundle {
                            image: image.clone(),
                            style: style.style.clone(),
                            background_color: style.background_color,
                            transform: style.transform,
                            visibility: style.visibility,
                            z_index: style.z_index,
                            ..default()
                        },
                        style.outline,
                    ))
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

fn parse_template_attributes(
    attributes: &[TemplateAttribute],
    background_color: Color,
    asset_server: &AssetServer,
) -> (StyleComponents, Text, UiImage) {
    let mut style = StyleComponents {
        background_color: BackgroundColor(background_color),
        ..default()
    };
    let mut text = Text::from_section("", TextStyle::default());
    let mut image = UiImage::default();
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
                &mut style.style,
                &mut style.border_color,
                &mut style.outline,
                &mut style.background_color,
                &mut style.transform,
                &mut style.visibility,
                &mut style.z_index,
                Some(&mut text),
                Some(&mut image),
                asset_server,
            );
        }
    }
    (style, text, image)
}

#[derive(Default)]
struct StyleComponents {
    style: Style,
    border_color: BorderColor,
    outline: Outline,
    background_color: BackgroundColor,
    transform: Transform,
    visibility: Visibility,
    z_index: ZIndex,
}
