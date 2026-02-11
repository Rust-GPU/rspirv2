use super::preamble::*;
#[doc = "opcode: 0x0"]
pub const OP_NOP: InstMeta = InstMeta {
    opname: "OpNop",
    class: Some(&PRINTING_CLASS_MISCELLANEOUS),
    opcode: 0u16,
    operands: &[],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1"]
pub const OP_UNDEF: InstMeta = InstMeta {
    opname: "OpUndef",
    class: Some(&PRINTING_CLASS_MISCELLANEOUS),
    opcode: 1u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x2"]
pub const OP_SOURCE_CONTINUED: InstMeta = InstMeta {
    opname: "OpSourceContinued",
    class: Some(&PRINTING_CLASS_DEBUG),
    opcode: 2u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_LITERAL_STRING,
        name: Some("Continued Source"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x3"]
pub const OP_SOURCE: InstMeta = InstMeta {
    opname: "OpSource",
    class: Some(&PRINTING_CLASS_DEBUG),
    opcode: 3u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_SOURCE_LANGUAGE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Version"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("File"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Source"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x4"]
pub const OP_SOURCE_EXTENSION: InstMeta = InstMeta {
    opname: "OpSourceExtension",
    class: Some(&PRINTING_CLASS_DEBUG),
    opcode: 4u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_LITERAL_STRING,
        name: Some("Extension"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x5"]
pub const OP_NAME: InstMeta = InstMeta {
    opname: "OpName",
    class: Some(&PRINTING_CLASS_DEBUG),
    opcode: 5u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Name"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x6"]
pub const OP_MEMBER_NAME: InstMeta = InstMeta {
    opname: "OpMemberName",
    class: Some(&PRINTING_CLASS_DEBUG),
    opcode: 6u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Member"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Name"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x7"]
pub const OP_STRING: InstMeta = InstMeta {
    opname: "OpString",
    class: Some(&PRINTING_CLASS_DEBUG),
    opcode: 7u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("String"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x8"]
pub const OP_LINE: InstMeta = InstMeta {
    opname: "OpLine",
    class: Some(&PRINTING_CLASS_DEBUG),
    opcode: 8u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("File"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Line"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Column"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa"]
pub const OP_EXTENSION: InstMeta = InstMeta {
    opname: "OpExtension",
    class: Some(&PRINTING_CLASS_EXTENSION),
    opcode: 10u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_LITERAL_STRING,
        name: Some("Name"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb"]
pub const OP_EXT_INST_IMPORT: InstMeta = InstMeta {
    opname: "OpExtInstImport",
    class: Some(&PRINTING_CLASS_EXTENSION),
    opcode: 11u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Name"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xc"]
pub const OP_EXT_INST: InstMeta = InstMeta {
    opname: "OpExtInst",
    class: Some(&PRINTING_CLASS_EXTENSION),
    opcode: 12u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Set"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_EXT_INST_INTEGER,
            name: Some("Instruction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1, Operand 2, ..."),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xe"]
pub const OP_MEMORY_MODEL: InstMeta = InstMeta {
    opname: "OpMemoryModel",
    class: Some(&PRINTING_CLASS_MODE_SETTING),
    opcode: 14u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ADDRESSING_MODEL,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_MODEL,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xf"]
pub const OP_ENTRY_POINT: InstMeta = InstMeta {
    opname: "OpEntryPoint",
    class: Some(&PRINTING_CLASS_MODE_SETTING),
    opcode: 15u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_EXECUTION_MODEL,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Entry Point"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Name"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Interface"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x10"]
pub const OP_EXECUTION_MODE: InstMeta = InstMeta {
    opname: "OpExecutionMode",
    class: Some(&PRINTING_CLASS_MODE_SETTING),
    opcode: 16u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Entry Point"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_EXECUTION_MODE,
            name: Some("Mode"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x11"]
pub const OP_CAPABILITY: InstMeta = InstMeta {
    opname: "OpCapability",
    class: Some(&PRINTING_CLASS_MODE_SETTING),
    opcode: 17u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_CAPABILITY,
        name: Some("Capability"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13"]
pub const OP_TYPE_VOID: InstMeta = InstMeta {
    opname: "OpTypeVoid",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 19u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14"]
pub const OP_TYPE_BOOL: InstMeta = InstMeta {
    opname: "OpTypeBool",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 20u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15"]
pub const OP_TYPE_INT: InstMeta = InstMeta {
    opname: "OpTypeInt",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 21u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Signedness"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16"]
pub const OP_TYPE_FLOAT: InstMeta = InstMeta {
    opname: "OpTypeFloat",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 22u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_FP_ENCODING,
            name: Some("Floating Point Encoding"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x17"]
pub const OP_TYPE_VECTOR: InstMeta = InstMeta {
    opname: "OpTypeVector",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 23u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Component Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Component Count"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x18"]
pub const OP_TYPE_MATRIX: InstMeta = InstMeta {
    opname: "OpTypeMatrix",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 24u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Column Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Column Count"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Matrix],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x19"]
pub const OP_TYPE_IMAGE: InstMeta = InstMeta {
    opname: "OpTypeImage",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 25u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_DIM,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Depth"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Arrayed"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("MS"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Sampled"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_FORMAT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ACCESS_QUALIFIER,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1a"]
pub const OP_TYPE_SAMPLER: InstMeta = InstMeta {
    opname: "OpTypeSampler",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 26u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1b"]
pub const OP_TYPE_SAMPLED_IMAGE: InstMeta = InstMeta {
    opname: "OpTypeSampledImage",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 27u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image Type"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1c"]
pub const OP_TYPE_ARRAY: InstMeta = InstMeta {
    opname: "OpTypeArray",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 28u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Length"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1d"]
pub const OP_TYPE_RUNTIME_ARRAY: InstMeta = InstMeta {
    opname: "OpTypeRuntimeArray",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 29u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element Type"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1e"]
pub const OP_TYPE_STRUCT: InstMeta = InstMeta {
    opname: "OpTypeStruct",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 30u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Member 0 type, member 1 type, ..."),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1f"]
pub const OP_TYPE_OPAQUE: InstMeta = InstMeta {
    opname: "OpTypeOpaque",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 31u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("The name of the opaque type."),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x20"]
pub const OP_TYPE_POINTER: InstMeta = InstMeta {
    opname: "OpTypePointer",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 32u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_STORAGE_CLASS,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Type"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x21"]
pub const OP_TYPE_FUNCTION: InstMeta = InstMeta {
    opname: "OpTypeFunction",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 33u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Return Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Parameter 0 Type, Parameter 1 Type, ..."),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x22"]
pub const OP_TYPE_EVENT: InstMeta = InstMeta {
    opname: "OpTypeEvent",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 34u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x23"]
pub const OP_TYPE_DEVICE_EVENT: InstMeta = InstMeta {
    opname: "OpTypeDeviceEvent",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 35u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x24"]
pub const OP_TYPE_RESERVE_ID: InstMeta = InstMeta {
    opname: "OpTypeReserveId",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 36u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x25"]
pub const OP_TYPE_QUEUE: InstMeta = InstMeta {
    opname: "OpTypeQueue",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 37u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x26"]
pub const OP_TYPE_PIPE: InstMeta = InstMeta {
    opname: "OpTypePipe",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 38u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ACCESS_QUALIFIER,
            name: Some("Qualifier"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x27"]
pub const OP_TYPE_FORWARD_POINTER: InstMeta = InstMeta {
    opname: "OpTypeForwardPointer",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 39u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_STORAGE_CLASS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::Addresses,
        Capability::PhysicalStorageBufferAddresses,
    ],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x29"]
pub const OP_CONSTANT_TRUE: InstMeta = InstMeta {
    opname: "OpConstantTrue",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 41u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x2a"]
pub const OP_CONSTANT_FALSE: InstMeta = InstMeta {
    opname: "OpConstantFalse",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 42u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x2b"]
pub const OP_CONSTANT: InstMeta = InstMeta {
    opname: "OpConstant",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 43u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_CONTEXT_DEPENDENT_NUMBER,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x2c"]
pub const OP_CONSTANT_COMPOSITE: InstMeta = InstMeta {
    opname: "OpConstantComposite",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 44u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Constituents"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x2d"]
pub const OP_CONSTANT_SAMPLER: InstMeta = InstMeta {
    opname: "OpConstantSampler",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 45u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_SAMPLER_ADDRESSING_MODE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Param"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_SAMPLER_FILTER_MODE,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::LiteralSampler],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x2e"]
pub const OP_CONSTANT_NULL: InstMeta = InstMeta {
    opname: "OpConstantNull",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 46u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x30"]
pub const OP_SPEC_CONSTANT_TRUE: InstMeta = InstMeta {
    opname: "OpSpecConstantTrue",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 48u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x31"]
pub const OP_SPEC_CONSTANT_FALSE: InstMeta = InstMeta {
    opname: "OpSpecConstantFalse",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 49u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x32"]
pub const OP_SPEC_CONSTANT: InstMeta = InstMeta {
    opname: "OpSpecConstant",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 50u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_CONTEXT_DEPENDENT_NUMBER,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x33"]
pub const OP_SPEC_CONSTANT_COMPOSITE: InstMeta = InstMeta {
    opname: "OpSpecConstantComposite",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 51u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Constituents"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x34"]
pub const OP_SPEC_CONSTANT_OP: InstMeta = InstMeta {
    opname: "OpSpecConstantOp",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 52u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_SPEC_CONSTANT_OP_INTEGER,
            name: Some("Opcode"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x36"]
pub const OP_FUNCTION: InstMeta = InstMeta {
    opname: "OpFunction",
    class: Some(&PRINTING_CLASS_FUNCTION),
    opcode: 54u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_FUNCTION_CONTROL,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Function Type"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x37"]
pub const OP_FUNCTION_PARAMETER: InstMeta = InstMeta {
    opname: "OpFunctionParameter",
    class: Some(&PRINTING_CLASS_FUNCTION),
    opcode: 55u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x38"]
pub const OP_FUNCTION_END: InstMeta = InstMeta {
    opname: "OpFunctionEnd",
    class: Some(&PRINTING_CLASS_FUNCTION),
    opcode: 56u16,
    operands: &[],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x39"]
pub const OP_FUNCTION_CALL: InstMeta = InstMeta {
    opname: "OpFunctionCall",
    class: Some(&PRINTING_CLASS_FUNCTION),
    opcode: 57u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Function"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Argument 0, Argument 1, ..."),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x3b"]
pub const OP_VARIABLE: InstMeta = InstMeta {
    opname: "OpVariable",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 59u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_STORAGE_CLASS,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Initializer"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x3c"]
pub const OP_IMAGE_TEXEL_POINTER: InstMeta = InstMeta {
    opname: "OpImageTexelPointer",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 60u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sample"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x3d"]
pub const OP_LOAD: InstMeta = InstMeta {
    opname: "OpLoad",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 61u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x3e"]
pub const OP_STORE: InstMeta = InstMeta {
    opname: "OpStore",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 62u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x3f"]
pub const OP_COPY_MEMORY: InstMeta = InstMeta {
    opname: "OpCopyMemory",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 63u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Source"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x40"]
pub const OP_COPY_MEMORY_SIZED: InstMeta = InstMeta {
    opname: "OpCopyMemorySized",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 64u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Source"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::Addresses, Capability::UntypedPointersKHR],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x41"]
pub const OP_ACCESS_CHAIN: InstMeta = InstMeta {
    opname: "OpAccessChain",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 65u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Indexes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x42"]
pub const OP_IN_BOUNDS_ACCESS_CHAIN: InstMeta = InstMeta {
    opname: "OpInBoundsAccessChain",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 66u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Indexes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x43"]
pub const OP_PTR_ACCESS_CHAIN: InstMeta = InstMeta {
    opname: "OpPtrAccessChain",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 67u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Indexes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[
        Capability::Addresses,
        Capability::VariablePointers,
        Capability::VariablePointersStorageBuffer,
        Capability::PhysicalStorageBufferAddresses,
    ],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x44"]
pub const OP_ARRAY_LENGTH: InstMeta = InstMeta {
    opname: "OpArrayLength",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 68u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Array member"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x45"]
pub const OP_GENERIC_PTR_MEM_SEMANTICS: InstMeta = InstMeta {
    opname: "OpGenericPtrMemSemantics",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 69u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x46"]
pub const OP_IN_BOUNDS_PTR_ACCESS_CHAIN: InstMeta = InstMeta {
    opname: "OpInBoundsPtrAccessChain",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 70u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Indexes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::Addresses],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x47"]
pub const OP_DECORATE: InstMeta = InstMeta {
    opname: "OpDecorate",
    class: Some(&PRINTING_CLASS_ANNOTATION),
    opcode: 71u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_DECORATION,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x48"]
pub const OP_MEMBER_DECORATE: InstMeta = InstMeta {
    opname: "OpMemberDecorate",
    class: Some(&PRINTING_CLASS_ANNOTATION),
    opcode: 72u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Structure Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Member"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_DECORATION,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x49"]
pub const OP_DECORATION_GROUP: InstMeta = InstMeta {
    opname: "OpDecorationGroup",
    class: Some(&PRINTING_CLASS_ANNOTATION),
    opcode: 73u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x4a"]
pub const OP_GROUP_DECORATE: InstMeta = InstMeta {
    opname: "OpGroupDecorate",
    class: Some(&PRINTING_CLASS_ANNOTATION),
    opcode: 74u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Decoration Group"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Targets"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x4b"]
pub const OP_GROUP_MEMBER_DECORATE: InstMeta = InstMeta {
    opname: "OpGroupMemberDecorate",
    class: Some(&PRINTING_CLASS_ANNOTATION),
    opcode: 75u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Decoration Group"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_PAIR_ID_REF_LITERAL_INTEGER,
            name: Some("Targets"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x4d"]
pub const OP_VECTOR_EXTRACT_DYNAMIC: InstMeta = InstMeta {
    opname: "OpVectorExtractDynamic",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 77u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Index"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x4e"]
pub const OP_VECTOR_INSERT_DYNAMIC: InstMeta = InstMeta {
    opname: "OpVectorInsertDynamic",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 78u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Component"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Index"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x4f"]
pub const OP_VECTOR_SHUFFLE: InstMeta = InstMeta {
    opname: "OpVectorShuffle",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 79u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 2"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Components"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x50"]
pub const OP_COMPOSITE_CONSTRUCT: InstMeta = InstMeta {
    opname: "OpCompositeConstruct",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 80u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Constituents"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x51"]
pub const OP_COMPOSITE_EXTRACT: InstMeta = InstMeta {
    opname: "OpCompositeExtract",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 81u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Composite"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Indexes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x52"]
pub const OP_COMPOSITE_INSERT: InstMeta = InstMeta {
    opname: "OpCompositeInsert",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 82u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Composite"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Indexes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x53"]
pub const OP_COPY_OBJECT: InstMeta = InstMeta {
    opname: "OpCopyObject",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 83u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x54"]
pub const OP_TRANSPOSE: InstMeta = InstMeta {
    opname: "OpTranspose",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 84u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Matrix],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x56"]
pub const OP_SAMPLED_IMAGE: InstMeta = InstMeta {
    opname: "OpSampledImage",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 86u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampler"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x57"]
pub const OP_IMAGE_SAMPLE_IMPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSampleImplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 87u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x58"]
pub const OP_IMAGE_SAMPLE_EXPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSampleExplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 88u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x59"]
pub const OP_IMAGE_SAMPLE_DREF_IMPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSampleDrefImplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 89u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("D~ref~"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x5a"]
pub const OP_IMAGE_SAMPLE_DREF_EXPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSampleDrefExplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 90u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("D~ref~"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x5b"]
pub const OP_IMAGE_SAMPLE_PROJ_IMPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSampleProjImplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 91u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x5c"]
pub const OP_IMAGE_SAMPLE_PROJ_EXPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSampleProjExplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 92u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x5d"]
pub const OP_IMAGE_SAMPLE_PROJ_DREF_IMPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSampleProjDrefImplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 93u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("D~ref~"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x5e"]
pub const OP_IMAGE_SAMPLE_PROJ_DREF_EXPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSampleProjDrefExplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 94u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("D~ref~"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x5f"]
pub const OP_IMAGE_FETCH: InstMeta = InstMeta {
    opname: "OpImageFetch",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 95u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x60"]
pub const OP_IMAGE_GATHER: InstMeta = InstMeta {
    opname: "OpImageGather",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 96u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Component"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x61"]
pub const OP_IMAGE_DREF_GATHER: InstMeta = InstMeta {
    opname: "OpImageDrefGather",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 97u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("D~ref~"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x62"]
pub const OP_IMAGE_READ: InstMeta = InstMeta {
    opname: "OpImageRead",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 98u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x63"]
pub const OP_IMAGE_WRITE: InstMeta = InstMeta {
    opname: "OpImageWrite",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 99u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Texel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x64"]
pub const OP_IMAGE: InstMeta = InstMeta {
    opname: "OpImage",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 100u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x65"]
pub const OP_IMAGE_QUERY_FORMAT: InstMeta = InstMeta {
    opname: "OpImageQueryFormat",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 101u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x66"]
pub const OP_IMAGE_QUERY_ORDER: InstMeta = InstMeta {
    opname: "OpImageQueryOrder",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 102u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x67"]
pub const OP_IMAGE_QUERY_SIZE_LOD: InstMeta = InstMeta {
    opname: "OpImageQuerySizeLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 103u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Level of Detail"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel, Capability::ImageQuery],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x68"]
pub const OP_IMAGE_QUERY_SIZE: InstMeta = InstMeta {
    opname: "OpImageQuerySize",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 104u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel, Capability::ImageQuery],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x69"]
pub const OP_IMAGE_QUERY_LOD: InstMeta = InstMeta {
    opname: "OpImageQueryLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 105u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ImageQuery],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x6a"]
pub const OP_IMAGE_QUERY_LEVELS: InstMeta = InstMeta {
    opname: "OpImageQueryLevels",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 106u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel, Capability::ImageQuery],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x6b"]
pub const OP_IMAGE_QUERY_SAMPLES: InstMeta = InstMeta {
    opname: "OpImageQuerySamples",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 107u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel, Capability::ImageQuery],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x6d"]
pub const OP_CONVERT_F_TO_U: InstMeta = InstMeta {
    opname: "OpConvertFToU",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 109u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Float Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x6e"]
pub const OP_CONVERT_F_TO_S: InstMeta = InstMeta {
    opname: "OpConvertFToS",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 110u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Float Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x6f"]
pub const OP_CONVERT_S_TO_F: InstMeta = InstMeta {
    opname: "OpConvertSToF",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 111u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Signed Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x70"]
pub const OP_CONVERT_U_TO_F: InstMeta = InstMeta {
    opname: "OpConvertUToF",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 112u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Unsigned Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x71"]
pub const OP_U_CONVERT: InstMeta = InstMeta {
    opname: "OpUConvert",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 113u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Unsigned Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x72"]
pub const OP_S_CONVERT: InstMeta = InstMeta {
    opname: "OpSConvert",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 114u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Signed Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x73"]
pub const OP_F_CONVERT: InstMeta = InstMeta {
    opname: "OpFConvert",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 115u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Float Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x74"]
pub const OP_QUANTIZE_TO_F_16: InstMeta = InstMeta {
    opname: "OpQuantizeToF16",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 116u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x75"]
pub const OP_CONVERT_PTR_TO_U: InstMeta = InstMeta {
    opname: "OpConvertPtrToU",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 117u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::Addresses,
        Capability::PhysicalStorageBufferAddresses,
    ],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x76"]
pub const OP_SAT_CONVERT_S_TO_U: InstMeta = InstMeta {
    opname: "OpSatConvertSToU",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 118u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Signed Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x77"]
pub const OP_SAT_CONVERT_U_TO_S: InstMeta = InstMeta {
    opname: "OpSatConvertUToS",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 119u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Unsigned Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x78"]
pub const OP_CONVERT_U_TO_PTR: InstMeta = InstMeta {
    opname: "OpConvertUToPtr",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 120u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Integer Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::Addresses,
        Capability::PhysicalStorageBufferAddresses,
    ],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x79"]
pub const OP_PTR_CAST_TO_GENERIC: InstMeta = InstMeta {
    opname: "OpPtrCastToGeneric",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 121u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x7a"]
pub const OP_GENERIC_CAST_TO_PTR: InstMeta = InstMeta {
    opname: "OpGenericCastToPtr",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 122u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x7b"]
pub const OP_GENERIC_CAST_TO_PTR_EXPLICIT: InstMeta = InstMeta {
    opname: "OpGenericCastToPtrExplicit",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 123u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_STORAGE_CLASS,
            name: Some("Storage"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x7c"]
pub const OP_BITCAST: InstMeta = InstMeta {
    opname: "OpBitcast",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 124u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x7e"]
pub const OP_S_NEGATE: InstMeta = InstMeta {
    opname: "OpSNegate",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 126u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x7f"]
pub const OP_F_NEGATE: InstMeta = InstMeta {
    opname: "OpFNegate",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 127u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x80"]
pub const OP_I_ADD: InstMeta = InstMeta {
    opname: "OpIAdd",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 128u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x81"]
pub const OP_F_ADD: InstMeta = InstMeta {
    opname: "OpFAdd",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 129u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x82"]
pub const OP_I_SUB: InstMeta = InstMeta {
    opname: "OpISub",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 130u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x83"]
pub const OP_F_SUB: InstMeta = InstMeta {
    opname: "OpFSub",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 131u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x84"]
pub const OP_I_MUL: InstMeta = InstMeta {
    opname: "OpIMul",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 132u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x85"]
pub const OP_F_MUL: InstMeta = InstMeta {
    opname: "OpFMul",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 133u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x86"]
pub const OP_U_DIV: InstMeta = InstMeta {
    opname: "OpUDiv",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 134u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x87"]
pub const OP_S_DIV: InstMeta = InstMeta {
    opname: "OpSDiv",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 135u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x88"]
pub const OP_F_DIV: InstMeta = InstMeta {
    opname: "OpFDiv",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 136u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x89"]
pub const OP_U_MOD: InstMeta = InstMeta {
    opname: "OpUMod",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 137u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x8a"]
pub const OP_S_REM: InstMeta = InstMeta {
    opname: "OpSRem",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 138u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x8b"]
pub const OP_S_MOD: InstMeta = InstMeta {
    opname: "OpSMod",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 139u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x8c"]
pub const OP_F_REM: InstMeta = InstMeta {
    opname: "OpFRem",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 140u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x8d"]
pub const OP_F_MOD: InstMeta = InstMeta {
    opname: "OpFMod",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 141u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x8e"]
pub const OP_VECTOR_TIMES_SCALAR: InstMeta = InstMeta {
    opname: "OpVectorTimesScalar",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 142u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Scalar"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x8f"]
pub const OP_MATRIX_TIMES_SCALAR: InstMeta = InstMeta {
    opname: "OpMatrixTimesScalar",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 143u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Scalar"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Matrix],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x90"]
pub const OP_VECTOR_TIMES_MATRIX: InstMeta = InstMeta {
    opname: "OpVectorTimesMatrix",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 144u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Matrix],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x91"]
pub const OP_MATRIX_TIMES_VECTOR: InstMeta = InstMeta {
    opname: "OpMatrixTimesVector",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 145u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Matrix],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x92"]
pub const OP_MATRIX_TIMES_MATRIX: InstMeta = InstMeta {
    opname: "OpMatrixTimesMatrix",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 146u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("LeftMatrix"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RightMatrix"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Matrix],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x93"]
pub const OP_OUTER_PRODUCT: InstMeta = InstMeta {
    opname: "OpOuterProduct",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 147u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Matrix],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x94"]
pub const OP_DOT: InstMeta = InstMeta {
    opname: "OpDot",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 148u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x95"]
pub const OP_I_ADD_CARRY: InstMeta = InstMeta {
    opname: "OpIAddCarry",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 149u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x96"]
pub const OP_I_SUB_BORROW: InstMeta = InstMeta {
    opname: "OpISubBorrow",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 150u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x97"]
pub const OP_U_MUL_EXTENDED: InstMeta = InstMeta {
    opname: "OpUMulExtended",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 151u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x98"]
pub const OP_S_MUL_EXTENDED: InstMeta = InstMeta {
    opname: "OpSMulExtended",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 152u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x9a"]
pub const OP_ANY: InstMeta = InstMeta {
    opname: "OpAny",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 154u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x9b"]
pub const OP_ALL: InstMeta = InstMeta {
    opname: "OpAll",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 155u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x9c"]
pub const OP_IS_NAN: InstMeta = InstMeta {
    opname: "OpIsNan",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 156u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x9d"]
pub const OP_IS_INF: InstMeta = InstMeta {
    opname: "OpIsInf",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 157u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x9e"]
pub const OP_IS_FINITE: InstMeta = InstMeta {
    opname: "OpIsFinite",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 158u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x9f"]
pub const OP_IS_NORMAL: InstMeta = InstMeta {
    opname: "OpIsNormal",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 159u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa0"]
pub const OP_SIGN_BIT_SET: InstMeta = InstMeta {
    opname: "OpSignBitSet",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 160u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa1"]
pub const OP_LESS_OR_GREATER: InstMeta = InstMeta {
    opname: "OpLessOrGreater",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 161u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: Some("1.5"),
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa2"]
pub const OP_ORDERED: InstMeta = InstMeta {
    opname: "OpOrdered",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 162u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa3"]
pub const OP_UNORDERED: InstMeta = InstMeta {
    opname: "OpUnordered",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 163u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa4"]
pub const OP_LOGICAL_EQUAL: InstMeta = InstMeta {
    opname: "OpLogicalEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 164u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa5"]
pub const OP_LOGICAL_NOT_EQUAL: InstMeta = InstMeta {
    opname: "OpLogicalNotEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 165u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa6"]
pub const OP_LOGICAL_OR: InstMeta = InstMeta {
    opname: "OpLogicalOr",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 166u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa7"]
pub const OP_LOGICAL_AND: InstMeta = InstMeta {
    opname: "OpLogicalAnd",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 167u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa8"]
pub const OP_LOGICAL_NOT: InstMeta = InstMeta {
    opname: "OpLogicalNot",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 168u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xa9"]
pub const OP_SELECT: InstMeta = InstMeta {
    opname: "OpSelect",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 169u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Condition"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Object 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Object 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xaa"]
pub const OP_I_EQUAL: InstMeta = InstMeta {
    opname: "OpIEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 170u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xab"]
pub const OP_I_NOT_EQUAL: InstMeta = InstMeta {
    opname: "OpINotEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 171u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xac"]
pub const OP_U_GREATER_THAN: InstMeta = InstMeta {
    opname: "OpUGreaterThan",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 172u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xad"]
pub const OP_S_GREATER_THAN: InstMeta = InstMeta {
    opname: "OpSGreaterThan",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 173u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xae"]
pub const OP_U_GREATER_THAN_EQUAL: InstMeta = InstMeta {
    opname: "OpUGreaterThanEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 174u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xaf"]
pub const OP_S_GREATER_THAN_EQUAL: InstMeta = InstMeta {
    opname: "OpSGreaterThanEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 175u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb0"]
pub const OP_U_LESS_THAN: InstMeta = InstMeta {
    opname: "OpULessThan",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 176u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb1"]
pub const OP_S_LESS_THAN: InstMeta = InstMeta {
    opname: "OpSLessThan",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 177u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb2"]
pub const OP_U_LESS_THAN_EQUAL: InstMeta = InstMeta {
    opname: "OpULessThanEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 178u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb3"]
pub const OP_S_LESS_THAN_EQUAL: InstMeta = InstMeta {
    opname: "OpSLessThanEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 179u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb4"]
pub const OP_F_ORD_EQUAL: InstMeta = InstMeta {
    opname: "OpFOrdEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 180u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb5"]
pub const OP_F_UNORD_EQUAL: InstMeta = InstMeta {
    opname: "OpFUnordEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 181u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb6"]
pub const OP_F_ORD_NOT_EQUAL: InstMeta = InstMeta {
    opname: "OpFOrdNotEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 182u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb7"]
pub const OP_F_UNORD_NOT_EQUAL: InstMeta = InstMeta {
    opname: "OpFUnordNotEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 183u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb8"]
pub const OP_F_ORD_LESS_THAN: InstMeta = InstMeta {
    opname: "OpFOrdLessThan",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 184u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xb9"]
pub const OP_F_UNORD_LESS_THAN: InstMeta = InstMeta {
    opname: "OpFUnordLessThan",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 185u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xba"]
pub const OP_F_ORD_GREATER_THAN: InstMeta = InstMeta {
    opname: "OpFOrdGreaterThan",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 186u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xbb"]
pub const OP_F_UNORD_GREATER_THAN: InstMeta = InstMeta {
    opname: "OpFUnordGreaterThan",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 187u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xbc"]
pub const OP_F_ORD_LESS_THAN_EQUAL: InstMeta = InstMeta {
    opname: "OpFOrdLessThanEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 188u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xbd"]
pub const OP_F_UNORD_LESS_THAN_EQUAL: InstMeta = InstMeta {
    opname: "OpFUnordLessThanEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 189u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xbe"]
pub const OP_F_ORD_GREATER_THAN_EQUAL: InstMeta = InstMeta {
    opname: "OpFOrdGreaterThanEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 190u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xbf"]
pub const OP_F_UNORD_GREATER_THAN_EQUAL: InstMeta = InstMeta {
    opname: "OpFUnordGreaterThanEqual",
    class: Some(&PRINTING_CLASS_RELATIONAL_AND_LOGICAL),
    opcode: 191u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xc2"]
pub const OP_SHIFT_RIGHT_LOGICAL: InstMeta = InstMeta {
    opname: "OpShiftRightLogical",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 194u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Shift"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xc3"]
pub const OP_SHIFT_RIGHT_ARITHMETIC: InstMeta = InstMeta {
    opname: "OpShiftRightArithmetic",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 195u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Shift"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xc4"]
pub const OP_SHIFT_LEFT_LOGICAL: InstMeta = InstMeta {
    opname: "OpShiftLeftLogical",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 196u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Shift"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xc5"]
pub const OP_BITWISE_OR: InstMeta = InstMeta {
    opname: "OpBitwiseOr",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 197u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xc6"]
pub const OP_BITWISE_XOR: InstMeta = InstMeta {
    opname: "OpBitwiseXor",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 198u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xc7"]
pub const OP_BITWISE_AND: InstMeta = InstMeta {
    opname: "OpBitwiseAnd",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 199u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xc8"]
pub const OP_NOT: InstMeta = InstMeta {
    opname: "OpNot",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 200u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xc9"]
pub const OP_BIT_FIELD_INSERT: InstMeta = InstMeta {
    opname: "OpBitFieldInsert",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 201u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Insert"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Count"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader, Capability::BitInstructions],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xca"]
pub const OP_BIT_FIELD_S_EXTRACT: InstMeta = InstMeta {
    opname: "OpBitFieldSExtract",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 202u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Count"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader, Capability::BitInstructions],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xcb"]
pub const OP_BIT_FIELD_U_EXTRACT: InstMeta = InstMeta {
    opname: "OpBitFieldUExtract",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 203u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Count"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader, Capability::BitInstructions],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xcc"]
pub const OP_BIT_REVERSE: InstMeta = InstMeta {
    opname: "OpBitReverse",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 204u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader, Capability::BitInstructions],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xcd"]
pub const OP_BIT_COUNT: InstMeta = InstMeta {
    opname: "OpBitCount",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 205u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xcf"]
pub const OP_D_PDX: InstMeta = InstMeta {
    opname: "OpDPdx",
    class: Some(&PRINTING_CLASS_DERIVATIVE),
    opcode: 207u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("P"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xd0"]
pub const OP_D_PDY: InstMeta = InstMeta {
    opname: "OpDPdy",
    class: Some(&PRINTING_CLASS_DERIVATIVE),
    opcode: 208u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("P"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xd1"]
pub const OP_FWIDTH: InstMeta = InstMeta {
    opname: "OpFwidth",
    class: Some(&PRINTING_CLASS_DERIVATIVE),
    opcode: 209u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("P"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xd2"]
pub const OP_D_PDX_FINE: InstMeta = InstMeta {
    opname: "OpDPdxFine",
    class: Some(&PRINTING_CLASS_DERIVATIVE),
    opcode: 210u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("P"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DerivativeControl],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xd3"]
pub const OP_D_PDY_FINE: InstMeta = InstMeta {
    opname: "OpDPdyFine",
    class: Some(&PRINTING_CLASS_DERIVATIVE),
    opcode: 211u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("P"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DerivativeControl],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xd4"]
pub const OP_FWIDTH_FINE: InstMeta = InstMeta {
    opname: "OpFwidthFine",
    class: Some(&PRINTING_CLASS_DERIVATIVE),
    opcode: 212u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("P"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DerivativeControl],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xd5"]
pub const OP_D_PDX_COARSE: InstMeta = InstMeta {
    opname: "OpDPdxCoarse",
    class: Some(&PRINTING_CLASS_DERIVATIVE),
    opcode: 213u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("P"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DerivativeControl],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xd6"]
pub const OP_D_PDY_COARSE: InstMeta = InstMeta {
    opname: "OpDPdyCoarse",
    class: Some(&PRINTING_CLASS_DERIVATIVE),
    opcode: 214u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("P"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DerivativeControl],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xd7"]
pub const OP_FWIDTH_COARSE: InstMeta = InstMeta {
    opname: "OpFwidthCoarse",
    class: Some(&PRINTING_CLASS_DERIVATIVE),
    opcode: 215u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("P"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DerivativeControl],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xda"]
pub const OP_EMIT_VERTEX: InstMeta = InstMeta {
    opname: "OpEmitVertex",
    class: Some(&PRINTING_CLASS_PRIMITIVE),
    opcode: 218u16,
    operands: &[],
    capabilities: &[Capability::Geometry],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xdb"]
pub const OP_END_PRIMITIVE: InstMeta = InstMeta {
    opname: "OpEndPrimitive",
    class: Some(&PRINTING_CLASS_PRIMITIVE),
    opcode: 219u16,
    operands: &[],
    capabilities: &[Capability::Geometry],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xdc"]
pub const OP_EMIT_STREAM_VERTEX: InstMeta = InstMeta {
    opname: "OpEmitStreamVertex",
    class: Some(&PRINTING_CLASS_PRIMITIVE),
    opcode: 220u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Stream"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::GeometryStreams],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xdd"]
pub const OP_END_STREAM_PRIMITIVE: InstMeta = InstMeta {
    opname: "OpEndStreamPrimitive",
    class: Some(&PRINTING_CLASS_PRIMITIVE),
    opcode: 221u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Stream"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::GeometryStreams],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xe0"]
pub const OP_CONTROL_BARRIER: InstMeta = InstMeta {
    opname: "OpControlBarrier",
    class: Some(&PRINTING_CLASS_BARRIER),
    opcode: 224u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xe1"]
pub const OP_MEMORY_BARRIER: InstMeta = InstMeta {
    opname: "OpMemoryBarrier",
    class: Some(&PRINTING_CLASS_BARRIER),
    opcode: 225u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xe3"]
pub const OP_ATOMIC_LOAD: InstMeta = InstMeta {
    opname: "OpAtomicLoad",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 227u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xe4"]
pub const OP_ATOMIC_STORE: InstMeta = InstMeta {
    opname: "OpAtomicStore",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 228u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xe5"]
pub const OP_ATOMIC_EXCHANGE: InstMeta = InstMeta {
    opname: "OpAtomicExchange",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 229u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xe6"]
pub const OP_ATOMIC_COMPARE_EXCHANGE: InstMeta = InstMeta {
    opname: "OpAtomicCompareExchange",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 230u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Equal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Unequal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Comparator"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xe7"]
pub const OP_ATOMIC_COMPARE_EXCHANGE_WEAK: InstMeta = InstMeta {
    opname: "OpAtomicCompareExchangeWeak",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 231u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Equal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Unequal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Comparator"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: Some("1.3"),
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xe8"]
pub const OP_ATOMIC_I_INCREMENT: InstMeta = InstMeta {
    opname: "OpAtomicIIncrement",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 232u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xe9"]
pub const OP_ATOMIC_I_DECREMENT: InstMeta = InstMeta {
    opname: "OpAtomicIDecrement",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 233u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xea"]
pub const OP_ATOMIC_I_ADD: InstMeta = InstMeta {
    opname: "OpAtomicIAdd",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 234u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xeb"]
pub const OP_ATOMIC_I_SUB: InstMeta = InstMeta {
    opname: "OpAtomicISub",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 235u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xec"]
pub const OP_ATOMIC_S_MIN: InstMeta = InstMeta {
    opname: "OpAtomicSMin",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 236u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xed"]
pub const OP_ATOMIC_U_MIN: InstMeta = InstMeta {
    opname: "OpAtomicUMin",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 237u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xee"]
pub const OP_ATOMIC_S_MAX: InstMeta = InstMeta {
    opname: "OpAtomicSMax",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 238u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xef"]
pub const OP_ATOMIC_U_MAX: InstMeta = InstMeta {
    opname: "OpAtomicUMax",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 239u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xf0"]
pub const OP_ATOMIC_AND: InstMeta = InstMeta {
    opname: "OpAtomicAnd",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 240u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xf1"]
pub const OP_ATOMIC_OR: InstMeta = InstMeta {
    opname: "OpAtomicOr",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 241u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xf2"]
pub const OP_ATOMIC_XOR: InstMeta = InstMeta {
    opname: "OpAtomicXor",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 242u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xf5"]
pub const OP_PHI: InstMeta = InstMeta {
    opname: "OpPhi",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 245u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_PAIR_ID_REF_ID_REF,
            name: Some("Variable, Parent, ..."),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xf6"]
pub const OP_LOOP_MERGE: InstMeta = InstMeta {
    opname: "OpLoopMerge",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 246u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Merge Block"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Continue Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LOOP_CONTROL,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xf7"]
pub const OP_SELECTION_MERGE: InstMeta = InstMeta {
    opname: "OpSelectionMerge",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 247u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Merge Block"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_SELECTION_CONTROL,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xf8"]
pub const OP_LABEL: InstMeta = InstMeta {
    opname: "OpLabel",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 248u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xf9"]
pub const OP_BRANCH: InstMeta = InstMeta {
    opname: "OpBranch",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 249u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Target Label"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xfa"]
pub const OP_BRANCH_CONDITIONAL: InstMeta = InstMeta {
    opname: "OpBranchConditional",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 250u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Condition"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("True Label"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("False Label"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Branch weights"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xfb"]
pub const OP_SWITCH: InstMeta = InstMeta {
    opname: "OpSwitch",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 251u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Selector"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Default"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_PAIR_LITERAL_INTEGER_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xfc"]
pub const OP_KILL: InstMeta = InstMeta {
    opname: "OpKill",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 252u16,
    operands: &[],
    capabilities: &[Capability::Shader],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xfd"]
pub const OP_RETURN: InstMeta = InstMeta {
    opname: "OpReturn",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 253u16,
    operands: &[],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xfe"]
pub const OP_RETURN_VALUE: InstMeta = InstMeta {
    opname: "OpReturnValue",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 254u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Value"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0xff"]
pub const OP_UNREACHABLE: InstMeta = InstMeta {
    opname: "OpUnreachable",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 255u16,
    operands: &[],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x100"]
pub const OP_LIFETIME_START: InstMeta = InstMeta {
    opname: "OpLifetimeStart",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 256u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Size"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x101"]
pub const OP_LIFETIME_STOP: InstMeta = InstMeta {
    opname: "OpLifetimeStop",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 257u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Size"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x103"]
pub const OP_GROUP_ASYNC_COPY: InstMeta = InstMeta {
    opname: "OpGroupAsyncCopy",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 259u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Destination"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Source"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Num Elements"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Event"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x104"]
pub const OP_GROUP_WAIT_EVENTS: InstMeta = InstMeta {
    opname: "OpGroupWaitEvents",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 260u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Num Events"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Events List"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x105"]
pub const OP_GROUP_ALL: InstMeta = InstMeta {
    opname: "OpGroupAll",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 261u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x106"]
pub const OP_GROUP_ANY: InstMeta = InstMeta {
    opname: "OpGroupAny",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 262u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x107"]
pub const OP_GROUP_BROADCAST: InstMeta = InstMeta {
    opname: "OpGroupBroadcast",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 263u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("LocalId"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x108"]
pub const OP_GROUP_I_ADD: InstMeta = InstMeta {
    opname: "OpGroupIAdd",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 264u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x109"]
pub const OP_GROUP_F_ADD: InstMeta = InstMeta {
    opname: "OpGroupFAdd",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 265u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x10a"]
pub const OP_GROUP_F_MIN: InstMeta = InstMeta {
    opname: "OpGroupFMin",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 266u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x10b"]
pub const OP_GROUP_U_MIN: InstMeta = InstMeta {
    opname: "OpGroupUMin",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 267u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x10c"]
pub const OP_GROUP_S_MIN: InstMeta = InstMeta {
    opname: "OpGroupSMin",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 268u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x10d"]
pub const OP_GROUP_F_MAX: InstMeta = InstMeta {
    opname: "OpGroupFMax",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 269u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x10e"]
pub const OP_GROUP_U_MAX: InstMeta = InstMeta {
    opname: "OpGroupUMax",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 270u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x10f"]
pub const OP_GROUP_S_MAX: InstMeta = InstMeta {
    opname: "OpGroupSMax",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 271u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x112"]
pub const OP_READ_PIPE: InstMeta = InstMeta {
    opname: "OpReadPipe",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 274u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x113"]
pub const OP_WRITE_PIPE: InstMeta = InstMeta {
    opname: "OpWritePipe",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 275u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x114"]
pub const OP_RESERVED_READ_PIPE: InstMeta = InstMeta {
    opname: "OpReservedReadPipe",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 276u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reserve Id"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x115"]
pub const OP_RESERVED_WRITE_PIPE: InstMeta = InstMeta {
    opname: "OpReservedWritePipe",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 277u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reserve Id"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x116"]
pub const OP_RESERVE_READ_PIPE_PACKETS: InstMeta = InstMeta {
    opname: "OpReserveReadPipePackets",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 278u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Num Packets"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x117"]
pub const OP_RESERVE_WRITE_PIPE_PACKETS: InstMeta = InstMeta {
    opname: "OpReserveWritePipePackets",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 279u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Num Packets"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x118"]
pub const OP_COMMIT_READ_PIPE: InstMeta = InstMeta {
    opname: "OpCommitReadPipe",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 280u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reserve Id"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x119"]
pub const OP_COMMIT_WRITE_PIPE: InstMeta = InstMeta {
    opname: "OpCommitWritePipe",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 281u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reserve Id"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x11a"]
pub const OP_IS_VALID_RESERVE_ID: InstMeta = InstMeta {
    opname: "OpIsValidReserveId",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 282u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reserve Id"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x11b"]
pub const OP_GET_NUM_PIPE_PACKETS: InstMeta = InstMeta {
    opname: "OpGetNumPipePackets",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 283u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x11c"]
pub const OP_GET_MAX_PIPE_PACKETS: InstMeta = InstMeta {
    opname: "OpGetMaxPipePackets",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 284u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x11d"]
pub const OP_GROUP_RESERVE_READ_PIPE_PACKETS: InstMeta = InstMeta {
    opname: "OpGroupReserveReadPipePackets",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 285u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Num Packets"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x11e"]
pub const OP_GROUP_RESERVE_WRITE_PIPE_PACKETS: InstMeta = InstMeta {
    opname: "OpGroupReserveWritePipePackets",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 286u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Num Packets"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x11f"]
pub const OP_GROUP_COMMIT_READ_PIPE: InstMeta = InstMeta {
    opname: "OpGroupCommitReadPipe",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 287u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reserve Id"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x120"]
pub const OP_GROUP_COMMIT_WRITE_PIPE: InstMeta = InstMeta {
    opname: "OpGroupCommitWritePipe",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 288u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reserve Id"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Pipes],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x123"]
pub const OP_ENQUEUE_MARKER: InstMeta = InstMeta {
    opname: "OpEnqueueMarker",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 291u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Queue"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Num Events"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Wait Events"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ret Event"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x124"]
pub const OP_ENQUEUE_KERNEL: InstMeta = InstMeta {
    opname: "OpEnqueueKernel",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 292u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Queue"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ND Range"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Num Events"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Wait Events"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ret Event"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Invoke"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Align"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Local Size"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x125"]
pub const OP_GET_KERNEL_N_DRANGE_SUB_GROUP_COUNT: InstMeta = InstMeta {
    opname: "OpGetKernelNDrangeSubGroupCount",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 293u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ND Range"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Invoke"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Align"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x126"]
pub const OP_GET_KERNEL_N_DRANGE_MAX_SUB_GROUP_SIZE: InstMeta = InstMeta {
    opname: "OpGetKernelNDrangeMaxSubGroupSize",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 294u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ND Range"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Invoke"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Align"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x127"]
pub const OP_GET_KERNEL_WORK_GROUP_SIZE: InstMeta = InstMeta {
    opname: "OpGetKernelWorkGroupSize",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 295u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Invoke"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Align"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x128"]
pub const OP_GET_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE: InstMeta = InstMeta {
    opname: "OpGetKernelPreferredWorkGroupSizeMultiple",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 296u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Invoke"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Align"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x129"]
pub const OP_RETAIN_EVENT: InstMeta = InstMeta {
    opname: "OpRetainEvent",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 297u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Event"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x12a"]
pub const OP_RELEASE_EVENT: InstMeta = InstMeta {
    opname: "OpReleaseEvent",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 298u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Event"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x12b"]
pub const OP_CREATE_USER_EVENT: InstMeta = InstMeta {
    opname: "OpCreateUserEvent",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 299u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x12c"]
pub const OP_IS_VALID_EVENT: InstMeta = InstMeta {
    opname: "OpIsValidEvent",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 300u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Event"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x12d"]
pub const OP_SET_USER_EVENT_STATUS: InstMeta = InstMeta {
    opname: "OpSetUserEventStatus",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 301u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Event"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Status"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x12e"]
pub const OP_CAPTURE_EVENT_PROFILING_INFO: InstMeta = InstMeta {
    opname: "OpCaptureEventProfilingInfo",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 302u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Event"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Profiling Info"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x12f"]
pub const OP_GET_DEFAULT_QUEUE: InstMeta = InstMeta {
    opname: "OpGetDefaultQueue",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 303u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x130"]
pub const OP_BUILD_ND_RANGE: InstMeta = InstMeta {
    opname: "OpBuildNDRange",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 304u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("GlobalWorkSize"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("LocalWorkSize"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("GlobalWorkOffset"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DeviceEnqueue],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x131"]
pub const OP_IMAGE_SPARSE_SAMPLE_IMPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSparseSampleImplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 305u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x132"]
pub const OP_IMAGE_SPARSE_SAMPLE_EXPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSparseSampleExplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 306u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x133"]
pub const OP_IMAGE_SPARSE_SAMPLE_DREF_IMPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSparseSampleDrefImplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 307u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("D~ref~"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x134"]
pub const OP_IMAGE_SPARSE_SAMPLE_DREF_EXPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSparseSampleDrefExplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 308u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("D~ref~"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x135"]
pub const OP_IMAGE_SPARSE_SAMPLE_PROJ_IMPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSparseSampleProjImplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 309u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x136"]
pub const OP_IMAGE_SPARSE_SAMPLE_PROJ_EXPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSparseSampleProjExplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 310u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x137"]
pub const OP_IMAGE_SPARSE_SAMPLE_PROJ_DREF_IMPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSparseSampleProjDrefImplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 311u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("D~ref~"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x138"]
pub const OP_IMAGE_SPARSE_SAMPLE_PROJ_DREF_EXPLICIT_LOD: InstMeta = InstMeta {
    opname: "OpImageSparseSampleProjDrefExplicitLod",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 312u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("D~ref~"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x139"]
pub const OP_IMAGE_SPARSE_FETCH: InstMeta = InstMeta {
    opname: "OpImageSparseFetch",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 313u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13a"]
pub const OP_IMAGE_SPARSE_GATHER: InstMeta = InstMeta {
    opname: "OpImageSparseGather",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 314u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Component"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13b"]
pub const OP_IMAGE_SPARSE_DREF_GATHER: InstMeta = InstMeta {
    opname: "OpImageSparseDrefGather",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 315u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("D~ref~"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13c"]
pub const OP_IMAGE_SPARSE_TEXELS_RESIDENT: InstMeta = InstMeta {
    opname: "OpImageSparseTexelsResident",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 316u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Resident Code"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13d"]
pub const OP_NO_LINE: InstMeta = InstMeta {
    opname: "OpNoLine",
    class: Some(&PRINTING_CLASS_DEBUG),
    opcode: 317u16,
    operands: &[],
    capabilities: &[],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13e"]
pub const OP_ATOMIC_FLAG_TEST_AND_SET: InstMeta = InstMeta {
    opname: "OpAtomicFlagTestAndSet",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 318u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13f"]
pub const OP_ATOMIC_FLAG_CLEAR: InstMeta = InstMeta {
    opname: "OpAtomicFlagClear",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 319u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Kernel],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x140"]
pub const OP_IMAGE_SPARSE_READ: InstMeta = InstMeta {
    opname: "OpImageSparseRead",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 320u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::SparseResidency],
    extensions: &[],
    version: Some("1.0"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x141"]
pub const OP_SIZE_OF: InstMeta = InstMeta {
    opname: "OpSizeOf",
    class: Some(&PRINTING_CLASS_MISCELLANEOUS),
    opcode: 321u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Addresses],
    extensions: &[],
    version: Some("1.1"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x142"]
pub const OP_TYPE_PIPE_STORAGE: InstMeta = InstMeta {
    opname: "OpTypePipeStorage",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 322u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::PipeStorage],
    extensions: &[],
    version: Some("1.1"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x143"]
pub const OP_CONSTANT_PIPE_STORAGE: InstMeta = InstMeta {
    opname: "OpConstantPipeStorage",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 323u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Capacity"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::PipeStorage],
    extensions: &[],
    version: Some("1.1"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x144"]
pub const OP_CREATE_PIPE_FROM_PIPE_STORAGE: InstMeta = InstMeta {
    opname: "OpCreatePipeFromPipeStorage",
    class: Some(&PRINTING_CLASS_PIPE),
    opcode: 324u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pipe Storage"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::PipeStorage],
    extensions: &[],
    version: Some("1.1"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x145"]
pub const OP_GET_KERNEL_LOCAL_SIZE_FOR_SUBGROUP_COUNT: InstMeta = InstMeta {
    opname: "OpGetKernelLocalSizeForSubgroupCount",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 325u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Subgroup Count"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Invoke"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Align"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupDispatch],
    extensions: &[],
    version: Some("1.1"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x146"]
pub const OP_GET_KERNEL_MAX_NUM_SUBGROUPS: InstMeta = InstMeta {
    opname: "OpGetKernelMaxNumSubgroups",
    class: Some(&PRINTING_CLASS_DEVICE_SIDE_ENQUEUE),
    opcode: 326u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Invoke"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Param Align"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupDispatch],
    extensions: &[],
    version: Some("1.1"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x147"]
pub const OP_TYPE_NAMED_BARRIER: InstMeta = InstMeta {
    opname: "OpTypeNamedBarrier",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 327u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::NamedBarrier],
    extensions: &[],
    version: Some("1.1"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x148"]
pub const OP_NAMED_BARRIER_INITIALIZE: InstMeta = InstMeta {
    opname: "OpNamedBarrierInitialize",
    class: Some(&PRINTING_CLASS_BARRIER),
    opcode: 328u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Subgroup Count"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::NamedBarrier],
    extensions: &[],
    version: Some("1.1"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x149"]
pub const OP_MEMORY_NAMED_BARRIER: InstMeta = InstMeta {
    opname: "OpMemoryNamedBarrier",
    class: Some(&PRINTING_CLASS_BARRIER),
    opcode: 329u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Named Barrier"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::NamedBarrier],
    extensions: &[],
    version: Some("1.1"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14a"]
pub const OP_MODULE_PROCESSED: InstMeta = InstMeta {
    opname: "OpModuleProcessed",
    class: Some(&PRINTING_CLASS_DEBUG),
    opcode: 330u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_LITERAL_STRING,
        name: Some("Process"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: Some("1.1"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14b"]
pub const OP_EXECUTION_MODE_ID: InstMeta = InstMeta {
    opname: "OpExecutionModeId",
    class: Some(&PRINTING_CLASS_MODE_SETTING),
    opcode: 331u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Entry Point"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_EXECUTION_MODE,
            name: Some("Mode"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.2"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c"]
pub const OP_DECORATE_ID: InstMeta = InstMeta {
    opname: "OpDecorateId",
    class: Some(&PRINTING_CLASS_ANNOTATION),
    opcode: 332u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_DECORATION,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[EXTENSION_SPV_GOOGLE_HLSL_FUNCTIONALITY_1],
    version: Some("1.2"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14d"]
pub const OP_GROUP_NON_UNIFORM_ELECT: InstMeta = InstMeta {
    opname: "OpGroupNonUniformElect",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 333u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniform],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14e"]
pub const OP_GROUP_NON_UNIFORM_ALL: InstMeta = InstMeta {
    opname: "OpGroupNonUniformAll",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 334u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformVote],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14f"]
pub const OP_GROUP_NON_UNIFORM_ANY: InstMeta = InstMeta {
    opname: "OpGroupNonUniformAny",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 335u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformVote],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x150"]
pub const OP_GROUP_NON_UNIFORM_ALL_EQUAL: InstMeta = InstMeta {
    opname: "OpGroupNonUniformAllEqual",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 336u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformVote],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x151"]
pub const OP_GROUP_NON_UNIFORM_BROADCAST: InstMeta = InstMeta {
    opname: "OpGroupNonUniformBroadcast",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 337u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Invocation Id"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformBallot],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x152"]
pub const OP_GROUP_NON_UNIFORM_BROADCAST_FIRST: InstMeta = InstMeta {
    opname: "OpGroupNonUniformBroadcastFirst",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 338u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformBallot],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x153"]
pub const OP_GROUP_NON_UNIFORM_BALLOT: InstMeta = InstMeta {
    opname: "OpGroupNonUniformBallot",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 339u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformBallot],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x154"]
pub const OP_GROUP_NON_UNIFORM_INVERSE_BALLOT: InstMeta = InstMeta {
    opname: "OpGroupNonUniformInverseBallot",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 340u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformBallot],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x155"]
pub const OP_GROUP_NON_UNIFORM_BALLOT_BIT_EXTRACT: InstMeta = InstMeta {
    opname: "OpGroupNonUniformBallotBitExtract",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 341u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Index"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformBallot],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x156"]
pub const OP_GROUP_NON_UNIFORM_BALLOT_BIT_COUNT: InstMeta = InstMeta {
    opname: "OpGroupNonUniformBallotBitCount",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 342u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformBallot],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x157"]
pub const OP_GROUP_NON_UNIFORM_BALLOT_FIND_LSB: InstMeta = InstMeta {
    opname: "OpGroupNonUniformBallotFindLSB",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 343u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformBallot],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x158"]
pub const OP_GROUP_NON_UNIFORM_BALLOT_FIND_MSB: InstMeta = InstMeta {
    opname: "OpGroupNonUniformBallotFindMSB",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 344u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformBallot],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x159"]
pub const OP_GROUP_NON_UNIFORM_SHUFFLE: InstMeta = InstMeta {
    opname: "OpGroupNonUniformShuffle",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 345u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Invocation Id"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformShuffle],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15a"]
pub const OP_GROUP_NON_UNIFORM_SHUFFLE_XOR: InstMeta = InstMeta {
    opname: "OpGroupNonUniformShuffleXor",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 346u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Mask"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformShuffle],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15b"]
pub const OP_GROUP_NON_UNIFORM_SHUFFLE_UP: InstMeta = InstMeta {
    opname: "OpGroupNonUniformShuffleUp",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 347u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Delta"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformShuffleRelative],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15c"]
pub const OP_GROUP_NON_UNIFORM_SHUFFLE_DOWN: InstMeta = InstMeta {
    opname: "OpGroupNonUniformShuffleDown",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 348u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Delta"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformShuffleRelative],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15d"]
pub const OP_GROUP_NON_UNIFORM_I_ADD: InstMeta = InstMeta {
    opname: "OpGroupNonUniformIAdd",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 349u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15e"]
pub const OP_GROUP_NON_UNIFORM_F_ADD: InstMeta = InstMeta {
    opname: "OpGroupNonUniformFAdd",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 350u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15f"]
pub const OP_GROUP_NON_UNIFORM_I_MUL: InstMeta = InstMeta {
    opname: "OpGroupNonUniformIMul",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 351u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x160"]
pub const OP_GROUP_NON_UNIFORM_F_MUL: InstMeta = InstMeta {
    opname: "OpGroupNonUniformFMul",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 352u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x161"]
pub const OP_GROUP_NON_UNIFORM_S_MIN: InstMeta = InstMeta {
    opname: "OpGroupNonUniformSMin",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 353u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x162"]
pub const OP_GROUP_NON_UNIFORM_U_MIN: InstMeta = InstMeta {
    opname: "OpGroupNonUniformUMin",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 354u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x163"]
pub const OP_GROUP_NON_UNIFORM_F_MIN: InstMeta = InstMeta {
    opname: "OpGroupNonUniformFMin",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 355u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x164"]
pub const OP_GROUP_NON_UNIFORM_S_MAX: InstMeta = InstMeta {
    opname: "OpGroupNonUniformSMax",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 356u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x165"]
pub const OP_GROUP_NON_UNIFORM_U_MAX: InstMeta = InstMeta {
    opname: "OpGroupNonUniformUMax",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 357u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x166"]
pub const OP_GROUP_NON_UNIFORM_F_MAX: InstMeta = InstMeta {
    opname: "OpGroupNonUniformFMax",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 358u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x167"]
pub const OP_GROUP_NON_UNIFORM_BITWISE_AND: InstMeta = InstMeta {
    opname: "OpGroupNonUniformBitwiseAnd",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 359u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x168"]
pub const OP_GROUP_NON_UNIFORM_BITWISE_OR: InstMeta = InstMeta {
    opname: "OpGroupNonUniformBitwiseOr",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 360u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x169"]
pub const OP_GROUP_NON_UNIFORM_BITWISE_XOR: InstMeta = InstMeta {
    opname: "OpGroupNonUniformBitwiseXor",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 361u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a"]
pub const OP_GROUP_NON_UNIFORM_LOGICAL_AND: InstMeta = InstMeta {
    opname: "OpGroupNonUniformLogicalAnd",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 362u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16b"]
pub const OP_GROUP_NON_UNIFORM_LOGICAL_OR: InstMeta = InstMeta {
    opname: "OpGroupNonUniformLogicalOr",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 363u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16c"]
pub const OP_GROUP_NON_UNIFORM_LOGICAL_XOR: InstMeta = InstMeta {
    opname: "OpGroupNonUniformLogicalXor",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 364u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::GroupNonUniformArithmetic,
        Capability::GroupNonUniformClustered,
        Capability::GroupNonUniformPartitionedEXT,
    ],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16d"]
pub const OP_GROUP_NON_UNIFORM_QUAD_BROADCAST: InstMeta = InstMeta {
    opname: "OpGroupNonUniformQuadBroadcast",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 365u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Index"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformQuad],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16e"]
pub const OP_GROUP_NON_UNIFORM_QUAD_SWAP: InstMeta = InstMeta {
    opname: "OpGroupNonUniformQuadSwap",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 366u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformQuad],
    extensions: &[],
    version: Some("1.3"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x190"]
pub const OP_COPY_LOGICAL: InstMeta = InstMeta {
    opname: "OpCopyLogical",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 400u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.4"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x191"]
pub const OP_PTR_EQUAL: InstMeta = InstMeta {
    opname: "OpPtrEqual",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 401u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.4"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x192"]
pub const OP_PTR_NOT_EQUAL: InstMeta = InstMeta {
    opname: "OpPtrNotEqual",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 402u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: Some("1.4"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x193"]
pub const OP_PTR_DIFF: InstMeta = InstMeta {
    opname: "OpPtrDiff",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 403u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::Addresses,
        Capability::VariablePointers,
        Capability::VariablePointersStorageBuffer,
    ],
    extensions: &[],
    version: Some("1.4"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1040"]
pub const OP_COLOR_ATTACHMENT_READ_EXT: InstMeta = InstMeta {
    opname: "OpColorAttachmentReadEXT",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4160u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Attachment"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sample"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::TileImageColorReadAccessEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1041"]
pub const OP_DEPTH_ATTACHMENT_READ_EXT: InstMeta = InstMeta {
    opname: "OpDepthAttachmentReadEXT",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4161u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sample"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::TileImageDepthReadAccessEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1042"]
pub const OP_STENCIL_ATTACHMENT_READ_EXT: InstMeta = InstMeta {
    opname: "OpStencilAttachmentReadEXT",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4162u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sample"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::TileImageStencilReadAccessEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1043"]
pub const OP_TYPE_TENSOR_ARM: InstMeta = InstMeta {
    opname: "OpTypeTensorARM",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 4163u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Rank"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Shape"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::TensorsARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1044"]
pub const OP_TENSOR_READ_ARM: InstMeta = InstMeta {
    opname: "OpTensorReadARM",
    class: Some(&PRINTING_CLASS_TENSOR),
    opcode: 4164u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Tensor"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_TENSOR_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::TensorsARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1045"]
pub const OP_TENSOR_WRITE_ARM: InstMeta = InstMeta {
    opname: "OpTensorWriteARM",
    class: Some(&PRINTING_CLASS_TENSOR),
    opcode: 4165u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Tensor"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_TENSOR_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::TensorsARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1046"]
pub const OP_TENSOR_QUERY_SIZE_ARM: InstMeta = InstMeta {
    opname: "OpTensorQuerySizeARM",
    class: Some(&PRINTING_CLASS_TENSOR),
    opcode: 4166u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Tensor"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Dimension"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TensorsARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1055"]
pub const OP_GRAPH_CONSTANT_ARM: InstMeta = InstMeta {
    opname: "OpGraphConstantARM",
    class: Some(&PRINTING_CLASS_GRAPH),
    opcode: 4181u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("GraphConstantID"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GraphARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1056"]
pub const OP_GRAPH_ENTRY_POINT_ARM: InstMeta = InstMeta {
    opname: "OpGraphEntryPointARM",
    class: Some(&PRINTING_CLASS_GRAPH),
    opcode: 4182u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Graph"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Name"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Interface"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::GraphARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1057"]
pub const OP_GRAPH_ARM: InstMeta = InstMeta {
    opname: "OpGraphARM",
    class: Some(&PRINTING_CLASS_GRAPH),
    opcode: 4183u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GraphARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1058"]
pub const OP_GRAPH_INPUT_ARM: InstMeta = InstMeta {
    opname: "OpGraphInputARM",
    class: Some(&PRINTING_CLASS_GRAPH),
    opcode: 4184u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("InputIndex"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ElementIndex"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::GraphARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1059"]
pub const OP_GRAPH_SET_OUTPUT_ARM: InstMeta = InstMeta {
    opname: "OpGraphSetOutputARM",
    class: Some(&PRINTING_CLASS_GRAPH),
    opcode: 4185u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("OutputIndex"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ElementIndex"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::GraphARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x105a"]
pub const OP_GRAPH_END_ARM: InstMeta = InstMeta {
    opname: "OpGraphEndARM",
    class: Some(&PRINTING_CLASS_GRAPH),
    opcode: 4186u16,
    operands: &[],
    capabilities: &[Capability::GraphARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x105e"]
pub const OP_TYPE_GRAPH_ARM: InstMeta = InstMeta {
    opname: "OpTypeGraphARM",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 4190u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("NumInputs"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("InOutTypes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::GraphARM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1140"]
pub const OP_TERMINATE_INVOCATION: InstMeta = InstMeta {
    opname: "OpTerminateInvocation",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 4416u16,
    operands: &[],
    capabilities: &[Capability::Shader],
    extensions: &[EXTENSION_SPV_KHR_TERMINATE_INVOCATION],
    version: Some("1.6"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1141"]
pub const OP_TYPE_UNTYPED_POINTER_KHR: InstMeta = InstMeta {
    opname: "OpTypeUntypedPointerKHR",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 4417u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_STORAGE_CLASS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::UntypedPointersKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1142"]
pub const OP_UNTYPED_VARIABLE_KHR: InstMeta = InstMeta {
    opname: "OpUntypedVariableKHR",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 4418u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_STORAGE_CLASS,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Data Type"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Initializer"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::UntypedPointersKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1143"]
pub const OP_UNTYPED_ACCESS_CHAIN_KHR: InstMeta = InstMeta {
    opname: "OpUntypedAccessChainKHR",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 4419u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Indexes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::UntypedPointersKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1144"]
pub const OP_UNTYPED_IN_BOUNDS_ACCESS_CHAIN_KHR: InstMeta = InstMeta {
    opname: "OpUntypedInBoundsAccessChainKHR",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 4420u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Indexes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::UntypedPointersKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1145"]
pub const OP_SUBGROUP_BALLOT_KHR: InstMeta = InstMeta {
    opname: "OpSubgroupBallotKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 4421u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupBallotKHR],
    extensions: &[EXTENSION_SPV_KHR_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1146"]
pub const OP_SUBGROUP_FIRST_INVOCATION_KHR: InstMeta = InstMeta {
    opname: "OpSubgroupFirstInvocationKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 4422u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupBallotKHR],
    extensions: &[EXTENSION_SPV_KHR_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1147"]
pub const OP_UNTYPED_PTR_ACCESS_CHAIN_KHR: InstMeta = InstMeta {
    opname: "OpUntypedPtrAccessChainKHR",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 4423u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Indexes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::UntypedPointersKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1148"]
pub const OP_UNTYPED_IN_BOUNDS_PTR_ACCESS_CHAIN_KHR: InstMeta = InstMeta {
    opname: "OpUntypedInBoundsPtrAccessChainKHR",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 4424u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Indexes"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::UntypedPointersKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1149"]
pub const OP_UNTYPED_ARRAY_LENGTH_KHR: InstMeta = InstMeta {
    opname: "OpUntypedArrayLengthKHR",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 4425u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Array member"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::UntypedPointersKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x114a"]
pub const OP_UNTYPED_PREFETCH_KHR: InstMeta = InstMeta {
    opname: "OpUntypedPrefetchKHR",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 4426u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Num Bytes"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RW"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Locality"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cache Type"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::UntypedPointersKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x114b"]
pub const OP_FMA_KHR: InstMeta = InstMeta {
    opname: "OpFmaKHR",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 4427u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 3"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::FMAKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x114c"]
pub const OP_SUBGROUP_ALL_KHR: InstMeta = InstMeta {
    opname: "OpSubgroupAllKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 4428u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupVoteKHR],
    extensions: &[EXTENSION_SPV_KHR_SUBGROUP_VOTE],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x114d"]
pub const OP_SUBGROUP_ANY_KHR: InstMeta = InstMeta {
    opname: "OpSubgroupAnyKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 4429u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupVoteKHR],
    extensions: &[EXTENSION_SPV_KHR_SUBGROUP_VOTE],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x114e"]
pub const OP_SUBGROUP_ALL_EQUAL_KHR: InstMeta = InstMeta {
    opname: "OpSubgroupAllEqualKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 4430u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupVoteKHR],
    extensions: &[EXTENSION_SPV_KHR_SUBGROUP_VOTE],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x114f"]
pub const OP_GROUP_NON_UNIFORM_ROTATE_KHR: InstMeta = InstMeta {
    opname: "OpGroupNonUniformRotateKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 4431u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Delta"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClusterSize"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::GroupNonUniformRotateKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1150"]
pub const OP_SUBGROUP_READ_INVOCATION_KHR: InstMeta = InstMeta {
    opname: "OpSubgroupReadInvocationKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 4432u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Index"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupBallotKHR],
    extensions: &[EXTENSION_SPV_KHR_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1151"]
pub const OP_EXT_INST_WITH_FORWARD_REFS_KHR: InstMeta = InstMeta {
    opname: "OpExtInstWithForwardRefsKHR",
    class: Some(&PRINTING_CLASS_EXTENSION),
    opcode: 4433u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Set"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_EXT_INST_INTEGER,
            name: Some("Instruction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1, Operand 2, ..."),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[EXTENSION_SPV_KHR_RELAXED_EXTENDED_INSTRUCTION],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1152"]
pub const OP_UNTYPED_GROUP_ASYNC_COPY_KHR: InstMeta = InstMeta {
    opname: "OpUntypedGroupAsyncCopyKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 4434u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Destination"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Source"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element Num Bytes"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Num Elements"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Event"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: Some("Destination Memory Operands"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: Some("Source Memory Operands"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::UntypedPointersKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x115d"]
pub const OP_TRACE_RAY_KHR: InstMeta = InstMeta {
    opname: "OpTraceRayKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4445u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cull Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_TRACING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x115e"]
pub const OP_EXECUTE_CALLABLE_KHR: InstMeta = InstMeta {
    opname: "OpExecuteCallableKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4446u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Callable Data"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_TRACING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x115f"]
pub const OP_CONVERT_U_TO_ACCELERATION_STRUCTURE_KHR: InstMeta = InstMeta {
    opname: "OpConvertUToAccelerationStructureKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4447u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accel"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingKHR, Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_TRACING, EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1160"]
pub const OP_IGNORE_INTERSECTION_KHR: InstMeta = InstMeta {
    opname: "OpIgnoreIntersectionKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4448u16,
    operands: &[],
    capabilities: &[Capability::RayTracingKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_TRACING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1161"]
pub const OP_TERMINATE_RAY_KHR: InstMeta = InstMeta {
    opname: "OpTerminateRayKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4449u16,
    operands: &[],
    capabilities: &[Capability::RayTracingKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_TRACING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1162"]
pub const OP_S_DOT: InstMeta = InstMeta {
    opname: "OpSDot",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 4450u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 2"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_PACKED_VECTOR_FORMAT,
            name: Some("Packed Vector Format"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::DotProduct],
    extensions: &[EXTENSION_SPV_KHR_INTEGER_DOT_PRODUCT],
    version: Some("1.6"),
    last_version: None,
    aliases: &["OpSDotKHR"],
    provisional: false,
};
#[doc = "opcode: 0x1163"]
pub const OP_U_DOT: InstMeta = InstMeta {
    opname: "OpUDot",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 4451u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 2"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_PACKED_VECTOR_FORMAT,
            name: Some("Packed Vector Format"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::DotProduct],
    extensions: &[EXTENSION_SPV_KHR_INTEGER_DOT_PRODUCT],
    version: Some("1.6"),
    last_version: None,
    aliases: &["OpUDotKHR"],
    provisional: false,
};
#[doc = "opcode: 0x1164"]
pub const OP_SU_DOT: InstMeta = InstMeta {
    opname: "OpSUDot",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 4452u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 2"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_PACKED_VECTOR_FORMAT,
            name: Some("Packed Vector Format"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::DotProduct],
    extensions: &[EXTENSION_SPV_KHR_INTEGER_DOT_PRODUCT],
    version: Some("1.6"),
    last_version: None,
    aliases: &["OpSUDotKHR"],
    provisional: false,
};
#[doc = "opcode: 0x1165"]
pub const OP_S_DOT_ACC_SAT: InstMeta = InstMeta {
    opname: "OpSDotAccSat",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 4453u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 2"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accumulator"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_PACKED_VECTOR_FORMAT,
            name: Some("Packed Vector Format"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::DotProduct],
    extensions: &[EXTENSION_SPV_KHR_INTEGER_DOT_PRODUCT],
    version: Some("1.6"),
    last_version: None,
    aliases: &["OpSDotAccSatKHR"],
    provisional: false,
};
#[doc = "opcode: 0x1166"]
pub const OP_U_DOT_ACC_SAT: InstMeta = InstMeta {
    opname: "OpUDotAccSat",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 4454u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 2"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accumulator"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_PACKED_VECTOR_FORMAT,
            name: Some("Packed Vector Format"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::DotProduct],
    extensions: &[EXTENSION_SPV_KHR_INTEGER_DOT_PRODUCT],
    version: Some("1.6"),
    last_version: None,
    aliases: &["OpUDotAccSatKHR"],
    provisional: false,
};
#[doc = "opcode: 0x1167"]
pub const OP_SU_DOT_ACC_SAT: InstMeta = InstMeta {
    opname: "OpSUDotAccSat",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 4455u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vector 2"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accumulator"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_PACKED_VECTOR_FORMAT,
            name: Some("Packed Vector Format"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::DotProduct],
    extensions: &[EXTENSION_SPV_KHR_INTEGER_DOT_PRODUCT],
    version: Some("1.6"),
    last_version: None,
    aliases: &["OpSUDotAccSatKHR"],
    provisional: false,
};
#[doc = "opcode: 0x1168"]
pub const OP_TYPE_COOPERATIVE_MATRIX_KHR: InstMeta = InstMeta {
    opname: "OpTypeCooperativeMatrixKHR",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 4456u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Component Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Scope"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Rows"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Columns"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Use"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1169"]
pub const OP_COOPERATIVE_MATRIX_LOAD_KHR: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixLoadKHR",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 4457u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MemoryLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Stride"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: Some("Memory Operand"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x116a"]
pub const OP_COOPERATIVE_MATRIX_STORE_KHR: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixStoreKHR",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 4458u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MemoryLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Stride"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: Some("Memory Operand"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x116b"]
pub const OP_COOPERATIVE_MATRIX_MUL_ADD_KHR: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixMulAddKHR",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 4459u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("C"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_COOPERATIVE_MATRIX_OPERANDS,
            name: Some("Cooperative Matrix Operands"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x116c"]
pub const OP_COOPERATIVE_MATRIX_LENGTH_KHR: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixLengthKHR",
    class: Some(&PRINTING_CLASS_MISCELLANEOUS),
    opcode: 4460u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Type"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x116d"]
pub const OP_CONSTANT_COMPOSITE_REPLICATE_EXT: InstMeta = InstMeta {
    opname: "OpConstantCompositeReplicateEXT",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 4461u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ReplicatedCompositesEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x116e"]
pub const OP_SPEC_CONSTANT_COMPOSITE_REPLICATE_EXT: InstMeta = InstMeta {
    opname: "OpSpecConstantCompositeReplicateEXT",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 4462u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ReplicatedCompositesEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x116f"]
pub const OP_COMPOSITE_CONSTRUCT_REPLICATE_EXT: InstMeta = InstMeta {
    opname: "OpCompositeConstructReplicateEXT",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 4463u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ReplicatedCompositesEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1178"]
pub const OP_TYPE_RAY_QUERY_KHR: InstMeta = InstMeta {
    opname: "OpTypeRayQueryKHR",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 4472u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1179"]
pub const OP_RAY_QUERY_INITIALIZE_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryInitializeKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4473u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayFlags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("CullMask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayOrigin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayTMin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayDirection"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayTMax"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x117a"]
pub const OP_RAY_QUERY_TERMINATE_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryTerminateKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4474u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("RayQuery"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x117b"]
pub const OP_RAY_QUERY_GENERATE_INTERSECTION_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGenerateIntersectionKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4475u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("HitT"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x117c"]
pub const OP_RAY_QUERY_CONFIRM_INTERSECTION_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryConfirmIntersectionKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4476u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("RayQuery"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x117d"]
pub const OP_RAY_QUERY_PROCEED_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryProceedKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4477u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x117f"]
pub const OP_RAY_QUERY_GET_INTERSECTION_TYPE_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionTypeKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 4479u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1180"]
pub const OP_IMAGE_SAMPLE_WEIGHTED_QCOM: InstMeta = InstMeta {
    opname: "OpImageSampleWeightedQCOM",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4480u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Texture"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Weights"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TextureSampleWeightedQCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1181"]
pub const OP_IMAGE_BOX_FILTER_QCOM: InstMeta = InstMeta {
    opname: "OpImageBoxFilterQCOM",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4481u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Texture"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Box Size"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TextureBoxFilterQCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1182"]
pub const OP_IMAGE_BLOCK_MATCH_SSDQCOM: InstMeta = InstMeta {
    opname: "OpImageBlockMatchSSDQCOM",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4482u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Size"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TextureBlockMatchQCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1183"]
pub const OP_IMAGE_BLOCK_MATCH_SADQCOM: InstMeta = InstMeta {
    opname: "OpImageBlockMatchSADQCOM",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4483u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Size"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TextureBlockMatchQCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1191"]
pub const OP_BIT_CAST_ARRAY_QCOM: InstMeta = InstMeta {
    opname: "OpBitCastArrayQCOM",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 4497u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Source Array"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixConversionQCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1194"]
pub const OP_IMAGE_BLOCK_MATCH_WINDOW_SSDQCOM: InstMeta = InstMeta {
    opname: "OpImageBlockMatchWindowSSDQCOM",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4500u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Size"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TextureBlockMatch2QCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1195"]
pub const OP_IMAGE_BLOCK_MATCH_WINDOW_SADQCOM: InstMeta = InstMeta {
    opname: "OpImageBlockMatchWindowSADQCOM",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4501u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Size"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TextureBlockMatch2QCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1196"]
pub const OP_IMAGE_BLOCK_MATCH_GATHER_SSDQCOM: InstMeta = InstMeta {
    opname: "OpImageBlockMatchGatherSSDQCOM",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4502u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Size"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TextureBlockMatch2QCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1197"]
pub const OP_IMAGE_BLOCK_MATCH_GATHER_SADQCOM: InstMeta = InstMeta {
    opname: "OpImageBlockMatchGatherSADQCOM",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 4503u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Coordinates"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Size"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TextureBlockMatch2QCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x11bc"]
pub const OP_COMPOSITE_CONSTRUCT_COOP_MAT_QCOM: InstMeta = InstMeta {
    opname: "OpCompositeConstructCoopMatQCOM",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 4540u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Source Array"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixConversionQCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x11bd"]
pub const OP_COMPOSITE_EXTRACT_COOP_MAT_QCOM: InstMeta = InstMeta {
    opname: "OpCompositeExtractCoopMatQCOM",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 4541u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Source Cooperative Matrix"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixConversionQCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x11be"]
pub const OP_EXTRACT_SUB_ARRAY_QCOM: InstMeta = InstMeta {
    opname: "OpExtractSubArrayQCOM",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 4542u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Source Array"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("index"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixConversionQCOM],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1388"]
pub const OP_GROUP_I_ADD_NON_UNIFORM_AMD: InstMeta = InstMeta {
    opname: "OpGroupIAddNonUniformAMD",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5000u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[EXTENSION_SPV_AMD_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1389"]
pub const OP_GROUP_F_ADD_NON_UNIFORM_AMD: InstMeta = InstMeta {
    opname: "OpGroupFAddNonUniformAMD",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5001u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[EXTENSION_SPV_AMD_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x138a"]
pub const OP_GROUP_F_MIN_NON_UNIFORM_AMD: InstMeta = InstMeta {
    opname: "OpGroupFMinNonUniformAMD",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5002u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[EXTENSION_SPV_AMD_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x138b"]
pub const OP_GROUP_U_MIN_NON_UNIFORM_AMD: InstMeta = InstMeta {
    opname: "OpGroupUMinNonUniformAMD",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5003u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[EXTENSION_SPV_AMD_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x138c"]
pub const OP_GROUP_S_MIN_NON_UNIFORM_AMD: InstMeta = InstMeta {
    opname: "OpGroupSMinNonUniformAMD",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5004u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[EXTENSION_SPV_AMD_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x138d"]
pub const OP_GROUP_F_MAX_NON_UNIFORM_AMD: InstMeta = InstMeta {
    opname: "OpGroupFMaxNonUniformAMD",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5005u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[EXTENSION_SPV_AMD_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x138e"]
pub const OP_GROUP_U_MAX_NON_UNIFORM_AMD: InstMeta = InstMeta {
    opname: "OpGroupUMaxNonUniformAMD",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5006u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[EXTENSION_SPV_AMD_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x138f"]
pub const OP_GROUP_S_MAX_NON_UNIFORM_AMD: InstMeta = InstMeta {
    opname: "OpGroupSMaxNonUniformAMD",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5007u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Groups],
    extensions: &[EXTENSION_SPV_AMD_SHADER_BALLOT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1393"]
pub const OP_FRAGMENT_MASK_FETCH_AMD: InstMeta = InstMeta {
    opname: "OpFragmentMaskFetchAMD",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5011u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::FragmentMaskAMD],
    extensions: &[EXTENSION_SPV_AMD_SHADER_FRAGMENT_MASK],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1394"]
pub const OP_FRAGMENT_FETCH_AMD: InstMeta = InstMeta {
    opname: "OpFragmentFetchAMD",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5012u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Fragment Index"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::FragmentMaskAMD],
    extensions: &[EXTENSION_SPV_AMD_SHADER_FRAGMENT_MASK],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13c0"]
pub const OP_READ_CLOCK_KHR: InstMeta = InstMeta {
    opname: "OpReadClockKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5056u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Scope"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderClockKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13d2"]
pub const OP_ALLOCATE_NODE_PAYLOADS_AMDX: InstMeta = InstMeta {
    opname: "OpAllocateNodePayloadsAMDX",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5074u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Visibility"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload Count"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Node Index"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderEnqueueAMDX],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x13d3"]
pub const OP_ENQUEUE_NODE_PAYLOADS_AMDX: InstMeta = InstMeta {
    opname: "OpEnqueueNodePayloadsAMDX",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5075u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Payload Array"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::ShaderEnqueueAMDX],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x13d4"]
pub const OP_TYPE_NODE_PAYLOAD_ARRAY_AMDX: InstMeta = InstMeta {
    opname: "OpTypeNodePayloadArrayAMDX",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5076u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload Type"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderEnqueueAMDX],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x13d6"]
pub const OP_FINISH_WRITING_NODE_PAYLOAD_AMDX: InstMeta = InstMeta {
    opname: "OpFinishWritingNodePayloadAMDX",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5078u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderEnqueueAMDX],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x13e2"]
pub const OP_NODE_PAYLOAD_ARRAY_LENGTH_AMDX: InstMeta = InstMeta {
    opname: "OpNodePayloadArrayLengthAMDX",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5090u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload Array"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderEnqueueAMDX],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x13ed"]
pub const OP_IS_NODE_PAYLOAD_VALID_AMDX: InstMeta = InstMeta {
    opname: "OpIsNodePayloadValidAMDX",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5101u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Node Index"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderEnqueueAMDX],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x13ef"]
pub const OP_CONSTANT_STRING_AMDX: InstMeta = InstMeta {
    opname: "OpConstantStringAMDX",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5103u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Literal String"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderEnqueueAMDX],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x13f0"]
pub const OP_SPEC_CONSTANT_STRING_AMDX: InstMeta = InstMeta {
    opname: "OpSpecConstantStringAMDX",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5104u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Literal String"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderEnqueueAMDX],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x13f6"]
pub const OP_GROUP_NON_UNIFORM_QUAD_ALL_KHR: InstMeta = InstMeta {
    opname: "OpGroupNonUniformQuadAllKHR",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 5110u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::QuadControlKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13f7"]
pub const OP_GROUP_NON_UNIFORM_QUAD_ANY_KHR: InstMeta = InstMeta {
    opname: "OpGroupNonUniformQuadAnyKHR",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 5111u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Predicate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::QuadControlKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13fb"]
pub const OP_TYPE_BUFFER_EXT: InstMeta = InstMeta {
    opname: "OpTypeBufferEXT",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 5115u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_STORAGE_CLASS,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DescriptorHeapEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x13ff"]
pub const OP_BUFFER_POINTER_EXT: InstMeta = InstMeta {
    opname: "OpBufferPointerEXT",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 5119u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Buffer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DescriptorHeapEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1406"]
pub const OP_UNTYPED_IMAGE_TEXEL_POINTER_EXT: InstMeta = InstMeta {
    opname: "OpUntypedImageTexelPointerEXT",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 5126u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ImageType"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sample"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DescriptorHeapEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1407"]
pub const OP_MEMBER_DECORATE_ID_EXT: InstMeta = InstMeta {
    opname: "OpMemberDecorateIdEXT",
    class: Some(&PRINTING_CLASS_ANNOTATION),
    opcode: 5127u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Structure Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Member"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_DECORATION,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DescriptorHeapEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1409"]
pub const OP_CONSTANT_SIZE_OF_EXT: InstMeta = InstMeta {
    opname: "OpConstantSizeOfEXT",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 5129u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Type"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DescriptorHeapEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1481"]
pub const OP_HIT_OBJECT_RECORD_HIT_MOTION_NV: InstMeta = InstMeta {
    opname: "OpHitObjectRecordHitMotionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5249u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Acceleration Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("InstanceId"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("PrimitiveId"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("GeometryIndex"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Kind"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Current Time"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("HitObject Attributes"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::ShaderInvocationReorderNV,
        Capability::RayTracingMotionBlurNV,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1482"]
pub const OP_HIT_OBJECT_RECORD_HIT_WITH_INDEX_MOTION_NV: InstMeta = InstMeta {
    opname: "OpHitObjectRecordHitWithIndexMotionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5250u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Acceleration Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("InstanceId"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("PrimitiveId"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("GeometryIndex"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Kind"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Current Time"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("HitObject Attributes"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::ShaderInvocationReorderNV,
        Capability::RayTracingMotionBlurNV,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1483"]
pub const OP_HIT_OBJECT_RECORD_MISS_MOTION_NV: InstMeta = InstMeta {
    opname: "OpHitObjectRecordMissMotionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5251u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Current Time"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::ShaderInvocationReorderNV,
        Capability::RayTracingMotionBlurNV,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1484"]
pub const OP_HIT_OBJECT_GET_WORLD_TO_OBJECT_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetWorldToObjectNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5252u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1485"]
pub const OP_HIT_OBJECT_GET_OBJECT_TO_WORLD_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetObjectToWorldNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5253u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1486"]
pub const OP_HIT_OBJECT_GET_OBJECT_RAY_DIRECTION_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetObjectRayDirectionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5254u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1487"]
pub const OP_HIT_OBJECT_GET_OBJECT_RAY_ORIGIN_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetObjectRayOriginNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5255u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1488"]
pub const OP_HIT_OBJECT_TRACE_RAY_MOTION_NV: InstMeta = InstMeta {
    opname: "OpHitObjectTraceRayMotionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5256u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Acceleration Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayFlags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cullmask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Time"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::ShaderInvocationReorderNV,
        Capability::RayTracingMotionBlurNV,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1489"]
pub const OP_HIT_OBJECT_GET_SHADER_RECORD_BUFFER_HANDLE_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetShaderRecordBufferHandleNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5257u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x148a"]
pub const OP_HIT_OBJECT_GET_SHADER_BINDING_TABLE_RECORD_INDEX_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetShaderBindingTableRecordIndexNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5258u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x148b"]
pub const OP_HIT_OBJECT_RECORD_EMPTY_NV: InstMeta = InstMeta {
    opname: "OpHitObjectRecordEmptyNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5259u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Hit Object"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x148c"]
pub const OP_HIT_OBJECT_TRACE_RAY_NV: InstMeta = InstMeta {
    opname: "OpHitObjectTraceRayNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5260u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Acceleration Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayFlags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cullmask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x148d"]
pub const OP_HIT_OBJECT_RECORD_HIT_NV: InstMeta = InstMeta {
    opname: "OpHitObjectRecordHitNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5261u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Acceleration Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("InstanceId"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("PrimitiveId"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("GeometryIndex"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Kind"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("HitObject Attributes"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x148e"]
pub const OP_HIT_OBJECT_RECORD_HIT_WITH_INDEX_NV: InstMeta = InstMeta {
    opname: "OpHitObjectRecordHitWithIndexNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5262u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Acceleration Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("InstanceId"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("PrimitiveId"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("GeometryIndex"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Kind"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("HitObject Attributes"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x148f"]
pub const OP_HIT_OBJECT_RECORD_MISS_NV: InstMeta = InstMeta {
    opname: "OpHitObjectRecordMissNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5263u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TMax"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1490"]
pub const OP_HIT_OBJECT_EXECUTE_SHADER_NV: InstMeta = InstMeta {
    opname: "OpHitObjectExecuteShaderNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5264u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1491"]
pub const OP_HIT_OBJECT_GET_CURRENT_TIME_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetCurrentTimeNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5265u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1492"]
pub const OP_HIT_OBJECT_GET_ATTRIBUTES_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetAttributesNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5266u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object Attribute"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1493"]
pub const OP_HIT_OBJECT_GET_HIT_KIND_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetHitKindNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5267u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1494"]
pub const OP_HIT_OBJECT_GET_PRIMITIVE_INDEX_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetPrimitiveIndexNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5268u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1495"]
pub const OP_HIT_OBJECT_GET_GEOMETRY_INDEX_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetGeometryIndexNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5269u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1496"]
pub const OP_HIT_OBJECT_GET_INSTANCE_ID_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetInstanceIdNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5270u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1497"]
pub const OP_HIT_OBJECT_GET_INSTANCE_CUSTOM_INDEX_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetInstanceCustomIndexNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5271u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1498"]
pub const OP_HIT_OBJECT_GET_WORLD_RAY_DIRECTION_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetWorldRayDirectionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5272u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1499"]
pub const OP_HIT_OBJECT_GET_WORLD_RAY_ORIGIN_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetWorldRayOriginNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5273u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x149a"]
pub const OP_HIT_OBJECT_GET_RAY_T_MAX_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetRayTMaxNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5274u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x149b"]
pub const OP_HIT_OBJECT_GET_RAY_T_MIN_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetRayTMinNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5275u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x149c"]
pub const OP_HIT_OBJECT_IS_EMPTY_NV: InstMeta = InstMeta {
    opname: "OpHitObjectIsEmptyNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5276u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x149d"]
pub const OP_HIT_OBJECT_IS_HIT_NV: InstMeta = InstMeta {
    opname: "OpHitObjectIsHitNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5277u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x149e"]
pub const OP_HIT_OBJECT_IS_MISS_NV: InstMeta = InstMeta {
    opname: "OpHitObjectIsMissNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5278u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x149f"]
pub const OP_REORDER_THREAD_WITH_HIT_OBJECT_NV: InstMeta = InstMeta {
    opname: "OpReorderThreadWithHitObjectNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5279u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hint"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bits"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14a0"]
pub const OP_REORDER_THREAD_WITH_HINT_NV: InstMeta = InstMeta {
    opname: "OpReorderThreadWithHintNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5280u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hint"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bits"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14a1"]
pub const OP_TYPE_HIT_OBJECT_NV: InstMeta = InstMeta {
    opname: "OpTypeHitObjectNV",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 5281u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::ShaderInvocationReorderNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14a3"]
pub const OP_IMAGE_SAMPLE_FOOTPRINT_NV: InstMeta = InstMeta {
    opname: "OpImageSampleFootprintNV",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 5283u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampled Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Granularity"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coarse"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_IMAGE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::ImageFootprintNV],
    extensions: &[EXTENSION_SPV_NV_SHADER_IMAGE_FOOTPRINT],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14a8"]
pub const OP_TYPE_VECTOR_ID_EXT: InstMeta = InstMeta {
    opname: "OpTypeVectorIdEXT",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 5288u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Component Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Component Count"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeVectorNV, Capability::LongVectorEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpTypeCooperativeVectorNV"],
    provisional: false,
};
#[doc = "opcode: 0x14a9"]
pub const OP_COOPERATIVE_VECTOR_MATRIX_MUL_NV: InstMeta = InstMeta {
    opname: "OpCooperativeVectorMatrixMulNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5289u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("InputInterpretation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MatrixOffset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MatrixInterpretation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("M"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("K"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MemoryLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Transpose"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MatrixStride"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_COOPERATIVE_MATRIX_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::CooperativeVectorNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14aa"]
pub const OP_COOPERATIVE_VECTOR_OUTER_PRODUCT_ACCUMULATE_NV: InstMeta = InstMeta {
    opname: "OpCooperativeVectorOuterProductAccumulateNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5290u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MemoryLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MatrixInterpretation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MatrixStride"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::CooperativeVectorTrainingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14ab"]
pub const OP_COOPERATIVE_VECTOR_REDUCE_SUM_ACCUMULATE_NV: InstMeta = InstMeta {
    opname: "OpCooperativeVectorReduceSumAccumulateNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5291u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("V"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeVectorTrainingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14ac"]
pub const OP_COOPERATIVE_VECTOR_MATRIX_MUL_ADD_NV: InstMeta = InstMeta {
    opname: "OpCooperativeVectorMatrixMulAddNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5292u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("InputInterpretation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MatrixOffset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MatrixInterpretation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bias"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("BiasOffset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("BiasInterpretation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("M"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("K"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MemoryLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Transpose"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("MatrixStride"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_COOPERATIVE_MATRIX_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::CooperativeVectorNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14ad"]
pub const OP_COOPERATIVE_MATRIX_CONVERT_NV: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixConvertNV",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 5293u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixConversionsNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14ae"]
pub const OP_EMIT_MESH_TASKS_EXT: InstMeta = InstMeta {
    opname: "OpEmitMeshTasksEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5294u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Group Count X"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Group Count Y"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Group Count Z"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::MeshShadingEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14af"]
pub const OP_SET_MESH_OUTPUTS_EXT: InstMeta = InstMeta {
    opname: "OpSetMeshOutputsEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5295u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Vertex Count"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Primitive Count"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::MeshShadingEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14b0"]
pub const OP_GROUP_NON_UNIFORM_PARTITION_EXT: InstMeta = InstMeta {
    opname: "OpGroupNonUniformPartitionEXT",
    class: Some(&PRINTING_CLASS_NON_UNIFORM),
    opcode: 5296u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupNonUniformPartitionedEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpGroupNonUniformPartitionNV"],
    provisional: false,
};
#[doc = "opcode: 0x14b3"]
pub const OP_WRITE_PACKED_PRIMITIVE_INDICES_4_X_8_NV: InstMeta = InstMeta {
    opname: "OpWritePackedPrimitiveIndices4x8NV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5299u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Index Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Indices"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::MeshShadingNV],
    extensions: &[EXTENSION_SPV_NV_MESH_SHADER],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14b4"]
pub const OP_FETCH_MICRO_TRIANGLE_VERTEX_POSITION_NV: InstMeta = InstMeta {
    opname: "OpFetchMicroTriangleVertexPositionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5300u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Instance Id"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Geometry Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Primitive Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Barycentric"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DisplacementMicromapNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14b5"]
pub const OP_FETCH_MICRO_TRIANGLE_VERTEX_BARYCENTRIC_NV: InstMeta = InstMeta {
    opname: "OpFetchMicroTriangleVertexBarycentricNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5301u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Instance Id"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Geometry Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Primitive Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Barycentric"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DisplacementMicromapNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14b6"]
pub const OP_COOPERATIVE_VECTOR_LOAD_NV: InstMeta = InstMeta {
    opname: "OpCooperativeVectorLoadNV",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 5302u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::CooperativeVectorNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14b7"]
pub const OP_COOPERATIVE_VECTOR_STORE_NV: InstMeta = InstMeta {
    opname: "OpCooperativeVectorStoreNV",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 5303u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::CooperativeVectorNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14b8"]
pub const OP_HIT_OBJECT_RECORD_FROM_QUERY_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectRecordFromQueryEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5304u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Query"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object Attributes"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14b9"]
pub const OP_HIT_OBJECT_RECORD_MISS_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectRecordMissEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5305u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmax"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14ba"]
pub const OP_HIT_OBJECT_RECORD_MISS_MOTION_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectRecordMissMotionEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5306u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Current Time"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::ShaderInvocationReorderEXT,
        Capability::RayTracingMotionBlurNV,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14bb"]
pub const OP_HIT_OBJECT_GET_INTERSECTION_TRIANGLE_VERTEX_POSITIONS_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetIntersectionTriangleVertexPositionsEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5307u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14bc"]
pub const OP_HIT_OBJECT_GET_RAY_FLAGS_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetRayFlagsEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5308u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14bd"]
pub const OP_HIT_OBJECT_SET_SHADER_BINDING_TABLE_RECORD_INDEX_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectSetShaderBindingTableRecordIndexEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5309u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Record Index"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14be"]
pub const OP_HIT_OBJECT_REORDER_EXECUTE_SHADER_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectReorderExecuteShaderEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5310u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hint"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bits"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14bf"]
pub const OP_HIT_OBJECT_TRACE_REORDER_EXECUTE_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectTraceReorderExecuteEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5311u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Acceleration Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cull Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hint"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bits"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c0"]
pub const OP_HIT_OBJECT_TRACE_MOTION_REORDER_EXECUTE_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectTraceMotionReorderExecuteEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5312u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Acceleration Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cull Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Current Time"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hint"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bits"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[
        Capability::ShaderInvocationReorderEXT,
        Capability::RayTracingMotionBlurNV,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c1"]
pub const OP_TYPE_HIT_OBJECT_EXT: InstMeta = InstMeta {
    opname: "OpTypeHitObjectEXT",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 5313u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c2"]
pub const OP_REORDER_THREAD_WITH_HINT_EXT: InstMeta = InstMeta {
    opname: "OpReorderThreadWithHintEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5314u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hint"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bits"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c3"]
pub const OP_REORDER_THREAD_WITH_HIT_OBJECT_EXT: InstMeta = InstMeta {
    opname: "OpReorderThreadWithHitObjectEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5315u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hint"),
            quantifier: Quantifier::ZeroOrOne,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bits"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c4"]
pub const OP_HIT_OBJECT_TRACE_RAY_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectTraceRayEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5316u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Acceleration Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cull Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c5"]
pub const OP_HIT_OBJECT_TRACE_RAY_MOTION_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectTraceRayMotionEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5317u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Acceleration Structure"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cull Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Current Time"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::ShaderInvocationReorderEXT,
        Capability::RayTracingMotionBlurNV,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c6"]
pub const OP_HIT_OBJECT_RECORD_EMPTY_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectRecordEmptyEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5318u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Hit Object"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c7"]
pub const OP_HIT_OBJECT_EXECUTE_SHADER_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectExecuteShaderEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5319u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c8"]
pub const OP_HIT_OBJECT_GET_CURRENT_TIME_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetCurrentTimeEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5320u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::ShaderInvocationReorderEXT,
        Capability::RayTracingMotionBlurNV,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14c9"]
pub const OP_HIT_OBJECT_GET_ATTRIBUTES_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetAttributesEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5321u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object Attribute"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14ca"]
pub const OP_HIT_OBJECT_GET_HIT_KIND_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetHitKindEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5322u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14cb"]
pub const OP_HIT_OBJECT_GET_PRIMITIVE_INDEX_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetPrimitiveIndexEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5323u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14cc"]
pub const OP_HIT_OBJECT_GET_GEOMETRY_INDEX_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetGeometryIndexEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5324u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14cd"]
pub const OP_HIT_OBJECT_GET_INSTANCE_ID_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetInstanceIdEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5325u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14ce"]
pub const OP_HIT_OBJECT_GET_INSTANCE_CUSTOM_INDEX_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetInstanceCustomIndexEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5326u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14cf"]
pub const OP_HIT_OBJECT_GET_OBJECT_RAY_ORIGIN_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetObjectRayOriginEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5327u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14d0"]
pub const OP_HIT_OBJECT_GET_OBJECT_RAY_DIRECTION_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetObjectRayDirectionEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5328u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14d1"]
pub const OP_HIT_OBJECT_GET_WORLD_RAY_DIRECTION_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetWorldRayDirectionEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5329u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14d2"]
pub const OP_HIT_OBJECT_GET_WORLD_RAY_ORIGIN_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetWorldRayOriginEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5330u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14d3"]
pub const OP_HIT_OBJECT_GET_OBJECT_TO_WORLD_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetObjectToWorldEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5331u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14d4"]
pub const OP_HIT_OBJECT_GET_WORLD_TO_OBJECT_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetWorldToObjectEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5332u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14d5"]
pub const OP_HIT_OBJECT_GET_RAY_T_MAX_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetRayTMaxEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5333u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14d6"]
pub const OP_REPORT_INTERSECTION_KHR: InstMeta = InstMeta {
    opname: "OpReportIntersectionKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5334u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("HitKind"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingNV, Capability::RayTracingKHR],
    extensions: &[EXTENSION_SPV_NV_RAY_TRACING, EXTENSION_SPV_KHR_RAY_TRACING],
    version: Some("None"),
    last_version: None,
    aliases: &["OpReportIntersectionNV"],
    provisional: false,
};
#[doc = "opcode: 0x14d7"]
pub const OP_IGNORE_INTERSECTION_NV: InstMeta = InstMeta {
    opname: "OpIgnoreIntersectionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5335u16,
    operands: &[],
    capabilities: &[Capability::RayTracingNV],
    extensions: &[EXTENSION_SPV_NV_RAY_TRACING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14d8"]
pub const OP_TERMINATE_RAY_NV: InstMeta = InstMeta {
    opname: "OpTerminateRayNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5336u16,
    operands: &[],
    capabilities: &[Capability::RayTracingNV],
    extensions: &[EXTENSION_SPV_NV_RAY_TRACING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14d9"]
pub const OP_TRACE_NV: InstMeta = InstMeta {
    opname: "OpTraceNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5337u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cull Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("PayloadId"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingNV],
    extensions: &[EXTENSION_SPV_NV_RAY_TRACING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14da"]
pub const OP_TRACE_MOTION_NV: InstMeta = InstMeta {
    opname: "OpTraceMotionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5338u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cull Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Time"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("PayloadId"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingMotionBlurNV],
    extensions: &[EXTENSION_SPV_NV_RAY_TRACING_MOTION_BLUR],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14db"]
pub const OP_TRACE_RAY_MOTION_NV: InstMeta = InstMeta {
    opname: "OpTraceRayMotionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5339u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Accel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Flags"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cull Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Miss Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Origin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmin"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ray Tmax"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Time"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingMotionBlurNV],
    extensions: &[EXTENSION_SPV_NV_RAY_TRACING_MOTION_BLUR],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14dc"]
pub const OP_RAY_QUERY_GET_INTERSECTION_TRIANGLE_VERTEX_POSITIONS_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionTriangleVertexPositionsKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5340u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryPositionFetchKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14dd"]
pub const OP_TYPE_ACCELERATION_STRUCTURE_KHR: InstMeta = InstMeta {
    opname: "OpTypeAccelerationStructureKHR",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 5341u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[
        Capability::RayTracingNV,
        Capability::RayTracingKHR,
        Capability::RayQueryKHR,
        Capability::DisplacementMicromapNV,
    ],
    extensions: &[
        EXTENSION_SPV_NV_RAY_TRACING,
        EXTENSION_SPV_KHR_RAY_TRACING,
        EXTENSION_SPV_KHR_RAY_QUERY,
        EXTENSION_SPV_NV_DISPLACEMENT_MICROMAP,
    ],
    version: Some("None"),
    last_version: None,
    aliases: &["OpTypeAccelerationStructureNV"],
    provisional: false,
};
#[doc = "opcode: 0x14e0"]
pub const OP_EXECUTE_CALLABLE_NV: InstMeta = InstMeta {
    opname: "OpExecuteCallableNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5344u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SBT Index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Callable DataId"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingNV],
    extensions: &[EXTENSION_SPV_NV_RAY_TRACING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14e1"]
pub const OP_RAY_QUERY_GET_INTERSECTION_CLUSTER_ID_NV: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionClusterIdNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5345u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingClusterAccelerationStructureNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpRayQueryGetClusterIdNV"],
    provisional: false,
};
#[doc = "opcode: 0x14e2"]
pub const OP_HIT_OBJECT_GET_CLUSTER_ID_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetClusterIdNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5346u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingClusterAccelerationStructureNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14e3"]
pub const OP_HIT_OBJECT_GET_RAY_T_MIN_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetRayTMinEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5347u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14e4"]
pub const OP_HIT_OBJECT_GET_SHADER_BINDING_TABLE_RECORD_INDEX_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetShaderBindingTableRecordIndexEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5348u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14e5"]
pub const OP_HIT_OBJECT_GET_SHADER_RECORD_BUFFER_HANDLE_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectGetShaderRecordBufferHandleEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5349u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14e6"]
pub const OP_HIT_OBJECT_IS_EMPTY_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectIsEmptyEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5350u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14e7"]
pub const OP_HIT_OBJECT_IS_HIT_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectIsHitEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5351u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14e8"]
pub const OP_HIT_OBJECT_IS_MISS_EXT: InstMeta = InstMeta {
    opname: "OpHitObjectIsMissEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5352u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ShaderInvocationReorderEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14ee"]
pub const OP_TYPE_COOPERATIVE_MATRIX_NV: InstMeta = InstMeta {
    opname: "OpTypeCooperativeMatrixNV",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 5358u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Component Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Rows"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Columns"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixNV],
    extensions: &[EXTENSION_SPV_NV_COOPERATIVE_MATRIX],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14ef"]
pub const OP_COOPERATIVE_MATRIX_LOAD_NV: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixLoadNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5359u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Column Major"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixNV],
    extensions: &[EXTENSION_SPV_NV_COOPERATIVE_MATRIX],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14f0"]
pub const OP_COOPERATIVE_MATRIX_STORE_NV: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixStoreNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5360u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Column Major"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixNV],
    extensions: &[EXTENSION_SPV_NV_COOPERATIVE_MATRIX],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14f1"]
pub const OP_COOPERATIVE_MATRIX_MUL_ADD_NV: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixMulAddNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5361u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("C"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixNV],
    extensions: &[EXTENSION_SPV_NV_COOPERATIVE_MATRIX],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14f2"]
pub const OP_COOPERATIVE_MATRIX_LENGTH_NV: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixLengthNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5362u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Type"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixNV],
    extensions: &[EXTENSION_SPV_NV_COOPERATIVE_MATRIX],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14f4"]
pub const OP_BEGIN_INVOCATION_INTERLOCK_EXT: InstMeta = InstMeta {
    opname: "OpBeginInvocationInterlockEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5364u16,
    operands: &[],
    capabilities: &[
        Capability::FragmentShaderSampleInterlockEXT,
        Capability::FragmentShaderPixelInterlockEXT,
        Capability::FragmentShaderShadingRateInterlockEXT,
    ],
    extensions: &[EXTENSION_SPV_EXT_FRAGMENT_SHADER_INTERLOCK],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14f5"]
pub const OP_END_INVOCATION_INTERLOCK_EXT: InstMeta = InstMeta {
    opname: "OpEndInvocationInterlockEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5365u16,
    operands: &[],
    capabilities: &[
        Capability::FragmentShaderSampleInterlockEXT,
        Capability::FragmentShaderPixelInterlockEXT,
        Capability::FragmentShaderShadingRateInterlockEXT,
    ],
    extensions: &[EXTENSION_SPV_EXT_FRAGMENT_SHADER_INTERLOCK],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14f6"]
pub const OP_COOPERATIVE_MATRIX_REDUCE_NV: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixReduceNV",
    class: Some(&PRINTING_CLASS_ARITHMETIC),
    opcode: 5366u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_COOPERATIVE_MATRIX_REDUCE,
            name: Some("Reduce"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("CombineFunc"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixReductionsNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14f7"]
pub const OP_COOPERATIVE_MATRIX_LOAD_TENSOR_NV: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixLoadTensorNV",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 5367u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TensorLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: Some("Memory Operand"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_TENSOR_ADDRESSING_OPERANDS,
            name: Some("Tensor Addressing Operands"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixTensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14f8"]
pub const OP_COOPERATIVE_MATRIX_STORE_TENSOR_NV: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixStoreTensorNV",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 5368u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Object"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TensorLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: Some("Memory Operand"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_TENSOR_ADDRESSING_OPERANDS,
            name: Some("Tensor Addressing Operands"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixTensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14f9"]
pub const OP_COOPERATIVE_MATRIX_PER_ELEMENT_OP_NV: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixPerElementOpNV",
    class: Some(&PRINTING_CLASS_FUNCTION),
    opcode: 5369u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Func"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operands"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixPerElementOperationsNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14fa"]
pub const OP_TYPE_TENSOR_LAYOUT_NV: InstMeta = InstMeta {
    opname: "OpTypeTensorLayoutNV",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 5370u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Dim"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClampMode"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14fb"]
pub const OP_TYPE_TENSOR_VIEW_NV: InstMeta = InstMeta {
    opname: "OpTypeTensorViewNV",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 5371u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Dim"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("HasDimensions"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("p"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14fc"]
pub const OP_CREATE_TENSOR_LAYOUT_NV: InstMeta = InstMeta {
    opname: "OpCreateTensorLayoutNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5372u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14fd"]
pub const OP_TENSOR_LAYOUT_SET_DIMENSION_NV: InstMeta = InstMeta {
    opname: "OpTensorLayoutSetDimensionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5373u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TensorLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Dim"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14fe"]
pub const OP_TENSOR_LAYOUT_SET_STRIDE_NV: InstMeta = InstMeta {
    opname: "OpTensorLayoutSetStrideNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5374u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TensorLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Stride"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x14ff"]
pub const OP_TENSOR_LAYOUT_SLICE_NV: InstMeta = InstMeta {
    opname: "OpTensorLayoutSliceNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5375u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TensorLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operands"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1500"]
pub const OP_TENSOR_LAYOUT_SET_CLAMP_VALUE_NV: InstMeta = InstMeta {
    opname: "OpTensorLayoutSetClampValueNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5376u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TensorLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1501"]
pub const OP_CREATE_TENSOR_VIEW_NV: InstMeta = InstMeta {
    opname: "OpCreateTensorViewNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5377u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1502"]
pub const OP_TENSOR_VIEW_SET_DIMENSION_NV: InstMeta = InstMeta {
    opname: "OpTensorViewSetDimensionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5378u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TensorView"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Dim"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1503"]
pub const OP_TENSOR_VIEW_SET_STRIDE_NV: InstMeta = InstMeta {
    opname: "OpTensorViewSetStrideNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5379u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TensorView"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Stride"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1504"]
pub const OP_DEMOTE_TO_HELPER_INVOCATION: InstMeta = InstMeta {
    opname: "OpDemoteToHelperInvocation",
    class: Some(&PRINTING_CLASS_CONTROL_FLOW),
    opcode: 5380u16,
    operands: &[],
    capabilities: &[Capability::DemoteToHelperInvocation],
    extensions: &[],
    version: Some("1.6"),
    last_version: None,
    aliases: &["OpDemoteToHelperInvocationEXT"],
    provisional: false,
};
#[doc = "opcode: 0x1505"]
pub const OP_IS_HELPER_INVOCATION_EXT: InstMeta = InstMeta {
    opname: "OpIsHelperInvocationEXT",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5381u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::DemoteToHelperInvocation],
    extensions: &[EXTENSION_SPV_EXT_DEMOTE_TO_HELPER_INVOCATION],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1506"]
pub const OP_TENSOR_VIEW_SET_CLIP_NV: InstMeta = InstMeta {
    opname: "OpTensorViewSetClipNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5382u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TensorView"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClipRowOffset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClipRowSpan"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClipColOffset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ClipColSpan"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1508"]
pub const OP_TENSOR_LAYOUT_SET_BLOCK_SIZE_NV: InstMeta = InstMeta {
    opname: "OpTensorLayoutSetBlockSizeNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5384u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("TensorLayout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("BlockSize"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::TensorAddressingNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x150e"]
pub const OP_COOPERATIVE_MATRIX_TRANSPOSE_NV: InstMeta = InstMeta {
    opname: "OpCooperativeMatrixTransposeNV",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 5390u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::CooperativeMatrixConversionsNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x150f"]
pub const OP_CONVERT_U_TO_IMAGE_NV: InstMeta = InstMeta {
    opname: "OpConvertUToImageNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5391u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BindlessTextureNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1510"]
pub const OP_CONVERT_U_TO_SAMPLER_NV: InstMeta = InstMeta {
    opname: "OpConvertUToSamplerNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5392u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BindlessTextureNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1511"]
pub const OP_CONVERT_IMAGE_TO_UNV: InstMeta = InstMeta {
    opname: "OpConvertImageToUNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5393u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BindlessTextureNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1512"]
pub const OP_CONVERT_SAMPLER_TO_UNV: InstMeta = InstMeta {
    opname: "OpConvertSamplerToUNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5394u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BindlessTextureNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1513"]
pub const OP_CONVERT_U_TO_SAMPLED_IMAGE_NV: InstMeta = InstMeta {
    opname: "OpConvertUToSampledImageNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5395u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BindlessTextureNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1514"]
pub const OP_CONVERT_SAMPLED_IMAGE_TO_UNV: InstMeta = InstMeta {
    opname: "OpConvertSampledImageToUNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5396u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BindlessTextureNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1515"]
pub const OP_SAMPLER_IMAGE_ADDRESSING_MODE_NV: InstMeta = InstMeta {
    opname: "OpSamplerImageAddressingModeNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5397u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_LITERAL_INTEGER,
        name: Some("Bit Width"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::BindlessTextureNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1516"]
pub const OP_RAW_ACCESS_CHAIN_NV: InstMeta = InstMeta {
    opname: "OpRawAccessChainNV",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 5398u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Base"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Byte stride"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element index"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Byte offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_RAW_ACCESS_CHAIN_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::RawAccessChainsNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1533"]
pub const OP_RAY_QUERY_GET_INTERSECTION_SPHERE_POSITION_NV: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionSpherePositionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5427u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1534"]
pub const OP_RAY_QUERY_GET_INTERSECTION_SPHERE_RADIUS_NV: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionSphereRadiusNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5428u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1535"]
pub const OP_RAY_QUERY_GET_INTERSECTION_LSS_POSITIONS_NV: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionLSSPositionsNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5429u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingLinearSweptSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1536"]
pub const OP_RAY_QUERY_GET_INTERSECTION_LSS_RADII_NV: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionLSSRadiiNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5430u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingLinearSweptSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1537"]
pub const OP_RAY_QUERY_GET_INTERSECTION_LSS_HIT_VALUE_NV: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionLSSHitValueNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5431u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingLinearSweptSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1538"]
pub const OP_HIT_OBJECT_GET_SPHERE_POSITION_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetSpherePositionNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5432u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1539"]
pub const OP_HIT_OBJECT_GET_SPHERE_RADIUS_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetSphereRadiusNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5433u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x153a"]
pub const OP_HIT_OBJECT_GET_LSS_POSITIONS_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetLSSPositionsNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5434u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingLinearSweptSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x153b"]
pub const OP_HIT_OBJECT_GET_LSS_RADII_NV: InstMeta = InstMeta {
    opname: "OpHitObjectGetLSSRadiiNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5435u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingLinearSweptSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x153c"]
pub const OP_HIT_OBJECT_IS_SPHERE_HIT_NV: InstMeta = InstMeta {
    opname: "OpHitObjectIsSphereHitNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5436u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x153d"]
pub const OP_HIT_OBJECT_IS_LSS_HIT_NV: InstMeta = InstMeta {
    opname: "OpHitObjectIsLSSHitNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5437u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Hit Object"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingLinearSweptSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x153e"]
pub const OP_RAY_QUERY_IS_SPHERE_HIT_NV: InstMeta = InstMeta {
    opname: "OpRayQueryIsSphereHitNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5438u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x153f"]
pub const OP_RAY_QUERY_IS_LSS_HIT_NV: InstMeta = InstMeta {
    opname: "OpRayQueryIsLSSHitNV",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5439u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayTracingLinearSweptSpheresGeometryNV],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15c3"]
pub const OP_SUBGROUP_SHUFFLE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupShuffleINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5571u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Data"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("InvocationId"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupShuffleINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15c4"]
pub const OP_SUBGROUP_SHUFFLE_DOWN_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupShuffleDownINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5572u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Current"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Next"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Delta"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupShuffleINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15c5"]
pub const OP_SUBGROUP_SHUFFLE_UP_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupShuffleUpINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5573u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Previous"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Current"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Delta"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupShuffleINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15c6"]
pub const OP_SUBGROUP_SHUFFLE_XOR_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupShuffleXorINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5574u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Data"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupShuffleINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15c7"]
pub const OP_SUBGROUP_BLOCK_READ_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupBlockReadINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5575u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ptr"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupBufferBlockIOINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15c8"]
pub const OP_SUBGROUP_BLOCK_WRITE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupBlockWriteINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5576u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ptr"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Data"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupBufferBlockIOINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15c9"]
pub const OP_SUBGROUP_IMAGE_BLOCK_READ_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupImageBlockReadINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5577u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupImageBlockIOINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15ca"]
pub const OP_SUBGROUP_IMAGE_BLOCK_WRITE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupImageBlockWriteINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5578u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Data"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupImageBlockIOINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15cc"]
pub const OP_SUBGROUP_IMAGE_MEDIA_BLOCK_READ_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupImageMediaBlockReadINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5580u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Height"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupImageMediaBlockIOINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15cd"]
pub const OP_SUBGROUP_IMAGE_MEDIA_BLOCK_WRITE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupImageMediaBlockWriteINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 5581u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Data"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupImageMediaBlockIOINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15d1"]
pub const OP_U_COUNT_LEADING_ZEROS_INTEL: InstMeta = InstMeta {
    opname: "OpUCountLeadingZerosINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5585u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15d2"]
pub const OP_U_COUNT_TRAILING_ZEROS_INTEL: InstMeta = InstMeta {
    opname: "OpUCountTrailingZerosINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5586u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15d3"]
pub const OP_ABS_I_SUB_INTEL: InstMeta = InstMeta {
    opname: "OpAbsISubINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5587u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15d4"]
pub const OP_ABS_U_SUB_INTEL: InstMeta = InstMeta {
    opname: "OpAbsUSubINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5588u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15d5"]
pub const OP_I_ADD_SAT_INTEL: InstMeta = InstMeta {
    opname: "OpIAddSatINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5589u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15d6"]
pub const OP_U_ADD_SAT_INTEL: InstMeta = InstMeta {
    opname: "OpUAddSatINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5590u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15d7"]
pub const OP_I_AVERAGE_INTEL: InstMeta = InstMeta {
    opname: "OpIAverageINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5591u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15d8"]
pub const OP_U_AVERAGE_INTEL: InstMeta = InstMeta {
    opname: "OpUAverageINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5592u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15d9"]
pub const OP_I_AVERAGE_ROUNDED_INTEL: InstMeta = InstMeta {
    opname: "OpIAverageRoundedINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5593u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15da"]
pub const OP_U_AVERAGE_ROUNDED_INTEL: InstMeta = InstMeta {
    opname: "OpUAverageRoundedINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5594u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15db"]
pub const OP_I_SUB_SAT_INTEL: InstMeta = InstMeta {
    opname: "OpISubSatINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5595u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15dc"]
pub const OP_U_SUB_SAT_INTEL: InstMeta = InstMeta {
    opname: "OpUSubSatINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5596u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15dd"]
pub const OP_I_MUL_32_X_16_INTEL: InstMeta = InstMeta {
    opname: "OpIMul32x16INTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5597u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15de"]
pub const OP_U_MUL_32_X_16_INTEL: InstMeta = InstMeta {
    opname: "OpUMul32x16INTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5598u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 2"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::IntegerFunctions2INTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15e0"]
pub const OP_CONSTANT_FUNCTION_POINTER_INTEL: InstMeta = InstMeta {
    opname: "OpConstantFunctionPointerINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5600u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Function"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::FunctionPointersINTEL],
    extensions: &[EXTENSION_SPV_INTEL_FUNCTION_POINTERS],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15e1"]
pub const OP_FUNCTION_POINTER_CALL_INTEL: InstMeta = InstMeta {
    opname: "OpFunctionPointerCallINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5601u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand 1"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::FunctionPointersINTEL],
    extensions: &[EXTENSION_SPV_INTEL_FUNCTION_POINTERS],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15e9"]
pub const OP_ASM_TARGET_INTEL: InstMeta = InstMeta {
    opname: "OpAsmTargetINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5609u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Asm target"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::AsmINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15ea"]
pub const OP_ASM_INTEL: InstMeta = InstMeta {
    opname: "OpAsmINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5610u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Asm type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Asm instructions"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Constraints"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::AsmINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15eb"]
pub const OP_ASM_CALL_INTEL: InstMeta = InstMeta {
    opname: "OpAsmCallINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5611u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Asm"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Argument"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::AsmINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15ee"]
pub const OP_ATOMIC_F_MIN_EXT: InstMeta = InstMeta {
    opname: "OpAtomicFMinEXT",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 5614u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::AtomicFloat16MinMaxEXT,
        Capability::AtomicFloat32MinMaxEXT,
        Capability::AtomicFloat64MinMaxEXT,
        Capability::AtomicFloat16VectorNV,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15ef"]
pub const OP_ATOMIC_F_MAX_EXT: InstMeta = InstMeta {
    opname: "OpAtomicFMaxEXT",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 5615u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::AtomicFloat16MinMaxEXT,
        Capability::AtomicFloat32MinMaxEXT,
        Capability::AtomicFloat64MinMaxEXT,
        Capability::AtomicFloat16VectorNV,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15fe"]
pub const OP_ASSUME_TRUE_KHR: InstMeta = InstMeta {
    opname: "OpAssumeTrueKHR",
    class: Some(&PRINTING_CLASS_MISCELLANEOUS),
    opcode: 5630u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Condition"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::ExpectAssumeKHR],
    extensions: &[EXTENSION_SPV_KHR_EXPECT_ASSUME],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x15ff"]
pub const OP_EXPECT_KHR: InstMeta = InstMeta {
    opname: "OpExpectKHR",
    class: Some(&PRINTING_CLASS_MISCELLANEOUS),
    opcode: 5631u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("ExpectedValue"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ExpectAssumeKHR],
    extensions: &[EXTENSION_SPV_KHR_EXPECT_ASSUME],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1600"]
pub const OP_DECORATE_STRING: InstMeta = InstMeta {
    opname: "OpDecorateString",
    class: Some(&PRINTING_CLASS_ANNOTATION),
    opcode: 5632u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_DECORATION,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[
        EXTENSION_SPV_GOOGLE_DECORATE_STRING,
        EXTENSION_SPV_GOOGLE_HLSL_FUNCTIONALITY_1,
    ],
    version: Some("1.4"),
    last_version: None,
    aliases: &["OpDecorateStringGOOGLE"],
    provisional: false,
};
#[doc = "opcode: 0x1601"]
pub const OP_MEMBER_DECORATE_STRING: InstMeta = InstMeta {
    opname: "OpMemberDecorateString",
    class: Some(&PRINTING_CLASS_ANNOTATION),
    opcode: 5633u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Struct Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Member"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_DECORATION,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[
        EXTENSION_SPV_GOOGLE_DECORATE_STRING,
        EXTENSION_SPV_GOOGLE_HLSL_FUNCTIONALITY_1,
    ],
    version: Some("1.4"),
    last_version: None,
    aliases: &["OpMemberDecorateStringGOOGLE"],
    provisional: false,
};
#[doc = "opcode: 0x1643"]
pub const OP_VME_IMAGE_INTEL: InstMeta = InstMeta {
    opname: "OpVmeImageINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5699u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sampler"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1644"]
pub const OP_TYPE_VME_IMAGE_INTEL: InstMeta = InstMeta {
    opname: "OpTypeVmeImageINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5700u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image Type"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1645"]
pub const OP_TYPE_AVC_IME_PAYLOAD_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcImePayloadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5701u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1646"]
pub const OP_TYPE_AVC_REF_PAYLOAD_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcRefPayloadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5702u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1647"]
pub const OP_TYPE_AVC_SIC_PAYLOAD_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcSicPayloadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5703u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1648"]
pub const OP_TYPE_AVC_MCE_PAYLOAD_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcMcePayloadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5704u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1649"]
pub const OP_TYPE_AVC_MCE_RESULT_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcMceResultINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5705u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x164a"]
pub const OP_TYPE_AVC_IME_RESULT_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcImeResultINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5706u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x164b"]
pub const OP_TYPE_AVC_IME_RESULT_SINGLE_REFERENCE_STREAMOUT_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcImeResultSingleReferenceStreamoutINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5707u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x164c"]
pub const OP_TYPE_AVC_IME_RESULT_DUAL_REFERENCE_STREAMOUT_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcImeResultDualReferenceStreamoutINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5708u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x164d"]
pub const OP_TYPE_AVC_IME_SINGLE_REFERENCE_STREAMIN_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcImeSingleReferenceStreaminINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5709u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x164e"]
pub const OP_TYPE_AVC_IME_DUAL_REFERENCE_STREAMIN_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcImeDualReferenceStreaminINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5710u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x164f"]
pub const OP_TYPE_AVC_REF_RESULT_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcRefResultINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5711u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1650"]
pub const OP_TYPE_AVC_SIC_RESULT_INTEL: InstMeta = InstMeta {
    opname: "OpTypeAvcSicResultINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5712u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1651"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_INTER_BASE_MULTI_REFERENCE_PENALTY_INTEL: InstMeta =
    InstMeta {
        opname: "OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL",
        class: Some(&PRINTING_CLASS_EXCLUDE),
        opcode: 5713u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Slice Type"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Qp"),
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
        extensions: &[],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x1652"]
pub const OP_SUBGROUP_AVC_MCE_SET_INTER_BASE_MULTI_REFERENCE_PENALTY_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5714u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Reference Base Penalty"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1653"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_INTER_SHAPE_PENALTY_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5715u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Slice Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Qp"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1654"]
pub const OP_SUBGROUP_AVC_MCE_SET_INTER_SHAPE_PENALTY_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceSetInterShapePenaltyINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5716u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Shape Penalty"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1655"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_INTER_DIRECTION_PENALTY_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5717u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Slice Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Qp"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1656"]
pub const OP_SUBGROUP_AVC_MCE_SET_INTER_DIRECTION_PENALTY_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceSetInterDirectionPenaltyINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5718u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction Cost"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1657"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_INTRA_LUMA_SHAPE_PENALTY_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5719u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Slice Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Qp"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1658"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_INTER_MOTION_VECTOR_COST_TABLE_INTEL: InstMeta =
    InstMeta {
        opname: "OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL",
        class: Some(&PRINTING_CLASS_EXCLUDE),
        opcode: 5720u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Slice Type"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Qp"),
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
        extensions: &[],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x1659"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_HIGH_PENALTY_COST_TABLE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5721u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x165a"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_MEDIUM_PENALTY_COST_TABLE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5722u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x165b"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_LOW_PENALTY_COST_TABLE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5723u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x165c"]
pub const OP_SUBGROUP_AVC_MCE_SET_MOTION_VECTOR_COST_FUNCTION_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5724u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Cost Center Delta"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Cost Table"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Cost Precision"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x165d"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_INTRA_LUMA_MODE_PENALTY_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5725u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Slice Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Qp"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x165e"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_NON_DC_LUMA_INTRA_PENALTY_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5726u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x165f"]
pub const OP_SUBGROUP_AVC_MCE_GET_DEFAULT_INTRA_CHROMA_MODE_BASE_PENALTY_INTEL: InstMeta =
    InstMeta {
        opname: "OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL",
        class: Some(&PRINTING_CLASS_EXCLUDE),
        opcode: 5727u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[
            Capability::SubgroupAvcMotionEstimationINTEL,
            Capability::SubgroupAvcMotionEstimationChromaINTEL,
        ],
        extensions: &[],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x1660"]
pub const OP_SUBGROUP_AVC_MCE_SET_AC_ONLY_HAAR_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceSetAcOnlyHaarINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5728u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1661"]
pub const OP_SUBGROUP_AVC_MCE_SET_SOURCE_INTERLACED_FIELD_POLARITY_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5729u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Source Field Polarity"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1662"]
pub const OP_SUBGROUP_AVC_MCE_SET_SINGLE_REFERENCE_INTERLACED_FIELD_POLARITY_INTEL: InstMeta =
    InstMeta {
        opname: "OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL",
        class: Some(&PRINTING_CLASS_EXCLUDE),
        opcode: 5730u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Reference Field Polarity"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Payload"),
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
        extensions: &[],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x1663"]
pub const OP_SUBGROUP_AVC_MCE_SET_DUAL_REFERENCE_INTERLACED_FIELD_POLARITIES_INTEL: InstMeta =
    InstMeta {
        opname: "OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL",
        class: Some(&PRINTING_CLASS_EXCLUDE),
        opcode: 5731u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Forward Reference Field Polarity"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Backward Reference Field Polarity"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Payload"),
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
        extensions: &[],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x1664"]
pub const OP_SUBGROUP_AVC_MCE_CONVERT_TO_IME_PAYLOAD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceConvertToImePayloadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5732u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1665"]
pub const OP_SUBGROUP_AVC_MCE_CONVERT_TO_IME_RESULT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceConvertToImeResultINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5733u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1666"]
pub const OP_SUBGROUP_AVC_MCE_CONVERT_TO_REF_PAYLOAD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceConvertToRefPayloadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5734u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1667"]
pub const OP_SUBGROUP_AVC_MCE_CONVERT_TO_REF_RESULT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceConvertToRefResultINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5735u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1668"]
pub const OP_SUBGROUP_AVC_MCE_CONVERT_TO_SIC_PAYLOAD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceConvertToSicPayloadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5736u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1669"]
pub const OP_SUBGROUP_AVC_MCE_CONVERT_TO_SIC_RESULT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceConvertToSicResultINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5737u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x166a"]
pub const OP_SUBGROUP_AVC_MCE_GET_MOTION_VECTORS_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetMotionVectorsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5738u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x166b"]
pub const OP_SUBGROUP_AVC_MCE_GET_INTER_DISTORTIONS_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetInterDistortionsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5739u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x166c"]
pub const OP_SUBGROUP_AVC_MCE_GET_BEST_INTER_DISTORTIONS_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetBestInterDistortionsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5740u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x166d"]
pub const OP_SUBGROUP_AVC_MCE_GET_INTER_MAJOR_SHAPE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetInterMajorShapeINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5741u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x166e"]
pub const OP_SUBGROUP_AVC_MCE_GET_INTER_MINOR_SHAPE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetInterMinorShapeINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5742u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x166f"]
pub const OP_SUBGROUP_AVC_MCE_GET_INTER_DIRECTIONS_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetInterDirectionsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5743u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1670"]
pub const OP_SUBGROUP_AVC_MCE_GET_INTER_MOTION_VECTOR_COUNT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetInterMotionVectorCountINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5744u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1671"]
pub const OP_SUBGROUP_AVC_MCE_GET_INTER_REFERENCE_IDS_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcMceGetInterReferenceIdsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5745u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1672"]
pub const OP_SUBGROUP_AVC_MCE_GET_INTER_REFERENCE_INTERLACED_FIELD_POLARITIES_INTEL: InstMeta =
    InstMeta {
        opname: "OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL",
        class: Some(&PRINTING_CLASS_EXCLUDE),
        opcode: 5746u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Packed Reference Ids"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Packed Reference Parameter Field Polarities"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Payload"),
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
        extensions: &[],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x1673"]
pub const OP_SUBGROUP_AVC_IME_INITIALIZE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeInitializeINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5747u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Coord"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Partition Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("SAD Adjustment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1674"]
pub const OP_SUBGROUP_AVC_IME_SET_SINGLE_REFERENCE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeSetSingleReferenceINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5748u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ref Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Search Window Config"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1675"]
pub const OP_SUBGROUP_AVC_IME_SET_DUAL_REFERENCE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeSetDualReferenceINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5749u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Fwd Ref Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bwd Ref Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Search Window Config"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1676"]
pub const OP_SUBGROUP_AVC_IME_REF_WINDOW_SIZE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeRefWindowSizeINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5750u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Search Window Config"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Dual Ref"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1677"]
pub const OP_SUBGROUP_AVC_IME_ADJUST_REF_OFFSET_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeAdjustRefOffsetINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5751u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ref Offset"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Coord"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ref Window Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image Size"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1678"]
pub const OP_SUBGROUP_AVC_IME_CONVERT_TO_MCE_PAYLOAD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeConvertToMcePayloadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5752u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1679"]
pub const OP_SUBGROUP_AVC_IME_SET_MAX_MOTION_VECTOR_COUNT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeSetMaxMotionVectorCountINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5753u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Max Motion Vector Count"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x167a"]
pub const OP_SUBGROUP_AVC_IME_SET_UNIDIRECTIONAL_MIX_DISABLE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5754u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x167b"]
pub const OP_SUBGROUP_AVC_IME_SET_EARLY_SEARCH_TERMINATION_THRESHOLD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5755u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Threshold"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x167c"]
pub const OP_SUBGROUP_AVC_IME_SET_WEIGHTED_SAD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeSetWeightedSadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5756u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Sad Weights"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x167d"]
pub const OP_SUBGROUP_AVC_IME_EVALUATE_WITH_SINGLE_REFERENCE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5757u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x167e"]
pub const OP_SUBGROUP_AVC_IME_EVALUATE_WITH_DUAL_REFERENCE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeEvaluateWithDualReferenceINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5758u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Fwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x167f"]
pub const OP_SUBGROUP_AVC_IME_EVALUATE_WITH_SINGLE_REFERENCE_STREAMIN_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5759u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Streamin Components"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1680"]
pub const OP_SUBGROUP_AVC_IME_EVALUATE_WITH_DUAL_REFERENCE_STREAMIN_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5760u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Fwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Streamin Components"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1681"]
pub const OP_SUBGROUP_AVC_IME_EVALUATE_WITH_SINGLE_REFERENCE_STREAMOUT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5761u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1682"]
pub const OP_SUBGROUP_AVC_IME_EVALUATE_WITH_DUAL_REFERENCE_STREAMOUT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5762u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Fwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1683"]
pub const OP_SUBGROUP_AVC_IME_EVALUATE_WITH_SINGLE_REFERENCE_STREAMINOUT_INTEL: InstMeta =
    InstMeta {
        opname: "OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL",
        class: Some(&PRINTING_CLASS_EXCLUDE),
        opcode: 5763u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Src Image"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Ref Image"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Payload"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Streamin Components"),
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
        extensions: &[],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x1684"]
pub const OP_SUBGROUP_AVC_IME_EVALUATE_WITH_DUAL_REFERENCE_STREAMINOUT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5764u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Fwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Streamin Components"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1685"]
pub const OP_SUBGROUP_AVC_IME_CONVERT_TO_MCE_RESULT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeConvertToMceResultINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5765u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1686"]
pub const OP_SUBGROUP_AVC_IME_GET_SINGLE_REFERENCE_STREAMIN_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeGetSingleReferenceStreaminINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5766u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1687"]
pub const OP_SUBGROUP_AVC_IME_GET_DUAL_REFERENCE_STREAMIN_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeGetDualReferenceStreaminINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5767u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1688"]
pub const OP_SUBGROUP_AVC_IME_STRIP_SINGLE_REFERENCE_STREAMOUT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5768u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1689"]
pub const OP_SUBGROUP_AVC_IME_STRIP_DUAL_REFERENCE_STREAMOUT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeStripDualReferenceStreamoutINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5769u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x168a"]
pub const OP_SUBGROUP_AVC_IME_GET_STREAMOUT_SINGLE_REFERENCE_MAJOR_SHAPE_MOTION_VECTORS_INTEL:
    InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5770u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Major Shape"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x168b"]
pub const OP_SUBGROUP_AVC_IME_GET_STREAMOUT_SINGLE_REFERENCE_MAJOR_SHAPE_DISTORTIONS_INTEL:
    InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5771u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Major Shape"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x168c"]
pub const OP_SUBGROUP_AVC_IME_GET_STREAMOUT_SINGLE_REFERENCE_MAJOR_SHAPE_REFERENCE_IDS_INTEL:
    InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5772u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Major Shape"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x168d"]
pub const OP_SUBGROUP_AVC_IME_GET_STREAMOUT_DUAL_REFERENCE_MAJOR_SHAPE_MOTION_VECTORS_INTEL:
    InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5773u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Major Shape"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x168e"]
pub const OP_SUBGROUP_AVC_IME_GET_STREAMOUT_DUAL_REFERENCE_MAJOR_SHAPE_DISTORTIONS_INTEL: InstMeta =
    InstMeta {
        opname: "OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL",
        class: Some(&PRINTING_CLASS_EXCLUDE),
        opcode: 5774u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Payload"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Major Shape"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Direction"),
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
        extensions: &[],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x168f"]
pub const OP_SUBGROUP_AVC_IME_GET_STREAMOUT_DUAL_REFERENCE_MAJOR_SHAPE_REFERENCE_IDS_INTEL:
    InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5775u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Major Shape"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1690"]
pub const OP_SUBGROUP_AVC_IME_GET_BORDER_REACHED_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeGetBorderReachedINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5776u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Image Select"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1691"]
pub const OP_SUBGROUP_AVC_IME_GET_TRUNCATED_SEARCH_INDICATION_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5777u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1692"]
pub const OP_SUBGROUP_AVC_IME_GET_UNIDIRECTIONAL_EARLY_SEARCH_TERMINATION_INTEL: InstMeta =
    InstMeta {
        opname: "OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL",
        class: Some(&PRINTING_CLASS_EXCLUDE),
        opcode: 5778u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Payload"),
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
        extensions: &[],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x1693"]
pub const OP_SUBGROUP_AVC_IME_GET_WEIGHTING_PATTERN_MINIMUM_MOTION_VECTOR_INTEL: InstMeta =
    InstMeta {
        opname: "OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL",
        class: Some(&PRINTING_CLASS_EXCLUDE),
        opcode: 5779u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Payload"),
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
        extensions: &[],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x1694"]
pub const OP_SUBGROUP_AVC_IME_GET_WEIGHTING_PATTERN_MINIMUM_DISTORTION_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5780u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1695"]
pub const OP_SUBGROUP_AVC_FME_INITIALIZE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcFmeInitializeINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5781u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Coord"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Motion Vectors"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Major Shapes"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Minor Shapes"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pixel Resolution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sad Adjustment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1696"]
pub const OP_SUBGROUP_AVC_BME_INITIALIZE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcBmeInitializeINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5782u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Coord"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Motion Vectors"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Major Shapes"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Minor Shapes"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pixel Resolution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bidirectional Weight"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sad Adjustment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1697"]
pub const OP_SUBGROUP_AVC_REF_CONVERT_TO_MCE_PAYLOAD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcRefConvertToMcePayloadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5783u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1698"]
pub const OP_SUBGROUP_AVC_REF_SET_BIDIRECTIONAL_MIX_DISABLE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcRefSetBidirectionalMixDisableINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5784u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1699"]
pub const OP_SUBGROUP_AVC_REF_SET_BILINEAR_FILTER_ENABLE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcRefSetBilinearFilterEnableINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5785u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x169a"]
pub const OP_SUBGROUP_AVC_REF_EVALUATE_WITH_SINGLE_REFERENCE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5786u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x169b"]
pub const OP_SUBGROUP_AVC_REF_EVALUATE_WITH_DUAL_REFERENCE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcRefEvaluateWithDualReferenceINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5787u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Fwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x169c"]
pub const OP_SUBGROUP_AVC_REF_EVALUATE_WITH_MULTI_REFERENCE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5788u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Reference Ids"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x169d"]
pub const OP_SUBGROUP_AVC_REF_EVALUATE_WITH_MULTI_REFERENCE_INTERLACED_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5789u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Reference Ids"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Reference Field Polarities"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x169e"]
pub const OP_SUBGROUP_AVC_REF_CONVERT_TO_MCE_RESULT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcRefConvertToMceResultINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5790u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x169f"]
pub const OP_SUBGROUP_AVC_SIC_INITIALIZE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicInitializeINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5791u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Coord"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a0"]
pub const OP_SUBGROUP_AVC_SIC_CONFIGURE_SKC_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicConfigureSkcINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5792u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Skip Block Partition Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Skip Motion Vector Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Motion Vectors"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bidirectional Weight"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sad Adjustment"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a1"]
pub const OP_SUBGROUP_AVC_SIC_CONFIGURE_IPE_LUMA_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicConfigureIpeLumaINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5793u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Luma Intra Partition Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intra Neighbour Availabilty"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Left Edge Luma Pixels"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Upper Left Corner Luma Pixel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Upper Edge Luma Pixels"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Upper Right Edge Luma Pixels"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sad Adjustment"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a2"]
pub const OP_SUBGROUP_AVC_SIC_CONFIGURE_IPE_LUMA_CHROMA_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicConfigureIpeLumaChromaINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5794u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Luma Intra Partition Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intra Neighbour Availabilty"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Left Edge Luma Pixels"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Upper Left Corner Luma Pixel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Upper Edge Luma Pixels"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Upper Right Edge Luma Pixels"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Left Edge Chroma Pixels"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Upper Left Corner Chroma Pixel"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Upper Edge Chroma Pixels"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sad Adjustment"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationChromaINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a3"]
pub const OP_SUBGROUP_AVC_SIC_GET_MOTION_VECTOR_MASK_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicGetMotionVectorMaskINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5795u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Skip Block Partition Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Direction"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a4"]
pub const OP_SUBGROUP_AVC_SIC_CONVERT_TO_MCE_PAYLOAD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicConvertToMcePayloadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5796u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a5"]
pub const OP_SUBGROUP_AVC_SIC_SET_INTRA_LUMA_SHAPE_PENALTY_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5797u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Shape Penalty"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a6"]
pub const OP_SUBGROUP_AVC_SIC_SET_INTRA_LUMA_MODE_COST_FUNCTION_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5798u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Luma Mode Penalty"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Luma Packed Neighbor Modes"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Luma Packed Non Dc Penalty"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a7"]
pub const OP_SUBGROUP_AVC_SIC_SET_INTRA_CHROMA_MODE_COST_FUNCTION_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5799u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Chroma Mode Base Penalty"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationChromaINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a8"]
pub const OP_SUBGROUP_AVC_SIC_SET_BILINEAR_FILTER_ENABLE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicSetBilinearFilterEnableINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5800u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16a9"]
pub const OP_SUBGROUP_AVC_SIC_SET_SKC_FORWARD_TRANSFORM_ENABLE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5801u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Sad Coefficients"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16aa"]
pub const OP_SUBGROUP_AVC_SIC_SET_BLOCK_BASED_RAW_SKIP_SAD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5802u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Based Skip Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ab"]
pub const OP_SUBGROUP_AVC_SIC_EVALUATE_IPE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicEvaluateIpeINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5803u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ac"]
pub const OP_SUBGROUP_AVC_SIC_EVALUATE_WITH_SINGLE_REFERENCE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5804u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ad"]
pub const OP_SUBGROUP_AVC_SIC_EVALUATE_WITH_DUAL_REFERENCE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicEvaluateWithDualReferenceINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5805u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Fwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Bwd Ref Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ae"]
pub const OP_SUBGROUP_AVC_SIC_EVALUATE_WITH_MULTI_REFERENCE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5806u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Reference Ids"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16af"]
pub const OP_SUBGROUP_AVC_SIC_EVALUATE_WITH_MULTI_REFERENCE_INTERLACED_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5807u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Image"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Reference Ids"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packed Reference Field Polarities"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16b0"]
pub const OP_SUBGROUP_AVC_SIC_CONVERT_TO_MCE_RESULT_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicConvertToMceResultINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5808u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16b1"]
pub const OP_SUBGROUP_AVC_SIC_GET_IPE_LUMA_SHAPE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicGetIpeLumaShapeINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5809u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16b2"]
pub const OP_SUBGROUP_AVC_SIC_GET_BEST_IPE_LUMA_DISTORTION_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5810u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16b3"]
pub const OP_SUBGROUP_AVC_SIC_GET_BEST_IPE_CHROMA_DISTORTION_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5811u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16b4"]
pub const OP_SUBGROUP_AVC_SIC_GET_PACKED_IPE_LUMA_MODES_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicGetPackedIpeLumaModesINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5812u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16b5"]
pub const OP_SUBGROUP_AVC_SIC_GET_IPE_CHROMA_MODE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicGetIpeChromaModeINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5813u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationChromaINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16b6"]
pub const OP_SUBGROUP_AVC_SIC_GET_PACKED_SKC_LUMA_COUNT_THRESHOLD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5814u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16b7"]
pub const OP_SUBGROUP_AVC_SIC_GET_PACKED_SKC_LUMA_SUM_THRESHOLD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5815u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::SubgroupAvcMotionEstimationINTEL,
        Capability::SubgroupAvcMotionEstimationIntraINTEL,
    ],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16b8"]
pub const OP_SUBGROUP_AVC_SIC_GET_INTER_RAW_SADS_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupAvcSicGetInterRawSadsINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5816u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Payload"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SubgroupAvcMotionEstimationINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ba"]
pub const OP_VARIABLE_LENGTH_ARRAY_INTEL: InstMeta = InstMeta {
    opname: "OpVariableLengthArrayINTEL",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 5818u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Length"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::VariableLengthArrayINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16bb"]
pub const OP_SAVE_MEMORY_INTEL: InstMeta = InstMeta {
    opname: "OpSaveMemoryINTEL",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 5819u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::VariableLengthArrayINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16bc"]
pub const OP_RESTORE_MEMORY_INTEL: InstMeta = InstMeta {
    opname: "OpRestoreMemoryINTEL",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 5820u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Ptr"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::VariableLengthArrayINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16d0"]
pub const OP_ARBITRARY_FLOAT_SIN_COS_PI_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatSinCosPiALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5840u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("MResult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("RoundingAccuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatSinCosPiINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16d1"]
pub const OP_ARBITRARY_FLOAT_CAST_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatCastALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5841u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatCastINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16d2"]
pub const OP_ARBITRARY_FLOAT_CAST_FROM_INT_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatCastFromIntALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5842u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("FromSign"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatCastFromIntINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16d3"]
pub const OP_ARBITRARY_FLOAT_CAST_TO_INT_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatCastToIntALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5843u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("ToSign"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatCastToIntINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16d6"]
pub const OP_ARBITRARY_FLOAT_ADD_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatAddALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5846u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("MResult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatAddINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16d7"]
pub const OP_ARBITRARY_FLOAT_SUB_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatSubALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5847u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatSubINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16d8"]
pub const OP_ARBITRARY_FLOAT_MUL_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatMulALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5848u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatMulINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16d9"]
pub const OP_ARBITRARY_FLOAT_DIV_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatDivALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5849u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatDivINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16da"]
pub const OP_ARBITRARY_FLOAT_GTALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatGTALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5850u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatGTINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16db"]
pub const OP_ARBITRARY_FLOAT_GEALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatGEALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5851u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatGEINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16dc"]
pub const OP_ARBITRARY_FLOAT_LTALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatLTALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5852u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatLTINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16dd"]
pub const OP_ARBITRARY_FLOAT_LEALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatLEALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5853u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatLEINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16de"]
pub const OP_ARBITRARY_FLOAT_EQALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatEQALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5854u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatEQINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16df"]
pub const OP_ARBITRARY_FLOAT_RECIP_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatRecipALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5855u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatRecipINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16e0"]
pub const OP_ARBITRARY_FLOAT_R_SQRT_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatRSqrtALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5856u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatRSqrtINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16e1"]
pub const OP_ARBITRARY_FLOAT_CBRT_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatCbrtALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5857u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatCbrtINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16e2"]
pub const OP_ARBITRARY_FLOAT_HYPOT_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatHypotALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5858u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatHypotINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16e3"]
pub const OP_ARBITRARY_FLOAT_SQRT_ALTERA: InstMeta = InstMeta {
    opname: "OpArbitraryFloatSqrtALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5859u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpArbitraryFloatSqrtINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x16e4"]
pub const OP_ARBITRARY_FLOAT_LOG_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatLogINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5860u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16e5"]
pub const OP_ARBITRARY_FLOAT_LOG_2_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatLog2INTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5861u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16e6"]
pub const OP_ARBITRARY_FLOAT_LOG_10_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatLog10INTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5862u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16e7"]
pub const OP_ARBITRARY_FLOAT_LOG_1_P_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatLog1pINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5863u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16e8"]
pub const OP_ARBITRARY_FLOAT_EXP_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatExpINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5864u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16e9"]
pub const OP_ARBITRARY_FLOAT_EXP_2_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatExp2INTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5865u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ea"]
pub const OP_ARBITRARY_FLOAT_EXP_10_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatExp10INTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5866u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16eb"]
pub const OP_ARBITRARY_FLOAT_EXPM_1_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatExpm1INTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5867u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ec"]
pub const OP_ARBITRARY_FLOAT_SIN_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatSinINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5868u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ed"]
pub const OP_ARBITRARY_FLOAT_COS_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatCosINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5869u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ee"]
pub const OP_ARBITRARY_FLOAT_SIN_COS_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatSinCosINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5870u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ef"]
pub const OP_ARBITRARY_FLOAT_SIN_PI_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatSinPiINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5871u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16f0"]
pub const OP_ARBITRARY_FLOAT_COS_PI_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatCosPiINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5872u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16f1"]
pub const OP_ARBITRARY_FLOAT_A_SIN_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatASinINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5873u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16f2"]
pub const OP_ARBITRARY_FLOAT_A_SIN_PI_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatASinPiINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5874u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16f3"]
pub const OP_ARBITRARY_FLOAT_A_COS_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatACosINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5875u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("M1"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mout"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("EnableSubnormals"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("RoundingMode"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("RoundingAccuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16f4"]
pub const OP_ARBITRARY_FLOAT_A_COS_PI_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatACosPiINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5876u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16f5"]
pub const OP_ARBITRARY_FLOAT_A_TAN_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatATanINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5877u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16f6"]
pub const OP_ARBITRARY_FLOAT_A_TAN_PI_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatATanPiINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5878u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16f7"]
pub const OP_ARBITRARY_FLOAT_A_TAN_2_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatATan2INTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5879u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16f8"]
pub const OP_ARBITRARY_FLOAT_POW_INTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatPowINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5880u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16f9"]
pub const OP_ARBITRARY_FLOAT_POW_RINTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatPowRINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5881u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mb"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16fa"]
pub const OP_ARBITRARY_FLOAT_POW_NINTEL: InstMeta = InstMeta {
    opname: "OpArbitraryFloatPowNINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5882u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Ma"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("SignOfB"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Mresult"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Subnormal"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Rounding"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Accuracy"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFloatingPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x16ff"]
pub const OP_LOOP_CONTROL_INTEL: InstMeta = InstMeta {
    opname: "OpLoopControlINTEL",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 5887u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_LITERAL_INTEGER,
        name: Some("Loop Control Parameters"),
        quantifier: Quantifier::ZeroOrMore,
    }],
    capabilities: &[Capability::UnstructuredLoopControlsINTEL],
    extensions: &[EXTENSION_SPV_INTEL_UNSTRUCTURED_LOOP_CONTROLS],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1717"]
pub const OP_ALIAS_DOMAIN_DECL_INTEL: InstMeta = InstMeta {
    opname: "OpAliasDomainDeclINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5911u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Name"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::MemoryAccessAliasingINTEL],
    extensions: &[EXTENSION_SPV_INTEL_MEMORY_ACCESS_ALIASING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1718"]
pub const OP_ALIAS_SCOPE_DECL_INTEL: InstMeta = InstMeta {
    opname: "OpAliasScopeDeclINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5912u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Alias Domain"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Name"),
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::MemoryAccessAliasingINTEL],
    extensions: &[EXTENSION_SPV_INTEL_MEMORY_ACCESS_ALIASING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1719"]
pub const OP_ALIAS_SCOPE_LIST_DECL_INTEL: InstMeta = InstMeta {
    opname: "OpAliasScopeListDeclINTEL",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5913u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("AliasScope 1, AliasScope 2, ..."),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::MemoryAccessAliasingINTEL],
    extensions: &[EXTENSION_SPV_INTEL_MEMORY_ACCESS_ALIASING],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1723"]
pub const OP_FIXED_SQRT_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedSqrtALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5923u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedSqrtINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1724"]
pub const OP_FIXED_RECIP_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedRecipALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5924u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedRecipINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1725"]
pub const OP_FIXED_RSQRT_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedRsqrtALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5925u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedRsqrtINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1726"]
pub const OP_FIXED_SIN_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedSinALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5926u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedSinINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1727"]
pub const OP_FIXED_COS_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedCosALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5927u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedCosINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1728"]
pub const OP_FIXED_SIN_COS_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedSinCosALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5928u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedSinCosINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1729"]
pub const OP_FIXED_SIN_PI_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedSinPiALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5929u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedSinPiINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x172a"]
pub const OP_FIXED_COS_PI_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedCosPiALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5930u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedCosPiINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x172b"]
pub const OP_FIXED_SIN_COS_PI_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedSinCosPiALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5931u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedSinCosPiINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x172c"]
pub const OP_FIXED_LOG_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedLogALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5932u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedLogINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x172d"]
pub const OP_FIXED_EXP_ALTERA: InstMeta = InstMeta {
    opname: "OpFixedExpALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5933u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("S"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("rI"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Q"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("O"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArbitraryPrecisionFixedPointALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFixedExpINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x172e"]
pub const OP_PTR_CAST_TO_CROSS_WORKGROUP_ALTERA: InstMeta = InstMeta {
    opname: "OpPtrCastToCrossWorkgroupALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5934u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::USMStorageClassesALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpPtrCastToCrossWorkgroupINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1732"]
pub const OP_CROSS_WORKGROUP_CAST_TO_PTR_ALTERA: InstMeta = InstMeta {
    opname: "OpCrossWorkgroupCastToPtrALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5938u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::USMStorageClassesALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpCrossWorkgroupCastToPtrINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x173a"]
pub const OP_READ_PIPE_BLOCKING_ALTERA: InstMeta = InstMeta {
    opname: "OpReadPipeBlockingALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5946u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BlockingPipesALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpReadPipeBlockingINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x173b"]
pub const OP_WRITE_PIPE_BLOCKING_ALTERA: InstMeta = InstMeta {
    opname: "OpWritePipeBlockingALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5947u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Packet Alignment"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BlockingPipesALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpWritePipeBlockingINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x173d"]
pub const OP_FPGA_REG_ALTERA: InstMeta = InstMeta {
    opname: "OpFPGARegALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 5949u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Input"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::FPGARegALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpFPGARegINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1780"]
pub const OP_RAY_QUERY_GET_RAY_T_MIN_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetRayTMinKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6016u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1781"]
pub const OP_RAY_QUERY_GET_RAY_FLAGS_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetRayFlagsKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6017u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1782"]
pub const OP_RAY_QUERY_GET_INTERSECTION_TKHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionTKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6018u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1783"]
pub const OP_RAY_QUERY_GET_INTERSECTION_INSTANCE_CUSTOM_INDEX_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionInstanceCustomIndexKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6019u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1784"]
pub const OP_RAY_QUERY_GET_INTERSECTION_INSTANCE_ID_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionInstanceIdKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6020u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1785"]
pub const OP_RAY_QUERY_GET_INTERSECTION_INSTANCE_SHADER_BINDING_TABLE_RECORD_OFFSET_KHR: InstMeta =
    InstMeta {
        opname: "OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR",
        class: Some(&PRINTING_CLASS_RESERVED),
        opcode: 6021u16,
        operands: &[
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT_TYPE,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_RESULT,
                name: None,
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("RayQuery"),
                quantifier: Quantifier::One,
            },
            OperandSpecMeta {
                kind: &OPERAND_KIND_ID_REF,
                name: Some("Intersection"),
                quantifier: Quantifier::One,
            },
        ],
        capabilities: &[Capability::RayQueryKHR],
        extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
        version: Some("None"),
        last_version: None,
        aliases: &[],
        provisional: false,
    };
#[doc = "opcode: 0x1786"]
pub const OP_RAY_QUERY_GET_INTERSECTION_GEOMETRY_INDEX_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionGeometryIndexKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6022u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1787"]
pub const OP_RAY_QUERY_GET_INTERSECTION_PRIMITIVE_INDEX_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionPrimitiveIndexKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6023u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1788"]
pub const OP_RAY_QUERY_GET_INTERSECTION_BARYCENTRICS_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionBarycentricsKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6024u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1789"]
pub const OP_RAY_QUERY_GET_INTERSECTION_FRONT_FACE_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionFrontFaceKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6025u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x178a"]
pub const OP_RAY_QUERY_GET_INTERSECTION_CANDIDATE_AABB_OPAQUE_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionCandidateAABBOpaqueKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6026u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x178b"]
pub const OP_RAY_QUERY_GET_INTERSECTION_OBJECT_RAY_DIRECTION_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionObjectRayDirectionKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6027u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x178c"]
pub const OP_RAY_QUERY_GET_INTERSECTION_OBJECT_RAY_ORIGIN_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionObjectRayOriginKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6028u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x178d"]
pub const OP_RAY_QUERY_GET_WORLD_RAY_DIRECTION_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetWorldRayDirectionKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6029u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x178e"]
pub const OP_RAY_QUERY_GET_WORLD_RAY_ORIGIN_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetWorldRayOriginKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6030u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x178f"]
pub const OP_RAY_QUERY_GET_INTERSECTION_OBJECT_TO_WORLD_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionObjectToWorldKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6031u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1790"]
pub const OP_RAY_QUERY_GET_INTERSECTION_WORLD_TO_OBJECT_KHR: InstMeta = InstMeta {
    opname: "OpRayQueryGetIntersectionWorldToObjectKHR",
    class: Some(&PRINTING_CLASS_RESERVED),
    opcode: 6032u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("RayQuery"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Intersection"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::RayQueryKHR],
    extensions: &[EXTENSION_SPV_KHR_RAY_QUERY],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1793"]
pub const OP_ATOMIC_F_ADD_EXT: InstMeta = InstMeta {
    opname: "OpAtomicFAddEXT",
    class: Some(&PRINTING_CLASS_ATOMIC),
    opcode: 6035u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[
        Capability::AtomicFloat16AddEXT,
        Capability::AtomicFloat32AddEXT,
        Capability::AtomicFloat64AddEXT,
        Capability::AtomicFloat16VectorNV,
    ],
    extensions: &[EXTENSION_SPV_EXT_SHADER_ATOMIC_FLOAT_ADD],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x17c6"]
pub const OP_TYPE_BUFFER_SURFACE_INTEL: InstMeta = InstMeta {
    opname: "OpTypeBufferSurfaceINTEL",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 6086u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ACCESS_QUALIFIER,
            name: Some("AccessQualifier"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::VectorComputeINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x17ca"]
pub const OP_TYPE_STRUCT_CONTINUED_INTEL: InstMeta = InstMeta {
    opname: "OpTypeStructContinuedINTEL",
    class: Some(&PRINTING_CLASS_TYPE_DECLARATION),
    opcode: 6090u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Member 0 type, member 1 type, ..."),
        quantifier: Quantifier::ZeroOrMore,
    }],
    capabilities: &[Capability::LongCompositesINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x17cb"]
pub const OP_CONSTANT_COMPOSITE_CONTINUED_INTEL: InstMeta = InstMeta {
    opname: "OpConstantCompositeContinuedINTEL",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 6091u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Constituents"),
        quantifier: Quantifier::ZeroOrMore,
    }],
    capabilities: &[Capability::LongCompositesINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x17cc"]
pub const OP_SPEC_CONSTANT_COMPOSITE_CONTINUED_INTEL: InstMeta = InstMeta {
    opname: "OpSpecConstantCompositeContinuedINTEL",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 6092u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Constituents"),
        quantifier: Quantifier::ZeroOrMore,
    }],
    capabilities: &[Capability::LongCompositesINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x17d0"]
pub const OP_COMPOSITE_CONSTRUCT_CONTINUED_INTEL: InstMeta = InstMeta {
    opname: "OpCompositeConstructContinuedINTEL",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 6096u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Constituents"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::LongCompositesINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x17e4"]
pub const OP_CONVERT_F_TO_BF_16_INTEL: InstMeta = InstMeta {
    opname: "OpConvertFToBF16INTEL",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 6116u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Float Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BFloat16ConversionINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x17e5"]
pub const OP_CONVERT_BF_16_TO_FINTEL: InstMeta = InstMeta {
    opname: "OpConvertBF16ToFINTEL",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 6117u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("BFloat16 Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BFloat16ConversionINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x17fe"]
pub const OP_CONTROL_BARRIER_ARRIVE_INTEL: InstMeta = InstMeta {
    opname: "OpControlBarrierArriveINTEL",
    class: Some(&PRINTING_CLASS_BARRIER),
    opcode: 6142u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SplitBarrierINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x17ff"]
pub const OP_CONTROL_BARRIER_WAIT_INTEL: InstMeta = InstMeta {
    opname: "OpControlBarrierWaitINTEL",
    class: Some(&PRINTING_CLASS_BARRIER),
    opcode: 6143u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Memory"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_MEMORY_SEMANTICS,
            name: Some("Semantics"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SplitBarrierINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1801"]
pub const OP_ARITHMETIC_FENCE_EXT: InstMeta = InstMeta {
    opname: "OpArithmeticFenceEXT",
    class: Some(&PRINTING_CLASS_MISCELLANEOUS),
    opcode: 6145u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::ArithmeticFenceEXT],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1813"]
pub const OP_TASK_SEQUENCE_CREATE_ALTERA: InstMeta = InstMeta {
    opname: "OpTaskSequenceCreateALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 6163u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Function"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Pipelined"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("UseStallEnableClusters"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("GetCapacity"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("AsyncCapacity"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TaskSequenceALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpTaskSequenceCreateINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1814"]
pub const OP_TASK_SEQUENCE_ASYNC_ALTERA: InstMeta = InstMeta {
    opname: "OpTaskSequenceAsyncALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 6164u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sequence"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Arguments"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::TaskSequenceALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpTaskSequenceAsyncINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1815"]
pub const OP_TASK_SEQUENCE_GET_ALTERA: InstMeta = InstMeta {
    opname: "OpTaskSequenceGetALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 6165u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Sequence"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TaskSequenceALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpTaskSequenceGetINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1816"]
pub const OP_TASK_SEQUENCE_RELEASE_ALTERA: InstMeta = InstMeta {
    opname: "OpTaskSequenceReleaseALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 6166u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Sequence"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::TaskSequenceALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpTaskSequenceReleaseINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x1837"]
pub const OP_TYPE_TASK_SEQUENCE_ALTERA: InstMeta = InstMeta {
    opname: "OpTypeTaskSequenceALTERA",
    class: Some(&PRINTING_CLASS_EXCLUDE),
    opcode: 6199u16,
    operands: &[OperandSpecMeta {
        kind: &OPERAND_KIND_ID_RESULT,
        name: None,
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::TaskSequenceALTERA],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &["OpTypeTaskSequenceINTEL"],
    provisional: false,
};
#[doc = "opcode: 0x184d"]
pub const OP_SUBGROUP_BLOCK_PREFETCH_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupBlockPrefetchINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6221u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Ptr"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("NumBytes"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MEMORY_ACCESS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::SubgroupBufferPrefetchINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1857"]
pub const OP_SUBGROUP_2_D_BLOCK_LOAD_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroup2DBlockLoadINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6231u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Count"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Base Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Pitch"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Dst Pointer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Subgroup2DBlockIOINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1858"]
pub const OP_SUBGROUP_2_D_BLOCK_LOAD_TRANSFORM_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroup2DBlockLoadTransformINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6232u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Count"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Base Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Pitch"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Dst Pointer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Subgroup2DBlockTransformINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1859"]
pub const OP_SUBGROUP_2_D_BLOCK_LOAD_TRANSPOSE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroup2DBlockLoadTransposeINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6233u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Count"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Base Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Pitch"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Dst Pointer"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Subgroup2DBlockTransposeINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x185a"]
pub const OP_SUBGROUP_2_D_BLOCK_PREFETCH_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroup2DBlockPrefetchINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6234u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Count"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Base Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Pitch"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Subgroup2DBlockIOINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x185b"]
pub const OP_SUBGROUP_2_D_BLOCK_STORE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroup2DBlockStoreINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6235u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element Size"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Block Count"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Src Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Dst Base Pointer"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Width"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Height"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Memory Pitch"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Coordinate"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::Subgroup2DBlockIOINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x185d"]
pub const OP_SUBGROUP_MATRIX_MULTIPLY_ACCUMULATE_INTEL: InstMeta = InstMeta {
    opname: "OpSubgroupMatrixMultiplyAccumulateINTEL",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6237u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("K Dim"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Matrix C"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_MATRIX_MULTIPLY_ACCUMULATE_OPERANDS,
            name: None,
            quantifier: Quantifier::ZeroOrOne,
        },
    ],
    capabilities: &[Capability::SubgroupMatrixMultiplyAccumulateINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1862"]
pub const OP_BITWISE_FUNCTION_INTEL: InstMeta = InstMeta {
    opname: "OpBitwiseFunctionINTEL",
    class: Some(&PRINTING_CLASS_BIT),
    opcode: 6242u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("A"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("B"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("C"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("LUTIndex"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TernaryBitwiseFunctionINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1864"]
pub const OP_UNTYPED_VARIABLE_LENGTH_ARRAY_INTEL: InstMeta = InstMeta {
    opname: "OpUntypedVariableLengthArrayINTEL",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 6244u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Element Type"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Length"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::UntypedVariableLengthArrayINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1868"]
pub const OP_CONDITIONAL_EXTENSION_INTEL: InstMeta = InstMeta {
    opname: "OpConditionalExtensionINTEL",
    class: Some(&PRINTING_CLASS_EXTENSION),
    opcode: 6248u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Condition"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Name"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SpecConditionalINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x1869"]
pub const OP_CONDITIONAL_ENTRY_POINT_INTEL: InstMeta = InstMeta {
    opname: "OpConditionalEntryPointINTEL",
    class: Some(&PRINTING_CLASS_MODE_SETTING),
    opcode: 6249u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Condition"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_EXECUTION_MODEL,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Entry Point"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_STRING,
            name: Some("Name"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Interface"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::SpecConditionalINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x186a"]
pub const OP_CONDITIONAL_CAPABILITY_INTEL: InstMeta = InstMeta {
    opname: "OpConditionalCapabilityINTEL",
    class: Some(&PRINTING_CLASS_MODE_SETTING),
    opcode: 6250u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Condition"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_CAPABILITY,
            name: Some("Capability"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::SpecConditionalINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x186b"]
pub const OP_SPEC_CONSTANT_TARGET_INTEL: InstMeta = InstMeta {
    opname: "OpSpecConstantTargetINTEL",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 6251u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Target"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Features"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::FunctionVariantsINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x186c"]
pub const OP_SPEC_CONSTANT_ARCHITECTURE_INTEL: InstMeta = InstMeta {
    opname: "OpSpecConstantArchitectureINTEL",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 6252u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Category"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Family"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Opcode"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Architecture"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::FunctionVariantsINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x186d"]
pub const OP_SPEC_CONSTANT_CAPABILITIES_INTEL: InstMeta = InstMeta {
    opname: "OpSpecConstantCapabilitiesINTEL",
    class: Some(&PRINTING_CLASS_CONSTANT_CREATION),
    opcode: 6253u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_CAPABILITY,
            name: Some("Capabilities"),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::FunctionVariantsINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x186e"]
pub const OP_CONDITIONAL_COPY_OBJECT_INTEL: InstMeta = InstMeta {
    opname: "OpConditionalCopyObjectINTEL",
    class: Some(&PRINTING_CLASS_COMPOSITE),
    opcode: 6254u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Condition 0, Operand 0, +\nCondition 1, Operand 1, +\n..."),
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[Capability::SpecConditionalINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x1901"]
pub const OP_GROUP_I_MUL_KHR: InstMeta = InstMeta {
    opname: "OpGroupIMulKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6401u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupUniformArithmeticKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1902"]
pub const OP_GROUP_F_MUL_KHR: InstMeta = InstMeta {
    opname: "OpGroupFMulKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6402u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupUniformArithmeticKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1903"]
pub const OP_GROUP_BITWISE_AND_KHR: InstMeta = InstMeta {
    opname: "OpGroupBitwiseAndKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6403u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupUniformArithmeticKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1904"]
pub const OP_GROUP_BITWISE_OR_KHR: InstMeta = InstMeta {
    opname: "OpGroupBitwiseOrKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6404u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupUniformArithmeticKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1905"]
pub const OP_GROUP_BITWISE_XOR_KHR: InstMeta = InstMeta {
    opname: "OpGroupBitwiseXorKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6405u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupUniformArithmeticKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1906"]
pub const OP_GROUP_LOGICAL_AND_KHR: InstMeta = InstMeta {
    opname: "OpGroupLogicalAndKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6406u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupUniformArithmeticKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1907"]
pub const OP_GROUP_LOGICAL_OR_KHR: InstMeta = InstMeta {
    opname: "OpGroupLogicalOrKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6407u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupUniformArithmeticKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1908"]
pub const OP_GROUP_LOGICAL_XOR_KHR: InstMeta = InstMeta {
    opname: "OpGroupLogicalXorKHR",
    class: Some(&PRINTING_CLASS_GROUP),
    opcode: 6408u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_SCOPE,
            name: Some("Execution"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_GROUP_OPERATION,
            name: Some("Operation"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("X"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::GroupUniformArithmeticKHR],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x191a"]
pub const OP_ROUND_F_TO_TF_32_INTEL: InstMeta = InstMeta {
    opname: "OpRoundFToTF32INTEL",
    class: Some(&PRINTING_CLASS_CONVERSION),
    opcode: 6426u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Float Value"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::TensorFloat32RoundingINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x191c"]
pub const OP_MASKED_GATHER_INTEL: InstMeta = InstMeta {
    opname: "OpMaskedGatherINTEL",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 6428u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("PtrVector"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Alignment"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Mask"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("FillEmpty"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::MaskedGatherScatterINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x191d"]
pub const OP_MASKED_SCATTER_INTEL: InstMeta = InstMeta {
    opname: "OpMaskedScatterINTEL",
    class: Some(&PRINTING_CLASS_MEMORY),
    opcode: 6429u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("InputVector"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("PtrVector"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_LITERAL_INTEGER,
            name: Some("Alignment"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Mask"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::MaskedGatherScatterINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: false,
};
#[doc = "opcode: 0x1981"]
pub const OP_CONVERT_HANDLE_TO_IMAGE_INTEL: InstMeta = InstMeta {
    opname: "OpConvertHandleToImageINTEL",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 6529u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BindlessImagesINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x1982"]
pub const OP_CONVERT_HANDLE_TO_SAMPLER_INTEL: InstMeta = InstMeta {
    opname: "OpConvertHandleToSamplerINTEL",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 6530u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BindlessImagesINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
#[doc = "opcode: 0x1983"]
pub const OP_CONVERT_HANDLE_TO_SAMPLED_IMAGE_INTEL: InstMeta = InstMeta {
    opname: "OpConvertHandleToSampledImageINTEL",
    class: Some(&PRINTING_CLASS_IMAGE),
    opcode: 6531u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT_TYPE,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_RESULT,
            name: None,
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Operand"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::BindlessImagesINTEL],
    extensions: &[],
    version: Some("None"),
    last_version: None,
    aliases: &[],
    provisional: true,
};
