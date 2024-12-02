# Nodes

- [Nodes](#nodes)
  - [Node](#node)
  - [Parsing Graphs](#parsing-graphs)

## Node

**Nodes** are the fundemental component of all parsing graphs. They define the operations that a parsing graph will undergo, declare types, and do many more things. Nodes are categorized into **3 types** (depending on their port configuration):
- **FN Nodes** (Function Nodes)
  - Takes at least **one** input and provides at least **one** output.
- **IN Nodes**
  - Provides at least **one** output and takes **no** inputs.
- **OUT Nodes**
  - Takes at least **one** input and provides **no** outputs.

Regardless of the node type, **all nodes are pure**; no side effects are allowed.

Data enters and exits nodes through **ports**. Ports are connection points for **cables** and are **strongly-typed**. They are identified by whether they're an input (`I`) or and output (`O`) and subscripted with their index (`_#`). Ports on index `0` (i.e `I_0`) are designated as the *primary* port and fall inline with the name of the node.

The **type** of the port is defined as it's identifier followed by `T` (i.e. `I_0T`). It limits the allowed cabling to the port, ensuring only valid data is directed into/out of the port.

Consider the below node layout:
```
       ┏━━━━━━━━━━━━━━━━━━━━━━━━┓
<I_0>◈┅┫(<I_0T>) <Name> (<O_0T>)┣┅◈<O_0>
       ┗━━━━━━━━━━━━━━━━━━━━━━━━┛
```
This is the simplest possible **FN Node**, with one input port (`I_0`) and one output port (`O_0`). Data flows in from the left and undergoes the transformation defined by the node.

Nodes are further categorized into `inbuilt` and `defined` nodes. `inbuilt` nodes are implemented internally and are the building blocks for all `defined` nodes. A `defined` node's parsing graph may contain both categories of nodes, with the exception that **recursive node usage is not allowed**.

Only one `inbuilt` type exists: `u`, a variable-size unsigned integer, all other types are derived from them via `defined` types and bit-level addressing.

## Parsing Graphs

