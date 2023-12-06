use super::DioxusUiRoot;
use bevy::{
    ecs::{
        entity::Entity,
        system::{CommandQueue, Commands},
        world::World,
    },
    prelude::{Deref, DerefMut},
    utils::synccell::SyncCell,
};
use dioxus_core::{Mutations, VirtualDom};
use std::{cell::RefCell, rc::Rc};

pub fn tick_dioxus_ui(world: &mut World) {
    unsafe {
        let world_cell = world.as_unsafe_world_cell();

        let apply_mutations = |mutations: Mutations, bevy_ui_root: Entity| {
            todo!("Modify bevy_ui entities based on mutations");
        };

        let mut command_queue = CommandQueue::default();

        for mut dioxus_ui_root in world_cell
            .world_mut()
            .query::<&mut DioxusUiRoot>()
            .iter_mut(world_cell.world_mut())
        {
            dioxus_ui_root
                .virtual_dom
                .get()
                .base_scope()
                .provide_context(EcsContext {
                    world_read_only: std::mem::transmute(world_cell.world()),
                    commands: Rc::new(RefCell::new(Commands::new(
                        std::mem::transmute(&mut command_queue),
                        std::mem::transmute(world_cell.world()),
                    ))),
                });

            let bevy_ui_root = dioxus_ui_root.root_entity.unwrap_or_else(|| {
                // TODO: Spawn bevy_ui_root as a child of dioxus_ui_root
                let bevy_ui_root = world_cell.world_mut().spawn(()).id();
                apply_mutations(dioxus_ui_root.virtual_dom.get().rebuild(), bevy_ui_root);
                bevy_ui_root
            });

            // TODO: Handle events from winit
            // dioxus_ui_root
            //     .virtual_dom
            //     .get()
            //     .handle_event(todo!(), todo!(), todo!(), todo!());

            apply_mutations(
                dioxus_ui_root.virtual_dom.get().render_immediate(),
                bevy_ui_root,
            );
        }

        command_queue.apply(world);
    }
}

#[derive(Clone)]
struct EcsContext {
    world_read_only: &'static World,
    commands: Rc<RefCell<Commands<'static, 'static>>>,
}

#[derive(Deref, DerefMut)]
pub struct VirtualDomUnsafe(pub SyncCell<VirtualDom>);
unsafe impl Send for VirtualDomUnsafe {}
