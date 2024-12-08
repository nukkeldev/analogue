use ratatui::{prelude::*, widgets::Block};
use symbols::{border, line};
use tracing::{debug, trace};

use super::*;

/// The minimum height a node can be. This includes the top border, the name (along with primary ports), and the bottom border.
pub const MINIMUM_NODE_HEIGHT: u16 = 3;

// RENDERER

#[derive(Debug)]
pub struct NodeRenderer<'a, NodeRefOrNone> {
    node_ref: NodeRefOrNone,
    cache: Option<RenderingCache>,
    display_options: &'a DisplayOptions,
}

impl<'a> Widget for &mut NodeRenderer<'a, &'a Node<'a>> {
    #[cfg_attr(test, tracing::instrument(skip_all))]
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        debug!("Started rendering node");

        // Ensure the node can be drawn
        let minimum_size = self.get_minimum_size();
        assert!(
            area.width >= minimum_size.width && area.height >= minimum_size.height,
            "Cannot draw node; Supplied area is too small: minimum = {minimum_size}, given = {}.",
            area.as_size()
        );

        trace!("Rendering borders");
        self.render_borders(area, buf);
        trace!("Rendering ports");
        self.render_ports(area, buf);

        trace!("Rendering name");
        let inner_area = area.inner(Margin::new(2, 1));
        self.render_name(inner_area, buf);

        debug!("Rendered node")
    }
}

impl<'a> NodeRenderer<'a, ()> {
    // Constructors

    pub fn new(display_options: &'a DisplayOptions) -> Self {
        Self {
            node_ref: (),
            cache: None,
            display_options,
        }
    }

    // Setters

    pub fn with_node(&self, node_ref: &'a Node) -> NodeRenderer<'a, &'a Node<'a>> {
        NodeRenderer {
            node_ref,
            cache: None,
            display_options: self.display_options,
        }
    }
}

impl<'a> NodeRenderer<'a, &'a Node<'a>> {
    // Setters

    #[must_use]
    pub fn release_node(self) -> NodeRenderer<'a, ()> {
        NodeRenderer {
            node_ref: (),
            cache: None,
            display_options: self.display_options,
        }
    }

    /// Swaps the current node in the renderer for a new one. Invalidates the cache.
    pub fn swap_node(&mut self, node_ref: &'a Node<'a>) {
        self.node_ref = node_ref;
        self.cache = None;
    }

    // Getters

    fn get_formatted_node_name(&self) -> String {
        format!(" {} ", self.node_ref.get_node_name_or_alias())
    }

    /// Gets the minimum size necessary to render this node.
    #[cfg_attr(test, tracing::instrument(skip_all))]
    pub fn get_minimum_size(&mut self) -> Size {
        use PortRenderingStrategy::*;

        let pc = &self.node_ref.port_configuration;
        let show_type_hints = self.display_options.show_type_hints;

        if let Some(ref cache) = self.cache {
            debug!(cached_minimum_size = %cache.minimum_size);
            return cache.minimum_size;
        }

        debug!(
            ?show_type_hints,
            "Started calculating minimum size for node"
        );
        trace!(
            has_primary_input = pc.get_primary_input().is_some(),
            has_primary_output = pc.get_primary_output().is_some(),
            input_count = pc.get_input_port_count(),
            output_count = pc.get_output_port_count()
        );
        trace!(port_render_strategy = ?pc.get_rendering_strategy());

        let mut width = 2 + 2;
        if show_type_hints && !pc.is_empty() {
            let max_slot_width = match pc.get_rendering_strategy() {
                Inline => {
                    // Get the longer of the two primaries, or 0
                    let max_primaries_len = get_max_compact_string_len(
                        pc.get_primary_input().map(Port::get_type_name).as_ref(),
                        pc.get_primary_output().map(Port::get_type_name).as_ref(),
                    );

                    // Use the longer primary length and pad the formatted name with it
                    let mut max_len = max_primaries_len * 2 + self.get_formatted_node_name().len();

                    let inputs = pc.get_input_ports();
                    let outputs = pc.get_output_ports();

                    for i in 0..inputs.len().max(outputs.len()) {
                        // Get the longer of the two ports for this slot, or 0
                        let max = get_max_compact_string_len(
                            inputs.get(i).map(|p| p.get_type_name()).as_ref(),
                            outputs.get(i).map(|p| p.get_type_name()).as_ref(),
                        );
                        // If both ports are present for this slot, add a space in between
                        let needs_spacing = inputs.get(i).is_some() && outputs.get(i).is_some();

                        max_len = max_len.max(max * 2 + if needs_spacing { 1 } else { 0 });
                    }

                    max_len as u16
                }
                InputsFirst => todo!(),
                OutputsFirst => todo!(),
            };
            trace!(max_slot_width);

            width += max_slot_width;
        } else {
            width += self.get_formatted_node_name().len() as u16;
        };

        // The minimum height of the node accounts for the borders, seperator, and ports
        let mut height = MINIMUM_NODE_HEIGHT;
        if pc.is_not_only_primaries() {
            height += match pc.get_rendering_strategy() {
                Inline => pc.get_input_port_count().max(pc.get_output_port_count()) as u16,
                InputsFirst | OutputsFirst => {
                    (pc.get_input_port_count() + pc.get_output_port_count()) as u16
                }
            } + 1 // Bottom Border
        }

        let size = Size::new(width, height);
        debug!(minimum_size = %size);

        if self.cache.is_none() {
            self.cache = Some(RenderingCache { minimum_size: size });
            debug!("Cached minimum size")
        }

        size
    }

    // Rendering

    /// Renders all the borders for the node. `area` should be an inner area that left padding for the port cells on either side.
    #[cfg_attr(test, tracing::instrument(skip_all))]
    fn render_borders(&self, area: Rect, buf: &mut Buffer) {
        let Node {
            ty,
            port_configuration: pc,
            ..
        } = self.node_ref;

        // Change the border depending on the node type
        let (seperator, line_set, border_set) = match ty {
            NodeType::StructInitializtion(_) => ("┅", line::DOUBLE, border::DOUBLE),
            NodeType::Builtin(_) | NodeType::Defined(_) => ("┅", line::THICK, border::THICK),
        };

        let outline = Block::bordered().border_set(border_set);
        let b_area = area.inner(Margin::new(1, 0));
        outline.render(b_area, buf);

        // Draw a seperator if other ports besides primaries exist
        if pc.is_not_only_primaries() {
            buf[(b_area.x, b_area.y + 2)].set_symbol(line_set.vertical_right);
            buf[(b_area.x + b_area.width - 1, b_area.y + 2)].set_symbol(line_set.vertical_left);

            buf.set_string(
                b_area.x + 1,
                b_area.y + 2,
                seperator.repeat(b_area.width as usize - 2),
                Style::new(),
            );
        }
    }

    #[cfg_attr(test, tracing::instrument(skip_all))]
    fn render_ports(&self, area: Rect, buf: &mut Buffer) {
        let NodeRenderer {
            node_ref:
                Node {
                    ty,
                    port_configuration: pc,
                    ..
                },
            display_options,
            ..
        } = self;

        trace!(
            has_primary_input = pc.get_primary_input().is_some(),
            has_primary_output = pc.get_primary_output().is_some(),
            input_count = pc.get_input_port_count(),
            output_count = pc.get_output_port_count()
        );

        let line_set = match ty {
            NodeType::StructInitializtion(_) => line::DOUBLE,
            NodeType::Builtin(_) | NodeType::Defined(_) => line::THICK,
        };

        // Calculate the column indices for inputs and outputs
        let input_port_x = area.x;
        let output_port_x = area.x + area.width - 1;

        if let Some(primary_input) = pc.get_primary_input() {
            buf[(input_port_x, area.y + 1)].set_symbol("◈");
            buf[(input_port_x + 1, area.y + 1)].set_symbol(line_set.vertical_left);

            if display_options.show_type_hints {
                buf.set_string(
                    input_port_x + 2,
                    area.y + 1,
                    primary_input.get_type_name(),
                    Style::new(),
                );
            }
        }

        if let Some(primary_output) = pc.get_primary_output() {
            buf[(output_port_x, area.y + 1)].set_symbol("◈");
            buf[(output_port_x - 1, area.y + 1)].set_symbol(line_set.vertical_right);

            if display_options.show_type_hints {
                let type_hint = primary_output.get_type_name();

                buf.set_string(
                    output_port_x - 1 - type_hint.len() as u16,
                    area.y + 1,
                    type_hint,
                    Style::new(),
                );
            }
        }

        // Iterate through the ports and set the character for the port's row to be a slot
        for slot in 0..pc.get_input_port_count() as u16 {
            let row = area.y + pc.get_cell_row_for_slot(slot, false);
            buf[(input_port_x, row)].set_symbol("◈");
            buf[(input_port_x + 1, row)].set_symbol(line_set.vertical_left);

            if display_options.show_type_hints {
                buf.set_string(
                    input_port_x + 2,
                    row,
                    pc.get_input_ports()[slot as usize].get_type_name(),
                    Style::new(),
                );
            }
        }

        for slot in 0..pc.get_output_port_count() as u16 {
            let row = area.y + pc.get_cell_row_for_slot(slot, true);
            buf[(output_port_x, row)].set_symbol("◈");
            buf[(output_port_x - 1, row)].set_symbol(&line_set.vertical_right);

            if display_options.show_type_hints {
                let type_hint = pc.get_output_ports()[slot as usize].get_type_name();

                buf.set_string(
                    output_port_x - 1 - type_hint.len() as u16,
                    row,
                    type_hint,
                    Style::new(),
                );
            }
        }
    }

    #[cfg_attr(test, tracing::instrument(skip_all))]
    fn render_name(&self, area: Rect, buf: &mut Buffer) {
        Line::from(self.get_formatted_node_name())
            .centered()
            .render(area, buf);
    }
}

// CACHE

#[derive(Debug, Clone)]
struct RenderingCache {
    minimum_size: Size,
}

// UTIL

/// Returns the maximum length in bytes of the two optionally supplied strings. If both strings
/// are none, `0` is returned.
fn get_max_compact_string_len(
    left: Option<&CompactString>,
    right: Option<&CompactString>,
) -> usize {
    let left_len = left.map(CompactString::len);
    let right_len = right.map(CompactString::len);

    left_len.max(right_len).unwrap_or(0)
}

// TESTS

#[cfg(test)]
mod rendering_tests {
    use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
    use render::NodeRenderer;
    use types::Type;

    use crate::util::{test::initialize, tui::BufferExt};

    use super::*;

    const TYPE_U8: Type = Type::U(8);
    const TYPE_U8_VARRAY: Type = Type::VArray(&TYPE_U8);

    #[test]
    fn test_render_builtin_node() {
        initialize();

        let mut node = test_node();
        node.port_configuration
            .set_primary_output(Port::primary(&TYPE_U8_VARRAY));

        let mut renderer = NodeRenderer::new(&DisplayOptions {
            show_type_hints: true,
        })
        .with_node(&node);
        let minimum_size = renderer.get_minimum_size();

        let mut render = Buffer::empty(Rect::new(0, 0, minimum_size.width, minimum_size.height));
        renderer.render(render.area, &mut render);

        println!("{render:?}");
    }

    #[test]
    fn test_get_minimum_width() {
        initialize();

        let test_cases = [
            (("_", None), " ┃ _ ┃ "),
            (
                (
                    "_",
                    Some(PortConfiguration::new(None, None, vec![], vec![])),
                ),
                " ┃ _ ┃ ",
            ),
            (
                (
                    "_",
                    Some(PortConfiguration::new(
                        Some(Port::new(&TYPE_U8, "Foo".into())),
                        Some(Port::new(&TYPE_U8_VARRAY, "Bar".into())),
                        vec![],
                        vec![],
                    )),
                ),
                "◈┫u8   _ u8[]┣◈",
            ),
        ];

        let mut node = test_node();
        let base_renderer = NodeRenderer::new(&DisplayOptions {
            show_type_hints: true,
        });

        for ((alias, pc), expected) in test_cases {
            node.alias = Some(alias.into());
            if let Some(ref pc) = pc {
                node.port_configuration = pc.clone();
            }

            let mut node_renderer = base_renderer.with_node(&node);
            let minimum_size = node_renderer.get_minimum_size();

            let mut buf = Buffer::empty(Rect::new(0, 0, minimum_size.width, minimum_size.height));
            node_renderer.render(buf.area, &mut buf);

            let rendered = buf.get_line(1).unwrap();
            assert_eq!(rendered, expected);
        }
    }

    #[test]
    fn test_render_borders() {
        initialize();

        let mut node = test_node();
        let base_renderer = NodeRenderer::new(&DisplayOptions {
            show_type_hints: true,
        });

        let dt = DefinedType::new("_____");
        #[rustfmt::skip]
        let test_cases = [
            (NodeType::Defined("_____"), vec![
                " ┏━━━━━━━┓ ",
                " ┃       ┃ ",
                " ┗━━━━━━━┛ ",
            ]),
            (NodeType::StructInitializtion(&dt), vec![
                " ╔═══════╗ ",
                " ║       ║ ",
                " ╚═══════╝ ",
            ]),
            (NodeType::Builtin(BuiltinType::ENTRY), vec![
                " ┏━━━━━━━┓ ",
                " ┃       ┃ ",
                " ┗━━━━━━━┛ ",
            ]),
        ];

        for (ty, expected) in test_cases {
            node.ty = ty;

            let mut node_renderer = base_renderer.with_node(&node);
            let minimum_size = node_renderer.get_minimum_size();

            let mut buf = Buffer::empty(Rect::new(0, 0, minimum_size.width, minimum_size.height));
            node_renderer.render_borders(buf.area, &mut buf);

            assert_eq!(buf, Buffer::with_lines(expected));
            buf.reset();
        }
    }

    #[test]
    fn test_render_ports() {
        initialize();

        let mut node = test_node();
        let base_renderer = NodeRenderer::new(&DisplayOptions {
            show_type_hints: true,
        });

        #[rustfmt::skip]
        let test_cases = [
            (PortConfiguration::new(None, None, vec![], vec![]), vec![
                "           ",
                "           ",
                "           ",
            ]),
            (PortConfiguration::new(Some(Port::primary(&TYPE_U8)), None, vec![], vec![]), vec![
                "               ",
                "◈┫u8           ",
                "               ",
            ]),
            (PortConfiguration::new(None, Some(Port::primary(&TYPE_U8)), vec![], vec![]), vec![
                "               ",
                "           u8┣◈",
                "               ",
            ]),
            (PortConfiguration::new(Some(Port::primary(&TYPE_U8)), Some(Port::primary(&TYPE_U8)), vec![], vec![]), vec![
                "               ",
                "◈┫u8       u8┣◈",
                "               ",
            ])
        ];

        for (pc, expected) in test_cases {
            node.port_configuration = pc;

            let mut node_renderer = base_renderer.with_node(&node);
            let minimum_size = node_renderer.get_minimum_size();

            let mut buf = Buffer::empty(Rect::new(0, 0, minimum_size.width, minimum_size.height));
            node_renderer.render_ports(buf.area, &mut buf);

            assert_eq!(buf, Buffer::with_lines(expected));
            buf.reset();
        }
    }

    #[test]
    fn test_render_name() {
        initialize();

        let mut node = test_node();
        let base_renderer = NodeRenderer::new(&DisplayOptions {
            show_type_hints: true,
        });
        let mut buf = Buffer::empty(Rect::new(0, 0, 9, 1));

        let test_cases = [(None, "  ENTRY  "), (Some("_".into()), "    _    ")];

        for (alias, expected) in test_cases {
            node.alias = alias;
            base_renderer
                .with_node(&node)
                .render_name(buf.area, &mut buf);

            assert_eq!(buf.get_line(0).unwrap(), expected);
            buf.reset();
        }
    }

    // Util

    fn test_node<'a>() -> Node<'a> {
        Node {
            ty: NodeType::Builtin(BuiltinType::ENTRY),
            alias: None,
            port_configuration: PortConfiguration::new(None, None, vec![], vec![]),
        }
    }
}

#[cfg(test)]
mod util_tests {
    use compact_str::CompactString;

    use crate::node::render::get_max_compact_string_len;

    #[test]
    fn test_get_max_compact_string_len() {
        const A: CompactString = CompactString::const_new("Hi!");
        const B: CompactString = CompactString::const_new("Goodbye.");

        assert_eq!(get_max_compact_string_len(None, None), 0);
        assert_eq!(get_max_compact_string_len(Some(&A), None), 3);
        assert_eq!(get_max_compact_string_len(None, Some(&B)), 8);
        assert_eq!(get_max_compact_string_len(Some(&A), Some(&B)), 8);
    }
}
