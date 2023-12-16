mod apply_mutations;
pub mod colors;
mod deferred_system;
mod events;
pub mod hooks;
mod tick;

use self::{
    apply_mutations::BevyTemplate, deferred_system::DeferredSystemRegistry, events::EventReaders,
    hooks::EcsSubscriptions, tick::tick_dioxus_ui,
};
use bevy::{
    app::{App, Plugin, Update},
    ecs::{bundle::Bundle, component::Component, entity::Entity},
    prelude::{Deref, DerefMut},
    ui::node_bundles::NodeBundle,
    utils::{EntityHashMap, HashMap},
};
use dioxus::core::{Element, ElementId, Scope, VirtualDom};

pub use bevy_mod_picking;
pub use dioxus;

pub struct DioxusUiPlugin;

impl Plugin for DioxusUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<UiRoots>()
            .init_resource::<EcsSubscriptions>()
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

#[derive(Component, Deref, Clone, Copy)]
pub struct DioxusUiRoot(fn(Scope) -> Element);

impl DioxusUiRoot {
    pub fn new(root_component: fn(Scope) -> Element) -> Self {
        Self(root_component)
    }
}

#[derive(Deref, DerefMut, Default)]
struct UiRoots(EntityHashMap<Entity, UiRoot>);

struct UiRoot {
    virtual_dom: VirtualDom,
    element_id_to_bevy_ui_entity: HashMap<ElementId, Entity>,
    bevy_ui_entity_to_element_id: EntityHashMap<Entity, ElementId>,
    templates: HashMap<String, BevyTemplate>,
    needs_rebuild: bool,
}

impl UiRoot {
    fn new(root_component: fn(Scope) -> Element) -> Self {
        Self {
            virtual_dom: VirtualDom::new(root_component),
            element_id_to_bevy_ui_entity: HashMap::new(),
            bevy_ui_entity_to_element_id: EntityHashMap::default(),
            templates: HashMap::new(),
            needs_rebuild: true,
        }
    }
}
