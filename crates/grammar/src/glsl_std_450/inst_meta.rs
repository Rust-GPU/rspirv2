use super::preamble::*;
pub const ROUND: InstMeta = InstMeta {
    opname: "Round",
    class: None,
    opcode: 1u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const ROUND_EVEN: InstMeta = InstMeta {
    opname: "RoundEven",
    class: None,
    opcode: 2u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const TRUNC: InstMeta = InstMeta {
    opname: "Trunc",
    class: None,
    opcode: 3u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const F_ABS: InstMeta = InstMeta {
    opname: "FAbs",
    class: None,
    opcode: 4u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const S_ABS: InstMeta = InstMeta {
    opname: "SAbs",
    class: None,
    opcode: 5u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const F_SIGN: InstMeta = InstMeta {
    opname: "FSign",
    class: None,
    opcode: 6u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const S_SIGN: InstMeta = InstMeta {
    opname: "SSign",
    class: None,
    opcode: 7u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const FLOOR: InstMeta = InstMeta {
    opname: "Floor",
    class: None,
    opcode: 8u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const CEIL: InstMeta = InstMeta {
    opname: "Ceil",
    class: None,
    opcode: 9u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const FRACT: InstMeta = InstMeta {
    opname: "Fract",
    class: None,
    opcode: 10u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const RADIANS: InstMeta = InstMeta {
    opname: "Radians",
    class: None,
    opcode: 11u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("degrees"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const DEGREES: InstMeta = InstMeta {
    opname: "Degrees",
    class: None,
    opcode: 12u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("radians"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const SIN: InstMeta = InstMeta {
    opname: "Sin",
    class: None,
    opcode: 13u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const COS: InstMeta = InstMeta {
    opname: "Cos",
    class: None,
    opcode: 14u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const TAN: InstMeta = InstMeta {
    opname: "Tan",
    class: None,
    opcode: 15u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const ASIN: InstMeta = InstMeta {
    opname: "Asin",
    class: None,
    opcode: 16u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const ACOS: InstMeta = InstMeta {
    opname: "Acos",
    class: None,
    opcode: 17u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const ATAN: InstMeta = InstMeta {
    opname: "Atan",
    class: None,
    opcode: 18u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("y_over_x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const SINH: InstMeta = InstMeta {
    opname: "Sinh",
    class: None,
    opcode: 19u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const COSH: InstMeta = InstMeta {
    opname: "Cosh",
    class: None,
    opcode: 20u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const TANH: InstMeta = InstMeta {
    opname: "Tanh",
    class: None,
    opcode: 21u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const ASINH: InstMeta = InstMeta {
    opname: "Asinh",
    class: None,
    opcode: 22u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const ACOSH: InstMeta = InstMeta {
    opname: "Acosh",
    class: None,
    opcode: 23u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const ATANH: InstMeta = InstMeta {
    opname: "Atanh",
    class: None,
    opcode: 24u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const ATAN_2: InstMeta = InstMeta {
    opname: "Atan2",
    class: None,
    opcode: 25u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const POW: InstMeta = InstMeta {
    opname: "Pow",
    class: None,
    opcode: 26u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const EXP: InstMeta = InstMeta {
    opname: "Exp",
    class: None,
    opcode: 27u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const LOG: InstMeta = InstMeta {
    opname: "Log",
    class: None,
    opcode: 28u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const EXP_2: InstMeta = InstMeta {
    opname: "Exp2",
    class: None,
    opcode: 29u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const LOG_2: InstMeta = InstMeta {
    opname: "Log2",
    class: None,
    opcode: 30u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const SQRT: InstMeta = InstMeta {
    opname: "Sqrt",
    class: None,
    opcode: 31u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const INVERSE_SQRT: InstMeta = InstMeta {
    opname: "InverseSqrt",
    class: None,
    opcode: 32u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const DETERMINANT: InstMeta = InstMeta {
    opname: "Determinant",
    class: None,
    opcode: 33u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const MATRIX_INVERSE: InstMeta = InstMeta {
    opname: "MatrixInverse",
    class: None,
    opcode: 34u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const MODF: InstMeta = InstMeta {
    opname: "Modf",
    class: None,
    opcode: 35u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("i"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const MODF_STRUCT: InstMeta = InstMeta {
    opname: "ModfStruct",
    class: None,
    opcode: 36u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const F_MIN: InstMeta = InstMeta {
    opname: "FMin",
    class: None,
    opcode: 37u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const U_MIN: InstMeta = InstMeta {
    opname: "UMin",
    class: None,
    opcode: 38u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const S_MIN: InstMeta = InstMeta {
    opname: "SMin",
    class: None,
    opcode: 39u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const F_MAX: InstMeta = InstMeta {
    opname: "FMax",
    class: None,
    opcode: 40u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const U_MAX: InstMeta = InstMeta {
    opname: "UMax",
    class: None,
    opcode: 41u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const S_MAX: InstMeta = InstMeta {
    opname: "SMax",
    class: None,
    opcode: 42u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const F_CLAMP: InstMeta = InstMeta {
    opname: "FClamp",
    class: None,
    opcode: 43u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("minVal"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("maxVal"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const U_CLAMP: InstMeta = InstMeta {
    opname: "UClamp",
    class: None,
    opcode: 44u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("minVal"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("maxVal"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const S_CLAMP: InstMeta = InstMeta {
    opname: "SClamp",
    class: None,
    opcode: 45u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("minVal"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("maxVal"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const F_MIX: InstMeta = InstMeta {
    opname: "FMix",
    class: None,
    opcode: 46u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("a"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const I_MIX: InstMeta = InstMeta {
    opname: "IMix",
    class: None,
    opcode: 47u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("a"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const STEP: InstMeta = InstMeta {
    opname: "Step",
    class: None,
    opcode: 48u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("edge"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const SMOOTH_STEP: InstMeta = InstMeta {
    opname: "SmoothStep",
    class: None,
    opcode: 49u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("edge0"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("edge1"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const FMA: InstMeta = InstMeta {
    opname: "Fma",
    class: None,
    opcode: 50u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("a"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("b"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("c"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const FREXP: InstMeta = InstMeta {
    opname: "Frexp",
    class: None,
    opcode: 51u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("exp"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const FREXP_STRUCT: InstMeta = InstMeta {
    opname: "FrexpStruct",
    class: None,
    opcode: 52u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const LDEXP: InstMeta = InstMeta {
    opname: "Ldexp",
    class: None,
    opcode: 53u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("exp"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const PACK_SNORM_4_X_8: InstMeta = InstMeta {
    opname: "PackSnorm4x8",
    class: None,
    opcode: 54u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("v"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const PACK_UNORM_4_X_8: InstMeta = InstMeta {
    opname: "PackUnorm4x8",
    class: None,
    opcode: 55u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("v"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const PACK_SNORM_2_X_16: InstMeta = InstMeta {
    opname: "PackSnorm2x16",
    class: None,
    opcode: 56u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("v"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const PACK_UNORM_2_X_16: InstMeta = InstMeta {
    opname: "PackUnorm2x16",
    class: None,
    opcode: 57u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("v"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const PACK_HALF_2_X_16: InstMeta = InstMeta {
    opname: "PackHalf2x16",
    class: None,
    opcode: 58u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("v"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const PACK_DOUBLE_2_X_32: InstMeta = InstMeta {
    opname: "PackDouble2x32",
    class: None,
    opcode: 59u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("v"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::Float64],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const UNPACK_SNORM_2_X_16: InstMeta = InstMeta {
    opname: "UnpackSnorm2x16",
    class: None,
    opcode: 60u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("p"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const UNPACK_UNORM_2_X_16: InstMeta = InstMeta {
    opname: "UnpackUnorm2x16",
    class: None,
    opcode: 61u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("p"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const UNPACK_HALF_2_X_16: InstMeta = InstMeta {
    opname: "UnpackHalf2x16",
    class: None,
    opcode: 62u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("v"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const UNPACK_SNORM_4_X_8: InstMeta = InstMeta {
    opname: "UnpackSnorm4x8",
    class: None,
    opcode: 63u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("p"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const UNPACK_UNORM_4_X_8: InstMeta = InstMeta {
    opname: "UnpackUnorm4x8",
    class: None,
    opcode: 64u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("p"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const UNPACK_DOUBLE_2_X_32: InstMeta = InstMeta {
    opname: "UnpackDouble2x32",
    class: None,
    opcode: 65u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("v"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::Float64],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const LENGTH: InstMeta = InstMeta {
    opname: "Length",
    class: None,
    opcode: 66u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const DISTANCE: InstMeta = InstMeta {
    opname: "Distance",
    class: None,
    opcode: 67u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("p0"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("p1"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const CROSS: InstMeta = InstMeta {
    opname: "Cross",
    class: None,
    opcode: 68u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const NORMALIZE: InstMeta = InstMeta {
    opname: "Normalize",
    class: None,
    opcode: 69u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("x"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const FACE_FORWARD: InstMeta = InstMeta {
    opname: "FaceForward",
    class: None,
    opcode: 70u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("N"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Nref"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const REFLECT: InstMeta = InstMeta {
    opname: "Reflect",
    class: None,
    opcode: 71u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("N"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const REFRACT: InstMeta = InstMeta {
    opname: "Refract",
    class: None,
    opcode: 72u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("I"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("N"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("eta"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const FIND_I_LSB: InstMeta = InstMeta {
    opname: "FindILsb",
    class: None,
    opcode: 73u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Value"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const FIND_S_MSB: InstMeta = InstMeta {
    opname: "FindSMsb",
    class: None,
    opcode: 74u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Value"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const FIND_U_MSB: InstMeta = InstMeta {
    opname: "FindUMsb",
    class: None,
    opcode: 75u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("Value"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const INTERPOLATE_AT_CENTROID: InstMeta = InstMeta {
    opname: "InterpolateAtCentroid",
    class: None,
    opcode: 76u16,
    operands: &[OperandMeta {
        kind: &OPERAND_KIND_ID_REF,
        name: Some("interpolant"),
        quantifier: Quantifier::One,
    }],
    capabilities: &[Capability::InterpolationFunction],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const INTERPOLATE_AT_SAMPLE: InstMeta = InstMeta {
    opname: "InterpolateAtSample",
    class: None,
    opcode: 77u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("interpolant"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("sample"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::InterpolationFunction],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const INTERPOLATE_AT_OFFSET: InstMeta = InstMeta {
    opname: "InterpolateAtOffset",
    class: None,
    opcode: 78u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("interpolant"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("offset"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[Capability::InterpolationFunction],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const N_MIN: InstMeta = InstMeta {
    opname: "NMin",
    class: None,
    opcode: 79u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const N_MAX: InstMeta = InstMeta {
    opname: "NMax",
    class: None,
    opcode: 80u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("y"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
pub const N_CLAMP: InstMeta = InstMeta {
    opname: "NClamp",
    class: None,
    opcode: 81u16,
    operands: &[
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("x"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("minVal"),
            quantifier: Quantifier::One,
        },
        OperandMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("maxVal"),
            quantifier: Quantifier::One,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
