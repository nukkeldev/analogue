use compact_str::CompactString;

use options::*;
use ports::*;
use types::DefinedType;

pub mod options;
pub mod ports;
pub mod render;
pub mod types;

// NODE

#[derive(Debug, PartialEq, Clone)]
pub struct Node<'a> {
    ty: NodeType<'a>,
    alias: Option<CompactString>,
    port_configuration: PortConfiguration<'a>,
}

impl<'a> Node<'a> {
    // Constructors

    pub fn new(ty: NodeType<'a>, port_configuration: PortConfiguration<'a>) -> Self {
        Self {
            ty,
            alias: None,
            port_configuration,
        }
    }

    pub fn aliased<S: Into<CompactString>>(
        ty: NodeType<'a>,
        alias: S,
        port_configuration: PortConfiguration<'a>,
    ) -> Self {
        Self {
            ty,
            alias: Some(alias.into()),
            port_configuration,
        }
    }

    // Getters

    pub fn get_node_name_or_alias(&self) -> &str {
        self.alias
            .as_ref()
            .map(|str| str.as_str())
            .unwrap_or_else(|| self.get_node_name())
    }

    pub fn get_node_name(&self) -> &str {
        match &self.ty {
            NodeType::Builtin(ty) => Self::get_builtin_node_name(ty),
            NodeType::StructInitializtion(dt) => dt.get_name(),
            NodeType::Defined(t_) => t_,
        }
    }

    fn get_builtin_node_name(ty: &BuiltinType) -> &str {
        match ty {
            BuiltinType::ENTRY => "ENTRY",
            BuiltinType::EXIT => "EXIT",
            BuiltinType::COMMENT => "COMMENT",
        }
    }
}

// NODE TYPES

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType<'a> {
    Builtin(BuiltinType),
    StructInitializtion(&'a DefinedType<'a>),
    Defined(&'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltinType {
    COMMENT,
    ENTRY,
    EXIT,
}
