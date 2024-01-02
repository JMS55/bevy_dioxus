use bevy::{
    asset::{AssetPath, AssetServer},
    math::Quat,
    render::{color::Color, view::Visibility},
    text::{Text, TextAlignment},
    transform::components::Transform,
    ui::*,
};
use std::f32::consts::PI;

#[allow(clippy::too_many_arguments)]
pub fn set_attribute(
    name: &str,
    value: &str,
    style: &mut Style,
    border_color: &mut BorderColor,
    outline: &mut Outline,
    background_color: &mut BackgroundColor,
    transform: &mut Transform,
    visibility: &mut Visibility,
    z_index: &mut ZIndex,
    text: Option<&mut Text>,
    image: Option<&mut UiImage>,
    asset_server: &AssetServer,
) {
    #[allow(unused_variables, unreachable_code)]
    match (name, value) {
        ("animate", value) => todo!(),
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
        ("left", value) => style.left = parse_val(value),
        ("right", value) => style.right = parse_val(value),
        ("top", value) => style.top = parse_val(value),
        ("bottom", value) => style.bottom = parse_val(value),
        ("width", value) => style.width = parse_val(value),
        ("height", value) => style.height = parse_val(value),
        ("min_width", value) => style.min_width = parse_val(value),
        ("min_height", value) => style.min_height = parse_val(value),
        ("max_width", value) => style.max_width = parse_val(value),
        ("max_height", value) => style.max_height = parse_val(value),
        ("aspect_ratio", "none") => style.aspect_ratio = None,
        ("aspect_ratio", value) => style.aspect_ratio = Some(parse_f32(value)),
        ("align_items", "default") => style.align_items = AlignItems::Default,
        ("align_items", "start") => style.align_items = AlignItems::Start,
        ("align_items", "end") => style.align_items = AlignItems::End,
        ("align_items", "flex_start") => style.align_items = AlignItems::FlexStart,
        ("align_items", "flex_end") => style.align_items = AlignItems::FlexEnd,
        ("align_items", "center") => style.align_items = AlignItems::Center,
        ("align_items", "baseline") => style.align_items = AlignItems::Baseline,
        ("align_items", "stretch") => style.align_items = AlignItems::Stretch,
        ("justify_items", "default") => style.justify_items = JustifyItems::Default,
        ("justify_items", "start") => style.justify_items = JustifyItems::Start,
        ("justify_items", "end") => style.justify_items = JustifyItems::End,
        ("justify_items", "center") => style.justify_items = JustifyItems::Center,
        ("justify_items", "baseline") => style.justify_items = JustifyItems::Baseline,
        ("justify_items", "stretch") => style.justify_items = JustifyItems::Stretch,
        ("align_self", "auto") => style.align_self = AlignSelf::Auto,
        ("align_self", "start") => style.align_self = AlignSelf::Start,
        ("align_self", "end") => style.align_self = AlignSelf::End,
        ("align_self", "flex_start") => style.align_self = AlignSelf::FlexStart,
        ("align_self", "flex_end") => style.align_self = AlignSelf::FlexEnd,
        ("align_self", "center") => style.align_self = AlignSelf::Center,
        ("align_self", "baseline") => style.align_self = AlignSelf::Baseline,
        ("align_self", "stretch") => style.align_self = AlignSelf::Stretch,
        ("justify_self", "auto") => style.justify_self = JustifySelf::Auto,
        ("justify_self", "start") => style.justify_self = JustifySelf::Start,
        ("justify_self", "end") => style.justify_self = JustifySelf::End,
        ("justify_self", "center") => style.justify_self = JustifySelf::Center,
        ("justify_self", "baseline") => style.justify_self = JustifySelf::Baseline,
        ("justify_self", "stretch") => style.justify_self = JustifySelf::Stretch,
        ("align_content", "default") => style.align_content = AlignContent::Default,
        ("align_content", "start") => style.align_content = AlignContent::Start,
        ("align_content", "end") => style.align_content = AlignContent::End,
        ("align_content", "flex_start") => style.align_content = AlignContent::FlexStart,
        ("align_content", "flex_end") => style.align_content = AlignContent::FlexEnd,
        ("align_content", "center") => style.align_content = AlignContent::Center,
        ("align_content", "stretch") => style.align_content = AlignContent::Stretch,
        ("align_content", "space_between") => style.align_content = AlignContent::SpaceBetween,
        ("align_content", "space_evenly") => style.align_content = AlignContent::SpaceEvenly,
        ("align_content", "space_around") => style.align_content = AlignContent::SpaceAround,
        ("justify_content", "default") => style.justify_content = JustifyContent::Default,
        ("justify_content", "start") => style.justify_content = JustifyContent::Start,
        ("justify_content", "end") => style.justify_content = JustifyContent::End,
        ("justify_content", "flex_start") => style.justify_content = JustifyContent::FlexStart,
        ("justify_content", "flex_end") => style.justify_content = JustifyContent::FlexEnd,
        ("justify_content", "center") => style.justify_content = JustifyContent::Center,
        ("justify_content", "stretch") => style.justify_content = JustifyContent::Stretch,
        ("justify_content", "space_between") => {
            style.justify_content = JustifyContent::SpaceBetween;
        }
        ("justify_content", "space_evenly") => style.justify_content = JustifyContent::SpaceEvenly,
        ("justify_content", "space_around") => style.justify_content = JustifyContent::SpaceAround,
        ("margin", value) => style.margin = UiRect::all(parse_val(value)),
        ("margin_left", value) => style.margin.left = parse_val(value),
        ("margin_right", value) => style.margin.right = parse_val(value),
        ("margin_top", value) => style.margin.top = parse_val(value),
        ("margin_bottom", value) => style.margin.bottom = parse_val(value),
        ("padding", value) => style.padding = UiRect::all(parse_val(value)),
        ("padding_left", value) => style.padding.left = parse_val(value),
        ("padding_right", value) => style.padding.right = parse_val(value),
        ("padding_top", value) => style.padding.top = parse_val(value),
        ("padding_bottom", value) => style.padding.bottom = parse_val(value),
        ("border_width", value) => style.border = UiRect::all(parse_val(value)),
        ("border_width_left", value) => style.border.left = parse_val(value),
        ("border_width_right", value) => style.border.right = parse_val(value),
        ("border_width_top", value) => style.border.top = parse_val(value),
        ("border_width_bottom", value) => style.border.bottom = parse_val(value),
        ("border_color", value) => border_color.0 = parse_color(value),
        ("outline_width", value) => outline.width = parse_val(value),
        ("outline_offset", value) => outline.offset = parse_val(value),
        ("outline_color", value) => outline.color = parse_color(value),
        ("flex_direction", "row") => style.flex_direction = FlexDirection::Row,
        ("flex_direction", "column") => style.flex_direction = FlexDirection::Column,
        ("flex_direction", "row_reverse") => style.flex_direction = FlexDirection::RowReverse,
        ("flex_direction", "column_reverse") => style.flex_direction = FlexDirection::ColumnReverse,
        ("flex_wrap", "no_wrap") => style.flex_wrap = FlexWrap::NoWrap,
        ("flex_wrap", "wrap") => style.flex_wrap = FlexWrap::Wrap,
        ("flex_wrap", "wrap_reverse") => style.flex_wrap = FlexWrap::WrapReverse,
        ("flex_grow", value) => style.flex_grow = parse_f32(value),
        ("flex_shrink", value) => style.flex_shrink = parse_f32(value),
        ("flex_basis", value) => style.flex_basis = parse_val(value),
        ("row_gap", value) => style.row_gap = parse_val(value),
        ("column_gap", value) => style.column_gap = parse_val(value),
        ("grid_auto_flow", "row") => style.grid_auto_flow = GridAutoFlow::Row,
        ("grid_auto_flow", "column") => style.grid_auto_flow = GridAutoFlow::Column,
        ("grid_auto_flow", "row_dense") => style.grid_auto_flow = GridAutoFlow::RowDense,
        ("grid_auto_flow", "column_dense") => style.grid_auto_flow = GridAutoFlow::ColumnDense,
        ("grid_template_rows", value) => {
            style.grid_template_rows = todo!();
        }
        ("grid_template_columns", value) => {
            style.grid_template_columns = todo!();
        }
        ("grid_auto_rows", value) => {
            style.grid_auto_rows = todo!();
        }
        ("grid_auto_columns", value) => {
            style.grid_auto_columns = todo!();
        }
        ("grid_row", value) => {
            style.grid_row = todo!();
        }
        ("grid_column", value) => {
            style.grid_column = todo!();
        }
        ("background_color", value) => background_color.0 = parse_color(value),
        ("translation", value) => {
            let value = parse_f32(value);
            transform.translation.x = value;
            transform.translation.y = value;
        }
        ("translation_x", value) => transform.translation.x = parse_f32(value),
        ("translation_y", value) => transform.translation.y = parse_f32(value),
        ("rotation", value) => {
            transform.rotation = Quat::from_rotation_y(parse_f32(value) * (180.0 / PI));
        }
        ("scale", value) => {
            let value = parse_f32(value);
            transform.scale.x = value;
            transform.scale.y = value;
        }
        ("scale_x", value) => transform.scale.x = parse_f32(value),
        ("scale_y", value) => transform.scale.y = parse_f32(value),
        ("visibility", "inherited") => *visibility = Visibility::Inherited,
        ("visibility", "hidden") => *visibility = Visibility::Hidden,
        ("visibility", "visible") => *visibility = Visibility::Visible,
        ("z_index", value) => match value.split_once(':') {
            Some(("local", value)) => *z_index = ZIndex::Local(parse_i32(value)),
            Some(("global", value)) => *z_index = ZIndex::Global(parse_i32(value)),
            None => *z_index = ZIndex::Local(parse_i32(value)),
            _ => panic!("Encountered invalid bevy_dioxus ZIndex `{value}`."),
        },
        ("text", value) if text.is_some() => text.unwrap().sections[0].value = value.to_owned(),
        ("text_direction", "inherit") if text.is_some() => style.direction = Direction::Inherit,
        ("text_direction", "left_to_right") if text.is_some() => {
            style.direction = Direction::LeftToRight;
        }
        ("text_direction", "right_to_left") if text.is_some() => {
            style.direction = Direction::RightToLeft;
        }
        ("text_multiline_alignment", "left") if text.is_some() => {
            text.unwrap().alignment = TextAlignment::Left;
        }
        ("text_multiline_alignment", "center") if text.is_some() => {
            text.unwrap().alignment = TextAlignment::Center;
        }
        ("text_multiline_alignment", "right") if text.is_some() => {
            text.unwrap().alignment = TextAlignment::Right;
        }
        ("text_size", value) if text.is_some() => {
            text.unwrap().sections[0].style.font_size = parse_f32(value);
        }
        ("text_color", value) if text.is_some() => {
            text.unwrap().sections[0].style.color = parse_color(value);
        }
        ("image_asset_path", value) if image.is_some() => {
            image.unwrap().texture = asset_server.load(AssetPath::parse(value));
        }
        _ => panic!("Encountered unsupported bevy_dioxus attribute `{name}: {value}`."),
    }
}

fn parse_color(hex: &str) -> Color {
    Color::hex(hex).unwrap_or_else(|_| panic!("Encountered invalid bevy_dioxus Color hex `{hex}`."))
}

fn parse_f32(float: &str) -> f32 {
    float
        .parse::<f32>()
        .unwrap_or_else(|val| panic!("Encountered invalid bevy_dioxus f32 `{val}`."))
}

fn parse_i32(int: &str) -> i32 {
    int.parse::<i32>()
        .unwrap_or_else(|val| panic!("Encountered invalid bevy_dioxus i32 `{val}`."))
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
