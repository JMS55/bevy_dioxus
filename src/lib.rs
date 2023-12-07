mod apply_mutations;
mod deferred_system;
mod hooks;
mod tick;

use self::{
    deferred_system::DeferredSystemRunQueue,
    tick::{tick_dioxus_ui, VirtualDomUnsafe},
};
use bevy::{
    app::{App, Plugin, Update},
    ecs::component::Component,
};

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
    needs_rebuild: bool,
}

impl DioxusUiRoot {
    pub fn new(root_component: fn(Scope) -> Element) -> Self {
        Self {
            virtual_dom: VirtualDomUnsafe::new(root_component),
            needs_rebuild: true,
        }
    }
}
