use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use render::get_minimum_size;

use super::*;

#[test]
fn render_builtin_node() {
    let node = Node {
        ty: NodeType::Builtin(BuiltinType::ENTRY),
        port_rendering_strategy: PortRenderingStrategy::Inline,
        input_port_count: 1,
        output_port_count: 3,
        alias: None,
    };

    let minimum_size = get_minimum_size(&node);

    let mut render = Buffer::empty(Rect::new(0, 0, minimum_size.width, minimum_size.height));
    node.render(render.area, &mut render);

    println!("{render:?}");
}
