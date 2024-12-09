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

    // Builder Methods

    pub fn with_alias<S: Into<CompactString>>(mut self, alias: S) -> Self {
        self.alias = Some(alias.into());
        self
    }

    pub fn without_alias(mut self) -> Self {
        self.alias = None;
        self
    }

    pub fn with_type(mut self, ty: NodeType<'a>) -> Self {
        self.ty = ty;
        self
    }

    pub fn with_port_configuration(mut self, pc: PortConfiguration<'a>) -> Self {
        self.port_configuration = pc;
        self
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
