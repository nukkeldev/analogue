use compact_str::{CompactString, ToCompactString};

use super::types::Type;

// PORT

#[derive(Debug, Clone, PartialEq)]
pub struct Port<'a> {
    ty: &'a Type<'a>,
    name: Option<CompactString>,
}

impl<'a> Port<'a> {
    // Constructors

    pub fn new(ty: &'a Type<'a>, name: CompactString) -> Self {
        Self {
            ty,
            name: Some(name),
        }
    }

    pub fn primary(ty: &'a Type<'a>) -> Self {
        Self { ty, name: None }
    }

    // Getters

    pub fn get_type_name(&self) -> CompactString {
        self.ty.to_compact_string()
    }
}

// PORT CONFIGURATION

/// The configuration of ports (and how they're rendered) for a node.
#[derive(Debug, Clone, PartialEq)]
pub struct PortConfiguration<'a> {
    /// The input ports of the node.
    inputs: Vec<Port<'a>>,
    /// The output ports of the node.
    outputs: Vec<Port<'a>>,
    /// Designates how the ports will be rendered. See [PortConfiguration::with_rendering_strategy] for more information.
    rendering_strategy: PortRenderingStrategy,
}

impl<'a> PortConfiguration<'a> {
    // Constructors

    /// Creates a new port configuration with the given inputs and outputs.
    pub const fn new(inputs: Vec<Port<'a>>, outputs: Vec<Port<'a>>) -> Self {
        Self {
            inputs,
            outputs,
            rendering_strategy: PortRenderingStrategy::Inline,
        }
    }

    // Builder Methods

    /// Modifies the rendering method of how the ports are drawn. By default, ports are drawn inline
    /// with each other (1st input port is on the same line as the 1st output port) with the
    /// [PortRenderingStrategy::Inline] strategy. See [PortRenderingStrategy] for available options.
    #[must_use]
    pub const fn with_rendering_strategy(
        mut self,
        rendering_strategy: PortRenderingStrategy,
    ) -> Self {
        self.rendering_strategy = rendering_strategy;
        self
    }

    // States

    /// Returns `true` if there are no input or output ports.
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty() && self.outputs.is_empty()
    }

    /// Returns `true` if there are at most one input and one output port.
    pub fn is_only_primaries(&self) -> bool {
        !self.is_empty() && self.inputs.len() <= 1 && self.outputs.len() <= 1
    }

    /// Returns `true` if there are at least two input or output ports.
    pub fn is_not_only_primaries(&self) -> bool {
        self.inputs.len() > 1 || self.outputs.len() > 1
    }

    // Getters

    pub fn get_input_port_count(&self) -> u16 {
        self.inputs.len() as u16
    }

    pub fn get_output_port_count(&self) -> u16 {
        self.outputs.len() as u16
    }

    pub fn get_rendering_strategy(&self) -> &PortRenderingStrategy {
        &self.rendering_strategy
    }

    pub fn get_input_ports(&self) -> &Vec<Port<'a>> {
        &self.inputs
    }

    pub fn get_output_ports(&self) -> &Vec<Port<'a>> {
        &self.outputs
    }

    // Setters

    pub fn add_input_port(&mut self, port: Port<'a>) {
        self.inputs.push(port);
    }

    pub fn add_output_port(&mut self, port: Port<'a>) {
        self.outputs.push(port);
    }

    // Util

    /// Gets the row relative to the top border of a node that the port on the specified side and slot is on.
    pub fn get_cell_row_for_slot(&self, slot: u16, is_output: bool) -> u16 {
        // The 'primary' slot will be inline with the name
        if slot == 0 {
            return 1;
        }

        // Disregard the primary slot as it doesn't follow the strategy
        let slot = slot - 1;

        use PortRenderingStrategy::*;
        // Bypass the top border, name, and seperator then add depending on the strategy
        crate::node::render::MINIMUM_NODE_HEIGHT
            + match self.rendering_strategy {
                Inline => slot,
                InputsFirst => {
                    if is_output {
                        self.inputs.len() as u16 + slot
                    } else {
                        slot
                    }
                }
                OutputsFirst => {
                    if !is_output {
                        self.outputs.len() as u16 + slot
                    } else {
                        slot
                    }
                }
            }
    }
}

// RENDERING STRATEGY

#[derive(Debug, PartialEq, Clone, Default)]
pub enum PortRenderingStrategy {
    /// Inputs and output slots are inline with each other.
    #[default]
    Inline,
    /// Inputs are above the outputs.
    InputsFirst,
    /// Outputs are above the inputs.
    OutputsFirst,
    // TODO: Other variants, such as Interspersed could be added later
}
