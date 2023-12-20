#[allow(non_camel_case_types, non_upper_case_globals)]
pub mod dioxus_elements {
    pub use crate::events::events;

    const NAME_SPACE: Option<&'static str> = Some("bevy_ui");
    pub type AttributeDescription = (&'static str, Option<&'static str>, bool);

    pub struct node;
    impl node {
        pub const TAG_NAME: &'static str = "node";
        pub const NAME_SPACE: Option<&'static str> = NAME_SPACE;

        // TODO: The rest of Style
        pub const display: AttributeDescription = ("display", None, false);
        pub const position: AttributeDescription = ("position", None, false);
        pub const flex_direction: AttributeDescription = ("flex-direction", None, false);
        pub const background_color: AttributeDescription = ("background-color", None, false);
        pub const padding: AttributeDescription = ("padding", None, false);
        pub const width: AttributeDescription = ("width", None, false);
        pub const height: AttributeDescription = ("height", None, false);
        pub const justify_content: AttributeDescription = ("justify-content", None, false);
        pub const align_content: AttributeDescription = ("align-content", None, false);
    }

    pub struct text;
    impl text {
        pub const TAG_NAME: &'static str = "text";
        pub const NAME_SPACE: Option<&'static str> = NAME_SPACE;

        pub const text: AttributeDescription = ("text", None, false);
        pub const text_alignment: AttributeDescription = ("text-alignment", None, false);
        pub const font_size: AttributeDescription = ("font-size", None, false);
        pub const font_color: AttributeDescription = ("font-color", None, false);
        // TODO: The rest of Style
        // TODO: Reduce duplication
        pub const display: AttributeDescription = ("display", None, false);
        pub const position: AttributeDescription = ("position", None, false);
        pub const flex_direction: AttributeDescription = ("flex-direction", None, false);
        pub const background_color: AttributeDescription = ("background-color", None, false);
        pub const padding: AttributeDescription = ("padding", None, false);
        pub const width: AttributeDescription = ("width", None, false);
        pub const height: AttributeDescription = ("height", None, false);
        pub const justify_content: AttributeDescription = ("justify-content", None, false);
        pub const align_content: AttributeDescription = ("align-content", None, false);
    }
}
