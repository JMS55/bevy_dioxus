use bevy::{
    app::{App, Startup},
    core_pipeline::{clear_color::ClearColor, core_2d::Camera2dBundle},
    ecs::system::{Commands, ResMut},
    render::color::Color,
    ui::node_bundles::NodeBundle,
    DefaultPlugins,
};
use bevy_dioxus::{
    bevy_mod_picking::DefaultPickingPlugins, dioxus::prelude::*, hooks::use_system, DioxusUiBundle,
    DioxusUiPlugin, DioxusUiRoot,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DioxusUiPlugin, DefaultPickingPlugins))
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
    let mut count = use_state(cx, || 0);
    let change_clear_color = use_system(cx, |mut clear_color: ResMut<ClearColor>| {
        clear_color.0 = Color::RED;
    });

    render!(
        div {
            onclick: move |_| {
                count += 1;
                change_clear_color.schedule();
            },
            "Count: {count}"
        }
    )
}
