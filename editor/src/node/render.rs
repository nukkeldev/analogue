use ratatui::{prelude::*, widgets::Block};
use symbols::{border, line};

use super::*;

/// The minimum height a node can be. This includes the top border, the name (along with primary ports), and the bottom border.
pub const MINIMUM_NODE_HEIGHT: u16 = 3;

// RENDERER

#[derive(Debug)]
pub struct NodeRenderer<'a, NodeRefOrNone>(NodeRefOrNone, &'a DisplayOptions);

impl<'a> Widget for &NodeRenderer<'a, &'a Node<'a>> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Ensure the node can be drawn
        let minimum_size = self.get_minimum_size();
        assert!(
            area.width >= minimum_size.width && area.height >= minimum_size.height,
            "Cannot draw node; Supplied area is too small: minimum = {minimum_size}, given = {}.",
            area.as_size()
        );

        // Render the border with one cell of horizontal padding
        self.render_borders_and_ports(area, buf);

        let inner_area = area.inner(Margin::new(2, 1));

        // Render the name (or alias) of the node
        self.render_name(inner_area, buf);
    }
}

impl<'a> NodeRenderer<'a, ()> {
    // Constructors

    pub fn new(display_options: &'a DisplayOptions) -> Self {
        Self((), display_options)
    }

    // Setters

    pub fn with_node(&self, node_ref: &'a Node) -> NodeRenderer<'a, &'a Node<'a>> {
        NodeRenderer(node_ref, self.1)
    }
}

impl<'a> NodeRenderer<'a, &'a Node<'a>> {
    // Setters

    #[must_use]
    pub fn release_node(self) -> NodeRenderer<'a, ()> {
        NodeRenderer::new(self.1)
    }

    pub fn swap_node(&mut self, node_ref: &'a Node<'a>) {
        self.0 = node_ref;
    }

    // Getters

    fn get_formatted_node_name(&self) -> String {
        format!(" {} ", self.0.get_node_name_or_alias())
    }

    pub fn get_minimum_size(&self) -> Size {
        use PortRenderingStrategy::*;

        let display_options = &self.1;
        let pc = &self.0.port_configuration;

        // The minimum width of the node accounts for the ports, borders, and name width (with padding)
        let width = (2 + 2)
            + if display_options.show_type_hints && !pc.is_empty() {
                let mut input_type_hints =
                    pc.get_input_ports().iter().map(|p| p.get_type_name().len());
                let mut output_type_hints = pc
                    .get_output_ports()
                    .iter()
                    .map(|p| p.get_type_name().len());

                match pc.get_rendering_strategy() {
                    Inline => {
                        let mut max_type_hint_len = || {
                            let mut need_space = true;
                            let max = input_type_hints
                                .next()
                                .unwrap_or_else(|| {
                                    need_space = false;
                                    0
                                })
                                .max(output_type_hints.next().unwrap_or_else(|| {
                                    need_space = false;
                                    0
                                }));

                            (max, need_space)
                        };

                        let primaries_len =
                            max_type_hint_len().0 * 2 + self.get_formatted_node_name().len();

                        (0..pc.get_input_port_count().max(pc.get_output_port_count()) - 1)
                            .into_iter()
                            .map(|_| {
                                let (max, need_space) = max_type_hint_len();
                                // A space between the type hints is only necessary if both are present
                                let spacing = if need_space { 0 } else { 1 };

                                max * 2 + spacing
                            })
                            .chain(std::iter::once(primaries_len))
                            .max()
                            .unwrap_or(0) as u16
                    }
                    InputsFirst => todo!(),
                    OutputsFirst => todo!(),
                }
            } else {
                self.get_formatted_node_name().len() as u16
            };

        // The minimum height of the node accounts for the borders, seperator, and ports
        let mut height = MINIMUM_NODE_HEIGHT;
        if pc.is_not_only_primaries() {
            height += match pc.get_rendering_strategy() {
                Inline => pc
                    .get_input_port_count()
                    .max(pc.get_output_port_count())
                    .saturating_sub(1),
                InputsFirst | OutputsFirst => {
                    pc.get_input_port_count().saturating_sub(1)
                        + pc.get_output_port_count().saturating_sub(1)
                }
            } + 1; // The final port(s) will be 1 above the bottom of the node
        }

        Size::new(width, height)
    }

    // Rendering

    /// Renders all the borders for the node. `area` should be an inner area that left padding for the port cells on either side.
    fn render_borders_and_ports(&self, area: Rect, buf: &mut Buffer) {
        let NodeRenderer(node, display_options) = self;

        // Change the border depending on the node type
        let (seperator, line_set, border_set) = match node.ty {
            NodeType::StructInitializtion(_) => ("┅", line::DOUBLE, border::DOUBLE),
            NodeType::Builtin(_) | NodeType::Defined() => ("┅", line::THICK, border::THICK),
        };

        let outline = Block::bordered().border_set(border_set);
        let b_area = area.inner(Margin::new(1, 0));
        outline.render(b_area, buf);

        // Draw a seperator if other ports besides primaries exist
        if node.port_configuration.is_not_only_primaries() {
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
                + node
                    .port_configuration
                    .get_cell_row_for_slot(slot, is_output)
        };

        // Iterate through the ports and set the character for the port's row to be a slot
        for slot in 0..node.port_configuration.get_input_port_count() {
            let row = get_cell_row_for_slot(slot, false);
            buf[(input_port_x, row)].set_symbol("◈");
            buf[(input_port_x + 1, row)].set_symbol(line_set.vertical_left);

            if display_options.show_type_hints {
                buf.set_string(
                    input_port_x + 2,
                    row,
                    node.port_configuration.get_input_ports()[slot as usize].get_type_name(),
                    Style::new(),
                );
            }
        }

        for slot in 0..node.port_configuration.get_output_port_count() {
            let row = get_cell_row_for_slot(slot, true);
            buf[(output_port_x, row)].set_symbol("◈");
            buf[(output_port_x - 1, row)].set_symbol(&line_set.vertical_right);

            if display_options.show_type_hints {
                let type_hint =
                    node.port_configuration.get_output_ports()[slot as usize].get_type_name();

                buf.set_string(
                    output_port_x - 1 - type_hint.len() as u16,
                    row,
                    type_hint,
                    Style::new(),
                );
            }
        }
    }

    fn render_name(&self, area: Rect, buf: &mut Buffer) {
        Line::from(self.get_formatted_node_name())
            .centered()
            .render(area, buf);
    }
}
