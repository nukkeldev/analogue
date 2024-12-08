use compact_str::CompactString;

pub(crate) mod render;
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    ty: NodeType,
    alias: Option<CompactString>,
    port_rendering_strategy: PortRenderingStrategy,
    input_port_count: u16,
    output_port_count: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Builtin(BuiltinType),
    StructInitializtion(),
    Defined(),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltinType {
    ENTRY,
    EXIT,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PortRenderingStrategy {
    /// Inputs and output slots are inline with each other.
    Inline,
    /// Inputs are above the outputs.
    InputsFirst,
    /// Outputs are above the inputs.
    OutputsFirst,
    // TODO: Other variants, such as Interspersed could be added later
}

impl Node {
    pub fn get_node_name_or_alias(&self) -> &str {
        self.alias
            .as_ref()
            .map(|str| str.as_str())
            .unwrap_or_else(|| self.get_node_name())
    }

    pub fn get_node_name(&self) -> &str {
        match &self.ty {
            NodeType::Builtin(ty) => Self::get_builtin_node_name(ty),
            NodeType::StructInitializtion() => todo!(),
            NodeType::Defined() => todo!(),
        }
    }

    fn get_builtin_node_name(ty: &BuiltinType) -> &str {
        match ty {
            BuiltinType::ENTRY => "ENTRY",
            BuiltinType::EXIT => "EXIT",
        }
    }
}
