const MAGIC: &[u8] = b"ANLG";

type StrPtr = u16;

#[derive(Debug, Clone, PartialEq)]
struct Header {
    name: StrPtr,
    help: StrPtr,
    node_count: u16,
    type_count: u16,
}

#[derive(Debug, Clone, PartialEq)]
struct AnaFile {
    header: Header,
}

#[derive(Debug, Clone, PartialEq)]
struct ParsingGraph {}

pub fn initialize() {
    println!("Hello, Analogue!");
}
