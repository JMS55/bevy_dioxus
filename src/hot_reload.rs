use crate::prelude::dioxus_elements;
use bevy::ecs::world::World;
use dioxus::core::{Template, VirtualDom};
use dioxus_hot_reload::{connect, HotReloadMsg};
use dioxus_rsx::HotReloadingContext;
use std::sync::mpsc::{channel, Receiver};

pub fn update_templates(world: &mut World, virtual_dom: &mut VirtualDom) {
    if !world.contains_non_send::<Receiver<Template<'static>>>() {
        let (updated_templates_sender, updated_templates_receiver) = channel();
        connect(move |msg| match msg {
            HotReloadMsg::UpdateTemplate(updated_templated) => {
                let _ = updated_templates_sender.send(updated_templated);
            }
            HotReloadMsg::Shutdown => {}
        });
        world.insert_non_send_resource(updated_templates_receiver);
    }

    let updated_templates_receiver = world.non_send_resource_mut::<Receiver<Template<'static>>>();
    while let Ok(updated_templated) = updated_templates_receiver.try_recv() {
        virtual_dom.replace_template(updated_templated);
    }
}

pub struct HotReloadContext;

impl HotReloadingContext for HotReloadContext {
    fn map_attribute(
        element_name_rust: &str,
        attribute_name_rust: &str,
    ) -> Option<(&'static str, Option<&'static str>)> {
        if element_name_rust == dioxus_elements::text::TAG_NAME {
            let attribute = match attribute_name_rust {
                "text" => Some(("text", None)),
                "text_direction" => Some(("text_direction", None)),
                "text_alignment" => Some(("text_alignment", None)),
                "text_size" => Some(("text_size", None)),
                "text_color" => Some(("text_color", None)),
                _ => None,
            };
            if let Some(attribute) = attribute {
                return Some(attribute);
            }
        }
        if let dioxus_elements::node::TAG_NAME | dioxus_elements::text::TAG_NAME = element_name_rust
        {
            match attribute_name_rust {
                "animate" => Some(("animate", None)),
                "display" => Some(("display", None)),
                "position" => Some(("position", None)),
                "overflow" => Some(("overflow", None)),
                "overflow_x" => Some(("overflow_x", None)),
                "overflow_y" => Some(("overflow_y", None)),
                "left" => Some(("left", None)),
                "right" => Some(("right", None)),
                "top" => Some(("top", None)),
                "bottom" => Some(("bottom", None)),
                "width" => Some(("width", None)),
                "height" => Some(("height", None)),
                "min_width" => Some(("min_width", None)),
                "min_height" => Some(("min_height", None)),
                "aspect_ratio" => Some(("aspect_ratio", None)),
                "align_items" => Some(("align_items", None)),
                "justify_items" => Some(("justify_items", None)),
                "align_self" => Some(("align_self", None)),
                "justify_self" => Some(("justify_self", None)),
                "align_content" => Some(("align_content", None)),
                "justify_content" => Some(("justify_content", None)),
                "margin" => Some(("margin", None)),
                "margin_left" => Some(("margin_left", None)),
                "margin_right" => Some(("margin_right", None)),
                "margin_top" => Some(("margin_top", None)),
                "margin_bottom" => Some(("margin_bottom", None)),
                "padding" => Some(("padding", None)),
                "padding_left" => Some(("padding_left", None)),
                "padding_right" => Some(("padding_right", None)),
                "padding_top" => Some(("padding_top", None)),
                "padding_bottom" => Some(("padding_bottom", None)),
                "border_width" => Some(("border_width", None)),
                "border_width_left" => Some(("border_width_left", None)),
                "border_width_right" => Some(("border_width_right", None)),
                "border_width_top" => Some(("border_width_top", None)),
                "border_width_bottom" => Some(("border_width_bottom", None)),
                "border_color" => Some(("border_color", None)),
                "outline_width" => Some(("outline_width", None)),
                "outline_offset" => Some(("outline_offset", None)),
                "outline_color" => Some(("outline_color", None)),
                "flex_direction" => Some(("flex_direction", None)),
                "flex_wrap" => Some(("flex_wrap", None)),
                "flex_grow" => Some(("flex_grow", None)),
                "flex_shrink" => Some(("flex_shrink", None)),
                "flex_basis" => Some(("flex_basis", None)),
                "row_gap" => Some(("row_gap", None)),
                "column_gap" => Some(("column_gap", None)),
                "grid_auto_flow" => Some(("grid_auto_flow", None)),
                "grid_template_rows" => Some(("grid_template_rows", None)),
                "grid_template_columns" => Some(("grid_template_columns", None)),
                "grid_auto_rows" => Some(("grid_auto_rows", None)),
                "grid_auto_columns" => Some(("grid_auto_columns", None)),
                "grid_row" => Some(("grid_row", None)),
                "grid_column" => Some(("grid_column", None)),
                "background_color" => Some(("background_color", None)),
                "translation" => Some(("translation", None)),
                "translation_x" => Some(("translation_x", None)),
                "translation_y" => Some(("translation_y", None)),
                "rotation" => Some(("rotation", None)),
                "scale" => Some(("scale", None)),
                "scale_x" => Some(("scale_x", None)),
                "scale_y" => Some(("scale_y", None)),
                "visibility" => Some(("visibility", None)),
                "z_index" => Some(("z_index", None)),
                _ => None,
            }
        } else {
            None
        }
    }

    fn map_element(element_name_rust: &str) -> Option<(&'static str, Option<&'static str>)> {
        match element_name_rust {
            dioxus_elements::node::TAG_NAME => Some((
                dioxus_elements::node::TAG_NAME,
                dioxus_elements::node::NAME_SPACE,
            )),
            dioxus_elements::text::TAG_NAME => Some((
                dioxus_elements::text::TAG_NAME,
                dioxus_elements::text::NAME_SPACE,
            )),
            _ => None,
        }
    }
}
