use super::preamble::*;
pub const PRINTING_CLASS_EXCLUDE: InstClass = InstClass {
    tag: "@exclude",
    heading: None,
};
pub const PRINTING_CLASS_MISCELLANEOUS: InstClass = InstClass {
    tag: "Miscellaneous",
    heading: Some("Miscellaneous Instructions"),
};
pub const PRINTING_CLASS_DEBUG: InstClass = InstClass {
    tag: "Debug",
    heading: Some("Debug Instructions"),
};
pub const PRINTING_CLASS_ANNOTATION: InstClass = InstClass {
    tag: "Annotation",
    heading: Some("Annotation Instructions"),
};
pub const PRINTING_CLASS_EXTENSION: InstClass = InstClass {
    tag: "Extension",
    heading: Some("Extension Instructions"),
};
pub const PRINTING_CLASS_MODE_SETTING: InstClass = InstClass {
    tag: "Mode-Setting",
    heading: Some("Mode-Setting Instructions"),
};
pub const PRINTING_CLASS_TYPE_DECLARATION: InstClass = InstClass {
    tag: "Type-Declaration",
    heading: Some("Type-Declaration Instructions"),
};
pub const PRINTING_CLASS_CONSTANT_CREATION: InstClass = InstClass {
    tag: "Constant-Creation",
    heading: Some("Constant-Creation Instructions"),
};
pub const PRINTING_CLASS_MEMORY: InstClass = InstClass {
    tag: "Memory",
    heading: Some("Memory Instructions"),
};
pub const PRINTING_CLASS_FUNCTION: InstClass = InstClass {
    tag: "Function",
    heading: Some("Function Instructions"),
};
pub const PRINTING_CLASS_IMAGE: InstClass = InstClass {
    tag: "Image",
    heading: Some("Image Instructions"),
};
pub const PRINTING_CLASS_CONVERSION: InstClass = InstClass {
    tag: "Conversion",
    heading: Some("Conversion Instructions"),
};
pub const PRINTING_CLASS_COMPOSITE: InstClass = InstClass {
    tag: "Composite",
    heading: Some("Composite Instructions"),
};
pub const PRINTING_CLASS_ARITHMETIC: InstClass = InstClass {
    tag: "Arithmetic",
    heading: Some("Arithmetic Instructions"),
};
pub const PRINTING_CLASS_BIT: InstClass = InstClass {
    tag: "Bit",
    heading: Some("Bit Instructions"),
};
pub const PRINTING_CLASS_RELATIONAL_AND_LOGICAL: InstClass = InstClass {
    tag: "Relational_and_Logical",
    heading: Some("Relational and Logical Instructions"),
};
pub const PRINTING_CLASS_DERIVATIVE: InstClass = InstClass {
    tag: "Derivative",
    heading: Some("Derivative Instructions"),
};
pub const PRINTING_CLASS_CONTROL_FLOW: InstClass = InstClass {
    tag: "Control-Flow",
    heading: Some("Control-Flow Instructions"),
};
pub const PRINTING_CLASS_ATOMIC: InstClass = InstClass {
    tag: "Atomic",
    heading: Some("Atomic Instructions"),
};
pub const PRINTING_CLASS_PRIMITIVE: InstClass = InstClass {
    tag: "Primitive",
    heading: Some("Primitive Instructions"),
};
pub const PRINTING_CLASS_BARRIER: InstClass = InstClass {
    tag: "Barrier",
    heading: Some("Barrier Instructions"),
};
pub const PRINTING_CLASS_GROUP: InstClass = InstClass {
    tag: "Group",
    heading: Some("Group and Subgroup Instructions"),
};
pub const PRINTING_CLASS_DEVICE_SIDE_ENQUEUE: InstClass = InstClass {
    tag: "Device-Side_Enqueue",
    heading: Some("Device-Side Enqueue Instructions"),
};
pub const PRINTING_CLASS_PIPE: InstClass = InstClass {
    tag: "Pipe",
    heading: Some("Pipe Instructions"),
};
pub const PRINTING_CLASS_NON_UNIFORM: InstClass = InstClass {
    tag: "Non-Uniform",
    heading: Some("Non-Uniform Instructions"),
};
pub const PRINTING_CLASS_TENSOR: InstClass = InstClass {
    tag: "Tensor",
    heading: Some("Tensor Instructions"),
};
pub const PRINTING_CLASS_GRAPH: InstClass = InstClass {
    tag: "Graph",
    heading: Some("Graph Instructions"),
};
pub const PRINTING_CLASS_RESERVED: InstClass = InstClass {
    tag: "Reserved",
    heading: Some("Reserved Instructions"),
};
