use super::preamble::*;
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct ImageOperands : u32 {
    const None = 0u32; const Bias = 1u32; const Lod = 2u32; const Grad = 4u32; const
    ConstOffset = 8u32; const Offset = 16u32; const ConstOffsets = 32u32; const Sample =
    64u32; const MinLod = 128u32; #[doc = "Since SPIR-V 1.5"] const MakeTexelAvailable =
    256u32; #[doc = "Since SPIR-V 1.5"] const MakeTexelVisible = 512u32; #[doc =
    "Since SPIR-V 1.5"] const NonPrivateTexel = 1024u32; #[doc = "Since SPIR-V 1.5"]
    const VolatileTexel = 2048u32; #[doc = "Since SPIR-V 1.4"] const SignExtend =
    4096u32; #[doc = "Since SPIR-V 1.4"] const ZeroExtend = 8192u32; #[doc =
    "Since SPIR-V 1.6"] const Nontemporal = 16384u32; const Offsets = 65536u32; }
}
unsafe impl Operand for ImageOperands {
    const KIND: &OperandKind = &OPERAND_KIND_IMAGE_OPERANDS;
}
unsafe impl OperandEncoding for ImageOperands {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<ImageOperands>(
            stringify!(ImageOperands),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::Bias) {
                write!(f, "{sep}Bias")?;
            }
            if self.contains(Self::Lod) {
                write!(f, "{sep}Lod")?;
            }
            if self.contains(Self::Grad) {
                write!(f, "{sep}Grad")?;
            }
            if self.contains(Self::ConstOffset) {
                write!(f, "{sep}ConstOffset")?;
            }
            if self.contains(Self::Offset) {
                write!(f, "{sep}Offset")?;
            }
            if self.contains(Self::ConstOffsets) {
                write!(f, "{sep}ConstOffsets")?;
            }
            if self.contains(Self::Sample) {
                write!(f, "{sep}Sample")?;
            }
            if self.contains(Self::MinLod) {
                write!(f, "{sep}MinLod")?;
            }
            if self.contains(Self::MakeTexelAvailable) {
                write!(f, "{sep}MakeTexelAvailable")?;
            }
            if self.contains(Self::MakeTexelVisible) {
                write!(f, "{sep}MakeTexelVisible")?;
            }
            if self.contains(Self::NonPrivateTexel) {
                write!(f, "{sep}NonPrivateTexel")?;
            }
            if self.contains(Self::VolatileTexel) {
                write!(f, "{sep}VolatileTexel")?;
            }
            if self.contains(Self::SignExtend) {
                write!(f, "{sep}SignExtend")?;
            }
            if self.contains(Self::ZeroExtend) {
                write!(f, "{sep}ZeroExtend")?;
            }
            if self.contains(Self::Nontemporal) {
                write!(f, "{sep}Nontemporal")?;
            }
            if self.contains(Self::Offsets) {
                write!(f, "{sep}Offsets")?;
            }
            Ok(())
        }
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct FPFastMathMode : u32 {
    const None = 0u32; const NotNaN = 1u32; const NotInf = 2u32; const NSZ = 4u32; const
    AllowRecip = 8u32; const Fast = 16u32; const AllowContract = 65536u32; const
    AllowReassoc = 131072u32; const AllowTransform = 262144u32; }
}
unsafe impl Operand for FPFastMathMode {
    const KIND: &OperandKind = &OPERAND_KIND_FP_FAST_MATH_MODE;
}
unsafe impl OperandEncoding for FPFastMathMode {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<FPFastMathMode>(
            stringify!(FPFastMathMode),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::NotNaN) {
                write!(f, "{sep}NotNaN")?;
            }
            if self.contains(Self::NotInf) {
                write!(f, "{sep}NotInf")?;
            }
            if self.contains(Self::NSZ) {
                write!(f, "{sep}NSZ")?;
            }
            if self.contains(Self::AllowRecip) {
                write!(f, "{sep}AllowRecip")?;
            }
            if self.contains(Self::Fast) {
                write!(f, "{sep}Fast")?;
            }
            if self.contains(Self::AllowContract) {
                write!(f, "{sep}AllowContract")?;
            }
            if self.contains(Self::AllowReassoc) {
                write!(f, "{sep}AllowReassoc")?;
            }
            if self.contains(Self::AllowTransform) {
                write!(f, "{sep}AllowTransform")?;
            }
            Ok(())
        }
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct SelectionControl : u32
    { const None = 0u32; const Flatten = 1u32; const DontFlatten = 2u32; }
}
unsafe impl Operand for SelectionControl {
    const KIND: &OperandKind = &OPERAND_KIND_SELECTION_CONTROL;
}
unsafe impl OperandEncoding for SelectionControl {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<SelectionControl>(
            stringify!(SelectionControl),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::Flatten) {
                write!(f, "{sep}Flatten")?;
            }
            if self.contains(Self::DontFlatten) {
                write!(f, "{sep}DontFlatten")?;
            }
            Ok(())
        }
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct LoopControl : u32 {
    const None = 0u32; const Unroll = 1u32; const DontUnroll = 2u32; #[doc =
    "Since SPIR-V 1.1"] const DependencyInfinite = 4u32; #[doc = "Since SPIR-V 1.1"]
    const DependencyLength = 8u32; #[doc = "Since SPIR-V 1.4"] const MinIterations =
    16u32; #[doc = "Since SPIR-V 1.4"] const MaxIterations = 32u32; #[doc =
    "Since SPIR-V 1.4"] const IterationMultiple = 64u32; #[doc = "Since SPIR-V 1.4"]
    const PeelCount = 128u32; #[doc = "Since SPIR-V 1.4"] const PartialCount = 256u32;
    const InitiationIntervalALTERA = 65536u32; const MaxConcurrencyALTERA = 131072u32;
    const DependencyArrayALTERA = 262144u32; const PipelineEnableALTERA = 524288u32;
    const LoopCoalesceALTERA = 1048576u32; const MaxInterleavingALTERA = 2097152u32;
    const SpeculatedIterationsALTERA = 4194304u32; const NoFusionALTERA = 8388608u32;
    const LoopCountALTERA = 16777216u32; const MaxReinvocationDelayALTERA = 33554432u32;
    }
}
unsafe impl Operand for LoopControl {
    const KIND: &OperandKind = &OPERAND_KIND_LOOP_CONTROL;
}
unsafe impl OperandEncoding for LoopControl {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<LoopControl>(
            stringify!(LoopControl),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::Unroll) {
                write!(f, "{sep}Unroll")?;
            }
            if self.contains(Self::DontUnroll) {
                write!(f, "{sep}DontUnroll")?;
            }
            if self.contains(Self::DependencyInfinite) {
                write!(f, "{sep}DependencyInfinite")?;
            }
            if self.contains(Self::DependencyLength) {
                write!(f, "{sep}DependencyLength")?;
            }
            if self.contains(Self::MinIterations) {
                write!(f, "{sep}MinIterations")?;
            }
            if self.contains(Self::MaxIterations) {
                write!(f, "{sep}MaxIterations")?;
            }
            if self.contains(Self::IterationMultiple) {
                write!(f, "{sep}IterationMultiple")?;
            }
            if self.contains(Self::PeelCount) {
                write!(f, "{sep}PeelCount")?;
            }
            if self.contains(Self::PartialCount) {
                write!(f, "{sep}PartialCount")?;
            }
            if self.contains(Self::InitiationIntervalALTERA) {
                write!(f, "{sep}InitiationIntervalALTERA")?;
            }
            if self.contains(Self::MaxConcurrencyALTERA) {
                write!(f, "{sep}MaxConcurrencyALTERA")?;
            }
            if self.contains(Self::DependencyArrayALTERA) {
                write!(f, "{sep}DependencyArrayALTERA")?;
            }
            if self.contains(Self::PipelineEnableALTERA) {
                write!(f, "{sep}PipelineEnableALTERA")?;
            }
            if self.contains(Self::LoopCoalesceALTERA) {
                write!(f, "{sep}LoopCoalesceALTERA")?;
            }
            if self.contains(Self::MaxInterleavingALTERA) {
                write!(f, "{sep}MaxInterleavingALTERA")?;
            }
            if self.contains(Self::SpeculatedIterationsALTERA) {
                write!(f, "{sep}SpeculatedIterationsALTERA")?;
            }
            if self.contains(Self::NoFusionALTERA) {
                write!(f, "{sep}NoFusionALTERA")?;
            }
            if self.contains(Self::LoopCountALTERA) {
                write!(f, "{sep}LoopCountALTERA")?;
            }
            if self.contains(Self::MaxReinvocationDelayALTERA) {
                write!(f, "{sep}MaxReinvocationDelayALTERA")?;
            }
            Ok(())
        }
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct FunctionControl : u32 {
    const None = 0u32; const Inline = 1u32; const DontInline = 2u32; const Pure = 4u32;
    const Const = 8u32; const OptNoneEXT = 65536u32; }
}
unsafe impl Operand for FunctionControl {
    const KIND: &OperandKind = &OPERAND_KIND_FUNCTION_CONTROL;
}
unsafe impl OperandEncoding for FunctionControl {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<FunctionControl>(
            stringify!(FunctionControl),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::Inline) {
                write!(f, "{sep}Inline")?;
            }
            if self.contains(Self::DontInline) {
                write!(f, "{sep}DontInline")?;
            }
            if self.contains(Self::Pure) {
                write!(f, "{sep}Pure")?;
            }
            if self.contains(Self::Const) {
                write!(f, "{sep}Const")?;
            }
            if self.contains(Self::OptNoneEXT) {
                write!(f, "{sep}OptNoneEXT")?;
            }
            Ok(())
        }
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct MemorySemantics : u32 {
    const Relaxed = 0u32; const Acquire = 2u32; const Release = 4u32; const
    AcquireRelease = 8u32; const SequentiallyConsistent = 16u32; const UniformMemory =
    64u32; const SubgroupMemory = 128u32; const WorkgroupMemory = 256u32; const
    CrossWorkgroupMemory = 512u32; const AtomicCounterMemory = 1024u32; const ImageMemory
    = 2048u32; #[doc = "Since SPIR-V 1.5"] const OutputMemory = 4096u32; #[doc =
    "Since SPIR-V 1.5"] const MakeAvailable = 8192u32; #[doc = "Since SPIR-V 1.5"] const
    MakeVisible = 16384u32; #[doc = "Since SPIR-V 1.5"] const Volatile = 32768u32; }
}
unsafe impl Operand for MemorySemantics {
    const KIND: &OperandKind = &OPERAND_KIND_MEMORY_SEMANTICS;
}
unsafe impl OperandEncoding for MemorySemantics {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<MemorySemantics>(
            stringify!(MemorySemantics),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::Acquire) {
                write!(f, "{sep}Acquire")?;
            }
            if self.contains(Self::Release) {
                write!(f, "{sep}Release")?;
            }
            if self.contains(Self::AcquireRelease) {
                write!(f, "{sep}AcquireRelease")?;
            }
            if self.contains(Self::SequentiallyConsistent) {
                write!(f, "{sep}SequentiallyConsistent")?;
            }
            if self.contains(Self::UniformMemory) {
                write!(f, "{sep}UniformMemory")?;
            }
            if self.contains(Self::SubgroupMemory) {
                write!(f, "{sep}SubgroupMemory")?;
            }
            if self.contains(Self::WorkgroupMemory) {
                write!(f, "{sep}WorkgroupMemory")?;
            }
            if self.contains(Self::CrossWorkgroupMemory) {
                write!(f, "{sep}CrossWorkgroupMemory")?;
            }
            if self.contains(Self::AtomicCounterMemory) {
                write!(f, "{sep}AtomicCounterMemory")?;
            }
            if self.contains(Self::ImageMemory) {
                write!(f, "{sep}ImageMemory")?;
            }
            if self.contains(Self::OutputMemory) {
                write!(f, "{sep}OutputMemory")?;
            }
            if self.contains(Self::MakeAvailable) {
                write!(f, "{sep}MakeAvailable")?;
            }
            if self.contains(Self::MakeVisible) {
                write!(f, "{sep}MakeVisible")?;
            }
            if self.contains(Self::Volatile) {
                write!(f, "{sep}Volatile")?;
            }
            Ok(())
        }
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct MemoryAccess : u32 {
    const None = 0u32; const Volatile = 1u32; const Aligned = 2u32; const Nontemporal =
    4u32; #[doc = "Since SPIR-V 1.5"] const MakePointerAvailable = 8u32; #[doc =
    "Since SPIR-V 1.5"] const MakePointerVisible = 16u32; #[doc = "Since SPIR-V 1.5"]
    const NonPrivatePointer = 32u32; const AliasScopeINTELMask = 65536u32; const
    NoAliasINTELMask = 131072u32; }
}
unsafe impl Operand for MemoryAccess {
    const KIND: &OperandKind = &OPERAND_KIND_MEMORY_ACCESS;
}
unsafe impl OperandEncoding for MemoryAccess {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<MemoryAccess>(
            stringify!(MemoryAccess),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::Volatile) {
                write!(f, "{sep}Volatile")?;
            }
            if self.contains(Self::Aligned) {
                write!(f, "{sep}Aligned")?;
            }
            if self.contains(Self::Nontemporal) {
                write!(f, "{sep}Nontemporal")?;
            }
            if self.contains(Self::MakePointerAvailable) {
                write!(f, "{sep}MakePointerAvailable")?;
            }
            if self.contains(Self::MakePointerVisible) {
                write!(f, "{sep}MakePointerVisible")?;
            }
            if self.contains(Self::NonPrivatePointer) {
                write!(f, "{sep}NonPrivatePointer")?;
            }
            if self.contains(Self::AliasScopeINTELMask) {
                write!(f, "{sep}AliasScopeINTELMask")?;
            }
            if self.contains(Self::NoAliasINTELMask) {
                write!(f, "{sep}NoAliasINTELMask")?;
            }
            Ok(())
        }
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct KernelProfilingInfo :
    u32 { const None = 0u32; const CmdExecTime = 1u32; }
}
unsafe impl Operand for KernelProfilingInfo {
    const KIND: &OperandKind = &OPERAND_KIND_KERNEL_PROFILING_INFO;
}
unsafe impl OperandEncoding for KernelProfilingInfo {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<KernelProfilingInfo>(
            stringify!(KernelProfilingInfo),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::CmdExecTime) {
                write!(f, "{sep}CmdExecTime")?;
            }
            Ok(())
        }
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct RayFlags : u32 { const
    NoneKHR = 0u32; const OpaqueKHR = 1u32; const NoOpaqueKHR = 2u32; const
    TerminateOnFirstHitKHR = 4u32; const SkipClosestHitShaderKHR = 8u32; const
    CullBackFacingTrianglesKHR = 16u32; const CullFrontFacingTrianglesKHR = 32u32; const
    CullOpaqueKHR = 64u32; const CullNoOpaqueKHR = 128u32; const SkipTrianglesKHR =
    256u32; const SkipAABBsKHR = 512u32; const ForceOpacityMicromap2StateEXT = 1024u32; }
}
unsafe impl Operand for RayFlags {
    const KIND: &OperandKind = &OPERAND_KIND_RAY_FLAGS;
}
unsafe impl OperandEncoding for RayFlags {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<RayFlags>(
            stringify!(RayFlags),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::OpaqueKHR) {
                write!(f, "{sep}OpaqueKHR")?;
            }
            if self.contains(Self::NoOpaqueKHR) {
                write!(f, "{sep}NoOpaqueKHR")?;
            }
            if self.contains(Self::TerminateOnFirstHitKHR) {
                write!(f, "{sep}TerminateOnFirstHitKHR")?;
            }
            if self.contains(Self::SkipClosestHitShaderKHR) {
                write!(f, "{sep}SkipClosestHitShaderKHR")?;
            }
            if self.contains(Self::CullBackFacingTrianglesKHR) {
                write!(f, "{sep}CullBackFacingTrianglesKHR")?;
            }
            if self.contains(Self::CullFrontFacingTrianglesKHR) {
                write!(f, "{sep}CullFrontFacingTrianglesKHR")?;
            }
            if self.contains(Self::CullOpaqueKHR) {
                write!(f, "{sep}CullOpaqueKHR")?;
            }
            if self.contains(Self::CullNoOpaqueKHR) {
                write!(f, "{sep}CullNoOpaqueKHR")?;
            }
            if self.contains(Self::SkipTrianglesKHR) {
                write!(f, "{sep}SkipTrianglesKHR")?;
            }
            if self.contains(Self::SkipAABBsKHR) {
                write!(f, "{sep}SkipAABBsKHR")?;
            }
            if self.contains(Self::ForceOpacityMicromap2StateEXT) {
                write!(f, "{sep}ForceOpacityMicromap2StateEXT")?;
            }
            Ok(())
        }
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct FragmentShadingRate :
    u32 { const Vertical2Pixels = 1u32; const Vertical4Pixels = 2u32; const
    Horizontal2Pixels = 4u32; const Horizontal4Pixels = 8u32; }
}
unsafe impl Operand for FragmentShadingRate {
    const KIND: &OperandKind = &OPERAND_KIND_FRAGMENT_SHADING_RATE;
}
unsafe impl OperandEncoding for FragmentShadingRate {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<FragmentShadingRate>(
            stringify!(FragmentShadingRate),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::Vertical2Pixels) {
                write!(f, "{sep}Vertical2Pixels")?;
            }
            if self.contains(Self::Vertical4Pixels) {
                write!(f, "{sep}Vertical4Pixels")?;
            }
            if self.contains(Self::Horizontal2Pixels) {
                write!(f, "{sep}Horizontal2Pixels")?;
            }
            if self.contains(Self::Horizontal4Pixels) {
                write!(f, "{sep}Horizontal4Pixels")?;
            }
            Ok(())
        }
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct RawAccessChainOperands
    : u32 { const None = 0u32; const RobustnessPerComponentNV = 1u32; const
    RobustnessPerElementNV = 2u32; }
}
unsafe impl Operand for RawAccessChainOperands {
    const KIND: &OperandKind = &OPERAND_KIND_RAW_ACCESS_CHAIN_OPERANDS;
}
unsafe impl OperandEncoding for RawAccessChainOperands {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<RawAccessChainOperands>(
            stringify!(RawAccessChainOperands),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::RobustnessPerComponentNV) {
                write!(f, "{sep}RobustnessPerComponentNV")?;
            }
            if self.contains(Self::RobustnessPerElementNV) {
                write!(f, "{sep}RobustnessPerElementNV")?;
            }
            Ok(())
        }
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SourceLanguage {
    Unknown = 0u32,
    ESSL = 1u32,
    GLSL = 2u32,
    OpenCL_C = 3u32,
    OpenCL_CPP = 4u32,
    HLSL = 5u32,
    CPP_for_OpenCL = 6u32,
    SYCL = 7u32,
    HERO_C = 8u32,
    NZSL = 9u32,
    WGSL = 10u32,
    Slang = 11u32,
    Zig = 12u32,
    Rust = 13u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for SourceLanguage {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for SourceLanguage {}
unsafe impl Operand for SourceLanguage {
    const KIND: &OperandKind = &OPERAND_KIND_SOURCE_LANGUAGE;
}
unsafe impl OperandEncoding for SourceLanguage {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Unknown,
            1u32 => Self::ESSL,
            2u32 => Self::GLSL,
            3u32 => Self::OpenCL_C,
            4u32 => Self::OpenCL_CPP,
            5u32 => Self::HLSL,
            6u32 => Self::CPP_for_OpenCL,
            7u32 => Self::SYCL,
            8u32 => Self::HERO_C,
            9u32 => Self::NZSL,
            10u32 => Self::WGSL,
            11u32 => Self::Slang,
            12u32 => Self::Zig,
            13u32 => Self::Rust,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(SourceLanguage),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ExecutionModel {
    Vertex = 0u32,
    TessellationControl = 1u32,
    TessellationEvaluation = 2u32,
    Geometry = 3u32,
    Fragment = 4u32,
    GLCompute = 5u32,
    Kernel = 6u32,
    TaskNV = 5267u32,
    MeshNV = 5268u32,
    RayGenerationKHR = 5313u32,
    IntersectionKHR = 5314u32,
    AnyHitKHR = 5315u32,
    ClosestHitKHR = 5316u32,
    MissKHR = 5317u32,
    CallableKHR = 5318u32,
    TaskEXT = 5364u32,
    MeshEXT = 5365u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for ExecutionModel {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for ExecutionModel {}
unsafe impl Operand for ExecutionModel {
    const KIND: &OperandKind = &OPERAND_KIND_EXECUTION_MODEL;
}
unsafe impl OperandEncoding for ExecutionModel {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Vertex,
            1u32 => Self::TessellationControl,
            2u32 => Self::TessellationEvaluation,
            3u32 => Self::Geometry,
            4u32 => Self::Fragment,
            5u32 => Self::GLCompute,
            6u32 => Self::Kernel,
            5267u32 => Self::TaskNV,
            5268u32 => Self::MeshNV,
            5313u32 => Self::RayGenerationKHR,
            5314u32 => Self::IntersectionKHR,
            5315u32 => Self::AnyHitKHR,
            5316u32 => Self::ClosestHitKHR,
            5317u32 => Self::MissKHR,
            5318u32 => Self::CallableKHR,
            5364u32 => Self::TaskEXT,
            5365u32 => Self::MeshEXT,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(ExecutionModel),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AddressingModel {
    Logical = 0u32,
    Physical32 = 1u32,
    Physical64 = 2u32,
    ///Since SPIR-V 1.5
    PhysicalStorageBuffer64 = 5348u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for AddressingModel {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for AddressingModel {}
unsafe impl Operand for AddressingModel {
    const KIND: &OperandKind = &OPERAND_KIND_ADDRESSING_MODEL;
}
unsafe impl OperandEncoding for AddressingModel {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Logical,
            1u32 => Self::Physical32,
            2u32 => Self::Physical64,
            5348u32 => Self::PhysicalStorageBuffer64,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(AddressingModel),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MemoryModel {
    Simple = 0u32,
    GLSL450 = 1u32,
    OpenCL = 2u32,
    ///Since SPIR-V 1.5
    Vulkan = 3u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for MemoryModel {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for MemoryModel {}
unsafe impl Operand for MemoryModel {
    const KIND: &OperandKind = &OPERAND_KIND_MEMORY_MODEL;
}
unsafe impl OperandEncoding for MemoryModel {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Simple,
            1u32 => Self::GLSL450,
            2u32 => Self::OpenCL,
            3u32 => Self::Vulkan,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(MemoryModel),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ExecutionMode {
    Invocations(
        ///Number of <<Invocation,invocations>>
        LiteralInteger,
    ),
    SpacingEqual,
    SpacingFractionalEven,
    SpacingFractionalOdd,
    VertexOrderCw,
    VertexOrderCcw,
    PixelCenterInteger,
    OriginUpperLeft,
    OriginLowerLeft,
    EarlyFragmentTests,
    PointMode,
    Xfb,
    DepthReplacing,
    DepthGreater,
    DepthLess,
    DepthUnchanged,
    LocalSize(
        ///x size
        LiteralInteger,
        ///y size
        LiteralInteger,
        ///z size
        LiteralInteger,
    ),
    LocalSizeHint(
        ///x size
        LiteralInteger,
        ///y size
        LiteralInteger,
        ///z size
        LiteralInteger,
    ),
    InputPoints,
    InputLines,
    InputLinesAdjacency,
    Triangles,
    InputTrianglesAdjacency,
    Quads,
    Isolines,
    OutputVertices(
        ///Vertex count
        LiteralInteger,
    ),
    OutputPoints,
    OutputLineStrip,
    OutputTriangleStrip,
    VecTypeHint(
        ///Vector type
        LiteralInteger,
    ),
    ContractionOff,
    ///Since SPIR-V 1.1
    Initializer,
    ///Since SPIR-V 1.1
    Finalizer,
    ///Since SPIR-V 1.1
    SubgroupSize(
        ///Subgroup Size
        LiteralInteger,
    ),
    ///Since SPIR-V 1.1
    SubgroupsPerWorkgroup(
        ///Subgroups Per Workgroup
        LiteralInteger,
    ),
    ///Since SPIR-V 1.2
    SubgroupsPerWorkgroupId(
        ///Subgroups Per Workgroup
        IdRef,
    ),
    ///Since SPIR-V 1.2
    LocalSizeId(
        ///x size
        IdRef,
        ///y size
        IdRef,
        ///z size
        IdRef,
    ),
    ///Since SPIR-V 1.2
    LocalSizeHintId(
        ///x size hint
        IdRef,
        ///y size hint
        IdRef,
        ///z size hint
        IdRef,
    ),
    NonCoherentColorAttachmentReadEXT,
    NonCoherentDepthAttachmentReadEXT,
    NonCoherentStencilAttachmentReadEXT,
    SubgroupUniformControlFlowKHR,
    PostDepthCoverage,
    ///Since SPIR-V 1.4
    DenormPreserve(
        ///Target Width
        LiteralInteger,
    ),
    ///Since SPIR-V 1.4
    DenormFlushToZero(
        ///Target Width
        LiteralInteger,
    ),
    ///Since SPIR-V 1.4
    SignedZeroInfNanPreserve(
        ///Target Width
        LiteralInteger,
    ),
    ///Since SPIR-V 1.4
    RoundingModeRTE(
        ///Target Width
        LiteralInteger,
    ),
    ///Since SPIR-V 1.4
    RoundingModeRTZ(
        ///Target Width
        LiteralInteger,
    ),
    NonCoherentTileAttachmentReadQCOM,
    TileShadingRateQCOM(
        ///x rate
        LiteralInteger,
        ///y rate
        LiteralInteger,
        ///z rate
        LiteralInteger,
    ),
    EarlyAndLateFragmentTestsAMD,
    StencilRefReplacingEXT,
    CoalescingAMDX,
    IsApiEntryAMDX(
        ///Is Entry
        IdRef,
    ),
    MaxNodeRecursionAMDX(
        ///Number of recursions
        IdRef,
    ),
    StaticNumWorkgroupsAMDX(
        ///x size
        IdRef,
        ///y size
        IdRef,
        ///z size
        IdRef,
    ),
    ShaderIndexAMDX(
        ///Shader Index
        IdRef,
    ),
    MaxNumWorkgroupsAMDX(
        ///x size
        IdRef,
        ///y size
        IdRef,
        ///z size
        IdRef,
    ),
    StencilRefUnchangedFrontAMD,
    StencilRefGreaterFrontAMD,
    StencilRefLessFrontAMD,
    StencilRefUnchangedBackAMD,
    StencilRefGreaterBackAMD,
    StencilRefLessBackAMD,
    QuadDerivativesKHR,
    RequireFullQuadsKHR,
    SharesInputWithAMDX(
        ///Node Name
        IdRef,
        ///Shader Index
        IdRef,
    ),
    OutputLinesEXT,
    OutputPrimitivesEXT(
        ///Primitive count
        LiteralInteger,
    ),
    DerivativeGroupQuadsKHR,
    DerivativeGroupLinearKHR,
    OutputTrianglesEXT,
    PixelInterlockOrderedEXT,
    PixelInterlockUnorderedEXT,
    SampleInterlockOrderedEXT,
    SampleInterlockUnorderedEXT,
    ShadingRateInterlockOrderedEXT,
    ShadingRateInterlockUnorderedEXT,
    Shader64BitIndexingEXT,
    SharedLocalMemorySizeINTEL(
        ///Size
        LiteralInteger,
    ),
    RoundingModeRTPINTEL(
        ///Target Width
        LiteralInteger,
    ),
    RoundingModeRTNINTEL(
        ///Target Width
        LiteralInteger,
    ),
    FloatingPointModeALTINTEL(
        ///Target Width
        LiteralInteger,
    ),
    FloatingPointModeIEEEINTEL(
        ///Target Width
        LiteralInteger,
    ),
    MaxWorkgroupSizeINTEL(
        ///max_x_size
        LiteralInteger,
        ///max_y_size
        LiteralInteger,
        ///max_z_size
        LiteralInteger,
    ),
    MaxWorkDimINTEL(
        ///max_dimensions
        LiteralInteger,
    ),
    NoGlobalOffsetINTEL,
    NumSIMDWorkitemsINTEL(
        ///vector_width
        LiteralInteger,
    ),
    SchedulerTargetFmaxMhzINTEL(
        ///target_fmax
        LiteralInteger,
    ),
    MaximallyReconvergesKHR,
    FPFastMathDefault(
        ///Target Type
        IdRef,
        ///Fast-Math Mode
        IdRef,
    ),
    StreamingInterfaceINTEL(
        ///StallFreeReturn
        LiteralInteger,
    ),
    RegisterMapInterfaceINTEL(
        ///WaitForDoneWrite
        LiteralInteger,
    ),
    NamedBarrierCountINTEL(
        ///Barrier Count
        LiteralInteger,
    ),
    MaximumRegistersINTEL(
        ///Number of Registers
        LiteralInteger,
    ),
    MaximumRegistersIdINTEL(
        ///Number of Registers
        IdRef,
    ),
    NamedMaximumRegistersINTEL(
        ///Named Maximum Number of Registers
        NamedMaximumNumberOfRegisters,
    ),
}
unsafe impl Operand for ExecutionMode {
    const KIND: &OperandKind = &OPERAND_KIND_EXECUTION_MODE;
}
unsafe impl OperandEncoding for ExecutionMode {
    const FIXED_LEN: Option<usize> = None;
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        match self {
            Self::Invocations(p0) => {
                writer.write(Word(0u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::SpacingEqual => writer.write(Word(1u32)),
            Self::SpacingFractionalEven => writer.write(Word(2u32)),
            Self::SpacingFractionalOdd => writer.write(Word(3u32)),
            Self::VertexOrderCw => writer.write(Word(4u32)),
            Self::VertexOrderCcw => writer.write(Word(5u32)),
            Self::PixelCenterInteger => writer.write(Word(6u32)),
            Self::OriginUpperLeft => writer.write(Word(7u32)),
            Self::OriginLowerLeft => writer.write(Word(8u32)),
            Self::EarlyFragmentTests => writer.write(Word(9u32)),
            Self::PointMode => writer.write(Word(10u32)),
            Self::Xfb => writer.write(Word(11u32)),
            Self::DepthReplacing => writer.write(Word(12u32)),
            Self::DepthGreater => writer.write(Word(14u32)),
            Self::DepthLess => writer.write(Word(15u32)),
            Self::DepthUnchanged => writer.write(Word(16u32)),
            Self::LocalSize(p0, p1, p2) => {
                writer.write(Word(17u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?;
                OperandEncoding::encode(p2, &mut *writer)?
            }
            Self::LocalSizeHint(p0, p1, p2) => {
                writer.write(Word(18u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?;
                OperandEncoding::encode(p2, &mut *writer)?
            }
            Self::InputPoints => writer.write(Word(19u32)),
            Self::InputLines => writer.write(Word(20u32)),
            Self::InputLinesAdjacency => writer.write(Word(21u32)),
            Self::Triangles => writer.write(Word(22u32)),
            Self::InputTrianglesAdjacency => writer.write(Word(23u32)),
            Self::Quads => writer.write(Word(24u32)),
            Self::Isolines => writer.write(Word(25u32)),
            Self::OutputVertices(p0) => {
                writer.write(Word(26u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::OutputPoints => writer.write(Word(27u32)),
            Self::OutputLineStrip => writer.write(Word(28u32)),
            Self::OutputTriangleStrip => writer.write(Word(29u32)),
            Self::VecTypeHint(p0) => {
                writer.write(Word(30u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::ContractionOff => writer.write(Word(31u32)),
            Self::Initializer => writer.write(Word(33u32)),
            Self::Finalizer => writer.write(Word(34u32)),
            Self::SubgroupSize(p0) => {
                writer.write(Word(35u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::SubgroupsPerWorkgroup(p0) => {
                writer.write(Word(36u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::SubgroupsPerWorkgroupId(p0) => {
                writer.write(Word(37u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::LocalSizeId(p0, p1, p2) => {
                writer.write(Word(38u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?;
                OperandEncoding::encode(p2, &mut *writer)?
            }
            Self::LocalSizeHintId(p0, p1, p2) => {
                writer.write(Word(39u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?;
                OperandEncoding::encode(p2, &mut *writer)?
            }
            Self::NonCoherentColorAttachmentReadEXT => writer.write(Word(4169u32)),
            Self::NonCoherentDepthAttachmentReadEXT => writer.write(Word(4170u32)),
            Self::NonCoherentStencilAttachmentReadEXT => writer.write(Word(4171u32)),
            Self::SubgroupUniformControlFlowKHR => writer.write(Word(4421u32)),
            Self::PostDepthCoverage => writer.write(Word(4446u32)),
            Self::DenormPreserve(p0) => {
                writer.write(Word(4459u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::DenormFlushToZero(p0) => {
                writer.write(Word(4460u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::SignedZeroInfNanPreserve(p0) => {
                writer.write(Word(4461u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::RoundingModeRTE(p0) => {
                writer.write(Word(4462u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::RoundingModeRTZ(p0) => {
                writer.write(Word(4463u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::NonCoherentTileAttachmentReadQCOM => writer.write(Word(4489u32)),
            Self::TileShadingRateQCOM(p0, p1, p2) => {
                writer.write(Word(4490u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?;
                OperandEncoding::encode(p2, &mut *writer)?
            }
            Self::EarlyAndLateFragmentTestsAMD => writer.write(Word(5017u32)),
            Self::StencilRefReplacingEXT => writer.write(Word(5027u32)),
            Self::CoalescingAMDX => writer.write(Word(5069u32)),
            Self::IsApiEntryAMDX(p0) => {
                writer.write(Word(5070u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MaxNodeRecursionAMDX(p0) => {
                writer.write(Word(5071u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::StaticNumWorkgroupsAMDX(p0, p1, p2) => {
                writer.write(Word(5072u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?;
                OperandEncoding::encode(p2, &mut *writer)?
            }
            Self::ShaderIndexAMDX(p0) => {
                writer.write(Word(5073u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MaxNumWorkgroupsAMDX(p0, p1, p2) => {
                writer.write(Word(5077u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?;
                OperandEncoding::encode(p2, &mut *writer)?
            }
            Self::StencilRefUnchangedFrontAMD => writer.write(Word(5079u32)),
            Self::StencilRefGreaterFrontAMD => writer.write(Word(5080u32)),
            Self::StencilRefLessFrontAMD => writer.write(Word(5081u32)),
            Self::StencilRefUnchangedBackAMD => writer.write(Word(5082u32)),
            Self::StencilRefGreaterBackAMD => writer.write(Word(5083u32)),
            Self::StencilRefLessBackAMD => writer.write(Word(5084u32)),
            Self::QuadDerivativesKHR => writer.write(Word(5088u32)),
            Self::RequireFullQuadsKHR => writer.write(Word(5089u32)),
            Self::SharesInputWithAMDX(p0, p1) => {
                writer.write(Word(5102u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
            Self::OutputLinesEXT => writer.write(Word(5269u32)),
            Self::OutputPrimitivesEXT(p0) => {
                writer.write(Word(5270u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::DerivativeGroupQuadsKHR => writer.write(Word(5289u32)),
            Self::DerivativeGroupLinearKHR => writer.write(Word(5290u32)),
            Self::OutputTrianglesEXT => writer.write(Word(5298u32)),
            Self::PixelInterlockOrderedEXT => writer.write(Word(5366u32)),
            Self::PixelInterlockUnorderedEXT => writer.write(Word(5367u32)),
            Self::SampleInterlockOrderedEXT => writer.write(Word(5368u32)),
            Self::SampleInterlockUnorderedEXT => writer.write(Word(5369u32)),
            Self::ShadingRateInterlockOrderedEXT => writer.write(Word(5370u32)),
            Self::ShadingRateInterlockUnorderedEXT => writer.write(Word(5371u32)),
            Self::Shader64BitIndexingEXT => writer.write(Word(5427u32)),
            Self::SharedLocalMemorySizeINTEL(p0) => {
                writer.write(Word(5618u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::RoundingModeRTPINTEL(p0) => {
                writer.write(Word(5620u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::RoundingModeRTNINTEL(p0) => {
                writer.write(Word(5621u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::FloatingPointModeALTINTEL(p0) => {
                writer.write(Word(5622u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::FloatingPointModeIEEEINTEL(p0) => {
                writer.write(Word(5623u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MaxWorkgroupSizeINTEL(p0, p1, p2) => {
                writer.write(Word(5893u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?;
                OperandEncoding::encode(p2, &mut *writer)?
            }
            Self::MaxWorkDimINTEL(p0) => {
                writer.write(Word(5894u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::NoGlobalOffsetINTEL => writer.write(Word(5895u32)),
            Self::NumSIMDWorkitemsINTEL(p0) => {
                writer.write(Word(5896u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::SchedulerTargetFmaxMhzINTEL(p0) => {
                writer.write(Word(5903u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MaximallyReconvergesKHR => writer.write(Word(6023u32)),
            Self::FPFastMathDefault(p0, p1) => {
                writer.write(Word(6028u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
            Self::StreamingInterfaceINTEL(p0) => {
                writer.write(Word(6154u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::RegisterMapInterfaceINTEL(p0) => {
                writer.write(Word(6160u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::NamedBarrierCountINTEL(p0) => {
                writer.write(Word(6417u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MaximumRegistersINTEL(p0) => {
                writer.write(Word(6461u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MaximumRegistersIdINTEL(p0) => {
                writer.write(Word(6462u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::NamedMaximumRegistersINTEL(p0) => {
                writer.write(Word(6463u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
        }
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Invocations(OperandEncoding::decode(&mut *reader)?),
            1u32 => Self::SpacingEqual,
            2u32 => Self::SpacingFractionalEven,
            3u32 => Self::SpacingFractionalOdd,
            4u32 => Self::VertexOrderCw,
            5u32 => Self::VertexOrderCcw,
            6u32 => Self::PixelCenterInteger,
            7u32 => Self::OriginUpperLeft,
            8u32 => Self::OriginLowerLeft,
            9u32 => Self::EarlyFragmentTests,
            10u32 => Self::PointMode,
            11u32 => Self::Xfb,
            12u32 => Self::DepthReplacing,
            14u32 => Self::DepthGreater,
            15u32 => Self::DepthLess,
            16u32 => Self::DepthUnchanged,
            17u32 => Self::LocalSize(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            18u32 => Self::LocalSizeHint(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            19u32 => Self::InputPoints,
            20u32 => Self::InputLines,
            21u32 => Self::InputLinesAdjacency,
            22u32 => Self::Triangles,
            23u32 => Self::InputTrianglesAdjacency,
            24u32 => Self::Quads,
            25u32 => Self::Isolines,
            26u32 => Self::OutputVertices(OperandEncoding::decode(&mut *reader)?),
            27u32 => Self::OutputPoints,
            28u32 => Self::OutputLineStrip,
            29u32 => Self::OutputTriangleStrip,
            30u32 => Self::VecTypeHint(OperandEncoding::decode(&mut *reader)?),
            31u32 => Self::ContractionOff,
            33u32 => Self::Initializer,
            34u32 => Self::Finalizer,
            35u32 => Self::SubgroupSize(OperandEncoding::decode(&mut *reader)?),
            36u32 => Self::SubgroupsPerWorkgroup(OperandEncoding::decode(&mut *reader)?),
            37u32 => Self::SubgroupsPerWorkgroupId(OperandEncoding::decode(&mut *reader)?),
            38u32 => Self::LocalSizeId(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            39u32 => Self::LocalSizeHintId(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            4169u32 => Self::NonCoherentColorAttachmentReadEXT,
            4170u32 => Self::NonCoherentDepthAttachmentReadEXT,
            4171u32 => Self::NonCoherentStencilAttachmentReadEXT,
            4421u32 => Self::SubgroupUniformControlFlowKHR,
            4446u32 => Self::PostDepthCoverage,
            4459u32 => Self::DenormPreserve(OperandEncoding::decode(&mut *reader)?),
            4460u32 => Self::DenormFlushToZero(OperandEncoding::decode(&mut *reader)?),
            4461u32 => Self::SignedZeroInfNanPreserve(OperandEncoding::decode(&mut *reader)?),
            4462u32 => Self::RoundingModeRTE(OperandEncoding::decode(&mut *reader)?),
            4463u32 => Self::RoundingModeRTZ(OperandEncoding::decode(&mut *reader)?),
            4489u32 => Self::NonCoherentTileAttachmentReadQCOM,
            4490u32 => Self::TileShadingRateQCOM(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            5017u32 => Self::EarlyAndLateFragmentTestsAMD,
            5027u32 => Self::StencilRefReplacingEXT,
            5069u32 => Self::CoalescingAMDX,
            5070u32 => Self::IsApiEntryAMDX(OperandEncoding::decode(&mut *reader)?),
            5071u32 => Self::MaxNodeRecursionAMDX(OperandEncoding::decode(&mut *reader)?),
            5072u32 => Self::StaticNumWorkgroupsAMDX(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            5073u32 => Self::ShaderIndexAMDX(OperandEncoding::decode(&mut *reader)?),
            5077u32 => Self::MaxNumWorkgroupsAMDX(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            5079u32 => Self::StencilRefUnchangedFrontAMD,
            5080u32 => Self::StencilRefGreaterFrontAMD,
            5081u32 => Self::StencilRefLessFrontAMD,
            5082u32 => Self::StencilRefUnchangedBackAMD,
            5083u32 => Self::StencilRefGreaterBackAMD,
            5084u32 => Self::StencilRefLessBackAMD,
            5088u32 => Self::QuadDerivativesKHR,
            5089u32 => Self::RequireFullQuadsKHR,
            5102u32 => Self::SharesInputWithAMDX(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            5269u32 => Self::OutputLinesEXT,
            5270u32 => Self::OutputPrimitivesEXT(OperandEncoding::decode(&mut *reader)?),
            5289u32 => Self::DerivativeGroupQuadsKHR,
            5290u32 => Self::DerivativeGroupLinearKHR,
            5298u32 => Self::OutputTrianglesEXT,
            5366u32 => Self::PixelInterlockOrderedEXT,
            5367u32 => Self::PixelInterlockUnorderedEXT,
            5368u32 => Self::SampleInterlockOrderedEXT,
            5369u32 => Self::SampleInterlockUnorderedEXT,
            5370u32 => Self::ShadingRateInterlockOrderedEXT,
            5371u32 => Self::ShadingRateInterlockUnorderedEXT,
            5427u32 => Self::Shader64BitIndexingEXT,
            5618u32 => Self::SharedLocalMemorySizeINTEL(OperandEncoding::decode(&mut *reader)?),
            5620u32 => Self::RoundingModeRTPINTEL(OperandEncoding::decode(&mut *reader)?),
            5621u32 => Self::RoundingModeRTNINTEL(OperandEncoding::decode(&mut *reader)?),
            5622u32 => Self::FloatingPointModeALTINTEL(OperandEncoding::decode(&mut *reader)?),
            5623u32 => Self::FloatingPointModeIEEEINTEL(OperandEncoding::decode(&mut *reader)?),
            5893u32 => Self::MaxWorkgroupSizeINTEL(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            5894u32 => Self::MaxWorkDimINTEL(OperandEncoding::decode(&mut *reader)?),
            5895u32 => Self::NoGlobalOffsetINTEL,
            5896u32 => Self::NumSIMDWorkitemsINTEL(OperandEncoding::decode(&mut *reader)?),
            5903u32 => Self::SchedulerTargetFmaxMhzINTEL(OperandEncoding::decode(&mut *reader)?),
            6023u32 => Self::MaximallyReconvergesKHR,
            6028u32 => Self::FPFastMathDefault(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            6154u32 => Self::StreamingInterfaceINTEL(OperandEncoding::decode(&mut *reader)?),
            6160u32 => Self::RegisterMapInterfaceINTEL(OperandEncoding::decode(&mut *reader)?),
            6417u32 => Self::NamedBarrierCountINTEL(OperandEncoding::decode(&mut *reader)?),
            6461u32 => Self::MaximumRegistersINTEL(OperandEncoding::decode(&mut *reader)?),
            6462u32 => Self::MaximumRegistersIdINTEL(OperandEncoding::decode(&mut *reader)?),
            6463u32 => Self::NamedMaximumRegistersINTEL(OperandEncoding::decode(&mut *reader)?),
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(ExecutionMode),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        match self {
            Self::Invocations(p0) => write!(f, " Invocations{}", p0.dis(_ctx)),
            Self::SpacingEqual => write!(f, " SpacingEqual"),
            Self::SpacingFractionalEven => write!(f, " SpacingFractionalEven"),
            Self::SpacingFractionalOdd => write!(f, " SpacingFractionalOdd"),
            Self::VertexOrderCw => write!(f, " VertexOrderCw"),
            Self::VertexOrderCcw => write!(f, " VertexOrderCcw"),
            Self::PixelCenterInteger => write!(f, " PixelCenterInteger"),
            Self::OriginUpperLeft => write!(f, " OriginUpperLeft"),
            Self::OriginLowerLeft => write!(f, " OriginLowerLeft"),
            Self::EarlyFragmentTests => write!(f, " EarlyFragmentTests"),
            Self::PointMode => write!(f, " PointMode"),
            Self::Xfb => write!(f, " Xfb"),
            Self::DepthReplacing => write!(f, " DepthReplacing"),
            Self::DepthGreater => write!(f, " DepthGreater"),
            Self::DepthLess => write!(f, " DepthLess"),
            Self::DepthUnchanged => write!(f, " DepthUnchanged"),
            Self::LocalSize(p0, p1, p2) => {
                write!(
                    f,
                    " LocalSize{}{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx),
                    p2.dis(_ctx)
                )
            }
            Self::LocalSizeHint(p0, p1, p2) => {
                write!(
                    f,
                    " LocalSizeHint{}{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx),
                    p2.dis(_ctx)
                )
            }
            Self::InputPoints => write!(f, " InputPoints"),
            Self::InputLines => write!(f, " InputLines"),
            Self::InputLinesAdjacency => write!(f, " InputLinesAdjacency"),
            Self::Triangles => write!(f, " Triangles"),
            Self::InputTrianglesAdjacency => write!(f, " InputTrianglesAdjacency"),
            Self::Quads => write!(f, " Quads"),
            Self::Isolines => write!(f, " Isolines"),
            Self::OutputVertices(p0) => write!(f, " OutputVertices{}", p0.dis(_ctx)),
            Self::OutputPoints => write!(f, " OutputPoints"),
            Self::OutputLineStrip => write!(f, " OutputLineStrip"),
            Self::OutputTriangleStrip => write!(f, " OutputTriangleStrip"),
            Self::VecTypeHint(p0) => write!(f, " VecTypeHint{}", p0.dis(_ctx)),
            Self::ContractionOff => write!(f, " ContractionOff"),
            Self::Initializer => write!(f, " Initializer"),
            Self::Finalizer => write!(f, " Finalizer"),
            Self::SubgroupSize(p0) => write!(f, " SubgroupSize{}", p0.dis(_ctx)),
            Self::SubgroupsPerWorkgroup(p0) => {
                write!(f, " SubgroupsPerWorkgroup{}", p0.dis(_ctx))
            }
            Self::SubgroupsPerWorkgroupId(p0) => {
                write!(f, " SubgroupsPerWorkgroupId{}", p0.dis(_ctx))
            }
            Self::LocalSizeId(p0, p1, p2) => {
                write!(
                    f,
                    " LocalSizeId{}{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx),
                    p2.dis(_ctx)
                )
            }
            Self::LocalSizeHintId(p0, p1, p2) => {
                write!(
                    f,
                    " LocalSizeHintId{}{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx),
                    p2.dis(_ctx)
                )
            }
            Self::NonCoherentColorAttachmentReadEXT => {
                write!(f, " NonCoherentColorAttachmentReadEXT")
            }
            Self::NonCoherentDepthAttachmentReadEXT => {
                write!(f, " NonCoherentDepthAttachmentReadEXT")
            }
            Self::NonCoherentStencilAttachmentReadEXT => {
                write!(f, " NonCoherentStencilAttachmentReadEXT")
            }
            Self::SubgroupUniformControlFlowKHR => {
                write!(f, " SubgroupUniformControlFlowKHR")
            }
            Self::PostDepthCoverage => write!(f, " PostDepthCoverage"),
            Self::DenormPreserve(p0) => write!(f, " DenormPreserve{}", p0.dis(_ctx)),
            Self::DenormFlushToZero(p0) => {
                write!(f, " DenormFlushToZero{}", p0.dis(_ctx))
            }
            Self::SignedZeroInfNanPreserve(p0) => {
                write!(f, " SignedZeroInfNanPreserve{}", p0.dis(_ctx))
            }
            Self::RoundingModeRTE(p0) => write!(f, " RoundingModeRTE{}", p0.dis(_ctx)),
            Self::RoundingModeRTZ(p0) => write!(f, " RoundingModeRTZ{}", p0.dis(_ctx)),
            Self::NonCoherentTileAttachmentReadQCOM => {
                write!(f, " NonCoherentTileAttachmentReadQCOM")
            }
            Self::TileShadingRateQCOM(p0, p1, p2) => {
                write!(
                    f,
                    " TileShadingRateQCOM{}{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx),
                    p2.dis(_ctx)
                )
            }
            Self::EarlyAndLateFragmentTestsAMD => {
                write!(f, " EarlyAndLateFragmentTestsAMD")
            }
            Self::StencilRefReplacingEXT => write!(f, " StencilRefReplacingEXT"),
            Self::CoalescingAMDX => write!(f, " CoalescingAMDX"),
            Self::IsApiEntryAMDX(p0) => write!(f, " IsApiEntryAMDX{}", p0.dis(_ctx)),
            Self::MaxNodeRecursionAMDX(p0) => {
                write!(f, " MaxNodeRecursionAMDX{}", p0.dis(_ctx))
            }
            Self::StaticNumWorkgroupsAMDX(p0, p1, p2) => {
                write!(
                    f,
                    " StaticNumWorkgroupsAMDX{}{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx),
                    p2.dis(_ctx)
                )
            }
            Self::ShaderIndexAMDX(p0) => write!(f, " ShaderIndexAMDX{}", p0.dis(_ctx)),
            Self::MaxNumWorkgroupsAMDX(p0, p1, p2) => {
                write!(
                    f,
                    " MaxNumWorkgroupsAMDX{}{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx),
                    p2.dis(_ctx)
                )
            }
            Self::StencilRefUnchangedFrontAMD => {
                write!(f, " StencilRefUnchangedFrontAMD")
            }
            Self::StencilRefGreaterFrontAMD => write!(f, " StencilRefGreaterFrontAMD"),
            Self::StencilRefLessFrontAMD => write!(f, " StencilRefLessFrontAMD"),
            Self::StencilRefUnchangedBackAMD => write!(f, " StencilRefUnchangedBackAMD"),
            Self::StencilRefGreaterBackAMD => write!(f, " StencilRefGreaterBackAMD"),
            Self::StencilRefLessBackAMD => write!(f, " StencilRefLessBackAMD"),
            Self::QuadDerivativesKHR => write!(f, " QuadDerivativesKHR"),
            Self::RequireFullQuadsKHR => write!(f, " RequireFullQuadsKHR"),
            Self::SharesInputWithAMDX(p0, p1) => {
                write!(f, " SharesInputWithAMDX{}{}", p0.dis(_ctx), p1.dis(_ctx))
            }
            Self::OutputLinesEXT => write!(f, " OutputLinesEXT"),
            Self::OutputPrimitivesEXT(p0) => {
                write!(f, " OutputPrimitivesEXT{}", p0.dis(_ctx))
            }
            Self::DerivativeGroupQuadsKHR => write!(f, " DerivativeGroupQuadsKHR"),
            Self::DerivativeGroupLinearKHR => write!(f, " DerivativeGroupLinearKHR"),
            Self::OutputTrianglesEXT => write!(f, " OutputTrianglesEXT"),
            Self::PixelInterlockOrderedEXT => write!(f, " PixelInterlockOrderedEXT"),
            Self::PixelInterlockUnorderedEXT => write!(f, " PixelInterlockUnorderedEXT"),
            Self::SampleInterlockOrderedEXT => write!(f, " SampleInterlockOrderedEXT"),
            Self::SampleInterlockUnorderedEXT => {
                write!(f, " SampleInterlockUnorderedEXT")
            }
            Self::ShadingRateInterlockOrderedEXT => {
                write!(f, " ShadingRateInterlockOrderedEXT")
            }
            Self::ShadingRateInterlockUnorderedEXT => {
                write!(f, " ShadingRateInterlockUnorderedEXT")
            }
            Self::Shader64BitIndexingEXT => write!(f, " Shader64BitIndexingEXT"),
            Self::SharedLocalMemorySizeINTEL(p0) => {
                write!(f, " SharedLocalMemorySizeINTEL{}", p0.dis(_ctx))
            }
            Self::RoundingModeRTPINTEL(p0) => {
                write!(f, " RoundingModeRTPINTEL{}", p0.dis(_ctx))
            }
            Self::RoundingModeRTNINTEL(p0) => {
                write!(f, " RoundingModeRTNINTEL{}", p0.dis(_ctx))
            }
            Self::FloatingPointModeALTINTEL(p0) => {
                write!(f, " FloatingPointModeALTINTEL{}", p0.dis(_ctx))
            }
            Self::FloatingPointModeIEEEINTEL(p0) => {
                write!(f, " FloatingPointModeIEEEINTEL{}", p0.dis(_ctx))
            }
            Self::MaxWorkgroupSizeINTEL(p0, p1, p2) => {
                write!(
                    f,
                    " MaxWorkgroupSizeINTEL{}{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx),
                    p2.dis(_ctx)
                )
            }
            Self::MaxWorkDimINTEL(p0) => write!(f, " MaxWorkDimINTEL{}", p0.dis(_ctx)),
            Self::NoGlobalOffsetINTEL => write!(f, " NoGlobalOffsetINTEL"),
            Self::NumSIMDWorkitemsINTEL(p0) => {
                write!(f, " NumSIMDWorkitemsINTEL{}", p0.dis(_ctx))
            }
            Self::SchedulerTargetFmaxMhzINTEL(p0) => {
                write!(f, " SchedulerTargetFmaxMhzINTEL{}", p0.dis(_ctx))
            }
            Self::MaximallyReconvergesKHR => write!(f, " MaximallyReconvergesKHR"),
            Self::FPFastMathDefault(p0, p1) => {
                write!(f, " FPFastMathDefault{}{}", p0.dis(_ctx), p1.dis(_ctx))
            }
            Self::StreamingInterfaceINTEL(p0) => {
                write!(f, " StreamingInterfaceINTEL{}", p0.dis(_ctx))
            }
            Self::RegisterMapInterfaceINTEL(p0) => {
                write!(f, " RegisterMapInterfaceINTEL{}", p0.dis(_ctx))
            }
            Self::NamedBarrierCountINTEL(p0) => {
                write!(f, " NamedBarrierCountINTEL{}", p0.dis(_ctx))
            }
            Self::MaximumRegistersINTEL(p0) => {
                write!(f, " MaximumRegistersINTEL{}", p0.dis(_ctx))
            }
            Self::MaximumRegistersIdINTEL(p0) => {
                write!(f, " MaximumRegistersIdINTEL{}", p0.dis(_ctx))
            }
            Self::NamedMaximumRegistersINTEL(p0) => {
                write!(f, " NamedMaximumRegistersINTEL{}", p0.dis(_ctx))
            }
        }
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StorageClass {
    UniformConstant = 0u32,
    Input = 1u32,
    Uniform = 2u32,
    Output = 3u32,
    Workgroup = 4u32,
    CrossWorkgroup = 5u32,
    Private = 6u32,
    Function = 7u32,
    Generic = 8u32,
    PushConstant = 9u32,
    AtomicCounter = 10u32,
    Image = 11u32,
    ///Since SPIR-V 1.3
    StorageBuffer = 12u32,
    TileImageEXT = 4172u32,
    TileAttachmentQCOM = 4491u32,
    NodePayloadAMDX = 5068u32,
    CallableDataKHR = 5328u32,
    IncomingCallableDataKHR = 5329u32,
    RayPayloadKHR = 5338u32,
    HitAttributeKHR = 5339u32,
    IncomingRayPayloadKHR = 5342u32,
    ShaderRecordBufferKHR = 5343u32,
    ///Since SPIR-V 1.5
    PhysicalStorageBuffer = 5349u32,
    HitObjectAttributeNV = 5385u32,
    ///Since SPIR-V 1.4
    TaskPayloadWorkgroupEXT = 5402u32,
    HitObjectAttributeEXT = 5411u32,
    CodeSectionINTEL = 5605u32,
    DeviceOnlyALTERA = 5936u32,
    HostOnlyALTERA = 5937u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for StorageClass {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for StorageClass {}
unsafe impl Operand for StorageClass {
    const KIND: &OperandKind = &OPERAND_KIND_STORAGE_CLASS;
}
unsafe impl OperandEncoding for StorageClass {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::UniformConstant,
            1u32 => Self::Input,
            2u32 => Self::Uniform,
            3u32 => Self::Output,
            4u32 => Self::Workgroup,
            5u32 => Self::CrossWorkgroup,
            6u32 => Self::Private,
            7u32 => Self::Function,
            8u32 => Self::Generic,
            9u32 => Self::PushConstant,
            10u32 => Self::AtomicCounter,
            11u32 => Self::Image,
            12u32 => Self::StorageBuffer,
            4172u32 => Self::TileImageEXT,
            4491u32 => Self::TileAttachmentQCOM,
            5068u32 => Self::NodePayloadAMDX,
            5328u32 => Self::CallableDataKHR,
            5329u32 => Self::IncomingCallableDataKHR,
            5338u32 => Self::RayPayloadKHR,
            5339u32 => Self::HitAttributeKHR,
            5342u32 => Self::IncomingRayPayloadKHR,
            5343u32 => Self::ShaderRecordBufferKHR,
            5349u32 => Self::PhysicalStorageBuffer,
            5385u32 => Self::HitObjectAttributeNV,
            5402u32 => Self::TaskPayloadWorkgroupEXT,
            5411u32 => Self::HitObjectAttributeEXT,
            5605u32 => Self::CodeSectionINTEL,
            5936u32 => Self::DeviceOnlyALTERA,
            5937u32 => Self::HostOnlyALTERA,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(StorageClass),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Dim {
    Dim1D = 0u32,
    Dim2D = 1u32,
    Dim3D = 2u32,
    Cube = 3u32,
    Rect = 4u32,
    Buffer = 5u32,
    SubpassData = 6u32,
    TileImageDataEXT = 4173u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Dim {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Dim {}
unsafe impl Operand for Dim {
    const KIND: &OperandKind = &OPERAND_KIND_DIM;
}
unsafe impl OperandEncoding for Dim {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Dim1D,
            1u32 => Self::Dim2D,
            2u32 => Self::Dim3D,
            3u32 => Self::Cube,
            4u32 => Self::Rect,
            5u32 => Self::Buffer,
            6u32 => Self::SubpassData,
            4173u32 => Self::TileImageDataEXT,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(Dim),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SamplerAddressingMode {
    None = 0u32,
    ClampToEdge = 1u32,
    Clamp = 2u32,
    Repeat = 3u32,
    RepeatMirrored = 4u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for SamplerAddressingMode {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for SamplerAddressingMode {}
unsafe impl Operand for SamplerAddressingMode {
    const KIND: &OperandKind = &OPERAND_KIND_SAMPLER_ADDRESSING_MODE;
}
unsafe impl OperandEncoding for SamplerAddressingMode {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::None,
            1u32 => Self::ClampToEdge,
            2u32 => Self::Clamp,
            3u32 => Self::Repeat,
            4u32 => Self::RepeatMirrored,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(SamplerAddressingMode),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SamplerFilterMode {
    Nearest = 0u32,
    Linear = 1u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for SamplerFilterMode {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for SamplerFilterMode {}
unsafe impl Operand for SamplerFilterMode {
    const KIND: &OperandKind = &OPERAND_KIND_SAMPLER_FILTER_MODE;
}
unsafe impl OperandEncoding for SamplerFilterMode {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Nearest,
            1u32 => Self::Linear,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(SamplerFilterMode),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ImageFormat {
    Unknown = 0u32,
    Rgba32f = 1u32,
    Rgba16f = 2u32,
    R32f = 3u32,
    Rgba8 = 4u32,
    Rgba8Snorm = 5u32,
    Rg32f = 6u32,
    Rg16f = 7u32,
    R11fG11fB10f = 8u32,
    R16f = 9u32,
    Rgba16 = 10u32,
    Rgb10A2 = 11u32,
    Rg16 = 12u32,
    Rg8 = 13u32,
    R16 = 14u32,
    R8 = 15u32,
    Rgba16Snorm = 16u32,
    Rg16Snorm = 17u32,
    Rg8Snorm = 18u32,
    R16Snorm = 19u32,
    R8Snorm = 20u32,
    Rgba32i = 21u32,
    Rgba16i = 22u32,
    Rgba8i = 23u32,
    R32i = 24u32,
    Rg32i = 25u32,
    Rg16i = 26u32,
    Rg8i = 27u32,
    R16i = 28u32,
    R8i = 29u32,
    Rgba32ui = 30u32,
    Rgba16ui = 31u32,
    Rgba8ui = 32u32,
    R32ui = 33u32,
    Rgb10a2ui = 34u32,
    Rg32ui = 35u32,
    Rg16ui = 36u32,
    Rg8ui = 37u32,
    R16ui = 38u32,
    R8ui = 39u32,
    R64ui = 40u32,
    R64i = 41u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for ImageFormat {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for ImageFormat {}
unsafe impl Operand for ImageFormat {
    const KIND: &OperandKind = &OPERAND_KIND_IMAGE_FORMAT;
}
unsafe impl OperandEncoding for ImageFormat {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Unknown,
            1u32 => Self::Rgba32f,
            2u32 => Self::Rgba16f,
            3u32 => Self::R32f,
            4u32 => Self::Rgba8,
            5u32 => Self::Rgba8Snorm,
            6u32 => Self::Rg32f,
            7u32 => Self::Rg16f,
            8u32 => Self::R11fG11fB10f,
            9u32 => Self::R16f,
            10u32 => Self::Rgba16,
            11u32 => Self::Rgb10A2,
            12u32 => Self::Rg16,
            13u32 => Self::Rg8,
            14u32 => Self::R16,
            15u32 => Self::R8,
            16u32 => Self::Rgba16Snorm,
            17u32 => Self::Rg16Snorm,
            18u32 => Self::Rg8Snorm,
            19u32 => Self::R16Snorm,
            20u32 => Self::R8Snorm,
            21u32 => Self::Rgba32i,
            22u32 => Self::Rgba16i,
            23u32 => Self::Rgba8i,
            24u32 => Self::R32i,
            25u32 => Self::Rg32i,
            26u32 => Self::Rg16i,
            27u32 => Self::Rg8i,
            28u32 => Self::R16i,
            29u32 => Self::R8i,
            30u32 => Self::Rgba32ui,
            31u32 => Self::Rgba16ui,
            32u32 => Self::Rgba8ui,
            33u32 => Self::R32ui,
            34u32 => Self::Rgb10a2ui,
            35u32 => Self::Rg32ui,
            36u32 => Self::Rg16ui,
            37u32 => Self::Rg8ui,
            38u32 => Self::R16ui,
            39u32 => Self::R8ui,
            40u32 => Self::R64ui,
            41u32 => Self::R64i,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(ImageFormat),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ImageChannelOrder {
    R = 0u32,
    A = 1u32,
    RG = 2u32,
    RA = 3u32,
    RGB = 4u32,
    RGBA = 5u32,
    BGRA = 6u32,
    ARGB = 7u32,
    Intensity = 8u32,
    Luminance = 9u32,
    Rx = 10u32,
    RGx = 11u32,
    RGBx = 12u32,
    Depth = 13u32,
    DepthStencil = 14u32,
    sRGB = 15u32,
    sRGBx = 16u32,
    sRGBA = 17u32,
    sBGRA = 18u32,
    ABGR = 19u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for ImageChannelOrder {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for ImageChannelOrder {}
unsafe impl Operand for ImageChannelOrder {
    const KIND: &OperandKind = &OPERAND_KIND_IMAGE_CHANNEL_ORDER;
}
unsafe impl OperandEncoding for ImageChannelOrder {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::R,
            1u32 => Self::A,
            2u32 => Self::RG,
            3u32 => Self::RA,
            4u32 => Self::RGB,
            5u32 => Self::RGBA,
            6u32 => Self::BGRA,
            7u32 => Self::ARGB,
            8u32 => Self::Intensity,
            9u32 => Self::Luminance,
            10u32 => Self::Rx,
            11u32 => Self::RGx,
            12u32 => Self::RGBx,
            13u32 => Self::Depth,
            14u32 => Self::DepthStencil,
            15u32 => Self::sRGB,
            16u32 => Self::sRGBx,
            17u32 => Self::sRGBA,
            18u32 => Self::sBGRA,
            19u32 => Self::ABGR,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(ImageChannelOrder),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ImageChannelDataType {
    SnormInt8 = 0u32,
    SnormInt16 = 1u32,
    UnormInt8 = 2u32,
    UnormInt16 = 3u32,
    UnormShort565 = 4u32,
    UnormShort555 = 5u32,
    UnormInt101010 = 6u32,
    SignedInt8 = 7u32,
    SignedInt16 = 8u32,
    SignedInt32 = 9u32,
    UnsignedInt8 = 10u32,
    UnsignedInt16 = 11u32,
    UnsignedInt32 = 12u32,
    HalfFloat = 13u32,
    Float = 14u32,
    UnormInt24 = 15u32,
    UnormInt101010_2 = 16u32,
    UnormInt10X6EXT = 17u32,
    UnsignedIntRaw10EXT = 19u32,
    UnsignedIntRaw12EXT = 20u32,
    UnormInt2_101010EXT = 21u32,
    UnsignedInt10X6EXT = 22u32,
    UnsignedInt12X4EXT = 23u32,
    UnsignedInt14X2EXT = 24u32,
    UnormInt12X4EXT = 25u32,
    UnormInt14X2EXT = 26u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for ImageChannelDataType {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for ImageChannelDataType {}
unsafe impl Operand for ImageChannelDataType {
    const KIND: &OperandKind = &OPERAND_KIND_IMAGE_CHANNEL_DATA_TYPE;
}
unsafe impl OperandEncoding for ImageChannelDataType {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::SnormInt8,
            1u32 => Self::SnormInt16,
            2u32 => Self::UnormInt8,
            3u32 => Self::UnormInt16,
            4u32 => Self::UnormShort565,
            5u32 => Self::UnormShort555,
            6u32 => Self::UnormInt101010,
            7u32 => Self::SignedInt8,
            8u32 => Self::SignedInt16,
            9u32 => Self::SignedInt32,
            10u32 => Self::UnsignedInt8,
            11u32 => Self::UnsignedInt16,
            12u32 => Self::UnsignedInt32,
            13u32 => Self::HalfFloat,
            14u32 => Self::Float,
            15u32 => Self::UnormInt24,
            16u32 => Self::UnormInt101010_2,
            17u32 => Self::UnormInt10X6EXT,
            19u32 => Self::UnsignedIntRaw10EXT,
            20u32 => Self::UnsignedIntRaw12EXT,
            21u32 => Self::UnormInt2_101010EXT,
            22u32 => Self::UnsignedInt10X6EXT,
            23u32 => Self::UnsignedInt12X4EXT,
            24u32 => Self::UnsignedInt14X2EXT,
            25u32 => Self::UnormInt12X4EXT,
            26u32 => Self::UnormInt14X2EXT,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(ImageChannelDataType),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FPRoundingMode {
    RTE = 0u32,
    RTZ = 1u32,
    RTP = 2u32,
    RTN = 3u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for FPRoundingMode {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for FPRoundingMode {}
unsafe impl Operand for FPRoundingMode {
    const KIND: &OperandKind = &OPERAND_KIND_FP_ROUNDING_MODE;
}
unsafe impl OperandEncoding for FPRoundingMode {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::RTE,
            1u32 => Self::RTZ,
            2u32 => Self::RTP,
            3u32 => Self::RTN,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(FPRoundingMode),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FPDenormMode {
    Preserve = 0u32,
    FlushToZero = 1u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for FPDenormMode {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for FPDenormMode {}
unsafe impl Operand for FPDenormMode {
    const KIND: &OperandKind = &OPERAND_KIND_FP_DENORM_MODE;
}
unsafe impl OperandEncoding for FPDenormMode {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Preserve,
            1u32 => Self::FlushToZero,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(FPDenormMode),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuantizationModes {
    TRN = 0u32,
    TRN_ZERO = 1u32,
    RND = 2u32,
    RND_ZERO = 3u32,
    RND_INF = 4u32,
    RND_MIN_INF = 5u32,
    RND_CONV = 6u32,
    RND_CONV_ODD = 7u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for QuantizationModes {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for QuantizationModes {}
unsafe impl Operand for QuantizationModes {
    const KIND: &OperandKind = &OPERAND_KIND_QUANTIZATION_MODES;
}
unsafe impl OperandEncoding for QuantizationModes {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::TRN,
            1u32 => Self::TRN_ZERO,
            2u32 => Self::RND,
            3u32 => Self::RND_ZERO,
            4u32 => Self::RND_INF,
            5u32 => Self::RND_MIN_INF,
            6u32 => Self::RND_CONV,
            7u32 => Self::RND_CONV_ODD,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(QuantizationModes),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FPOperationMode {
    IEEE = 0u32,
    ALT = 1u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for FPOperationMode {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for FPOperationMode {}
unsafe impl Operand for FPOperationMode {
    const KIND: &OperandKind = &OPERAND_KIND_FP_OPERATION_MODE;
}
unsafe impl OperandEncoding for FPOperationMode {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::IEEE,
            1u32 => Self::ALT,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(FPOperationMode),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OverflowModes {
    WRAP = 0u32,
    SAT = 1u32,
    SAT_ZERO = 2u32,
    SAT_SYM = 3u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for OverflowModes {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for OverflowModes {}
unsafe impl Operand for OverflowModes {
    const KIND: &OperandKind = &OPERAND_KIND_OVERFLOW_MODES;
}
unsafe impl OperandEncoding for OverflowModes {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::WRAP,
            1u32 => Self::SAT,
            2u32 => Self::SAT_ZERO,
            3u32 => Self::SAT_SYM,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(OverflowModes),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LinkageType {
    Export = 0u32,
    Import = 1u32,
    LinkOnceODR = 2u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for LinkageType {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for LinkageType {}
unsafe impl Operand for LinkageType {
    const KIND: &OperandKind = &OPERAND_KIND_LINKAGE_TYPE;
}
unsafe impl OperandEncoding for LinkageType {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Export,
            1u32 => Self::Import,
            2u32 => Self::LinkOnceODR,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(LinkageType),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AccessQualifier {
    ReadOnly = 0u32,
    WriteOnly = 1u32,
    ReadWrite = 2u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for AccessQualifier {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for AccessQualifier {}
unsafe impl Operand for AccessQualifier {
    const KIND: &OperandKind = &OPERAND_KIND_ACCESS_QUALIFIER;
}
unsafe impl OperandEncoding for AccessQualifier {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::ReadOnly,
            1u32 => Self::WriteOnly,
            2u32 => Self::ReadWrite,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(AccessQualifier),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HostAccessQualifier {
    NoneINTEL = 0u32,
    ReadINTEL = 1u32,
    WriteINTEL = 2u32,
    ReadWriteINTEL = 3u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for HostAccessQualifier {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for HostAccessQualifier {}
unsafe impl Operand for HostAccessQualifier {
    const KIND: &OperandKind = &OPERAND_KIND_HOST_ACCESS_QUALIFIER;
}
unsafe impl OperandEncoding for HostAccessQualifier {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::NoneINTEL,
            1u32 => Self::ReadINTEL,
            2u32 => Self::WriteINTEL,
            3u32 => Self::ReadWriteINTEL,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(HostAccessQualifier),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FunctionParameterAttribute {
    Zext = 0u32,
    Sext = 1u32,
    ByVal = 2u32,
    Sret = 3u32,
    NoAlias = 4u32,
    NoCapture = 5u32,
    NoWrite = 6u32,
    NoReadWrite = 7u32,
    RuntimeAlignedALTERA = 5940u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for FunctionParameterAttribute {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for FunctionParameterAttribute {}
unsafe impl Operand for FunctionParameterAttribute {
    const KIND: &OperandKind = &OPERAND_KIND_FUNCTION_PARAMETER_ATTRIBUTE;
}
unsafe impl OperandEncoding for FunctionParameterAttribute {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Zext,
            1u32 => Self::Sext,
            2u32 => Self::ByVal,
            3u32 => Self::Sret,
            4u32 => Self::NoAlias,
            5u32 => Self::NoCapture,
            6u32 => Self::NoWrite,
            7u32 => Self::NoReadWrite,
            5940u32 => Self::RuntimeAlignedALTERA,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(FunctionParameterAttribute),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Decoration {
    RelaxedPrecision,
    SpecId(
        ///Specialization Constant ID
        LiteralInteger,
    ),
    Block,
    ///Deprecated in SPIR-V 1.3
    #[deprecated]
    BufferBlock,
    RowMajor,
    ColMajor,
    ArrayStride(
        ///Array Stride
        LiteralInteger,
    ),
    MatrixStride(
        ///Matrix Stride
        LiteralInteger,
    ),
    GLSLShared,
    GLSLPacked,
    CPacked,
    BuiltIn(BuiltIn),
    NoPerspective,
    Flat,
    Patch,
    Centroid,
    Sample,
    Invariant,
    Restrict,
    Aliased,
    Volatile,
    Constant,
    Coherent,
    NonWritable,
    NonReadable,
    Uniform,
    ///Since SPIR-V 1.4
    UniformId(
        ///Execution
        IdScope,
    ),
    SaturatedConversion,
    Stream(
        ///Stream Number
        LiteralInteger,
    ),
    Location(
        ///Location
        LiteralInteger,
    ),
    Component(
        ///Component
        LiteralInteger,
    ),
    Index(
        ///Index
        LiteralInteger,
    ),
    Binding(
        ///Binding Point
        LiteralInteger,
    ),
    DescriptorSet(
        ///Descriptor Set
        LiteralInteger,
    ),
    Offset(
        ///Byte Offset
        LiteralInteger,
    ),
    XfbBuffer(
        ///XFB Buffer Number
        LiteralInteger,
    ),
    XfbStride(
        ///XFB Stride
        LiteralInteger,
    ),
    FuncParamAttr(
        ///Function Parameter Attribute
        FunctionParameterAttribute,
    ),
    FPRoundingMode(
        ///Floating-Point Rounding Mode
        FPRoundingMode,
    ),
    FPFastMathMode(
        ///Fast-Math Mode
        FPFastMathMode,
    ),
    LinkageAttributes(
        ///Name
        LiteralString,
        ///Linkage Type
        LinkageType,
    ),
    NoContraction,
    InputAttachmentIndex(
        ///Attachment Index
        LiteralInteger,
    ),
    Alignment(
        ///Alignment
        LiteralInteger,
    ),
    ///Since SPIR-V 1.1
    MaxByteOffset(
        ///Max Byte Offset
        LiteralInteger,
    ),
    ///Since SPIR-V 1.2
    AlignmentId(
        ///Alignment
        IdRef,
    ),
    ///Since SPIR-V 1.2
    MaxByteOffsetId(
        ///Max Byte Offset
        IdRef,
    ),
    SaturatedToLargestFloat8NormalConversionEXT,
    ///Since SPIR-V 1.4
    NoSignedWrap,
    ///Since SPIR-V 1.4
    NoUnsignedWrap,
    WeightTextureQCOM,
    BlockMatchTextureQCOM,
    BlockMatchSamplerQCOM,
    ExplicitInterpAMD,
    NodeSharesPayloadLimitsWithAMDX(
        ///Payload Type
        IdRef,
    ),
    NodeMaxPayloadsAMDX(
        ///Max number of payloads
        IdRef,
    ),
    TrackFinishWritingAMDX,
    PayloadNodeNameAMDX(
        ///Node Name
        IdRef,
    ),
    PayloadNodeBaseIndexAMDX(
        ///Base Index
        IdRef,
    ),
    PayloadNodeSparseArrayAMDX,
    PayloadNodeArraySizeAMDX(
        ///Array Size
        IdRef,
    ),
    PayloadDispatchIndirectAMDX,
    ArrayStrideIdEXT(
        ///Array Stride
        IdRef,
    ),
    OffsetIdEXT(
        ///Byte Offset
        IdRef,
    ),
    OverrideCoverageNV,
    PassthroughNV,
    ViewportRelativeNV,
    SecondaryViewportRelativeNV(
        ///Offset
        LiteralInteger,
    ),
    PerPrimitiveEXT,
    PerViewNV,
    PerTaskNV,
    PerVertexKHR,
    ///Since SPIR-V 1.5
    NonUniform,
    ///Since SPIR-V 1.5
    RestrictPointer,
    ///Since SPIR-V 1.5
    AliasedPointer,
    MemberOffsetNV(
        ///memberOffset
        LiteralInteger,
    ),
    HitObjectShaderRecordBufferNV,
    HitObjectShaderRecordBufferEXT,
    BankNV(
        ///Bank
        LiteralInteger,
    ),
    BindlessSamplerNV,
    BindlessImageNV,
    BoundSamplerNV,
    BoundImageNV,
    SIMTCallINTEL(
        ///N
        LiteralInteger,
    ),
    ReferencedIndirectlyINTEL,
    ClobberINTEL(
        ///Register
        LiteralString,
    ),
    SideEffectsINTEL,
    VectorComputeVariableINTEL,
    FuncParamIOKindINTEL(
        ///Kind
        LiteralInteger,
    ),
    VectorComputeFunctionINTEL,
    StackCallINTEL,
    GlobalVariableOffsetINTEL(
        ///Offset
        LiteralInteger,
    ),
    ///Since SPIR-V 1.4
    CounterBuffer(
        ///Counter Buffer
        IdRef,
    ),
    ///Since SPIR-V 1.4
    UserSemantic(
        ///Semantic
        LiteralString,
    ),
    UserTypeGOOGLE(
        ///User Type
        LiteralString,
    ),
    FunctionRoundingModeINTEL(
        ///Target Width
        LiteralInteger,
        ///FP Rounding Mode
        FPRoundingMode,
    ),
    FunctionDenormModeINTEL(
        ///Target Width
        LiteralInteger,
        ///FP Denorm Mode
        FPDenormMode,
    ),
    RegisterALTERA,
    MemoryALTERA(
        ///Memory Type
        LiteralString,
    ),
    NumbanksALTERA(
        ///Banks
        LiteralInteger,
    ),
    BankwidthALTERA(
        ///Bank Width
        LiteralInteger,
    ),
    MaxPrivateCopiesALTERA(
        ///Maximum Copies
        LiteralInteger,
    ),
    SinglepumpALTERA,
    DoublepumpALTERA,
    MaxReplicatesALTERA(
        ///Maximum Replicates
        LiteralInteger,
    ),
    SimpleDualPortALTERA,
    MergeALTERA(
        ///Merge Key
        LiteralString,
        ///Merge Type
        LiteralString,
    ),
    BankBitsALTERA(
        ///Bank Bits
        Vec<LiteralInteger>,
    ),
    ForcePow2DepthALTERA(
        ///Force Key
        LiteralInteger,
    ),
    StridesizeALTERA(
        ///Stride Size
        LiteralInteger,
    ),
    WordsizeALTERA(
        ///Word Size
        LiteralInteger,
    ),
    TrueDualPortALTERA,
    BurstCoalesceALTERA,
    CacheSizeALTERA(
        ///Cache Size in bytes
        LiteralInteger,
    ),
    DontStaticallyCoalesceALTERA,
    PrefetchALTERA(
        ///Prefetcher Size in bytes
        LiteralInteger,
    ),
    StallEnableALTERA,
    FuseLoopsInFunctionALTERA,
    MathOpDSPModeALTERA(
        ///Mode
        LiteralInteger,
        ///Propagate
        LiteralInteger,
    ),
    AliasScopeINTEL(
        ///Aliasing Scopes List
        IdRef,
    ),
    NoAliasINTEL(
        ///Aliasing Scopes List
        IdRef,
    ),
    InitiationIntervalALTERA(
        ///Cycles
        LiteralInteger,
    ),
    MaxConcurrencyALTERA(
        ///Invocations
        LiteralInteger,
    ),
    PipelineEnableALTERA(
        ///Enable
        LiteralInteger,
    ),
    BufferLocationALTERA(
        ///Buffer Location ID
        LiteralInteger,
    ),
    IOPipeStorageALTERA(
        ///IO Pipe ID
        LiteralInteger,
    ),
    FunctionFloatingPointModeINTEL(
        ///Target Width
        LiteralInteger,
        ///FP Operation Mode
        FPOperationMode,
    ),
    SingleElementVectorINTEL,
    VectorComputeCallableFunctionINTEL,
    MediaBlockIOINTEL,
    StallFreeALTERA,
    FPMaxErrorDecorationINTEL(
        ///Max Error
        LiteralFloat,
    ),
    LatencyControlLabelALTERA(
        ///Latency Label
        LiteralInteger,
    ),
    LatencyControlConstraintALTERA(
        ///Relative To
        LiteralInteger,
        ///Control Type
        LiteralInteger,
        ///Relative Cycle
        LiteralInteger,
    ),
    ConduitKernelArgumentALTERA,
    RegisterMapKernelArgumentALTERA,
    MMHostInterfaceAddressWidthALTERA(
        ///AddressWidth
        LiteralInteger,
    ),
    MMHostInterfaceDataWidthALTERA(
        ///DataWidth
        LiteralInteger,
    ),
    MMHostInterfaceLatencyALTERA(
        ///Latency
        LiteralInteger,
    ),
    MMHostInterfaceReadWriteModeALTERA(
        ///ReadWriteMode
        AccessQualifier,
    ),
    MMHostInterfaceMaxBurstALTERA(
        ///MaxBurstCount
        LiteralInteger,
    ),
    MMHostInterfaceWaitRequestALTERA(
        ///Waitrequest
        LiteralInteger,
    ),
    StableKernelArgumentALTERA,
    HostAccessINTEL(
        ///Access
        HostAccessQualifier,
        ///Name
        LiteralString,
    ),
    InitModeALTERA(
        ///Trigger
        InitializationModeQualifier,
    ),
    ImplementInRegisterMapALTERA(
        ///Value
        LiteralInteger,
    ),
    ConditionalINTEL(
        ///Condition
        IdRef,
    ),
    CacheControlLoadINTEL(
        ///Cache Level
        LiteralInteger,
        ///Cache Control
        LoadCacheControl,
    ),
    CacheControlStoreINTEL(
        ///Cache Level
        LiteralInteger,
        ///Cache Control
        StoreCacheControl,
    ),
}
unsafe impl Operand for Decoration {
    const KIND: &OperandKind = &OPERAND_KIND_DECORATION;
}
unsafe impl OperandEncoding for Decoration {
    const FIXED_LEN: Option<usize> = None;
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        match self {
            Self::RelaxedPrecision => writer.write(Word(0u32)),
            Self::SpecId(p0) => {
                writer.write(Word(1u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::Block => writer.write(Word(2u32)),
            Self::BufferBlock => writer.write(Word(3u32)),
            Self::RowMajor => writer.write(Word(4u32)),
            Self::ColMajor => writer.write(Word(5u32)),
            Self::ArrayStride(p0) => {
                writer.write(Word(6u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MatrixStride(p0) => {
                writer.write(Word(7u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::GLSLShared => writer.write(Word(8u32)),
            Self::GLSLPacked => writer.write(Word(9u32)),
            Self::CPacked => writer.write(Word(10u32)),
            Self::BuiltIn(p0) => {
                writer.write(Word(11u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::NoPerspective => writer.write(Word(13u32)),
            Self::Flat => writer.write(Word(14u32)),
            Self::Patch => writer.write(Word(15u32)),
            Self::Centroid => writer.write(Word(16u32)),
            Self::Sample => writer.write(Word(17u32)),
            Self::Invariant => writer.write(Word(18u32)),
            Self::Restrict => writer.write(Word(19u32)),
            Self::Aliased => writer.write(Word(20u32)),
            Self::Volatile => writer.write(Word(21u32)),
            Self::Constant => writer.write(Word(22u32)),
            Self::Coherent => writer.write(Word(23u32)),
            Self::NonWritable => writer.write(Word(24u32)),
            Self::NonReadable => writer.write(Word(25u32)),
            Self::Uniform => writer.write(Word(26u32)),
            Self::UniformId(p0) => {
                writer.write(Word(27u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::SaturatedConversion => writer.write(Word(28u32)),
            Self::Stream(p0) => {
                writer.write(Word(29u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::Location(p0) => {
                writer.write(Word(30u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::Component(p0) => {
                writer.write(Word(31u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::Index(p0) => {
                writer.write(Word(32u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::Binding(p0) => {
                writer.write(Word(33u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::DescriptorSet(p0) => {
                writer.write(Word(34u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::Offset(p0) => {
                writer.write(Word(35u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::XfbBuffer(p0) => {
                writer.write(Word(36u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::XfbStride(p0) => {
                writer.write(Word(37u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::FuncParamAttr(p0) => {
                writer.write(Word(38u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::FPRoundingMode(p0) => {
                writer.write(Word(39u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::FPFastMathMode(p0) => {
                writer.write(Word(40u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::LinkageAttributes(p0, p1) => {
                writer.write(Word(41u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
            Self::NoContraction => writer.write(Word(42u32)),
            Self::InputAttachmentIndex(p0) => {
                writer.write(Word(43u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::Alignment(p0) => {
                writer.write(Word(44u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MaxByteOffset(p0) => {
                writer.write(Word(45u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::AlignmentId(p0) => {
                writer.write(Word(46u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MaxByteOffsetId(p0) => {
                writer.write(Word(47u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::SaturatedToLargestFloat8NormalConversionEXT => writer.write(Word(4216u32)),
            Self::NoSignedWrap => writer.write(Word(4469u32)),
            Self::NoUnsignedWrap => writer.write(Word(4470u32)),
            Self::WeightTextureQCOM => writer.write(Word(4487u32)),
            Self::BlockMatchTextureQCOM => writer.write(Word(4488u32)),
            Self::BlockMatchSamplerQCOM => writer.write(Word(4499u32)),
            Self::ExplicitInterpAMD => writer.write(Word(4999u32)),
            Self::NodeSharesPayloadLimitsWithAMDX(p0) => {
                writer.write(Word(5019u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::NodeMaxPayloadsAMDX(p0) => {
                writer.write(Word(5020u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::TrackFinishWritingAMDX => writer.write(Word(5078u32)),
            Self::PayloadNodeNameAMDX(p0) => {
                writer.write(Word(5091u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::PayloadNodeBaseIndexAMDX(p0) => {
                writer.write(Word(5098u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::PayloadNodeSparseArrayAMDX => writer.write(Word(5099u32)),
            Self::PayloadNodeArraySizeAMDX(p0) => {
                writer.write(Word(5100u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::PayloadDispatchIndirectAMDX => writer.write(Word(5105u32)),
            Self::ArrayStrideIdEXT(p0) => {
                writer.write(Word(5124u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::OffsetIdEXT(p0) => {
                writer.write(Word(5125u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::OverrideCoverageNV => writer.write(Word(5248u32)),
            Self::PassthroughNV => writer.write(Word(5250u32)),
            Self::ViewportRelativeNV => writer.write(Word(5252u32)),
            Self::SecondaryViewportRelativeNV(p0) => {
                writer.write(Word(5256u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::PerPrimitiveEXT => writer.write(Word(5271u32)),
            Self::PerViewNV => writer.write(Word(5272u32)),
            Self::PerTaskNV => writer.write(Word(5273u32)),
            Self::PerVertexKHR => writer.write(Word(5285u32)),
            Self::NonUniform => writer.write(Word(5300u32)),
            Self::RestrictPointer => writer.write(Word(5355u32)),
            Self::AliasedPointer => writer.write(Word(5356u32)),
            Self::MemberOffsetNV(p0) => {
                writer.write(Word(5358u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::HitObjectShaderRecordBufferNV => writer.write(Word(5386u32)),
            Self::HitObjectShaderRecordBufferEXT => writer.write(Word(5389u32)),
            Self::BankNV(p0) => {
                writer.write(Word(5397u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::BindlessSamplerNV => writer.write(Word(5398u32)),
            Self::BindlessImageNV => writer.write(Word(5399u32)),
            Self::BoundSamplerNV => writer.write(Word(5400u32)),
            Self::BoundImageNV => writer.write(Word(5401u32)),
            Self::SIMTCallINTEL(p0) => {
                writer.write(Word(5599u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::ReferencedIndirectlyINTEL => writer.write(Word(5602u32)),
            Self::ClobberINTEL(p0) => {
                writer.write(Word(5607u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::SideEffectsINTEL => writer.write(Word(5608u32)),
            Self::VectorComputeVariableINTEL => writer.write(Word(5624u32)),
            Self::FuncParamIOKindINTEL(p0) => {
                writer.write(Word(5625u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::VectorComputeFunctionINTEL => writer.write(Word(5626u32)),
            Self::StackCallINTEL => writer.write(Word(5627u32)),
            Self::GlobalVariableOffsetINTEL(p0) => {
                writer.write(Word(5628u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::CounterBuffer(p0) => {
                writer.write(Word(5634u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::UserSemantic(p0) => {
                writer.write(Word(5635u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::UserTypeGOOGLE(p0) => {
                writer.write(Word(5636u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::FunctionRoundingModeINTEL(p0, p1) => {
                writer.write(Word(5822u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
            Self::FunctionDenormModeINTEL(p0, p1) => {
                writer.write(Word(5823u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
            Self::RegisterALTERA => writer.write(Word(5825u32)),
            Self::MemoryALTERA(p0) => {
                writer.write(Word(5826u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::NumbanksALTERA(p0) => {
                writer.write(Word(5827u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::BankwidthALTERA(p0) => {
                writer.write(Word(5828u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MaxPrivateCopiesALTERA(p0) => {
                writer.write(Word(5829u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::SinglepumpALTERA => writer.write(Word(5830u32)),
            Self::DoublepumpALTERA => writer.write(Word(5831u32)),
            Self::MaxReplicatesALTERA(p0) => {
                writer.write(Word(5832u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::SimpleDualPortALTERA => writer.write(Word(5833u32)),
            Self::MergeALTERA(p0, p1) => {
                writer.write(Word(5834u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
            Self::BankBitsALTERA(p0) => {
                writer.write(Word(5835u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::ForcePow2DepthALTERA(p0) => {
                writer.write(Word(5836u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::StridesizeALTERA(p0) => {
                writer.write(Word(5883u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::WordsizeALTERA(p0) => {
                writer.write(Word(5884u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::TrueDualPortALTERA => writer.write(Word(5885u32)),
            Self::BurstCoalesceALTERA => writer.write(Word(5899u32)),
            Self::CacheSizeALTERA(p0) => {
                writer.write(Word(5900u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::DontStaticallyCoalesceALTERA => writer.write(Word(5901u32)),
            Self::PrefetchALTERA(p0) => {
                writer.write(Word(5902u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::StallEnableALTERA => writer.write(Word(5905u32)),
            Self::FuseLoopsInFunctionALTERA => writer.write(Word(5907u32)),
            Self::MathOpDSPModeALTERA(p0, p1) => {
                writer.write(Word(5909u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
            Self::AliasScopeINTEL(p0) => {
                writer.write(Word(5914u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::NoAliasINTEL(p0) => {
                writer.write(Word(5915u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::InitiationIntervalALTERA(p0) => {
                writer.write(Word(5917u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MaxConcurrencyALTERA(p0) => {
                writer.write(Word(5918u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::PipelineEnableALTERA(p0) => {
                writer.write(Word(5919u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::BufferLocationALTERA(p0) => {
                writer.write(Word(5921u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::IOPipeStorageALTERA(p0) => {
                writer.write(Word(5944u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::FunctionFloatingPointModeINTEL(p0, p1) => {
                writer.write(Word(6080u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
            Self::SingleElementVectorINTEL => writer.write(Word(6085u32)),
            Self::VectorComputeCallableFunctionINTEL => writer.write(Word(6087u32)),
            Self::MediaBlockIOINTEL => writer.write(Word(6140u32)),
            Self::StallFreeALTERA => writer.write(Word(6151u32)),
            Self::FPMaxErrorDecorationINTEL(p0) => {
                writer.write(Word(6170u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::LatencyControlLabelALTERA(p0) => {
                writer.write(Word(6172u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::LatencyControlConstraintALTERA(p0, p1, p2) => {
                writer.write(Word(6173u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?;
                OperandEncoding::encode(p2, &mut *writer)?
            }
            Self::ConduitKernelArgumentALTERA => writer.write(Word(6175u32)),
            Self::RegisterMapKernelArgumentALTERA => writer.write(Word(6176u32)),
            Self::MMHostInterfaceAddressWidthALTERA(p0) => {
                writer.write(Word(6177u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MMHostInterfaceDataWidthALTERA(p0) => {
                writer.write(Word(6178u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MMHostInterfaceLatencyALTERA(p0) => {
                writer.write(Word(6179u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MMHostInterfaceReadWriteModeALTERA(p0) => {
                writer.write(Word(6180u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MMHostInterfaceMaxBurstALTERA(p0) => {
                writer.write(Word(6181u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::MMHostInterfaceWaitRequestALTERA(p0) => {
                writer.write(Word(6182u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::StableKernelArgumentALTERA => writer.write(Word(6183u32)),
            Self::HostAccessINTEL(p0, p1) => {
                writer.write(Word(6188u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
            Self::InitModeALTERA(p0) => {
                writer.write(Word(6190u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::ImplementInRegisterMapALTERA(p0) => {
                writer.write(Word(6191u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::ConditionalINTEL(p0) => {
                writer.write(Word(6247u32));
                OperandEncoding::encode(p0, &mut *writer)?
            }
            Self::CacheControlLoadINTEL(p0, p1) => {
                writer.write(Word(6442u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
            Self::CacheControlStoreINTEL(p0, p1) => {
                writer.write(Word(6443u32));
                OperandEncoding::encode(p0, &mut *writer)?;
                OperandEncoding::encode(p1, &mut *writer)?
            }
        }
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::RelaxedPrecision,
            1u32 => Self::SpecId(OperandEncoding::decode(&mut *reader)?),
            2u32 => Self::Block,
            3u32 => Self::BufferBlock,
            4u32 => Self::RowMajor,
            5u32 => Self::ColMajor,
            6u32 => Self::ArrayStride(OperandEncoding::decode(&mut *reader)?),
            7u32 => Self::MatrixStride(OperandEncoding::decode(&mut *reader)?),
            8u32 => Self::GLSLShared,
            9u32 => Self::GLSLPacked,
            10u32 => Self::CPacked,
            11u32 => Self::BuiltIn(OperandEncoding::decode(&mut *reader)?),
            13u32 => Self::NoPerspective,
            14u32 => Self::Flat,
            15u32 => Self::Patch,
            16u32 => Self::Centroid,
            17u32 => Self::Sample,
            18u32 => Self::Invariant,
            19u32 => Self::Restrict,
            20u32 => Self::Aliased,
            21u32 => Self::Volatile,
            22u32 => Self::Constant,
            23u32 => Self::Coherent,
            24u32 => Self::NonWritable,
            25u32 => Self::NonReadable,
            26u32 => Self::Uniform,
            27u32 => Self::UniformId(OperandEncoding::decode(&mut *reader)?),
            28u32 => Self::SaturatedConversion,
            29u32 => Self::Stream(OperandEncoding::decode(&mut *reader)?),
            30u32 => Self::Location(OperandEncoding::decode(&mut *reader)?),
            31u32 => Self::Component(OperandEncoding::decode(&mut *reader)?),
            32u32 => Self::Index(OperandEncoding::decode(&mut *reader)?),
            33u32 => Self::Binding(OperandEncoding::decode(&mut *reader)?),
            34u32 => Self::DescriptorSet(OperandEncoding::decode(&mut *reader)?),
            35u32 => Self::Offset(OperandEncoding::decode(&mut *reader)?),
            36u32 => Self::XfbBuffer(OperandEncoding::decode(&mut *reader)?),
            37u32 => Self::XfbStride(OperandEncoding::decode(&mut *reader)?),
            38u32 => Self::FuncParamAttr(OperandEncoding::decode(&mut *reader)?),
            39u32 => Self::FPRoundingMode(OperandEncoding::decode(&mut *reader)?),
            40u32 => Self::FPFastMathMode(OperandEncoding::decode(&mut *reader)?),
            41u32 => Self::LinkageAttributes(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            42u32 => Self::NoContraction,
            43u32 => Self::InputAttachmentIndex(OperandEncoding::decode(&mut *reader)?),
            44u32 => Self::Alignment(OperandEncoding::decode(&mut *reader)?),
            45u32 => Self::MaxByteOffset(OperandEncoding::decode(&mut *reader)?),
            46u32 => Self::AlignmentId(OperandEncoding::decode(&mut *reader)?),
            47u32 => Self::MaxByteOffsetId(OperandEncoding::decode(&mut *reader)?),
            4216u32 => Self::SaturatedToLargestFloat8NormalConversionEXT,
            4469u32 => Self::NoSignedWrap,
            4470u32 => Self::NoUnsignedWrap,
            4487u32 => Self::WeightTextureQCOM,
            4488u32 => Self::BlockMatchTextureQCOM,
            4499u32 => Self::BlockMatchSamplerQCOM,
            4999u32 => Self::ExplicitInterpAMD,
            5019u32 => {
                Self::NodeSharesPayloadLimitsWithAMDX(OperandEncoding::decode(&mut *reader)?)
            }
            5020u32 => Self::NodeMaxPayloadsAMDX(OperandEncoding::decode(&mut *reader)?),
            5078u32 => Self::TrackFinishWritingAMDX,
            5091u32 => Self::PayloadNodeNameAMDX(OperandEncoding::decode(&mut *reader)?),
            5098u32 => Self::PayloadNodeBaseIndexAMDX(OperandEncoding::decode(&mut *reader)?),
            5099u32 => Self::PayloadNodeSparseArrayAMDX,
            5100u32 => Self::PayloadNodeArraySizeAMDX(OperandEncoding::decode(&mut *reader)?),
            5105u32 => Self::PayloadDispatchIndirectAMDX,
            5124u32 => Self::ArrayStrideIdEXT(OperandEncoding::decode(&mut *reader)?),
            5125u32 => Self::OffsetIdEXT(OperandEncoding::decode(&mut *reader)?),
            5248u32 => Self::OverrideCoverageNV,
            5250u32 => Self::PassthroughNV,
            5252u32 => Self::ViewportRelativeNV,
            5256u32 => Self::SecondaryViewportRelativeNV(OperandEncoding::decode(&mut *reader)?),
            5271u32 => Self::PerPrimitiveEXT,
            5272u32 => Self::PerViewNV,
            5273u32 => Self::PerTaskNV,
            5285u32 => Self::PerVertexKHR,
            5300u32 => Self::NonUniform,
            5355u32 => Self::RestrictPointer,
            5356u32 => Self::AliasedPointer,
            5358u32 => Self::MemberOffsetNV(OperandEncoding::decode(&mut *reader)?),
            5386u32 => Self::HitObjectShaderRecordBufferNV,
            5389u32 => Self::HitObjectShaderRecordBufferEXT,
            5397u32 => Self::BankNV(OperandEncoding::decode(&mut *reader)?),
            5398u32 => Self::BindlessSamplerNV,
            5399u32 => Self::BindlessImageNV,
            5400u32 => Self::BoundSamplerNV,
            5401u32 => Self::BoundImageNV,
            5599u32 => Self::SIMTCallINTEL(OperandEncoding::decode(&mut *reader)?),
            5602u32 => Self::ReferencedIndirectlyINTEL,
            5607u32 => Self::ClobberINTEL(OperandEncoding::decode(&mut *reader)?),
            5608u32 => Self::SideEffectsINTEL,
            5624u32 => Self::VectorComputeVariableINTEL,
            5625u32 => Self::FuncParamIOKindINTEL(OperandEncoding::decode(&mut *reader)?),
            5626u32 => Self::VectorComputeFunctionINTEL,
            5627u32 => Self::StackCallINTEL,
            5628u32 => Self::GlobalVariableOffsetINTEL(OperandEncoding::decode(&mut *reader)?),
            5634u32 => Self::CounterBuffer(OperandEncoding::decode(&mut *reader)?),
            5635u32 => Self::UserSemantic(OperandEncoding::decode(&mut *reader)?),
            5636u32 => Self::UserTypeGOOGLE(OperandEncoding::decode(&mut *reader)?),
            5822u32 => Self::FunctionRoundingModeINTEL(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            5823u32 => Self::FunctionDenormModeINTEL(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            5825u32 => Self::RegisterALTERA,
            5826u32 => Self::MemoryALTERA(OperandEncoding::decode(&mut *reader)?),
            5827u32 => Self::NumbanksALTERA(OperandEncoding::decode(&mut *reader)?),
            5828u32 => Self::BankwidthALTERA(OperandEncoding::decode(&mut *reader)?),
            5829u32 => Self::MaxPrivateCopiesALTERA(OperandEncoding::decode(&mut *reader)?),
            5830u32 => Self::SinglepumpALTERA,
            5831u32 => Self::DoublepumpALTERA,
            5832u32 => Self::MaxReplicatesALTERA(OperandEncoding::decode(&mut *reader)?),
            5833u32 => Self::SimpleDualPortALTERA,
            5834u32 => Self::MergeALTERA(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            5835u32 => Self::BankBitsALTERA(OperandEncoding::decode(&mut *reader)?),
            5836u32 => Self::ForcePow2DepthALTERA(OperandEncoding::decode(&mut *reader)?),
            5883u32 => Self::StridesizeALTERA(OperandEncoding::decode(&mut *reader)?),
            5884u32 => Self::WordsizeALTERA(OperandEncoding::decode(&mut *reader)?),
            5885u32 => Self::TrueDualPortALTERA,
            5899u32 => Self::BurstCoalesceALTERA,
            5900u32 => Self::CacheSizeALTERA(OperandEncoding::decode(&mut *reader)?),
            5901u32 => Self::DontStaticallyCoalesceALTERA,
            5902u32 => Self::PrefetchALTERA(OperandEncoding::decode(&mut *reader)?),
            5905u32 => Self::StallEnableALTERA,
            5907u32 => Self::FuseLoopsInFunctionALTERA,
            5909u32 => Self::MathOpDSPModeALTERA(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            5914u32 => Self::AliasScopeINTEL(OperandEncoding::decode(&mut *reader)?),
            5915u32 => Self::NoAliasINTEL(OperandEncoding::decode(&mut *reader)?),
            5917u32 => Self::InitiationIntervalALTERA(OperandEncoding::decode(&mut *reader)?),
            5918u32 => Self::MaxConcurrencyALTERA(OperandEncoding::decode(&mut *reader)?),
            5919u32 => Self::PipelineEnableALTERA(OperandEncoding::decode(&mut *reader)?),
            5921u32 => Self::BufferLocationALTERA(OperandEncoding::decode(&mut *reader)?),
            5944u32 => Self::IOPipeStorageALTERA(OperandEncoding::decode(&mut *reader)?),
            6080u32 => Self::FunctionFloatingPointModeINTEL(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            6085u32 => Self::SingleElementVectorINTEL,
            6087u32 => Self::VectorComputeCallableFunctionINTEL,
            6140u32 => Self::MediaBlockIOINTEL,
            6151u32 => Self::StallFreeALTERA,
            6170u32 => Self::FPMaxErrorDecorationINTEL(OperandEncoding::decode(&mut *reader)?),
            6172u32 => Self::LatencyControlLabelALTERA(OperandEncoding::decode(&mut *reader)?),
            6173u32 => Self::LatencyControlConstraintALTERA(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            6175u32 => Self::ConduitKernelArgumentALTERA,
            6176u32 => Self::RegisterMapKernelArgumentALTERA,
            6177u32 => {
                Self::MMHostInterfaceAddressWidthALTERA(OperandEncoding::decode(&mut *reader)?)
            }
            6178u32 => Self::MMHostInterfaceDataWidthALTERA(OperandEncoding::decode(&mut *reader)?),
            6179u32 => Self::MMHostInterfaceLatencyALTERA(OperandEncoding::decode(&mut *reader)?),
            6180u32 => {
                Self::MMHostInterfaceReadWriteModeALTERA(OperandEncoding::decode(&mut *reader)?)
            }
            6181u32 => Self::MMHostInterfaceMaxBurstALTERA(OperandEncoding::decode(&mut *reader)?),
            6182u32 => {
                Self::MMHostInterfaceWaitRequestALTERA(OperandEncoding::decode(&mut *reader)?)
            }
            6183u32 => Self::StableKernelArgumentALTERA,
            6188u32 => Self::HostAccessINTEL(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            6190u32 => Self::InitModeALTERA(OperandEncoding::decode(&mut *reader)?),
            6191u32 => Self::ImplementInRegisterMapALTERA(OperandEncoding::decode(&mut *reader)?),
            6247u32 => Self::ConditionalINTEL(OperandEncoding::decode(&mut *reader)?),
            6442u32 => Self::CacheControlLoadINTEL(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            6443u32 => Self::CacheControlStoreINTEL(
                OperandEncoding::decode(&mut *reader)?,
                OperandEncoding::decode(&mut *reader)?,
            ),
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(Decoration),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        match self {
            Self::RelaxedPrecision => write!(f, " RelaxedPrecision"),
            Self::SpecId(p0) => write!(f, " SpecId{}", p0.dis(_ctx)),
            Self::Block => write!(f, " Block"),
            Self::BufferBlock => write!(f, " BufferBlock"),
            Self::RowMajor => write!(f, " RowMajor"),
            Self::ColMajor => write!(f, " ColMajor"),
            Self::ArrayStride(p0) => write!(f, " ArrayStride{}", p0.dis(_ctx)),
            Self::MatrixStride(p0) => write!(f, " MatrixStride{}", p0.dis(_ctx)),
            Self::GLSLShared => write!(f, " GLSLShared"),
            Self::GLSLPacked => write!(f, " GLSLPacked"),
            Self::CPacked => write!(f, " CPacked"),
            Self::BuiltIn(p0) => write!(f, " BuiltIn{}", p0.dis(_ctx)),
            Self::NoPerspective => write!(f, " NoPerspective"),
            Self::Flat => write!(f, " Flat"),
            Self::Patch => write!(f, " Patch"),
            Self::Centroid => write!(f, " Centroid"),
            Self::Sample => write!(f, " Sample"),
            Self::Invariant => write!(f, " Invariant"),
            Self::Restrict => write!(f, " Restrict"),
            Self::Aliased => write!(f, " Aliased"),
            Self::Volatile => write!(f, " Volatile"),
            Self::Constant => write!(f, " Constant"),
            Self::Coherent => write!(f, " Coherent"),
            Self::NonWritable => write!(f, " NonWritable"),
            Self::NonReadable => write!(f, " NonReadable"),
            Self::Uniform => write!(f, " Uniform"),
            Self::UniformId(p0) => write!(f, " UniformId{}", p0.dis(_ctx)),
            Self::SaturatedConversion => write!(f, " SaturatedConversion"),
            Self::Stream(p0) => write!(f, " Stream{}", p0.dis(_ctx)),
            Self::Location(p0) => write!(f, " Location{}", p0.dis(_ctx)),
            Self::Component(p0) => write!(f, " Component{}", p0.dis(_ctx)),
            Self::Index(p0) => write!(f, " Index{}", p0.dis(_ctx)),
            Self::Binding(p0) => write!(f, " Binding{}", p0.dis(_ctx)),
            Self::DescriptorSet(p0) => write!(f, " DescriptorSet{}", p0.dis(_ctx)),
            Self::Offset(p0) => write!(f, " Offset{}", p0.dis(_ctx)),
            Self::XfbBuffer(p0) => write!(f, " XfbBuffer{}", p0.dis(_ctx)),
            Self::XfbStride(p0) => write!(f, " XfbStride{}", p0.dis(_ctx)),
            Self::FuncParamAttr(p0) => write!(f, " FuncParamAttr{}", p0.dis(_ctx)),
            Self::FPRoundingMode(p0) => write!(f, " FPRoundingMode{}", p0.dis(_ctx)),
            Self::FPFastMathMode(p0) => write!(f, " FPFastMathMode{}", p0.dis(_ctx)),
            Self::LinkageAttributes(p0, p1) => {
                write!(f, " LinkageAttributes{}{}", p0.dis(_ctx), p1.dis(_ctx))
            }
            Self::NoContraction => write!(f, " NoContraction"),
            Self::InputAttachmentIndex(p0) => {
                write!(f, " InputAttachmentIndex{}", p0.dis(_ctx))
            }
            Self::Alignment(p0) => write!(f, " Alignment{}", p0.dis(_ctx)),
            Self::MaxByteOffset(p0) => write!(f, " MaxByteOffset{}", p0.dis(_ctx)),
            Self::AlignmentId(p0) => write!(f, " AlignmentId{}", p0.dis(_ctx)),
            Self::MaxByteOffsetId(p0) => write!(f, " MaxByteOffsetId{}", p0.dis(_ctx)),
            Self::SaturatedToLargestFloat8NormalConversionEXT => {
                write!(f, " SaturatedToLargestFloat8NormalConversionEXT")
            }
            Self::NoSignedWrap => write!(f, " NoSignedWrap"),
            Self::NoUnsignedWrap => write!(f, " NoUnsignedWrap"),
            Self::WeightTextureQCOM => write!(f, " WeightTextureQCOM"),
            Self::BlockMatchTextureQCOM => write!(f, " BlockMatchTextureQCOM"),
            Self::BlockMatchSamplerQCOM => write!(f, " BlockMatchSamplerQCOM"),
            Self::ExplicitInterpAMD => write!(f, " ExplicitInterpAMD"),
            Self::NodeSharesPayloadLimitsWithAMDX(p0) => {
                write!(f, " NodeSharesPayloadLimitsWithAMDX{}", p0.dis(_ctx))
            }
            Self::NodeMaxPayloadsAMDX(p0) => {
                write!(f, " NodeMaxPayloadsAMDX{}", p0.dis(_ctx))
            }
            Self::TrackFinishWritingAMDX => write!(f, " TrackFinishWritingAMDX"),
            Self::PayloadNodeNameAMDX(p0) => {
                write!(f, " PayloadNodeNameAMDX{}", p0.dis(_ctx))
            }
            Self::PayloadNodeBaseIndexAMDX(p0) => {
                write!(f, " PayloadNodeBaseIndexAMDX{}", p0.dis(_ctx))
            }
            Self::PayloadNodeSparseArrayAMDX => write!(f, " PayloadNodeSparseArrayAMDX"),
            Self::PayloadNodeArraySizeAMDX(p0) => {
                write!(f, " PayloadNodeArraySizeAMDX{}", p0.dis(_ctx))
            }
            Self::PayloadDispatchIndirectAMDX => {
                write!(f, " PayloadDispatchIndirectAMDX")
            }
            Self::ArrayStrideIdEXT(p0) => write!(f, " ArrayStrideIdEXT{}", p0.dis(_ctx)),
            Self::OffsetIdEXT(p0) => write!(f, " OffsetIdEXT{}", p0.dis(_ctx)),
            Self::OverrideCoverageNV => write!(f, " OverrideCoverageNV"),
            Self::PassthroughNV => write!(f, " PassthroughNV"),
            Self::ViewportRelativeNV => write!(f, " ViewportRelativeNV"),
            Self::SecondaryViewportRelativeNV(p0) => {
                write!(f, " SecondaryViewportRelativeNV{}", p0.dis(_ctx))
            }
            Self::PerPrimitiveEXT => write!(f, " PerPrimitiveEXT"),
            Self::PerViewNV => write!(f, " PerViewNV"),
            Self::PerTaskNV => write!(f, " PerTaskNV"),
            Self::PerVertexKHR => write!(f, " PerVertexKHR"),
            Self::NonUniform => write!(f, " NonUniform"),
            Self::RestrictPointer => write!(f, " RestrictPointer"),
            Self::AliasedPointer => write!(f, " AliasedPointer"),
            Self::MemberOffsetNV(p0) => write!(f, " MemberOffsetNV{}", p0.dis(_ctx)),
            Self::HitObjectShaderRecordBufferNV => {
                write!(f, " HitObjectShaderRecordBufferNV")
            }
            Self::HitObjectShaderRecordBufferEXT => {
                write!(f, " HitObjectShaderRecordBufferEXT")
            }
            Self::BankNV(p0) => write!(f, " BankNV{}", p0.dis(_ctx)),
            Self::BindlessSamplerNV => write!(f, " BindlessSamplerNV"),
            Self::BindlessImageNV => write!(f, " BindlessImageNV"),
            Self::BoundSamplerNV => write!(f, " BoundSamplerNV"),
            Self::BoundImageNV => write!(f, " BoundImageNV"),
            Self::SIMTCallINTEL(p0) => write!(f, " SIMTCallINTEL{}", p0.dis(_ctx)),
            Self::ReferencedIndirectlyINTEL => write!(f, " ReferencedIndirectlyINTEL"),
            Self::ClobberINTEL(p0) => write!(f, " ClobberINTEL{}", p0.dis(_ctx)),
            Self::SideEffectsINTEL => write!(f, " SideEffectsINTEL"),
            Self::VectorComputeVariableINTEL => write!(f, " VectorComputeVariableINTEL"),
            Self::FuncParamIOKindINTEL(p0) => {
                write!(f, " FuncParamIOKindINTEL{}", p0.dis(_ctx))
            }
            Self::VectorComputeFunctionINTEL => write!(f, " VectorComputeFunctionINTEL"),
            Self::StackCallINTEL => write!(f, " StackCallINTEL"),
            Self::GlobalVariableOffsetINTEL(p0) => {
                write!(f, " GlobalVariableOffsetINTEL{}", p0.dis(_ctx))
            }
            Self::CounterBuffer(p0) => write!(f, " CounterBuffer{}", p0.dis(_ctx)),
            Self::UserSemantic(p0) => write!(f, " UserSemantic{}", p0.dis(_ctx)),
            Self::UserTypeGOOGLE(p0) => write!(f, " UserTypeGOOGLE{}", p0.dis(_ctx)),
            Self::FunctionRoundingModeINTEL(p0, p1) => {
                write!(
                    f,
                    " FunctionRoundingModeINTEL{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx)
                )
            }
            Self::FunctionDenormModeINTEL(p0, p1) => {
                write!(
                    f,
                    " FunctionDenormModeINTEL{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx)
                )
            }
            Self::RegisterALTERA => write!(f, " RegisterALTERA"),
            Self::MemoryALTERA(p0) => write!(f, " MemoryALTERA{}", p0.dis(_ctx)),
            Self::NumbanksALTERA(p0) => write!(f, " NumbanksALTERA{}", p0.dis(_ctx)),
            Self::BankwidthALTERA(p0) => write!(f, " BankwidthALTERA{}", p0.dis(_ctx)),
            Self::MaxPrivateCopiesALTERA(p0) => {
                write!(f, " MaxPrivateCopiesALTERA{}", p0.dis(_ctx))
            }
            Self::SinglepumpALTERA => write!(f, " SinglepumpALTERA"),
            Self::DoublepumpALTERA => write!(f, " DoublepumpALTERA"),
            Self::MaxReplicatesALTERA(p0) => {
                write!(f, " MaxReplicatesALTERA{}", p0.dis(_ctx))
            }
            Self::SimpleDualPortALTERA => write!(f, " SimpleDualPortALTERA"),
            Self::MergeALTERA(p0, p1) => {
                write!(f, " MergeALTERA{}{}", p0.dis(_ctx), p1.dis(_ctx))
            }
            Self::BankBitsALTERA(p0) => write!(f, " BankBitsALTERA{}", p0.dis(_ctx)),
            Self::ForcePow2DepthALTERA(p0) => {
                write!(f, " ForcePow2DepthALTERA{}", p0.dis(_ctx))
            }
            Self::StridesizeALTERA(p0) => write!(f, " StridesizeALTERA{}", p0.dis(_ctx)),
            Self::WordsizeALTERA(p0) => write!(f, " WordsizeALTERA{}", p0.dis(_ctx)),
            Self::TrueDualPortALTERA => write!(f, " TrueDualPortALTERA"),
            Self::BurstCoalesceALTERA => write!(f, " BurstCoalesceALTERA"),
            Self::CacheSizeALTERA(p0) => write!(f, " CacheSizeALTERA{}", p0.dis(_ctx)),
            Self::DontStaticallyCoalesceALTERA => {
                write!(f, " DontStaticallyCoalesceALTERA")
            }
            Self::PrefetchALTERA(p0) => write!(f, " PrefetchALTERA{}", p0.dis(_ctx)),
            Self::StallEnableALTERA => write!(f, " StallEnableALTERA"),
            Self::FuseLoopsInFunctionALTERA => write!(f, " FuseLoopsInFunctionALTERA"),
            Self::MathOpDSPModeALTERA(p0, p1) => {
                write!(f, " MathOpDSPModeALTERA{}{}", p0.dis(_ctx), p1.dis(_ctx))
            }
            Self::AliasScopeINTEL(p0) => write!(f, " AliasScopeINTEL{}", p0.dis(_ctx)),
            Self::NoAliasINTEL(p0) => write!(f, " NoAliasINTEL{}", p0.dis(_ctx)),
            Self::InitiationIntervalALTERA(p0) => {
                write!(f, " InitiationIntervalALTERA{}", p0.dis(_ctx))
            }
            Self::MaxConcurrencyALTERA(p0) => {
                write!(f, " MaxConcurrencyALTERA{}", p0.dis(_ctx))
            }
            Self::PipelineEnableALTERA(p0) => {
                write!(f, " PipelineEnableALTERA{}", p0.dis(_ctx))
            }
            Self::BufferLocationALTERA(p0) => {
                write!(f, " BufferLocationALTERA{}", p0.dis(_ctx))
            }
            Self::IOPipeStorageALTERA(p0) => {
                write!(f, " IOPipeStorageALTERA{}", p0.dis(_ctx))
            }
            Self::FunctionFloatingPointModeINTEL(p0, p1) => {
                write!(
                    f,
                    " FunctionFloatingPointModeINTEL{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx)
                )
            }
            Self::SingleElementVectorINTEL => write!(f, " SingleElementVectorINTEL"),
            Self::VectorComputeCallableFunctionINTEL => {
                write!(f, " VectorComputeCallableFunctionINTEL")
            }
            Self::MediaBlockIOINTEL => write!(f, " MediaBlockIOINTEL"),
            Self::StallFreeALTERA => write!(f, " StallFreeALTERA"),
            Self::FPMaxErrorDecorationINTEL(p0) => {
                write!(f, " FPMaxErrorDecorationINTEL{}", p0.dis(_ctx))
            }
            Self::LatencyControlLabelALTERA(p0) => {
                write!(f, " LatencyControlLabelALTERA{}", p0.dis(_ctx))
            }
            Self::LatencyControlConstraintALTERA(p0, p1, p2) => {
                write!(
                    f,
                    " LatencyControlConstraintALTERA{}{}{}",
                    p0.dis(_ctx),
                    p1.dis(_ctx),
                    p2.dis(_ctx)
                )
            }
            Self::ConduitKernelArgumentALTERA => {
                write!(f, " ConduitKernelArgumentALTERA")
            }
            Self::RegisterMapKernelArgumentALTERA => {
                write!(f, " RegisterMapKernelArgumentALTERA")
            }
            Self::MMHostInterfaceAddressWidthALTERA(p0) => {
                write!(f, " MMHostInterfaceAddressWidthALTERA{}", p0.dis(_ctx))
            }
            Self::MMHostInterfaceDataWidthALTERA(p0) => {
                write!(f, " MMHostInterfaceDataWidthALTERA{}", p0.dis(_ctx))
            }
            Self::MMHostInterfaceLatencyALTERA(p0) => {
                write!(f, " MMHostInterfaceLatencyALTERA{}", p0.dis(_ctx))
            }
            Self::MMHostInterfaceReadWriteModeALTERA(p0) => {
                write!(f, " MMHostInterfaceReadWriteModeALTERA{}", p0.dis(_ctx))
            }
            Self::MMHostInterfaceMaxBurstALTERA(p0) => {
                write!(f, " MMHostInterfaceMaxBurstALTERA{}", p0.dis(_ctx))
            }
            Self::MMHostInterfaceWaitRequestALTERA(p0) => {
                write!(f, " MMHostInterfaceWaitRequestALTERA{}", p0.dis(_ctx))
            }
            Self::StableKernelArgumentALTERA => write!(f, " StableKernelArgumentALTERA"),
            Self::HostAccessINTEL(p0, p1) => {
                write!(f, " HostAccessINTEL{}{}", p0.dis(_ctx), p1.dis(_ctx))
            }
            Self::InitModeALTERA(p0) => write!(f, " InitModeALTERA{}", p0.dis(_ctx)),
            Self::ImplementInRegisterMapALTERA(p0) => {
                write!(f, " ImplementInRegisterMapALTERA{}", p0.dis(_ctx))
            }
            Self::ConditionalINTEL(p0) => write!(f, " ConditionalINTEL{}", p0.dis(_ctx)),
            Self::CacheControlLoadINTEL(p0, p1) => {
                write!(f, " CacheControlLoadINTEL{}{}", p0.dis(_ctx), p1.dis(_ctx))
            }
            Self::CacheControlStoreINTEL(p0, p1) => {
                write!(f, " CacheControlStoreINTEL{}{}", p0.dis(_ctx), p1.dis(_ctx))
            }
        }
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BuiltIn {
    Position = 0u32,
    PointSize = 1u32,
    ClipDistance = 3u32,
    CullDistance = 4u32,
    VertexId = 5u32,
    InstanceId = 6u32,
    PrimitiveId = 7u32,
    InvocationId = 8u32,
    Layer = 9u32,
    ViewportIndex = 10u32,
    TessLevelOuter = 11u32,
    TessLevelInner = 12u32,
    TessCoord = 13u32,
    PatchVertices = 14u32,
    FragCoord = 15u32,
    PointCoord = 16u32,
    FrontFacing = 17u32,
    SampleId = 18u32,
    SamplePosition = 19u32,
    SampleMask = 20u32,
    FragDepth = 22u32,
    HelperInvocation = 23u32,
    NumWorkgroups = 24u32,
    WorkgroupSize = 25u32,
    WorkgroupId = 26u32,
    LocalInvocationId = 27u32,
    GlobalInvocationId = 28u32,
    LocalInvocationIndex = 29u32,
    WorkDim = 30u32,
    GlobalSize = 31u32,
    EnqueuedWorkgroupSize = 32u32,
    GlobalOffset = 33u32,
    GlobalLinearId = 34u32,
    SubgroupSize = 36u32,
    SubgroupMaxSize = 37u32,
    NumSubgroups = 38u32,
    NumEnqueuedSubgroups = 39u32,
    SubgroupId = 40u32,
    SubgroupLocalInvocationId = 41u32,
    VertexIndex = 42u32,
    InstanceIndex = 43u32,
    CoreIDARM = 4160u32,
    CoreCountARM = 4161u32,
    CoreMaxIDARM = 4162u32,
    WarpIDARM = 4163u32,
    WarpMaxIDARM = 4164u32,
    ///Since SPIR-V 1.3
    SubgroupEqMask = 4416u32,
    ///Since SPIR-V 1.3
    SubgroupGeMask = 4417u32,
    ///Since SPIR-V 1.3
    SubgroupGtMask = 4418u32,
    ///Since SPIR-V 1.3
    SubgroupLeMask = 4419u32,
    ///Since SPIR-V 1.3
    SubgroupLtMask = 4420u32,
    ///Since SPIR-V 1.3
    BaseVertex = 4424u32,
    ///Since SPIR-V 1.3
    BaseInstance = 4425u32,
    ///Since SPIR-V 1.3
    DrawIndex = 4426u32,
    PrimitiveShadingRateKHR = 4432u32,
    ///Since SPIR-V 1.3
    DeviceIndex = 4438u32,
    ///Since SPIR-V 1.3
    ViewIndex = 4440u32,
    ShadingRateKHR = 4444u32,
    TileOffsetQCOM = 4492u32,
    TileDimensionQCOM = 4493u32,
    TileApronSizeQCOM = 4494u32,
    BaryCoordNoPerspAMD = 4992u32,
    BaryCoordNoPerspCentroidAMD = 4993u32,
    BaryCoordNoPerspSampleAMD = 4994u32,
    BaryCoordSmoothAMD = 4995u32,
    BaryCoordSmoothCentroidAMD = 4996u32,
    BaryCoordSmoothSampleAMD = 4997u32,
    BaryCoordPullModelAMD = 4998u32,
    FragStencilRefEXT = 5014u32,
    RemainingRecursionLevelsAMDX = 5021u32,
    ShaderIndexAMDX = 5073u32,
    SamplerHeapEXT = 5122u32,
    ResourceHeapEXT = 5123u32,
    ViewportMaskNV = 5253u32,
    SecondaryPositionNV = 5257u32,
    SecondaryViewportMaskNV = 5258u32,
    PositionPerViewNV = 5261u32,
    ViewportMaskPerViewNV = 5262u32,
    FullyCoveredEXT = 5264u32,
    TaskCountNV = 5274u32,
    PrimitiveCountNV = 5275u32,
    PrimitiveIndicesNV = 5276u32,
    ClipDistancePerViewNV = 5277u32,
    CullDistancePerViewNV = 5278u32,
    LayerPerViewNV = 5279u32,
    MeshViewCountNV = 5280u32,
    MeshViewIndicesNV = 5281u32,
    BaryCoordKHR = 5286u32,
    BaryCoordNoPerspKHR = 5287u32,
    FragSizeEXT = 5292u32,
    FragInvocationCountEXT = 5293u32,
    PrimitivePointIndicesEXT = 5294u32,
    PrimitiveLineIndicesEXT = 5295u32,
    PrimitiveTriangleIndicesEXT = 5296u32,
    CullPrimitiveEXT = 5299u32,
    LaunchIdKHR = 5319u32,
    LaunchSizeKHR = 5320u32,
    WorldRayOriginKHR = 5321u32,
    WorldRayDirectionKHR = 5322u32,
    ObjectRayOriginKHR = 5323u32,
    ObjectRayDirectionKHR = 5324u32,
    RayTminKHR = 5325u32,
    RayTmaxKHR = 5326u32,
    InstanceCustomIndexKHR = 5327u32,
    ObjectToWorldKHR = 5330u32,
    WorldToObjectKHR = 5331u32,
    HitTNV = 5332u32,
    HitKindKHR = 5333u32,
    CurrentRayTimeNV = 5334u32,
    HitTriangleVertexPositionsKHR = 5335u32,
    HitMicroTriangleVertexPositionsNV = 5337u32,
    HitMicroTriangleVertexBarycentricsNV = 5344u32,
    IncomingRayFlagsKHR = 5351u32,
    RayGeometryIndexKHR = 5352u32,
    HitIsSphereNV = 5359u32,
    HitIsLSSNV = 5360u32,
    HitSpherePositionNV = 5361u32,
    WarpsPerSMNV = 5374u32,
    SMCountNV = 5375u32,
    WarpIDNV = 5376u32,
    SMIDNV = 5377u32,
    HitLSSPositionsNV = 5396u32,
    HitKindFrontFacingMicroTriangleNV = 5405u32,
    HitKindBackFacingMicroTriangleNV = 5406u32,
    HitSphereRadiusNV = 5420u32,
    HitLSSRadiiNV = 5421u32,
    ClusterIDNV = 5436u32,
    CullMaskKHR = 6021u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for BuiltIn {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for BuiltIn {}
unsafe impl Operand for BuiltIn {
    const KIND: &OperandKind = &OPERAND_KIND_BUILT_IN;
}
unsafe impl OperandEncoding for BuiltIn {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Position,
            1u32 => Self::PointSize,
            3u32 => Self::ClipDistance,
            4u32 => Self::CullDistance,
            5u32 => Self::VertexId,
            6u32 => Self::InstanceId,
            7u32 => Self::PrimitiveId,
            8u32 => Self::InvocationId,
            9u32 => Self::Layer,
            10u32 => Self::ViewportIndex,
            11u32 => Self::TessLevelOuter,
            12u32 => Self::TessLevelInner,
            13u32 => Self::TessCoord,
            14u32 => Self::PatchVertices,
            15u32 => Self::FragCoord,
            16u32 => Self::PointCoord,
            17u32 => Self::FrontFacing,
            18u32 => Self::SampleId,
            19u32 => Self::SamplePosition,
            20u32 => Self::SampleMask,
            22u32 => Self::FragDepth,
            23u32 => Self::HelperInvocation,
            24u32 => Self::NumWorkgroups,
            25u32 => Self::WorkgroupSize,
            26u32 => Self::WorkgroupId,
            27u32 => Self::LocalInvocationId,
            28u32 => Self::GlobalInvocationId,
            29u32 => Self::LocalInvocationIndex,
            30u32 => Self::WorkDim,
            31u32 => Self::GlobalSize,
            32u32 => Self::EnqueuedWorkgroupSize,
            33u32 => Self::GlobalOffset,
            34u32 => Self::GlobalLinearId,
            36u32 => Self::SubgroupSize,
            37u32 => Self::SubgroupMaxSize,
            38u32 => Self::NumSubgroups,
            39u32 => Self::NumEnqueuedSubgroups,
            40u32 => Self::SubgroupId,
            41u32 => Self::SubgroupLocalInvocationId,
            42u32 => Self::VertexIndex,
            43u32 => Self::InstanceIndex,
            4160u32 => Self::CoreIDARM,
            4161u32 => Self::CoreCountARM,
            4162u32 => Self::CoreMaxIDARM,
            4163u32 => Self::WarpIDARM,
            4164u32 => Self::WarpMaxIDARM,
            4416u32 => Self::SubgroupEqMask,
            4417u32 => Self::SubgroupGeMask,
            4418u32 => Self::SubgroupGtMask,
            4419u32 => Self::SubgroupLeMask,
            4420u32 => Self::SubgroupLtMask,
            4424u32 => Self::BaseVertex,
            4425u32 => Self::BaseInstance,
            4426u32 => Self::DrawIndex,
            4432u32 => Self::PrimitiveShadingRateKHR,
            4438u32 => Self::DeviceIndex,
            4440u32 => Self::ViewIndex,
            4444u32 => Self::ShadingRateKHR,
            4492u32 => Self::TileOffsetQCOM,
            4493u32 => Self::TileDimensionQCOM,
            4494u32 => Self::TileApronSizeQCOM,
            4992u32 => Self::BaryCoordNoPerspAMD,
            4993u32 => Self::BaryCoordNoPerspCentroidAMD,
            4994u32 => Self::BaryCoordNoPerspSampleAMD,
            4995u32 => Self::BaryCoordSmoothAMD,
            4996u32 => Self::BaryCoordSmoothCentroidAMD,
            4997u32 => Self::BaryCoordSmoothSampleAMD,
            4998u32 => Self::BaryCoordPullModelAMD,
            5014u32 => Self::FragStencilRefEXT,
            5021u32 => Self::RemainingRecursionLevelsAMDX,
            5073u32 => Self::ShaderIndexAMDX,
            5122u32 => Self::SamplerHeapEXT,
            5123u32 => Self::ResourceHeapEXT,
            5253u32 => Self::ViewportMaskNV,
            5257u32 => Self::SecondaryPositionNV,
            5258u32 => Self::SecondaryViewportMaskNV,
            5261u32 => Self::PositionPerViewNV,
            5262u32 => Self::ViewportMaskPerViewNV,
            5264u32 => Self::FullyCoveredEXT,
            5274u32 => Self::TaskCountNV,
            5275u32 => Self::PrimitiveCountNV,
            5276u32 => Self::PrimitiveIndicesNV,
            5277u32 => Self::ClipDistancePerViewNV,
            5278u32 => Self::CullDistancePerViewNV,
            5279u32 => Self::LayerPerViewNV,
            5280u32 => Self::MeshViewCountNV,
            5281u32 => Self::MeshViewIndicesNV,
            5286u32 => Self::BaryCoordKHR,
            5287u32 => Self::BaryCoordNoPerspKHR,
            5292u32 => Self::FragSizeEXT,
            5293u32 => Self::FragInvocationCountEXT,
            5294u32 => Self::PrimitivePointIndicesEXT,
            5295u32 => Self::PrimitiveLineIndicesEXT,
            5296u32 => Self::PrimitiveTriangleIndicesEXT,
            5299u32 => Self::CullPrimitiveEXT,
            5319u32 => Self::LaunchIdKHR,
            5320u32 => Self::LaunchSizeKHR,
            5321u32 => Self::WorldRayOriginKHR,
            5322u32 => Self::WorldRayDirectionKHR,
            5323u32 => Self::ObjectRayOriginKHR,
            5324u32 => Self::ObjectRayDirectionKHR,
            5325u32 => Self::RayTminKHR,
            5326u32 => Self::RayTmaxKHR,
            5327u32 => Self::InstanceCustomIndexKHR,
            5330u32 => Self::ObjectToWorldKHR,
            5331u32 => Self::WorldToObjectKHR,
            5332u32 => Self::HitTNV,
            5333u32 => Self::HitKindKHR,
            5334u32 => Self::CurrentRayTimeNV,
            5335u32 => Self::HitTriangleVertexPositionsKHR,
            5337u32 => Self::HitMicroTriangleVertexPositionsNV,
            5344u32 => Self::HitMicroTriangleVertexBarycentricsNV,
            5351u32 => Self::IncomingRayFlagsKHR,
            5352u32 => Self::RayGeometryIndexKHR,
            5359u32 => Self::HitIsSphereNV,
            5360u32 => Self::HitIsLSSNV,
            5361u32 => Self::HitSpherePositionNV,
            5374u32 => Self::WarpsPerSMNV,
            5375u32 => Self::SMCountNV,
            5376u32 => Self::WarpIDNV,
            5377u32 => Self::SMIDNV,
            5396u32 => Self::HitLSSPositionsNV,
            5405u32 => Self::HitKindFrontFacingMicroTriangleNV,
            5406u32 => Self::HitKindBackFacingMicroTriangleNV,
            5420u32 => Self::HitSphereRadiusNV,
            5421u32 => Self::HitLSSRadiiNV,
            5436u32 => Self::ClusterIDNV,
            6021u32 => Self::CullMaskKHR,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(BuiltIn),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Scope {
    CrossDevice = 0u32,
    Device = 1u32,
    Workgroup = 2u32,
    Subgroup = 3u32,
    Invocation = 4u32,
    ///Since SPIR-V 1.5
    QueueFamily = 5u32,
    ShaderCallKHR = 6u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Scope {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Scope {}
unsafe impl Operand for Scope {
    const KIND: &OperandKind = &OPERAND_KIND_SCOPE;
}
unsafe impl OperandEncoding for Scope {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::CrossDevice,
            1u32 => Self::Device,
            2u32 => Self::Workgroup,
            3u32 => Self::Subgroup,
            4u32 => Self::Invocation,
            5u32 => Self::QueueFamily,
            6u32 => Self::ShaderCallKHR,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(Scope),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GroupOperation {
    Reduce = 0u32,
    InclusiveScan = 1u32,
    ExclusiveScan = 2u32,
    ///Since SPIR-V 1.3
    ClusteredReduce = 3u32,
    PartitionedReduceEXT = 6u32,
    PartitionedInclusiveScanEXT = 7u32,
    PartitionedExclusiveScanEXT = 8u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for GroupOperation {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for GroupOperation {}
unsafe impl Operand for GroupOperation {
    const KIND: &OperandKind = &OPERAND_KIND_GROUP_OPERATION;
}
unsafe impl OperandEncoding for GroupOperation {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Reduce,
            1u32 => Self::InclusiveScan,
            2u32 => Self::ExclusiveScan,
            3u32 => Self::ClusteredReduce,
            6u32 => Self::PartitionedReduceEXT,
            7u32 => Self::PartitionedInclusiveScanEXT,
            8u32 => Self::PartitionedExclusiveScanEXT,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(GroupOperation),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum KernelEnqueueFlags {
    NoWait = 0u32,
    WaitKernel = 1u32,
    WaitWorkGroup = 2u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for KernelEnqueueFlags {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for KernelEnqueueFlags {}
unsafe impl Operand for KernelEnqueueFlags {
    const KIND: &OperandKind = &OPERAND_KIND_KERNEL_ENQUEUE_FLAGS;
}
unsafe impl OperandEncoding for KernelEnqueueFlags {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::NoWait,
            1u32 => Self::WaitKernel,
            2u32 => Self::WaitWorkGroup,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(KernelEnqueueFlags),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Capability {
    Matrix = 0u32,
    Shader = 1u32,
    Geometry = 2u32,
    Tessellation = 3u32,
    Addresses = 4u32,
    Linkage = 5u32,
    Kernel = 6u32,
    Vector16 = 7u32,
    Float16Buffer = 8u32,
    Float16 = 9u32,
    Float64 = 10u32,
    Int64 = 11u32,
    Int64Atomics = 12u32,
    ImageBasic = 13u32,
    ImageReadWrite = 14u32,
    ImageMipmap = 15u32,
    Pipes = 17u32,
    Groups = 18u32,
    DeviceEnqueue = 19u32,
    LiteralSampler = 20u32,
    AtomicStorage = 21u32,
    Int16 = 22u32,
    TessellationPointSize = 23u32,
    GeometryPointSize = 24u32,
    ImageGatherExtended = 25u32,
    StorageImageMultisample = 27u32,
    UniformBufferArrayDynamicIndexing = 28u32,
    SampledImageArrayDynamicIndexing = 29u32,
    StorageBufferArrayDynamicIndexing = 30u32,
    StorageImageArrayDynamicIndexing = 31u32,
    ClipDistance = 32u32,
    CullDistance = 33u32,
    ImageCubeArray = 34u32,
    SampleRateShading = 35u32,
    ImageRect = 36u32,
    SampledRect = 37u32,
    GenericPointer = 38u32,
    Int8 = 39u32,
    InputAttachment = 40u32,
    SparseResidency = 41u32,
    MinLod = 42u32,
    Sampled1D = 43u32,
    Image1D = 44u32,
    SampledCubeArray = 45u32,
    SampledBuffer = 46u32,
    ImageBuffer = 47u32,
    ImageMSArray = 48u32,
    StorageImageExtendedFormats = 49u32,
    ImageQuery = 50u32,
    DerivativeControl = 51u32,
    InterpolationFunction = 52u32,
    TransformFeedback = 53u32,
    GeometryStreams = 54u32,
    StorageImageReadWithoutFormat = 55u32,
    StorageImageWriteWithoutFormat = 56u32,
    MultiViewport = 57u32,
    ///Since SPIR-V 1.1
    SubgroupDispatch = 58u32,
    ///Since SPIR-V 1.1
    NamedBarrier = 59u32,
    ///Since SPIR-V 1.1
    PipeStorage = 60u32,
    ///Since SPIR-V 1.3
    GroupNonUniform = 61u32,
    ///Since SPIR-V 1.3
    GroupNonUniformVote = 62u32,
    ///Since SPIR-V 1.3
    GroupNonUniformArithmetic = 63u32,
    ///Since SPIR-V 1.3
    GroupNonUniformBallot = 64u32,
    ///Since SPIR-V 1.3
    GroupNonUniformShuffle = 65u32,
    ///Since SPIR-V 1.3
    GroupNonUniformShuffleRelative = 66u32,
    ///Since SPIR-V 1.3
    GroupNonUniformClustered = 67u32,
    ///Since SPIR-V 1.3
    GroupNonUniformQuad = 68u32,
    ///Since SPIR-V 1.5
    ShaderLayer = 69u32,
    ///Since SPIR-V 1.5
    ShaderViewportIndex = 70u32,
    ///Since SPIR-V 1.6
    UniformDecoration = 71u32,
    CoreBuiltinsARM = 4165u32,
    TileImageColorReadAccessEXT = 4166u32,
    TileImageDepthReadAccessEXT = 4167u32,
    TileImageStencilReadAccessEXT = 4168u32,
    TensorsARM = 4174u32,
    StorageTensorArrayDynamicIndexingARM = 4175u32,
    StorageTensorArrayNonUniformIndexingARM = 4176u32,
    GraphARM = 4191u32,
    CooperativeMatrixLayoutsARM = 4201u32,
    Float8EXT = 4212u32,
    Float8CooperativeMatrixEXT = 4213u32,
    FragmentShadingRateKHR = 4422u32,
    SubgroupBallotKHR = 4423u32,
    ///Since SPIR-V 1.3
    DrawParameters = 4427u32,
    WorkgroupMemoryExplicitLayoutKHR = 4428u32,
    WorkgroupMemoryExplicitLayout8BitAccessKHR = 4429u32,
    WorkgroupMemoryExplicitLayout16BitAccessKHR = 4430u32,
    SubgroupVoteKHR = 4431u32,
    ///Since SPIR-V 1.3
    StorageBuffer16BitAccess = 4433u32,
    ///Since SPIR-V 1.3
    UniformAndStorageBuffer16BitAccess = 4434u32,
    ///Since SPIR-V 1.3
    StoragePushConstant16 = 4435u32,
    ///Since SPIR-V 1.3
    StorageInputOutput16 = 4436u32,
    ///Since SPIR-V 1.3
    DeviceGroup = 4437u32,
    ///Since SPIR-V 1.3
    MultiView = 4439u32,
    ///Since SPIR-V 1.3
    VariablePointersStorageBuffer = 4441u32,
    ///Since SPIR-V 1.3
    VariablePointers = 4442u32,
    AtomicStorageOps = 4445u32,
    SampleMaskPostDepthCoverage = 4447u32,
    ///Since SPIR-V 1.5
    StorageBuffer8BitAccess = 4448u32,
    ///Since SPIR-V 1.5
    UniformAndStorageBuffer8BitAccess = 4449u32,
    ///Since SPIR-V 1.5
    StoragePushConstant8 = 4450u32,
    ///Since SPIR-V 1.4
    DenormPreserve = 4464u32,
    ///Since SPIR-V 1.4
    DenormFlushToZero = 4465u32,
    ///Since SPIR-V 1.4
    SignedZeroInfNanPreserve = 4466u32,
    ///Since SPIR-V 1.4
    RoundingModeRTE = 4467u32,
    ///Since SPIR-V 1.4
    RoundingModeRTZ = 4468u32,
    RayQueryProvisionalKHR = 4471u32,
    RayQueryKHR = 4472u32,
    UntypedPointersKHR = 4473u32,
    RayTraversalPrimitiveCullingKHR = 4478u32,
    RayTracingKHR = 4479u32,
    TextureSampleWeightedQCOM = 4484u32,
    TextureBoxFilterQCOM = 4485u32,
    TextureBlockMatchQCOM = 4486u32,
    TileShadingQCOM = 4495u32,
    CooperativeMatrixConversionQCOM = 4496u32,
    TextureBlockMatch2QCOM = 4498u32,
    Float16ImageAMD = 5008u32,
    ImageGatherBiasLodAMD = 5009u32,
    FragmentMaskAMD = 5010u32,
    StencilExportEXT = 5013u32,
    ImageReadWriteLodAMD = 5015u32,
    Int64ImageEXT = 5016u32,
    ShaderClockKHR = 5055u32,
    ShaderEnqueueAMDX = 5067u32,
    QuadControlKHR = 5087u32,
    Int4TypeINTEL = 5112u32,
    Int4CooperativeMatrixINTEL = 5114u32,
    BFloat16TypeKHR = 5116u32,
    BFloat16DotProductKHR = 5117u32,
    BFloat16CooperativeMatrixKHR = 5118u32,
    DescriptorHeapEXT = 5128u32,
    SampleMaskOverrideCoverageNV = 5249u32,
    GeometryShaderPassthroughNV = 5251u32,
    ShaderViewportIndexLayerEXT = 5254u32,
    ShaderViewportMaskNV = 5255u32,
    ShaderStereoViewNV = 5259u32,
    PerViewAttributesNV = 5260u32,
    FragmentFullyCoveredEXT = 5265u32,
    MeshShadingNV = 5266u32,
    ImageFootprintNV = 5282u32,
    MeshShadingEXT = 5283u32,
    FragmentBarycentricKHR = 5284u32,
    ComputeDerivativeGroupQuadsKHR = 5288u32,
    FragmentDensityEXT = 5291u32,
    GroupNonUniformPartitionedEXT = 5297u32,
    ///Since SPIR-V 1.5
    ShaderNonUniform = 5301u32,
    ///Since SPIR-V 1.5
    RuntimeDescriptorArray = 5302u32,
    ///Since SPIR-V 1.5
    InputAttachmentArrayDynamicIndexing = 5303u32,
    ///Since SPIR-V 1.5
    UniformTexelBufferArrayDynamicIndexing = 5304u32,
    ///Since SPIR-V 1.5
    StorageTexelBufferArrayDynamicIndexing = 5305u32,
    ///Since SPIR-V 1.5
    UniformBufferArrayNonUniformIndexing = 5306u32,
    ///Since SPIR-V 1.5
    SampledImageArrayNonUniformIndexing = 5307u32,
    ///Since SPIR-V 1.5
    StorageBufferArrayNonUniformIndexing = 5308u32,
    ///Since SPIR-V 1.5
    StorageImageArrayNonUniformIndexing = 5309u32,
    ///Since SPIR-V 1.5
    InputAttachmentArrayNonUniformIndexing = 5310u32,
    ///Since SPIR-V 1.5
    UniformTexelBufferArrayNonUniformIndexing = 5311u32,
    ///Since SPIR-V 1.5
    StorageTexelBufferArrayNonUniformIndexing = 5312u32,
    RayTracingPositionFetchKHR = 5336u32,
    RayTracingNV = 5340u32,
    RayTracingMotionBlurNV = 5341u32,
    ///Since SPIR-V 1.5
    VulkanMemoryModel = 5345u32,
    ///Since SPIR-V 1.5
    VulkanMemoryModelDeviceScope = 5346u32,
    ///Since SPIR-V 1.5
    PhysicalStorageBufferAddresses = 5347u32,
    ComputeDerivativeGroupLinearKHR = 5350u32,
    RayTracingProvisionalKHR = 5353u32,
    CooperativeMatrixNV = 5357u32,
    FragmentShaderSampleInterlockEXT = 5363u32,
    FragmentShaderShadingRateInterlockEXT = 5372u32,
    ShaderSMBuiltinsNV = 5373u32,
    FragmentShaderPixelInterlockEXT = 5378u32,
    ///Since SPIR-V 1.6
    DemoteToHelperInvocation = 5379u32,
    DisplacementMicromapNV = 5380u32,
    RayTracingOpacityMicromapEXT = 5381u32,
    ShaderInvocationReorderNV = 5383u32,
    ShaderInvocationReorderEXT = 5388u32,
    BindlessTextureNV = 5390u32,
    RayQueryPositionFetchKHR = 5391u32,
    CooperativeVectorNV = 5394u32,
    AtomicFloat16VectorNV = 5404u32,
    RayTracingDisplacementMicromapNV = 5409u32,
    RawAccessChainsNV = 5414u32,
    RayTracingSpheresGeometryNV = 5418u32,
    RayTracingLinearSweptSpheresGeometryNV = 5419u32,
    PushConstantBanksNV = 5423u32,
    LongVectorEXT = 5425u32,
    Shader64BitIndexingEXT = 5426u32,
    CooperativeMatrixReductionsNV = 5430u32,
    CooperativeMatrixConversionsNV = 5431u32,
    CooperativeMatrixPerElementOperationsNV = 5432u32,
    CooperativeMatrixTensorAddressingNV = 5433u32,
    CooperativeMatrixBlockLoadsNV = 5434u32,
    CooperativeVectorTrainingNV = 5435u32,
    RayTracingClusterAccelerationStructureNV = 5437u32,
    TensorAddressingNV = 5439u32,
    SubgroupShuffleINTEL = 5568u32,
    SubgroupBufferBlockIOINTEL = 5569u32,
    SubgroupImageBlockIOINTEL = 5570u32,
    SubgroupImageMediaBlockIOINTEL = 5579u32,
    RoundToInfinityINTEL = 5582u32,
    FloatingPointModeINTEL = 5583u32,
    IntegerFunctions2INTEL = 5584u32,
    FunctionPointersINTEL = 5603u32,
    IndirectReferencesINTEL = 5604u32,
    AsmINTEL = 5606u32,
    AtomicFloat32MinMaxEXT = 5612u32,
    AtomicFloat64MinMaxEXT = 5613u32,
    AtomicFloat16MinMaxEXT = 5616u32,
    VectorComputeINTEL = 5617u32,
    VectorAnyINTEL = 5619u32,
    ExpectAssumeKHR = 5629u32,
    SubgroupAvcMotionEstimationINTEL = 5696u32,
    SubgroupAvcMotionEstimationIntraINTEL = 5697u32,
    SubgroupAvcMotionEstimationChromaINTEL = 5698u32,
    VariableLengthArrayINTEL = 5817u32,
    FunctionFloatControlINTEL = 5821u32,
    FPGAMemoryAttributesALTERA = 5824u32,
    FPFastMathModeINTEL = 5837u32,
    ArbitraryPrecisionIntegersALTERA = 5844u32,
    ArbitraryPrecisionFloatingPointALTERA = 5845u32,
    UnstructuredLoopControlsINTEL = 5886u32,
    FPGALoopControlsALTERA = 5888u32,
    KernelAttributesINTEL = 5892u32,
    FPGAKernelAttributesINTEL = 5897u32,
    FPGAMemoryAccessesALTERA = 5898u32,
    FPGAClusterAttributesALTERA = 5904u32,
    LoopFuseALTERA = 5906u32,
    FPGADSPControlALTERA = 5908u32,
    MemoryAccessAliasingINTEL = 5910u32,
    FPGAInvocationPipeliningAttributesALTERA = 5916u32,
    FPGABufferLocationALTERA = 5920u32,
    ArbitraryPrecisionFixedPointALTERA = 5922u32,
    USMStorageClassesALTERA = 5935u32,
    RuntimeAlignedAttributeALTERA = 5939u32,
    IOPipesALTERA = 5943u32,
    BlockingPipesALTERA = 5945u32,
    FPGARegALTERA = 5948u32,
    ///Since SPIR-V 1.6
    DotProductInputAll = 6016u32,
    ///Since SPIR-V 1.6
    DotProductInput4x8Bit = 6017u32,
    ///Since SPIR-V 1.6
    DotProductInput4x8BitPacked = 6018u32,
    ///Since SPIR-V 1.6
    DotProduct = 6019u32,
    RayCullMaskKHR = 6020u32,
    CooperativeMatrixKHR = 6022u32,
    ReplicatedCompositesEXT = 6024u32,
    BitInstructions = 6025u32,
    GroupNonUniformRotateKHR = 6026u32,
    FloatControls2 = 6029u32,
    FMAKHR = 6030u32,
    AtomicFloat32AddEXT = 6033u32,
    AtomicFloat64AddEXT = 6034u32,
    LongCompositesINTEL = 6089u32,
    OptNoneEXT = 6094u32,
    AtomicFloat16AddEXT = 6095u32,
    DebugInfoModuleINTEL = 6114u32,
    BFloat16ConversionINTEL = 6115u32,
    SplitBarrierINTEL = 6141u32,
    ArithmeticFenceEXT = 6144u32,
    FPGAClusterAttributesV2ALTERA = 6150u32,
    FPGAKernelAttributesv2INTEL = 6161u32,
    TaskSequenceALTERA = 6162u32,
    FPMaxErrorINTEL = 6169u32,
    FPGALatencyControlALTERA = 6171u32,
    FPGAArgumentInterfacesALTERA = 6174u32,
    GlobalVariableHostAccessINTEL = 6187u32,
    GlobalVariableFPGADecorationsALTERA = 6189u32,
    SubgroupBufferPrefetchINTEL = 6220u32,
    Subgroup2DBlockIOINTEL = 6228u32,
    Subgroup2DBlockTransformINTEL = 6229u32,
    Subgroup2DBlockTransposeINTEL = 6230u32,
    SubgroupMatrixMultiplyAccumulateINTEL = 6236u32,
    TernaryBitwiseFunctionINTEL = 6241u32,
    UntypedVariableLengthArrayINTEL = 6243u32,
    SpecConditionalINTEL = 6245u32,
    FunctionVariantsINTEL = 6246u32,
    GroupUniformArithmeticKHR = 6400u32,
    TensorFloat32RoundingINTEL = 6425u32,
    MaskedGatherScatterINTEL = 6427u32,
    CacheControlsINTEL = 6441u32,
    RegisterLimitsINTEL = 6460u32,
    BindlessImagesINTEL = 6528u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Capability {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Capability {}
unsafe impl Operand for Capability {
    const KIND: &OperandKind = &OPERAND_KIND_CAPABILITY;
}
unsafe impl OperandEncoding for Capability {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Matrix,
            1u32 => Self::Shader,
            2u32 => Self::Geometry,
            3u32 => Self::Tessellation,
            4u32 => Self::Addresses,
            5u32 => Self::Linkage,
            6u32 => Self::Kernel,
            7u32 => Self::Vector16,
            8u32 => Self::Float16Buffer,
            9u32 => Self::Float16,
            10u32 => Self::Float64,
            11u32 => Self::Int64,
            12u32 => Self::Int64Atomics,
            13u32 => Self::ImageBasic,
            14u32 => Self::ImageReadWrite,
            15u32 => Self::ImageMipmap,
            17u32 => Self::Pipes,
            18u32 => Self::Groups,
            19u32 => Self::DeviceEnqueue,
            20u32 => Self::LiteralSampler,
            21u32 => Self::AtomicStorage,
            22u32 => Self::Int16,
            23u32 => Self::TessellationPointSize,
            24u32 => Self::GeometryPointSize,
            25u32 => Self::ImageGatherExtended,
            27u32 => Self::StorageImageMultisample,
            28u32 => Self::UniformBufferArrayDynamicIndexing,
            29u32 => Self::SampledImageArrayDynamicIndexing,
            30u32 => Self::StorageBufferArrayDynamicIndexing,
            31u32 => Self::StorageImageArrayDynamicIndexing,
            32u32 => Self::ClipDistance,
            33u32 => Self::CullDistance,
            34u32 => Self::ImageCubeArray,
            35u32 => Self::SampleRateShading,
            36u32 => Self::ImageRect,
            37u32 => Self::SampledRect,
            38u32 => Self::GenericPointer,
            39u32 => Self::Int8,
            40u32 => Self::InputAttachment,
            41u32 => Self::SparseResidency,
            42u32 => Self::MinLod,
            43u32 => Self::Sampled1D,
            44u32 => Self::Image1D,
            45u32 => Self::SampledCubeArray,
            46u32 => Self::SampledBuffer,
            47u32 => Self::ImageBuffer,
            48u32 => Self::ImageMSArray,
            49u32 => Self::StorageImageExtendedFormats,
            50u32 => Self::ImageQuery,
            51u32 => Self::DerivativeControl,
            52u32 => Self::InterpolationFunction,
            53u32 => Self::TransformFeedback,
            54u32 => Self::GeometryStreams,
            55u32 => Self::StorageImageReadWithoutFormat,
            56u32 => Self::StorageImageWriteWithoutFormat,
            57u32 => Self::MultiViewport,
            58u32 => Self::SubgroupDispatch,
            59u32 => Self::NamedBarrier,
            60u32 => Self::PipeStorage,
            61u32 => Self::GroupNonUniform,
            62u32 => Self::GroupNonUniformVote,
            63u32 => Self::GroupNonUniformArithmetic,
            64u32 => Self::GroupNonUniformBallot,
            65u32 => Self::GroupNonUniformShuffle,
            66u32 => Self::GroupNonUniformShuffleRelative,
            67u32 => Self::GroupNonUniformClustered,
            68u32 => Self::GroupNonUniformQuad,
            69u32 => Self::ShaderLayer,
            70u32 => Self::ShaderViewportIndex,
            71u32 => Self::UniformDecoration,
            4165u32 => Self::CoreBuiltinsARM,
            4166u32 => Self::TileImageColorReadAccessEXT,
            4167u32 => Self::TileImageDepthReadAccessEXT,
            4168u32 => Self::TileImageStencilReadAccessEXT,
            4174u32 => Self::TensorsARM,
            4175u32 => Self::StorageTensorArrayDynamicIndexingARM,
            4176u32 => Self::StorageTensorArrayNonUniformIndexingARM,
            4191u32 => Self::GraphARM,
            4201u32 => Self::CooperativeMatrixLayoutsARM,
            4212u32 => Self::Float8EXT,
            4213u32 => Self::Float8CooperativeMatrixEXT,
            4422u32 => Self::FragmentShadingRateKHR,
            4423u32 => Self::SubgroupBallotKHR,
            4427u32 => Self::DrawParameters,
            4428u32 => Self::WorkgroupMemoryExplicitLayoutKHR,
            4429u32 => Self::WorkgroupMemoryExplicitLayout8BitAccessKHR,
            4430u32 => Self::WorkgroupMemoryExplicitLayout16BitAccessKHR,
            4431u32 => Self::SubgroupVoteKHR,
            4433u32 => Self::StorageBuffer16BitAccess,
            4434u32 => Self::UniformAndStorageBuffer16BitAccess,
            4435u32 => Self::StoragePushConstant16,
            4436u32 => Self::StorageInputOutput16,
            4437u32 => Self::DeviceGroup,
            4439u32 => Self::MultiView,
            4441u32 => Self::VariablePointersStorageBuffer,
            4442u32 => Self::VariablePointers,
            4445u32 => Self::AtomicStorageOps,
            4447u32 => Self::SampleMaskPostDepthCoverage,
            4448u32 => Self::StorageBuffer8BitAccess,
            4449u32 => Self::UniformAndStorageBuffer8BitAccess,
            4450u32 => Self::StoragePushConstant8,
            4464u32 => Self::DenormPreserve,
            4465u32 => Self::DenormFlushToZero,
            4466u32 => Self::SignedZeroInfNanPreserve,
            4467u32 => Self::RoundingModeRTE,
            4468u32 => Self::RoundingModeRTZ,
            4471u32 => Self::RayQueryProvisionalKHR,
            4472u32 => Self::RayQueryKHR,
            4473u32 => Self::UntypedPointersKHR,
            4478u32 => Self::RayTraversalPrimitiveCullingKHR,
            4479u32 => Self::RayTracingKHR,
            4484u32 => Self::TextureSampleWeightedQCOM,
            4485u32 => Self::TextureBoxFilterQCOM,
            4486u32 => Self::TextureBlockMatchQCOM,
            4495u32 => Self::TileShadingQCOM,
            4496u32 => Self::CooperativeMatrixConversionQCOM,
            4498u32 => Self::TextureBlockMatch2QCOM,
            5008u32 => Self::Float16ImageAMD,
            5009u32 => Self::ImageGatherBiasLodAMD,
            5010u32 => Self::FragmentMaskAMD,
            5013u32 => Self::StencilExportEXT,
            5015u32 => Self::ImageReadWriteLodAMD,
            5016u32 => Self::Int64ImageEXT,
            5055u32 => Self::ShaderClockKHR,
            5067u32 => Self::ShaderEnqueueAMDX,
            5087u32 => Self::QuadControlKHR,
            5112u32 => Self::Int4TypeINTEL,
            5114u32 => Self::Int4CooperativeMatrixINTEL,
            5116u32 => Self::BFloat16TypeKHR,
            5117u32 => Self::BFloat16DotProductKHR,
            5118u32 => Self::BFloat16CooperativeMatrixKHR,
            5128u32 => Self::DescriptorHeapEXT,
            5249u32 => Self::SampleMaskOverrideCoverageNV,
            5251u32 => Self::GeometryShaderPassthroughNV,
            5254u32 => Self::ShaderViewportIndexLayerEXT,
            5255u32 => Self::ShaderViewportMaskNV,
            5259u32 => Self::ShaderStereoViewNV,
            5260u32 => Self::PerViewAttributesNV,
            5265u32 => Self::FragmentFullyCoveredEXT,
            5266u32 => Self::MeshShadingNV,
            5282u32 => Self::ImageFootprintNV,
            5283u32 => Self::MeshShadingEXT,
            5284u32 => Self::FragmentBarycentricKHR,
            5288u32 => Self::ComputeDerivativeGroupQuadsKHR,
            5291u32 => Self::FragmentDensityEXT,
            5297u32 => Self::GroupNonUniformPartitionedEXT,
            5301u32 => Self::ShaderNonUniform,
            5302u32 => Self::RuntimeDescriptorArray,
            5303u32 => Self::InputAttachmentArrayDynamicIndexing,
            5304u32 => Self::UniformTexelBufferArrayDynamicIndexing,
            5305u32 => Self::StorageTexelBufferArrayDynamicIndexing,
            5306u32 => Self::UniformBufferArrayNonUniformIndexing,
            5307u32 => Self::SampledImageArrayNonUniformIndexing,
            5308u32 => Self::StorageBufferArrayNonUniformIndexing,
            5309u32 => Self::StorageImageArrayNonUniformIndexing,
            5310u32 => Self::InputAttachmentArrayNonUniformIndexing,
            5311u32 => Self::UniformTexelBufferArrayNonUniformIndexing,
            5312u32 => Self::StorageTexelBufferArrayNonUniformIndexing,
            5336u32 => Self::RayTracingPositionFetchKHR,
            5340u32 => Self::RayTracingNV,
            5341u32 => Self::RayTracingMotionBlurNV,
            5345u32 => Self::VulkanMemoryModel,
            5346u32 => Self::VulkanMemoryModelDeviceScope,
            5347u32 => Self::PhysicalStorageBufferAddresses,
            5350u32 => Self::ComputeDerivativeGroupLinearKHR,
            5353u32 => Self::RayTracingProvisionalKHR,
            5357u32 => Self::CooperativeMatrixNV,
            5363u32 => Self::FragmentShaderSampleInterlockEXT,
            5372u32 => Self::FragmentShaderShadingRateInterlockEXT,
            5373u32 => Self::ShaderSMBuiltinsNV,
            5378u32 => Self::FragmentShaderPixelInterlockEXT,
            5379u32 => Self::DemoteToHelperInvocation,
            5380u32 => Self::DisplacementMicromapNV,
            5381u32 => Self::RayTracingOpacityMicromapEXT,
            5383u32 => Self::ShaderInvocationReorderNV,
            5388u32 => Self::ShaderInvocationReorderEXT,
            5390u32 => Self::BindlessTextureNV,
            5391u32 => Self::RayQueryPositionFetchKHR,
            5394u32 => Self::CooperativeVectorNV,
            5404u32 => Self::AtomicFloat16VectorNV,
            5409u32 => Self::RayTracingDisplacementMicromapNV,
            5414u32 => Self::RawAccessChainsNV,
            5418u32 => Self::RayTracingSpheresGeometryNV,
            5419u32 => Self::RayTracingLinearSweptSpheresGeometryNV,
            5423u32 => Self::PushConstantBanksNV,
            5425u32 => Self::LongVectorEXT,
            5426u32 => Self::Shader64BitIndexingEXT,
            5430u32 => Self::CooperativeMatrixReductionsNV,
            5431u32 => Self::CooperativeMatrixConversionsNV,
            5432u32 => Self::CooperativeMatrixPerElementOperationsNV,
            5433u32 => Self::CooperativeMatrixTensorAddressingNV,
            5434u32 => Self::CooperativeMatrixBlockLoadsNV,
            5435u32 => Self::CooperativeVectorTrainingNV,
            5437u32 => Self::RayTracingClusterAccelerationStructureNV,
            5439u32 => Self::TensorAddressingNV,
            5568u32 => Self::SubgroupShuffleINTEL,
            5569u32 => Self::SubgroupBufferBlockIOINTEL,
            5570u32 => Self::SubgroupImageBlockIOINTEL,
            5579u32 => Self::SubgroupImageMediaBlockIOINTEL,
            5582u32 => Self::RoundToInfinityINTEL,
            5583u32 => Self::FloatingPointModeINTEL,
            5584u32 => Self::IntegerFunctions2INTEL,
            5603u32 => Self::FunctionPointersINTEL,
            5604u32 => Self::IndirectReferencesINTEL,
            5606u32 => Self::AsmINTEL,
            5612u32 => Self::AtomicFloat32MinMaxEXT,
            5613u32 => Self::AtomicFloat64MinMaxEXT,
            5616u32 => Self::AtomicFloat16MinMaxEXT,
            5617u32 => Self::VectorComputeINTEL,
            5619u32 => Self::VectorAnyINTEL,
            5629u32 => Self::ExpectAssumeKHR,
            5696u32 => Self::SubgroupAvcMotionEstimationINTEL,
            5697u32 => Self::SubgroupAvcMotionEstimationIntraINTEL,
            5698u32 => Self::SubgroupAvcMotionEstimationChromaINTEL,
            5817u32 => Self::VariableLengthArrayINTEL,
            5821u32 => Self::FunctionFloatControlINTEL,
            5824u32 => Self::FPGAMemoryAttributesALTERA,
            5837u32 => Self::FPFastMathModeINTEL,
            5844u32 => Self::ArbitraryPrecisionIntegersALTERA,
            5845u32 => Self::ArbitraryPrecisionFloatingPointALTERA,
            5886u32 => Self::UnstructuredLoopControlsINTEL,
            5888u32 => Self::FPGALoopControlsALTERA,
            5892u32 => Self::KernelAttributesINTEL,
            5897u32 => Self::FPGAKernelAttributesINTEL,
            5898u32 => Self::FPGAMemoryAccessesALTERA,
            5904u32 => Self::FPGAClusterAttributesALTERA,
            5906u32 => Self::LoopFuseALTERA,
            5908u32 => Self::FPGADSPControlALTERA,
            5910u32 => Self::MemoryAccessAliasingINTEL,
            5916u32 => Self::FPGAInvocationPipeliningAttributesALTERA,
            5920u32 => Self::FPGABufferLocationALTERA,
            5922u32 => Self::ArbitraryPrecisionFixedPointALTERA,
            5935u32 => Self::USMStorageClassesALTERA,
            5939u32 => Self::RuntimeAlignedAttributeALTERA,
            5943u32 => Self::IOPipesALTERA,
            5945u32 => Self::BlockingPipesALTERA,
            5948u32 => Self::FPGARegALTERA,
            6016u32 => Self::DotProductInputAll,
            6017u32 => Self::DotProductInput4x8Bit,
            6018u32 => Self::DotProductInput4x8BitPacked,
            6019u32 => Self::DotProduct,
            6020u32 => Self::RayCullMaskKHR,
            6022u32 => Self::CooperativeMatrixKHR,
            6024u32 => Self::ReplicatedCompositesEXT,
            6025u32 => Self::BitInstructions,
            6026u32 => Self::GroupNonUniformRotateKHR,
            6029u32 => Self::FloatControls2,
            6030u32 => Self::FMAKHR,
            6033u32 => Self::AtomicFloat32AddEXT,
            6034u32 => Self::AtomicFloat64AddEXT,
            6089u32 => Self::LongCompositesINTEL,
            6094u32 => Self::OptNoneEXT,
            6095u32 => Self::AtomicFloat16AddEXT,
            6114u32 => Self::DebugInfoModuleINTEL,
            6115u32 => Self::BFloat16ConversionINTEL,
            6141u32 => Self::SplitBarrierINTEL,
            6144u32 => Self::ArithmeticFenceEXT,
            6150u32 => Self::FPGAClusterAttributesV2ALTERA,
            6161u32 => Self::FPGAKernelAttributesv2INTEL,
            6162u32 => Self::TaskSequenceALTERA,
            6169u32 => Self::FPMaxErrorINTEL,
            6171u32 => Self::FPGALatencyControlALTERA,
            6174u32 => Self::FPGAArgumentInterfacesALTERA,
            6187u32 => Self::GlobalVariableHostAccessINTEL,
            6189u32 => Self::GlobalVariableFPGADecorationsALTERA,
            6220u32 => Self::SubgroupBufferPrefetchINTEL,
            6228u32 => Self::Subgroup2DBlockIOINTEL,
            6229u32 => Self::Subgroup2DBlockTransformINTEL,
            6230u32 => Self::Subgroup2DBlockTransposeINTEL,
            6236u32 => Self::SubgroupMatrixMultiplyAccumulateINTEL,
            6241u32 => Self::TernaryBitwiseFunctionINTEL,
            6243u32 => Self::UntypedVariableLengthArrayINTEL,
            6245u32 => Self::SpecConditionalINTEL,
            6246u32 => Self::FunctionVariantsINTEL,
            6400u32 => Self::GroupUniformArithmeticKHR,
            6425u32 => Self::TensorFloat32RoundingINTEL,
            6427u32 => Self::MaskedGatherScatterINTEL,
            6441u32 => Self::CacheControlsINTEL,
            6460u32 => Self::RegisterLimitsINTEL,
            6528u32 => Self::BindlessImagesINTEL,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(Capability),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RayQueryIntersection {
    RayQueryCandidateIntersectionKHR = 0u32,
    RayQueryCommittedIntersectionKHR = 1u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for RayQueryIntersection {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for RayQueryIntersection {}
unsafe impl Operand for RayQueryIntersection {
    const KIND: &OperandKind = &OPERAND_KIND_RAY_QUERY_INTERSECTION;
}
unsafe impl OperandEncoding for RayQueryIntersection {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::RayQueryCandidateIntersectionKHR,
            1u32 => Self::RayQueryCommittedIntersectionKHR,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(RayQueryIntersection),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RayQueryCommittedIntersectionType {
    RayQueryCommittedIntersectionNoneKHR = 0u32,
    RayQueryCommittedIntersectionTriangleKHR = 1u32,
    RayQueryCommittedIntersectionGeneratedKHR = 2u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for RayQueryCommittedIntersectionType {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for RayQueryCommittedIntersectionType {}
unsafe impl Operand for RayQueryCommittedIntersectionType {
    const KIND: &OperandKind = &OPERAND_KIND_RAY_QUERY_COMMITTED_INTERSECTION_TYPE;
}
unsafe impl OperandEncoding for RayQueryCommittedIntersectionType {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::RayQueryCommittedIntersectionNoneKHR,
            1u32 => Self::RayQueryCommittedIntersectionTriangleKHR,
            2u32 => Self::RayQueryCommittedIntersectionGeneratedKHR,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(RayQueryCommittedIntersectionType),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RayQueryCandidateIntersectionType {
    RayQueryCandidateIntersectionTriangleKHR = 0u32,
    RayQueryCandidateIntersectionAABBKHR = 1u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for RayQueryCandidateIntersectionType {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for RayQueryCandidateIntersectionType {}
unsafe impl Operand for RayQueryCandidateIntersectionType {
    const KIND: &OperandKind = &OPERAND_KIND_RAY_QUERY_CANDIDATE_INTERSECTION_TYPE;
}
unsafe impl OperandEncoding for RayQueryCandidateIntersectionType {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::RayQueryCandidateIntersectionTriangleKHR,
            1u32 => Self::RayQueryCandidateIntersectionAABBKHR,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(RayQueryCandidateIntersectionType),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PackedVectorFormat {
    ///Since SPIR-V 1.6
    PackedVectorFormat4x8Bit = 0u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for PackedVectorFormat {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for PackedVectorFormat {}
unsafe impl Operand for PackedVectorFormat {
    const KIND: &OperandKind = &OPERAND_KIND_PACKED_VECTOR_FORMAT;
}
unsafe impl OperandEncoding for PackedVectorFormat {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::PackedVectorFormat4x8Bit,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(PackedVectorFormat),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct
    CooperativeMatrixOperands : u32 { const NoneKHR = 0u32; const
    MatrixASignedComponentsKHR = 1u32; const MatrixBSignedComponentsKHR = 2u32; const
    MatrixCSignedComponentsKHR = 4u32; const MatrixResultSignedComponentsKHR = 8u32;
    const SaturatingAccumulationKHR = 16u32; }
}
unsafe impl Operand for CooperativeMatrixOperands {
    const KIND: &OperandKind = &OPERAND_KIND_COOPERATIVE_MATRIX_OPERANDS;
}
unsafe impl OperandEncoding for CooperativeMatrixOperands {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<CooperativeMatrixOperands>(
            stringify!(CooperativeMatrixOperands),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::MatrixASignedComponentsKHR) {
                write!(f, "{sep}MatrixASignedComponentsKHR")?;
            }
            if self.contains(Self::MatrixBSignedComponentsKHR) {
                write!(f, "{sep}MatrixBSignedComponentsKHR")?;
            }
            if self.contains(Self::MatrixCSignedComponentsKHR) {
                write!(f, "{sep}MatrixCSignedComponentsKHR")?;
            }
            if self.contains(Self::MatrixResultSignedComponentsKHR) {
                write!(f, "{sep}MatrixResultSignedComponentsKHR")?;
            }
            if self.contains(Self::SaturatingAccumulationKHR) {
                write!(f, "{sep}SaturatingAccumulationKHR")?;
            }
            Ok(())
        }
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CooperativeMatrixLayout {
    RowMajorKHR = 0u32,
    ColumnMajorKHR = 1u32,
    RowBlockedInterleavedARM = 4202u32,
    ColumnBlockedInterleavedARM = 4203u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for CooperativeMatrixLayout {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for CooperativeMatrixLayout {}
unsafe impl Operand for CooperativeMatrixLayout {
    const KIND: &OperandKind = &OPERAND_KIND_COOPERATIVE_MATRIX_LAYOUT;
}
unsafe impl OperandEncoding for CooperativeMatrixLayout {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::RowMajorKHR,
            1u32 => Self::ColumnMajorKHR,
            4202u32 => Self::RowBlockedInterleavedARM,
            4203u32 => Self::ColumnBlockedInterleavedARM,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(CooperativeMatrixLayout),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CooperativeMatrixUse {
    MatrixAKHR = 0u32,
    MatrixBKHR = 1u32,
    MatrixAccumulatorKHR = 2u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for CooperativeMatrixUse {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for CooperativeMatrixUse {}
unsafe impl Operand for CooperativeMatrixUse {
    const KIND: &OperandKind = &OPERAND_KIND_COOPERATIVE_MATRIX_USE;
}
unsafe impl OperandEncoding for CooperativeMatrixUse {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::MatrixAKHR,
            1u32 => Self::MatrixBKHR,
            2u32 => Self::MatrixAccumulatorKHR,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(CooperativeMatrixUse),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct CooperativeMatrixReduce
    : u32 { const Row = 1u32; const Column = 2u32; const TwoByTwo = 4u32; }
}
unsafe impl Operand for CooperativeMatrixReduce {
    const KIND: &OperandKind = &OPERAND_KIND_COOPERATIVE_MATRIX_REDUCE;
}
unsafe impl OperandEncoding for CooperativeMatrixReduce {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<CooperativeMatrixReduce>(
            stringify!(CooperativeMatrixReduce),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::Row) {
                write!(f, "{sep}Row")?;
            }
            if self.contains(Self::Column) {
                write!(f, "{sep}Column")?;
            }
            if self.contains(Self::TwoByTwo) {
                write!(f, "{sep}TwoByTwo")?;
            }
            Ok(())
        }
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TensorClampMode {
    Undefined = 0u32,
    Constant = 1u32,
    ClampToEdge = 2u32,
    Repeat = 3u32,
    RepeatMirrored = 4u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for TensorClampMode {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for TensorClampMode {}
unsafe impl Operand for TensorClampMode {
    const KIND: &OperandKind = &OPERAND_KIND_TENSOR_CLAMP_MODE;
}
unsafe impl OperandEncoding for TensorClampMode {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Undefined,
            1u32 => Self::Constant,
            2u32 => Self::ClampToEdge,
            3u32 => Self::Repeat,
            4u32 => Self::RepeatMirrored,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(TensorClampMode),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct
    TensorAddressingOperands : u32 { const None = 0u32; const TensorView = 1u32; const
    DecodeFunc = 2u32; }
}
unsafe impl Operand for TensorAddressingOperands {
    const KIND: &OperandKind = &OPERAND_KIND_TENSOR_ADDRESSING_OPERANDS;
}
unsafe impl OperandEncoding for TensorAddressingOperands {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<TensorAddressingOperands>(
            stringify!(TensorAddressingOperands),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::TensorView) {
                write!(f, "{sep}TensorView")?;
            }
            if self.contains(Self::DecodeFunc) {
                write!(f, "{sep}DecodeFunc")?;
            }
            Ok(())
        }
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InitializationModeQualifier {
    InitOnDeviceReprogramALTERA = 0u32,
    InitOnDeviceResetALTERA = 1u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for InitializationModeQualifier {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for InitializationModeQualifier {}
unsafe impl Operand for InitializationModeQualifier {
    const KIND: &OperandKind = &OPERAND_KIND_INITIALIZATION_MODE_QUALIFIER;
}
unsafe impl OperandEncoding for InitializationModeQualifier {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::InitOnDeviceReprogramALTERA,
            1u32 => Self::InitOnDeviceResetALTERA,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(InitializationModeQualifier),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LoadCacheControl {
    UncachedINTEL = 0u32,
    CachedINTEL = 1u32,
    StreamingINTEL = 2u32,
    InvalidateAfterReadINTEL = 3u32,
    ConstCachedINTEL = 4u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for LoadCacheControl {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for LoadCacheControl {}
unsafe impl Operand for LoadCacheControl {
    const KIND: &OperandKind = &OPERAND_KIND_LOAD_CACHE_CONTROL;
}
unsafe impl OperandEncoding for LoadCacheControl {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::UncachedINTEL,
            1u32 => Self::CachedINTEL,
            2u32 => Self::StreamingINTEL,
            3u32 => Self::InvalidateAfterReadINTEL,
            4u32 => Self::ConstCachedINTEL,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(LoadCacheControl),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StoreCacheControl {
    UncachedINTEL = 0u32,
    WriteThroughINTEL = 1u32,
    WriteBackINTEL = 2u32,
    StreamingINTEL = 3u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for StoreCacheControl {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for StoreCacheControl {}
unsafe impl Operand for StoreCacheControl {
    const KIND: &OperandKind = &OPERAND_KIND_STORE_CACHE_CONTROL;
}
unsafe impl OperandEncoding for StoreCacheControl {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::UncachedINTEL,
            1u32 => Self::WriteThroughINTEL,
            2u32 => Self::WriteBackINTEL,
            3u32 => Self::StreamingINTEL,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(StoreCacheControl),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NamedMaximumNumberOfRegisters {
    AutoINTEL = 0u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for NamedMaximumNumberOfRegisters {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for NamedMaximumNumberOfRegisters {}
unsafe impl Operand for NamedMaximumNumberOfRegisters {
    const KIND: &OperandKind = &OPERAND_KIND_NAMED_MAXIMUM_NUMBER_OF_REGISTERS;
}
unsafe impl OperandEncoding for NamedMaximumNumberOfRegisters {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::AutoINTEL,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(NamedMaximumNumberOfRegisters),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct
    MatrixMultiplyAccumulateOperands : u32 { const None = 0u32; const
    MatrixASignedComponentsINTEL = 1u32; const MatrixBSignedComponentsINTEL = 2u32; const
    MatrixCBFloat16INTEL = 4u32; const MatrixResultBFloat16INTEL = 8u32; const
    MatrixAPackedInt8INTEL = 16u32; const MatrixBPackedInt8INTEL = 32u32; const
    MatrixAPackedInt4INTEL = 64u32; const MatrixBPackedInt4INTEL = 128u32; const
    MatrixATF32INTEL = 256u32; const MatrixBTF32INTEL = 512u32; const
    MatrixAPackedFloat16INTEL = 1024u32; const MatrixBPackedFloat16INTEL = 2048u32; const
    MatrixAPackedBFloat16INTEL = 4096u32; const MatrixBPackedBFloat16INTEL = 8192u32; }
}
unsafe impl Operand for MatrixMultiplyAccumulateOperands {
    const KIND: &OperandKind = &OPERAND_KIND_MATRIX_MULTIPLY_ACCUMULATE_OPERANDS;
}
unsafe impl OperandEncoding for MatrixMultiplyAccumulateOperands {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<
            MatrixMultiplyAccumulateOperands,
        >(stringify!(MatrixMultiplyAccumulateOperands), bits))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::MatrixASignedComponentsINTEL) {
                write!(f, "{sep}MatrixASignedComponentsINTEL")?;
            }
            if self.contains(Self::MatrixBSignedComponentsINTEL) {
                write!(f, "{sep}MatrixBSignedComponentsINTEL")?;
            }
            if self.contains(Self::MatrixCBFloat16INTEL) {
                write!(f, "{sep}MatrixCBFloat16INTEL")?;
            }
            if self.contains(Self::MatrixResultBFloat16INTEL) {
                write!(f, "{sep}MatrixResultBFloat16INTEL")?;
            }
            if self.contains(Self::MatrixAPackedInt8INTEL) {
                write!(f, "{sep}MatrixAPackedInt8INTEL")?;
            }
            if self.contains(Self::MatrixBPackedInt8INTEL) {
                write!(f, "{sep}MatrixBPackedInt8INTEL")?;
            }
            if self.contains(Self::MatrixAPackedInt4INTEL) {
                write!(f, "{sep}MatrixAPackedInt4INTEL")?;
            }
            if self.contains(Self::MatrixBPackedInt4INTEL) {
                write!(f, "{sep}MatrixBPackedInt4INTEL")?;
            }
            if self.contains(Self::MatrixATF32INTEL) {
                write!(f, "{sep}MatrixATF32INTEL")?;
            }
            if self.contains(Self::MatrixBTF32INTEL) {
                write!(f, "{sep}MatrixBTF32INTEL")?;
            }
            if self.contains(Self::MatrixAPackedFloat16INTEL) {
                write!(f, "{sep}MatrixAPackedFloat16INTEL")?;
            }
            if self.contains(Self::MatrixBPackedFloat16INTEL) {
                write!(f, "{sep}MatrixBPackedFloat16INTEL")?;
            }
            if self.contains(Self::MatrixAPackedBFloat16INTEL) {
                write!(f, "{sep}MatrixAPackedBFloat16INTEL")?;
            }
            if self.contains(Self::MatrixBPackedBFloat16INTEL) {
                write!(f, "{sep}MatrixBPackedBFloat16INTEL")?;
            }
            Ok(())
        }
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FPEncoding {
    BFloat16KHR = 0u32,
    Float8E4M3EXT = 4214u32,
    Float8E5M2EXT = 4215u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for FPEncoding {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for FPEncoding {}
unsafe impl Operand for FPEncoding {
    const KIND: &OperandKind = &OPERAND_KIND_FP_ENCODING;
}
unsafe impl OperandEncoding for FPEncoding {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::BFloat16KHR,
            4214u32 => Self::Float8E4M3EXT,
            4215u32 => Self::Float8E5M2EXT,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(FPEncoding),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CooperativeVectorMatrixLayout {
    RowMajorNV = 0u32,
    ColumnMajorNV = 1u32,
    InferencingOptimalNV = 2u32,
    TrainingOptimalNV = 3u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for CooperativeVectorMatrixLayout {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for CooperativeVectorMatrixLayout {}
unsafe impl Operand for CooperativeVectorMatrixLayout {
    const KIND: &OperandKind = &OPERAND_KIND_COOPERATIVE_VECTOR_MATRIX_LAYOUT;
}
unsafe impl OperandEncoding for CooperativeVectorMatrixLayout {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::RowMajorNV,
            1u32 => Self::ColumnMajorNV,
            2u32 => Self::InferencingOptimalNV,
            3u32 => Self::TrainingOptimalNV,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(CooperativeVectorMatrixLayout),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ComponentType {
    Float16NV = 0u32,
    Float32NV = 1u32,
    Float64NV = 2u32,
    SignedInt8NV = 3u32,
    SignedInt16NV = 4u32,
    SignedInt32NV = 5u32,
    SignedInt64NV = 6u32,
    UnsignedInt8NV = 7u32,
    UnsignedInt16NV = 8u32,
    UnsignedInt32NV = 9u32,
    UnsignedInt64NV = 10u32,
    SignedInt8PackedNV = 1000491000u32,
    UnsignedInt8PackedNV = 1000491001u32,
    FloatE4M3NV = 1000491002u32,
    FloatE5M2NV = 1000491003u32,
}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for ComponentType {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for ComponentType {}
unsafe impl Operand for ComponentType {
    const KIND: &OperandKind = &OPERAND_KIND_COMPONENT_TYPE;
}
unsafe impl OperandEncoding for ComponentType {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(*self as u32));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let variant = reader.pull()?.0;
        Ok(match variant {
            0u32 => Self::Float16NV,
            1u32 => Self::Float32NV,
            2u32 => Self::Float64NV,
            3u32 => Self::SignedInt8NV,
            4u32 => Self::SignedInt16NV,
            5u32 => Self::SignedInt32NV,
            6u32 => Self::SignedInt64NV,
            7u32 => Self::UnsignedInt8NV,
            8u32 => Self::UnsignedInt16NV,
            9u32 => Self::UnsignedInt32NV,
            10u32 => Self::UnsignedInt64NV,
            1000491000u32 => Self::SignedInt8PackedNV,
            1000491001u32 => Self::UnsignedInt8PackedNV,
            1000491002u32 => Self::FloatE4M3NV,
            1000491003u32 => Self::FloatE5M2NV,
            _ => {
                return Err(DecodeError::UnknownEnumVariant {
                    name: stringify!(ComponentType),
                    variant,
                });
            }
        })
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        write!(f, " {:?}", self)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PairLiteralIntegerIdRef(LiteralInteger, IdRef);
unsafe impl Operand for PairLiteralIntegerIdRef {
    const KIND: &OperandKind = &OPERAND_KIND_PAIR_LITERAL_INTEGER_ID_REF;
}
unsafe impl OperandEncoding for PairLiteralIntegerIdRef {
    const FIXED_LEN: Option<usize> = FixedLenComposer::new()
        .append(<LiteralInteger as OperandEncoding>::FIXED_LEN)
        .append(<IdRef as OperandEncoding>::FIXED_LEN)
        .finish();
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        OperandEncoding::encode(&self.0, &mut *writer)?;
        OperandEncoding::encode(&self.1, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Ok(Self(
            OperandEncoding::decode(&mut *reader)?,
            OperandEncoding::decode(&mut *reader)?,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            " {} {}",
            OperandEncoding::dis(&self.0, ctx),
            OperandEncoding::dis(&self.1, ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PairIdRefLiteralInteger(IdRef, LiteralInteger);
unsafe impl Operand for PairIdRefLiteralInteger {
    const KIND: &OperandKind = &OPERAND_KIND_PAIR_ID_REF_LITERAL_INTEGER;
}
unsafe impl OperandEncoding for PairIdRefLiteralInteger {
    const FIXED_LEN: Option<usize> = FixedLenComposer::new()
        .append(<IdRef as OperandEncoding>::FIXED_LEN)
        .append(<LiteralInteger as OperandEncoding>::FIXED_LEN)
        .finish();
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        OperandEncoding::encode(&self.0, &mut *writer)?;
        OperandEncoding::encode(&self.1, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Ok(Self(
            OperandEncoding::decode(&mut *reader)?,
            OperandEncoding::decode(&mut *reader)?,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            " {} {}",
            OperandEncoding::dis(&self.0, ctx),
            OperandEncoding::dis(&self.1, ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PairIdRefIdRef(IdRef, IdRef);
unsafe impl Operand for PairIdRefIdRef {
    const KIND: &OperandKind = &OPERAND_KIND_PAIR_ID_REF_ID_REF;
}
unsafe impl OperandEncoding for PairIdRefIdRef {
    const FIXED_LEN: Option<usize> = FixedLenComposer::new()
        .append(<IdRef as OperandEncoding>::FIXED_LEN)
        .append(<IdRef as OperandEncoding>::FIXED_LEN)
        .finish();
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        OperandEncoding::encode(&self.0, &mut *writer)?;
        OperandEncoding::encode(&self.1, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Ok(Self(
            OperandEncoding::decode(&mut *reader)?,
            OperandEncoding::decode(&mut *reader)?,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            " {} {}",
            OperandEncoding::dis(&self.0, ctx),
            OperandEncoding::dis(&self.1, ctx)
        )
    }
}
bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub struct TensorOperands : u32 {
    const NoneARM = 0u32; const NontemporalARM = 1u32; const OutOfBoundsValueARM = 2u32;
    const MakeElementAvailableARM = 4u32; const MakeElementVisibleARM = 8u32; const
    NonPrivateElementARM = 16u32; }
}
unsafe impl Operand for TensorOperands {
    const KIND: &OperandKind = &OPERAND_KIND_TENSOR_OPERANDS;
}
unsafe impl OperandEncoding for TensorOperands {
    const FIXED_LEN: Option<usize> = Some(1);
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(Word(self.bits()));
        Ok(())
    }
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = reader.pull()?.0;
        Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<TensorOperands>(
            stringify!(TensorOperands),
            bits,
        ))
    }
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, " None")
        } else {
            write!(f, " ")?;
            let sep = SeparatorJoiner::new("|");
            if self.contains(Self::NontemporalARM) {
                write!(f, "{sep}NontemporalARM")?;
            }
            if self.contains(Self::OutOfBoundsValueARM) {
                write!(f, "{sep}OutOfBoundsValueARM")?;
            }
            if self.contains(Self::MakeElementAvailableARM) {
                write!(f, "{sep}MakeElementAvailableARM")?;
            }
            if self.contains(Self::MakeElementVisibleARM) {
                write!(f, "{sep}MakeElementVisibleARM")?;
            }
            if self.contains(Self::NonPrivateElementARM) {
                write!(f, "{sep}NonPrivateElementARM")?;
            }
            Ok(())
        }
    }
}
