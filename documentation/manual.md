# Manual

## Table of Contents

- [Manual](#manual)
  - [Table of Contents](#table-of-contents)
  - [What is `Analogue`?](#what-is-analogue)
  - [Nodes](#nodes)
  - [Ports](#ports)
  - [Visual Layouts](#visual-layouts)

## What is `Analogue`?

**Analogue** is a binary description language for parsing graphs, a rather unconventional way to define grammar. The parsing graphs, along with metadata, are stored in *packed binary* `.ana` files that can be edited through the **Analog Editor** (or another [specification](specification.md) compliant software) as used by this manual.

*TLDR*, **parsing graphs** are a type of [node graph](https://en.wikipedia.org/wiki/Node_graph_architecture) that describe a transformation of bytes. Furthermore, parsing graphs have various features that allow for a much better developer experience. These include, but are not limited to:
- Strongly-typed data flow
- User-defined types
  - Along with meta-programming for type definitions
- Node Composition
- Exporting to various backends

## Nodes

**Nodes** are the fundamental component of all parsing graphs. They define the operations that data will undergo, declare types, and do much more.

Nodes are primarily categorized into **3 types** (depending on their port configuration):

- **`FN` Nodes** (Function Nodes)
  - Takes at least **one** input and provides at least **one** output.
- **`IN` Nodes**
  - Provides at least **one** output and takes **no** inputs.
- **`OUT` Nodes**
  - Takes at least **one** input and provides **no** outputs.

Regardless of the node type, **all nodes are pure**; ana has no concept of side-effects and nodes have no concept of state besides the data traveling along the cabling.

Nodes are further categorized into `inbuilt` and `defined` nodes. `inbuilt` nodes are implemented internally and are the building blocks for all `defined` nodes. A `defined` node's parsing graph may contain both categories of nodes, with the exception that **recursive node usage is not allowed**.

`defined` nodes reside in **namespaces** that serve as containers for similarly purposed constructs, as specified in the format `<namespace>:**:<node>` (i.e. `example:Example` refers to the node `Example` in the namespace `example`). If `<namespace>` is left empty (i.e. `:str`), that is shorthand that refers to the `root` namespace, Analogue's standard library.

> [!NOTE]
> `inbuilt` nodes have no namespace and are named in all caps (i.e. `CAST`).

## Ports

Data enters and exits nodes through **ports**. Ports are **strongly-typed** connection points for **cables**. They are identified by strings comprised of whether they're an input (`I`) or and output (`O`) and subscripted with their index (`_#`). Ports on index `0` (i.e `I_0`) are designated as the _primary_ port and fall inline with the name of the node.

The **type** of the port is defined as it's identifier followed by `T` (i.e. `I_0T`). It limits the allowed cabling to the port, ensuring only valid data is directed into/out of the port. By default, all types are strictly type checked. For instance, if `I_0T` was a `Foo`, which contains two `u<8>` fields, then passing in an `[u<8>|2]` would be invalid. Single-fielded structs are an exception to this and are *auto-casted* by default.

Usually, a `[u<8>|2]` would go through a `CAST` node to explicitly covert a byte array into a struct of the same size. This can be changed using environment variables, see below.

Like nodes, types are categorized into `inbuilt` and `defined`. Only one `inbuilt` type exists: `u`, a variable-size unsigned integer, all other types are derived from it via `defined` types (many common types live in `root:*`).

## Visual Layouts

*All nodes* follow a similar layout. Consider the below node layout:

```
       ┏━━━━━━━━━━━━━━━━━━━━━━━━┓
<I_0>◈┅┫(<I_0T>) <Name> (<O_0T>)┣┅◈<O_0>
       ┗━━━━━━━━━━━━━━━━━━━━━━━━┛
```

This is the simplest possible **`FN` Node**, with one input port (`I_0`) and one output port (`O_0`). All nodes are identified by their `<name>`, centered and with enough space to have at least a space on either side of it. Nodes are automatically sized depending on their contents (with type hints enabled, even if not shown). Type hints are enabled by default but can be disabled or enabled when a specific shortcut is held (see [Settings](#Settings)).

Suppose this node now needed an input to customize it's behavior:

```
       (0)
       ┏━━━━━━━━━━━━━━━━━━━━━━━━┓
<I_0>◈┅┫(<I_0T>) <Name> (<O_0T>)┣┅◈<O_0>
       ┣┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┫
<I_1>◈┅┫(<I_1T>)<Field>         ┃
       ┗━━━━━━━━━━━━━━━━━━━━━━━━┛
       (1)
       ┏━━━━━━━━━━━━━━━━━━━━━━━━┓
<I_0>◈┅┫(<I_0T>) <Name> (<O_0T>)┣┅◈<O_0>
       ┣┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┫
     x┅┫(<I_1T>)<Field>[<value>]┃
       ┗━━━━━━━━━━━━━━━━━━━━━━━━┛
```
