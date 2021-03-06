fn main() {
    let node = stretch::style::Node {
        size: stretch::geometry::Size {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
        },
        justify_content: stretch::style::JustifyContent::Center,

        children: vec![stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Percent(0.5),
                height: stretch::style::Dimension::Auto,
            },
            ..Default::default()
        }],

        ..Default::default()
    };

    let layout = stretch::compute(&node);

    println!("{:#?}", layout);
}
