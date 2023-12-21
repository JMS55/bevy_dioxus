use bevy::{
    render::color::Color,
    text::{Text, TextAlignment},
    ui::*,
};

pub fn set_attribute(
    name: &str,
    value: &str,
    style: &mut Style,
    background_color: &mut BackgroundColor,
    text: Option<&mut Text>,
) {
    match (name, value) {
        ("display", "flex") => style.display = Display::Flex,
        ("display", "grid") => style.display = Display::Grid,
        ("display", "none") => style.display = Display::None,
        ("position", "relative") => style.position_type = PositionType::Relative,
        ("position", "absolute") => style.position_type = PositionType::Absolute,
        ("overflow", "visible") => style.overflow = Overflow::visible(),
        ("overflow", "clip") => style.overflow = Overflow::clip(),
        ("overflow_x", "visible") => style.overflow.x = OverflowAxis::Visible,
        ("overflow_x", "clip") => style.overflow.x = OverflowAxis::Clip,
        ("overflow_y", "visible") => style.overflow.y = OverflowAxis::Visible,
        ("overflow_y", "clip") => style.overflow.y = OverflowAxis::Clip,
        ("left", val) => style.left = parse_val(val),
        ("right", val) => style.right = parse_val(val),
        ("top", val) => style.top = parse_val(val),
        ("bottom", val) => style.bottom = parse_val(val),
        ("width", val) => style.width = parse_val(val),
        ("height", val) => style.height = parse_val(val),
        ("min_width", val) => style.min_width = parse_val(val),
        ("min_height", val) => style.min_height = parse_val(val),
        ("max_width", val) => style.max_width = parse_val(val),
        ("max_height", val) => style.max_height = parse_val(val),
        ("aspect_ratio", "none") => style.aspect_ratio = None,
        ("aspect_ratio", float) => {
            style.aspect_ratio = Some(
                float
                    .parse::<f32>()
                    .unwrap_or_else(|val| panic!("Encountered invalid bevy_dioxus f32 `{val}`.")),
            );
        }
        ("align_items", "default") => style.align_items = AlignItems::Default,
        ("align_items", "start") => style.align_items = AlignItems::Start,
        ("align_items", "end") => style.align_items = AlignItems::End,
        ("align_items", "flex_start") => style.align_items = AlignItems::FlexStart,
        ("align_items", "flex_end") => style.align_items = AlignItems::FlexEnd,
        ("align_items", "center") => style.align_items = AlignItems::Center,
        ("align_items", "baseline") => style.align_items = AlignItems::Baseline,
        ("align_items", "stretch") => style.align_items = AlignItems::Stretch,
        // TODO: The rest of the attributes from here on out
        ("flex_direction", "column") => style.flex_direction = FlexDirection::Column,
        ("background_color", hex) => {
            background_color.0 = Color::hex(hex).expect(&format!(
                "Encountered invalid bevy_dioxus Color hex `{hex}`."
            ));
        }
        ("padding", val) => style.padding = UiRect::all(parse_val(val)),
        ("justify_content", "space_between") => {
            style.justify_content = JustifyContent::SpaceBetween;
        }
        ("align_content", "space_between") => style.align_content = AlignContent::SpaceBetween,
        ("text", new_text) if text.is_some() => text.unwrap().sections[0] = new_text.into(),
        ("text_direction", "inherit") if text.is_some() => style.direction = Direction::Inherit,
        ("text_direction", "left_to_right") if text.is_some() => {
            style.direction = Direction::LeftToRight;
        }
        ("text_direction", "right_to_left") if text.is_some() => {
            style.direction = Direction::RightToLeft;
        }
        ("text_alignment", "left") if text.is_some() => {
            text.unwrap().alignment = TextAlignment::Left;
        }
        ("text_alignment", "center") if text.is_some() => {
            text.unwrap().alignment = TextAlignment::Center;
        }
        ("text_alignment", "right") if text.is_some() => {
            text.unwrap().alignment = TextAlignment::Right;
        }
        ("text_size", val) if text.is_some() => {
            text.unwrap().sections[0].style.font_size = val
                .parse::<f32>()
                .unwrap_or_else(|val| panic!("Encountered invalid bevy_dioxus f32 `{val}`."));
        }
        ("text_color", hex) if text.is_some() => {
            text.unwrap().sections[0].style.color = Color::hex(hex).expect(&format!(
                "Encountered invalid bevy_dioxus Color hex `{hex}`."
            ));
        }
        _ => panic!("Encountered unsupported bevy_dioxus attribute `{name}: {value}`."),
    }
}

fn parse_val(val: &str) -> Val {
    if let Ok(val) = val.parse::<f32>() {
        return Val::Px(val);
    }
    if let Some((val, "")) = val.split_once("px") {
        if let Ok(val) = val.parse::<f32>() {
            return Val::Px(val);
        }
    }
    if let Some((val, "")) = val.split_once("vw") {
        if let Ok(val) = val.parse::<f32>() {
            return Val::Vw(val);
        }
    }
    if let Some((val, "")) = val.split_once("vh") {
        if let Ok(val) = val.parse::<f32>() {
            return Val::Vh(val);
        }
    }
    panic!("Encountered invalid bevy_dioxus Val `{val}`.");
}
