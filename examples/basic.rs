use bevy::{
    app::{App, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::Commands,
    ui::node_bundles::NodeBundle,
    DefaultPlugins,
};
use bevy_dioxus::{dioxus::prelude::*, DioxusUiBundle, DioxusUiPlugin, DioxusUiRoot};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DioxusUiPlugin))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(DioxusUiBundle {
                dioxus_ui_root: DioxusUiRoot::new(ui_root),
                node_bundle: NodeBundle::default(),
            });
            commands.spawn(Camera2dBundle::default());
        })
        .run();
}

fn ui_root(cx: Scope) -> Element {
    render!("Hello")
}
