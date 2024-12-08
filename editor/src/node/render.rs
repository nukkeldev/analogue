use ratatui::{prelude::*, widgets::Block};
use symbols::{border, line};

use super::*;

/// The minimum height a node can be. This includes the top border, the name (along with primary ports), and the bottom border.
const MINIMUM_NODE_HEIGHT: u16 = 3;

// WIDGET

impl Widget for &Node {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Ensure the node can be drawn
        let minimum_size = get_minimum_size(self);
        assert!(
            area.width >= minimum_size.width && area.height >= minimum_size.height,
            "Cannot draw node; Supplied area is too small: minimum = {minimum_size}, given = {}.",
            area.as_size()
        );

        // Render the border with one cell of horizontal padding
        render_borders_and_ports(area, buf, self);

        let inner_area = area.inner(Margin::new(2, 1));

        // Render the name (or alias) of the node
        render_name(inner_area, buf, self);
    }
}

// RENDERING

/// Renders all the borders for the node. `area` should be an inner area that left padding for the port cells on either side.
fn render_borders_and_ports(
    area: Rect,
    buf: &mut Buffer,
    Node {
        ty,
        port_rendering_strategy,
        input_port_count,
        output_port_count,
        ..
    }: &Node,
) {
    let input_port_count = *input_port_count;
    let output_port_count = *output_port_count;

    // Change the border depending on the node type
    let (seperator, line_set, border_set) = match ty {
        NodeType::StructInitializtion() => ("┅", line::DOUBLE, border::DOUBLE),
        NodeType::Builtin(_) | NodeType::Defined() => ("┅", line::THICK, border::THICK),
    };

    let outline = Block::bordered().border_set(border_set);
    let b_area = area.inner(Margin::new(1, 0));
    outline.render(b_area, buf);

    // Draw a seperator if other ports besides primaries exist
    if input_port_count > 1 || output_port_count > 1 {
        buf[(b_area.x, b_area.y + 2)].set_symbol(line_set.vertical_right);
        buf[(b_area.x + b_area.width - 1, b_area.y + 2)].set_symbol(line_set.vertical_left);

        buf.set_string(
            b_area.x + 1,
            b_area.y + 2,
            seperator.repeat(b_area.width as usize - 2),
            Style::new(),
        );
    }

    // Calculate the column indices for inputs and outputs
    let input_port_x = area.x;
    let output_port_x = area.x + area.width - 1;

    let get_cell_row_for_slot = |slot, is_output| {
        area.y
            + port_rendering_strategy.get_cell_row_for_slot(
                slot,
                input_port_count,
                output_port_count,
                is_output,
            )
    };

    // Iterate through the ports and set the character for the port's row to be a slot
    (0..input_port_count)
        .map(|slot| get_cell_row_for_slot(slot, false))
        .for_each(|row| {
            buf[(input_port_x, row)].set_symbol("◈");
            buf[(input_port_x + 1, row)].set_symbol(line_set.vertical_left);
        });

    (0..output_port_count)
        .map(|slot| get_cell_row_for_slot(slot, true))
        .for_each(|row| {
            buf[(output_port_x, row)].set_symbol("◈");
            buf[(output_port_x - 1, row)].set_symbol(line_set.vertical_right);
        });
}

fn render_name(area: Rect, buf: &mut Buffer, node: &Node) {
    Line::from(format_name(node.get_node_name_or_alias()))
        .centered()
        .render(area, buf);
}

// UTIL

fn format_name(name: &str) -> String {
    format!(" {name} ")
}

pub fn get_minimum_size(node: &Node, /* TODO: Add options for type hint toggling, etc. */) -> Size {
    use PortRenderingStrategy::*;

    // The minimum width of the node accounts for the ports, borders, and name width (with padding)
    let width = 2 + format_name(node.get_node_name_or_alias()).len() as u16 + 2;
    // The minimum height of the node accounts for the borders, seperator, and ports
    let height = MINIMUM_NODE_HEIGHT
        + match node.port_rendering_strategy {
            Inline => node
                .input_port_count
                .max(node.output_port_count)
                .saturating_sub(1),
            InputsFirst | OutputsFirst => {
                node.input_port_count.saturating_sub(1) + node.output_port_count.saturating_sub(1)
            }
        }
        + 1; // The final port(s) will be 1 above the bottom of the node

    Size::new(width, height)
}

impl PortRenderingStrategy {
    fn get_cell_row_for_slot(
        &self,
        slot: u16,
        input_port_count: u16,
        output_port_count: u16,
        is_output: bool,
    ) -> u16 {
        // The 'primary' slot will be inline with the name
        if slot == 0 {
            return 1;
        }

        // Disregard the primary slot as it doesn't follow the strategy
        let slot = slot - 1;

        use PortRenderingStrategy::*;
        // Bypass the top border, name, and seperator then add depending on the strategy
        MINIMUM_NODE_HEIGHT
            + match self {
                Inline => slot,
                InputsFirst => {
                    if is_output {
                        input_port_count + slot
                    } else {
                        slot
                    }
                }
                OutputsFirst => {
                    if !is_output {
                        output_port_count + slot
                    } else {
                        slot
                    }
                }
            }
    }
}
