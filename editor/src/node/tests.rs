use std::vec;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use render::NodeRenderer;
use types::Type;

use super::*;

#[test]
fn test_render_builtin_node() {
    let r#u8 = Type::U(8);
    let u8_varray = Type::VArray(&r#u8);

    let mut node = test_node();
    node.port_configuration
        .add_output_port(Port::primary(&u8_varray));

    let renderer = NodeRenderer::new(&DisplayOptions {
        show_type_hints: true,
    })
    .with_node(&node);

    let minimum_size = renderer.get_minimum_size();

    let mut render = Buffer::empty(Rect::new(0, 0, minimum_size.width, minimum_size.height));
    renderer.render(render.area, &mut render);

    println!("{render:?}");
}

#[test]
fn test_minimum_width() {
    // Test Cases

    const TEST_TYPE_1: Type = Type::U(8);
    const TEST_TYPE_2: Type = Type::VArray(&TEST_TYPE_1);
    let test_cases = [
        (("_", None), " ┃ _ ┃ "),
        (
            ("_", Some(PortConfiguration::new(vec![], vec![]))),
            " ┃ _ ┃ ",
        ),
        (
            (
                "_",
                Some(PortConfiguration::new(
                    vec![Port::new(&TEST_TYPE_1, "Foo".into())],
                    vec![Port::new(&TEST_TYPE_2, "Bar".into())],
                )),
            ),
            "◈┫u8   _ u8[]┣◈",
        ),
    ];

    // Rendering

    let mut node = test_node();

    let renderer = NodeRenderer::new(&DisplayOptions {
        show_type_hints: true,
    });

    let target_line = |node: &Node, line: usize| {
        let node_renderer = renderer.with_node(&node);
        let minimum_size = node_renderer.get_minimum_size();

        let mut buffer = Buffer::empty(Rect::new(0, 0, minimum_size.width, minimum_size.height));
        node_renderer.render(buffer.area, &mut buffer);

        get_buffer_line(&buffer, line)
    };

    // Assertions

    for ((alias, pc), expected) in test_cases {
        node.alias = Some(alias.into());
        if let Some(ref pc) = pc {
            node.port_configuration = pc.clone();
        }

        let rendered = target_line(&node, 1);
        assert_eq!(rendered, expected, "Given alias '{alias}' and pc '{pc:?}'");
    }
}

// Util

fn test_node<'a>() -> Node<'a> {
    Node {
        ty: NodeType::Builtin(BuiltinType::COMMENT),
        alias: None,
        port_configuration: PortConfiguration::new(vec![], vec![]),
    }
}

fn get_buffer_line(buf: &Buffer, line: usize) -> String {
    buf.content()
        .chunks(buf.area.width as usize)
        .nth(line)
        .unwrap()
        .iter()
        .map(|c| c.symbol())
        .collect::<String>()
}
