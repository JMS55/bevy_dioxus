mod apply_mutations;
mod deferred_system;
mod hooks;
mod tick;

use self::{
    apply_mutations::BevyTemplate,
    deferred_system::DeferredSystemRegistry,
    tick::{tick_dioxus_ui, VirtualDomUnsafe},
};
use bevy::{
    app::{App, Plugin, Update},
    ecs::{bundle::Bundle, component::Component, entity::Entity},
    ui::node_bundles::NodeBundle,
    utils::HashMap,
};
use dioxus::core::{Element, ElementId, Scope};

pub use self::{
    deferred_system::DeferredSystem,
    hooks::{DioxusUiHooks, DioxusUiQuery},
};
pub use dioxus;

pub struct DioxusUiPlugin;

impl Plugin for DioxusUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeferredSystemRegistry>()
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
    element_id_to_bevy_ui_entity: HashMap<ElementId, Entity>,
    templates: HashMap<String, BevyTemplate>,
    needs_rebuild: bool,
}

impl DioxusUiRoot {
    pub fn new(root_component: fn(Scope) -> Element) -> Self {
        Self {
            virtual_dom: VirtualDomUnsafe::new(root_component),
            element_id_to_bevy_ui_entity: HashMap::new(),
            templates: HashMap::new(),
            needs_rebuild: true,
        }
    }
}
