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
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PortConfiguration<'a> {
    /// The primary input of the node; inline with the node's name.
    primary_input: Option<Port<'a>>,
    /// The primary output of the node; inline with the node's name.
    primary_output: Option<Port<'a>>,
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
    pub const fn new(
        primary_input: Option<Port<'a>>,
        primary_output: Option<Port<'a>>,
        inputs: Vec<Port<'a>>,
        outputs: Vec<Port<'a>>,
    ) -> Self {
        Self {
            primary_input,
            primary_output,
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

    pub fn with_primary_input(mut self, port: Port<'a>) -> Self {
        self.primary_input = Some(port);
        self
    }

    pub fn with_primary_output(mut self, port: Port<'a>) -> Self {
        self.primary_output = Some(port);
        self
    }

    pub fn with_input(mut self, port: Port<'a>) -> Self {
        self.inputs.push(port);
        self
    }

    pub fn with_output(mut self, port: Port<'a>) -> Self {
        self.outputs.push(port);
        self
    }

    // States

    /// Returns `true` if there are no input or output ports.
    pub fn is_empty(&self) -> bool {
        self.primary_input.is_none()
            && self.primary_output.is_none()
            && self.inputs.is_empty()
            && self.outputs.is_empty()
    }

    /// Returns `true` if there are no non-primary ports.
    pub fn is_only_primaries(&self) -> bool {
        (self.primary_input.is_some() || self.primary_output.is_some())
            && self.inputs.is_empty()
            && self.outputs.is_empty()
    }

    /// Returns `true` if there are non-primary ports.
    pub fn is_not_only_primaries(&self) -> bool {
        !self.inputs.is_empty() || !self.outputs.is_empty()
    }

    // Getters

    /// Gets the number of input ports, excluding the primary port (if present).
    pub fn get_input_port_count(&self) -> usize {
        self.inputs.len()
    }

    /// Gets the number of output ports, excluding the primary port (if present).\
    pub fn get_output_port_count(&self) -> usize {
        self.outputs.len()
    }

    /// Gets the rendering strategy.
    pub fn get_rendering_strategy(&self) -> &PortRenderingStrategy {
        &self.rendering_strategy
    }

    /// Gets a reference to the primary input port (if present).
    pub fn get_primary_input(&self) -> Option<&Port<'a>> {
        self.primary_input.as_ref()
    }

    /// Gets a reference to the primary output port (if present).
    pub fn get_primary_output(&self) -> Option<&Port<'a>> {
        self.primary_output.as_ref()
    }

    /// Gets a reference to the input ports.
    pub fn get_input_ports(&self) -> &Vec<Port<'a>> {
        &self.inputs
    }

    /// Gets a reference to the output ports.
    pub fn get_output_ports(&self) -> &Vec<Port<'a>> {
        &self.outputs
    }

    // Setters

    /// Sets the primary input to `port`.
    pub fn set_primary_input(&mut self, port: Port<'a>) {
        self.primary_input = Some(port);
    }

    /// Sets the primary output to `port`.
    pub fn set_primary_output(&mut self, port: Port<'a>) {
        self.primary_output = Some(port);
    }

    /// Removes the primary input.
    pub fn remove_primary_input(&mut self) {
        self.primary_input = None;
    }

    /// Removes the primary output.
    pub fn remove_primary_output(&mut self) {
        self.primary_output = None;
    }

    /// Adds an input port.
    pub fn add_input(&mut self, port: Port<'a>) {
        self.inputs.push(port);
    }

    /// Adds an output port.
    pub fn add_output(&mut self, port: Port<'a>) {
        self.outputs.push(port);
    }

    // Util

    /// Gets the row relative to the top border of a node that the port on the specified side and slot is on.
    pub fn get_cell_row_for_slot(&self, slot: u16, is_output: bool) -> u16 {
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
