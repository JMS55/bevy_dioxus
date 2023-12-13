mod apply_mutations;
mod deferred_system;
mod events;
pub mod hooks;
mod tick;

use self::{
    apply_mutations::BevyTemplate,
    deferred_system::DeferredSystemRegistry,
    events::EventReaders,
    hooks::EcsSubscriptions,
    tick::{tick_dioxus_ui, VirtualDomUnsafe},
};
use bevy::{
    app::{App, Plugin, Update},
    ecs::{bundle::Bundle, component::Component, entity::Entity},
    ui::node_bundles::NodeBundle,
    utils::{EntityHashMap, HashMap},
};
use dioxus::core::{Element, ElementId, Scope};

pub use self::deferred_system::DeferredSystem;
pub use bevy_mod_picking;
pub use dioxus;

pub struct DioxusUiPlugin;

impl Plugin for DioxusUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EcsSubscriptions>()
            .init_resource::<DeferredSystemRegistry>()
            .init_resource::<EventReaders>()
            .add_systems(Update, tick_dioxus_ui);
    }
}

#[derive(Bundle)]
pub struct DioxusUiBundle {
    pub dioxus_ui_root: DioxusUiRoot,
    pub node_bundle: NodeBundle,
}

#[derive(Component)]
pub struct DioxusUiRoot {
    virtual_dom: VirtualDomUnsafe,
    parent_to_children: HashMap<(Entity, u8), Entity>,
    children_to_parent: EntityHashMap<Entity, Entity>,
    element_id_to_bevy_ui_entity: HashMap<ElementId, Entity>,
    bevy_ui_entity_to_element_id: EntityHashMap<Entity, ElementId>,
    templates: HashMap<String, BevyTemplate>,
    needs_rebuild: bool,
}

impl DioxusUiRoot {
    pub fn new(root_component: fn(Scope) -> Element) -> Self {
        Self {
            virtual_dom: VirtualDomUnsafe::new(root_component),
            parent_to_children: HashMap::new(),
            children_to_parent: EntityHashMap::default(),
            element_id_to_bevy_ui_entity: HashMap::new(),
            bevy_ui_entity_to_element_id: EntityHashMap::default(),
            templates: HashMap::new(),
            needs_rebuild: true,
        }
    }
}
