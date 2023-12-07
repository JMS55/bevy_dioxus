mod apply_mutations;
mod bsn;
mod deferred_system;
mod hooks;
mod tick;

use self::{
    bsn::Bsn,
    deferred_system::DeferredSystemRunQueue,
    tick::{tick_dioxus_ui, VirtualDomUnsafe},
};
use bevy::{
    app::{App, Plugin, Update},
    ecs::{component::Component, entity::Entity},
    utils::HashMap,
};
use dioxus_core::ElementId;

pub use self::hooks::DioxusUiHooks;
pub use dioxus_core::{Element, Scope};

pub struct DioxusUiPlugin;

impl Plugin for DioxusUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeferredSystemRunQueue>()
            .add_systems(Update, tick_dioxus_ui);
    }
}

#[derive(Component)]
pub struct DioxusUiRoot {
    virtual_dom: VirtualDomUnsafe,
    element_id_to_bevy_ui_entity: HashMap<ElementId, Entity>,
    templates: HashMap<String, Bsn>,
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
