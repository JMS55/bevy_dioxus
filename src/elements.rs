macro_rules! node_attributes {
    () => {
        pub const animate: AttributeDescription = ("animate", None, false);
        pub const display: AttributeDescription = ("display", None, false);
        pub const position: AttributeDescription = ("position", None, false);
        pub const overflow: AttributeDescription = ("overflow", None, false);
        pub const overflow_x: AttributeDescription = ("overflow_x", None, false);
        pub const overflow_y: AttributeDescription = ("overflow_y", None, false);
        pub const left: AttributeDescription = ("left", None, false);
        pub const right: AttributeDescription = ("right", None, false);
        pub const top: AttributeDescription = ("top", None, false);
        pub const bottom: AttributeDescription = ("bottom", None, false);
        pub const width: AttributeDescription = ("width", None, false);
        pub const height: AttributeDescription = ("height", None, false);
        pub const min_width: AttributeDescription = ("min_width", None, false);
        pub const min_height: AttributeDescription = ("min_height", None, false);
        pub const aspect_ratio: AttributeDescription = ("aspect_ratio", None, false);
        pub const align_items: AttributeDescription = ("align_items", None, false);
        pub const justify_items: AttributeDescription = ("justify_items", None, false);
        pub const align_self: AttributeDescription = ("align_self", None, false);
        pub const justify_self: AttributeDescription = ("justify_self", None, false);
        pub const align_content: AttributeDescription = ("align_content", None, false);
        pub const justify_content: AttributeDescription = ("justify_content", None, false);
        pub const margin: AttributeDescription = ("margin", None, false);
        pub const margin_left: AttributeDescription = ("margin_left", None, false);
        pub const margin_right: AttributeDescription = ("margin_right", None, false);
        pub const margin_top: AttributeDescription = ("margin_top", None, false);
        pub const margin_bottom: AttributeDescription = ("margin_bottom", None, false);
        pub const padding: AttributeDescription = ("padding", None, false);
        pub const padding_left: AttributeDescription = ("padding_left", None, false);
        pub const padding_right: AttributeDescription = ("padding_right", None, false);
        pub const padding_top: AttributeDescription = ("padding_top", None, false);
        pub const padding_bottom: AttributeDescription = ("padding_bottom", None, false);
        pub const border_width: AttributeDescription = ("border_width", None, false);
        pub const border_width_left: AttributeDescription = ("border_width_left", None, false);
        pub const border_width_right: AttributeDescription = ("border_width_right", None, false);
        pub const border_width_top: AttributeDescription = ("border_width_top", None, false);
        pub const border_width_bottom: AttributeDescription = ("border_width_bottom", None, false);
        pub const border_color: AttributeDescription = ("border_color", None, false);
        pub const outline_width: AttributeDescription = ("outline_width", None, false);
        pub const outline_offset: AttributeDescription = ("outline_offset", None, false);
        pub const outline_color: AttributeDescription = ("outline_color", None, false);
        pub const flex_direction: AttributeDescription = ("flex_direction", None, false);
        pub const flex_wrap: AttributeDescription = ("flex_wrap", None, false);
        pub const flex_grow: AttributeDescription = ("flex_grow", None, false);
        pub const flex_shrink: AttributeDescription = ("flex_shrink", None, false);
        pub const flex_basis: AttributeDescription = ("flex_basis", None, false);
        pub const row_gap: AttributeDescription = ("row_gap", None, false);
        pub const column_gap: AttributeDescription = ("column_gap", None, false);
        pub const grid_auto_flow: AttributeDescription = ("grid_auto_flow", None, false);
        pub const grid_template_rows: AttributeDescription = ("grid_template_rows", None, false);
        pub const grid_template_columns: AttributeDescription =
            ("grid_template_columns", None, false);
        pub const grid_auto_rows: AttributeDescription = ("grid_auto_rows", None, false);
        pub const grid_auto_columns: AttributeDescription = ("grid_auto_columns", None, false);
        pub const grid_row: AttributeDescription = ("grid_row", None, false);
        pub const grid_column: AttributeDescription = ("grid_column", None, false);
        pub const background_color: AttributeDescription = ("background_color", None, false);
        pub const translation: AttributeDescription = ("translation", None, false);
        pub const translation_x: AttributeDescription = ("translation", None, false);
        pub const translation_y: AttributeDescription = ("translation", None, false);
        pub const rotation: AttributeDescription = ("rotation", None, false);
        pub const scale: AttributeDescription = ("scale", None, false);
        pub const scale_x: AttributeDescription = ("scale_x", None, false);
        pub const scale_y: AttributeDescription = ("scale_y", None, false);
        pub const visibility: AttributeDescription = ("visibility", None, false);
        pub const z_index: AttributeDescription = ("z_index", None, false);
    };
}

#[allow(non_camel_case_types, non_upper_case_globals)]
pub mod dioxus_elements {
    pub use crate::events::events;

    pub type AttributeDescription = (&'static str, Option<&'static str>, bool);
    const NAME_SPACE: Option<&'static str> = Some("bevy_ui");

    pub struct node;
    impl node {
        pub const TAG_NAME: &'static str = "node";
        pub const NAME_SPACE: Option<&'static str> = NAME_SPACE;
        node_attributes!();
    }

    pub struct text;
    impl text {
        pub const TAG_NAME: &'static str = "text";
        pub const NAME_SPACE: Option<&'static str> = NAME_SPACE;
        pub const text: AttributeDescription = ("text", None, false);
        pub const text_direction: AttributeDescription = ("text_direction", None, false);
        pub const text_multiline_justification: AttributeDescription =
            ("text_multiline_justification", None, false);
        pub const text_size: AttributeDescription = ("text_size", None, false);
        pub const text_color: AttributeDescription = ("text_color", None, false);
        node_attributes!();
    }

    pub struct image;
    impl image {
        pub const TAG_NAME: &'static str = "image";
        pub const NAME_SPACE: Option<&'static str> = NAME_SPACE;
        pub const image_asset_path: AttributeDescription = ("image_asset_path", None, false);
        node_attributes!();
    }
}
