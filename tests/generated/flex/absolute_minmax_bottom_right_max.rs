#[test]
#[allow(non_snake_case)]
fn absolute_minmax_bottom_right_max__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(100f32),
                height: taffy::style::Dimension::Length(100f32),
            },
            max_size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(40f32),
                height: taffy::style::Dimension::Length(30f32),
            },
            inset: taffy::geometry::Rect {
                left: auto(),
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: auto(),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(100f32),
                    height: taffy::style::Dimension::Length(100f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node0, 30f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node0, 50f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node0, 60f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn absolute_minmax_bottom_right_max__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(100f32),
                height: taffy::style::Dimension::Length(100f32),
            },
            max_size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(40f32),
                height: taffy::style::Dimension::Length(30f32),
            },
            inset: taffy::geometry::Rect {
                left: auto(),
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: auto(),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(100f32),
                    height: taffy::style::Dimension::Length(100f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node0, 30f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node0, 50f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node0, 60f32, location.y);
}
