#[test]
fn wrapped_column_max_height() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200f32),
                    height: stretch::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(20f32),
                    end: stretch::style::Dimension::Points(20f32),
                    top: stretch::style::Dimension::Points(20f32),
                    bottom: stretch::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_wrap: stretch::style::FlexWrap::Wrap,
                align_items: stretch::style::AlignItems::Center,
                align_content: stretch::style::AlignContent::Center,
                justify_content: stretch::style::JustifyContent::Center,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(700f32),
                    height: stretch::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 700f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 500f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 200f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 250f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 30f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 200f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 200f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 200f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 250f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 420f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 200f32);
}
