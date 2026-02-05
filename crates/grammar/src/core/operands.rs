use super::preamble::*;
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct ImageOperands : u32 { const None = 0u32 ; const Bias = 1u32 ; const Lod = 2u32 ; const Grad = 4u32 ; const ConstOffset = 8u32 ; const Offset = 16u32 ; const ConstOffsets = 32u32 ; const Sample = 64u32 ; const MinLod = 128u32 ; # [doc = "Since SPIR-V 1.5"] const MakeTexelAvailable = 256u32 ; # [doc = "Since SPIR-V 1.5"] const MakeTexelVisible = 512u32 ; # [doc = "Since SPIR-V 1.5"] const NonPrivateTexel = 1024u32 ; # [doc = "Since SPIR-V 1.5"] const VolatileTexel = 2048u32 ; # [doc = "Since SPIR-V 1.4"] const SignExtend = 4096u32 ; # [doc = "Since SPIR-V 1.4"] const ZeroExtend = 8192u32 ; # [doc = "Since SPIR-V 1.6"] const Nontemporal = 16384u32 ; const Offsets = 65536u32 ; } }
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct FPFastMathMode : u32 { const None = 0u32 ; const NotNaN = 1u32 ; const NotInf = 2u32 ; const NSZ = 4u32 ; const AllowRecip = 8u32 ; const Fast = 16u32 ; const AllowContract = 65536u32 ; const AllowReassoc = 131072u32 ; const AllowTransform = 262144u32 ; } }
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct SelectionControl : u32 { const None = 0u32 ; const Flatten = 1u32 ; const DontFlatten = 2u32 ; } }
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct LoopControl : u32 { const None = 0u32 ; const Unroll = 1u32 ; const DontUnroll = 2u32 ; # [doc = "Since SPIR-V 1.1"] const DependencyInfinite = 4u32 ; # [doc = "Since SPIR-V 1.1"] const DependencyLength = 8u32 ; # [doc = "Since SPIR-V 1.4"] const MinIterations = 16u32 ; # [doc = "Since SPIR-V 1.4"] const MaxIterations = 32u32 ; # [doc = "Since SPIR-V 1.4"] const IterationMultiple = 64u32 ; # [doc = "Since SPIR-V 1.4"] const PeelCount = 128u32 ; # [doc = "Since SPIR-V 1.4"] const PartialCount = 256u32 ; const InitiationIntervalALTERA = 65536u32 ; const MaxConcurrencyALTERA = 131072u32 ; const DependencyArrayALTERA = 262144u32 ; const PipelineEnableALTERA = 524288u32 ; const LoopCoalesceALTERA = 1048576u32 ; const MaxInterleavingALTERA = 2097152u32 ; const SpeculatedIterationsALTERA = 4194304u32 ; const NoFusionALTERA = 8388608u32 ; const LoopCountALTERA = 16777216u32 ; const MaxReinvocationDelayALTERA = 33554432u32 ; } }
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct FunctionControl : u32 { const None = 0u32 ; const Inline = 1u32 ; const DontInline = 2u32 ; const Pure = 4u32 ; const Const = 8u32 ; const OptNoneEXT = 65536u32 ; } }
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct MemorySemantics : u32 { const Relaxed = 0u32 ; const Acquire = 2u32 ; const Release = 4u32 ; const AcquireRelease = 8u32 ; const SequentiallyConsistent = 16u32 ; const UniformMemory = 64u32 ; const SubgroupMemory = 128u32 ; const WorkgroupMemory = 256u32 ; const CrossWorkgroupMemory = 512u32 ; const AtomicCounterMemory = 1024u32 ; const ImageMemory = 2048u32 ; # [doc = "Since SPIR-V 1.5"] const OutputMemory = 4096u32 ; # [doc = "Since SPIR-V 1.5"] const MakeAvailable = 8192u32 ; # [doc = "Since SPIR-V 1.5"] const MakeVisible = 16384u32 ; # [doc = "Since SPIR-V 1.5"] const Volatile = 32768u32 ; } }
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct MemoryAccess : u32 { const None = 0u32 ; const Volatile = 1u32 ; const Aligned = 2u32 ; const Nontemporal = 4u32 ; # [doc = "Since SPIR-V 1.5"] const MakePointerAvailable = 8u32 ; # [doc = "Since SPIR-V 1.5"] const MakePointerVisible = 16u32 ; # [doc = "Since SPIR-V 1.5"] const NonPrivatePointer = 32u32 ; const AliasScopeINTELMask = 65536u32 ; const NoAliasINTELMask = 131072u32 ; } }
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct KernelProfilingInfo : u32 { const None = 0u32 ; const CmdExecTime = 1u32 ; } }
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct RayFlags : u32 { const NoneKHR = 0u32 ; const OpaqueKHR = 1u32 ; const NoOpaqueKHR = 2u32 ; const TerminateOnFirstHitKHR = 4u32 ; const SkipClosestHitShaderKHR = 8u32 ; const CullBackFacingTrianglesKHR = 16u32 ; const CullFrontFacingTrianglesKHR = 32u32 ; const CullOpaqueKHR = 64u32 ; const CullNoOpaqueKHR = 128u32 ; const SkipTrianglesKHR = 256u32 ; const SkipAABBsKHR = 512u32 ; const ForceOpacityMicromap2StateEXT = 1024u32 ; } }
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct FragmentShadingRate : u32 { const Vertical2Pixels = 1u32 ; const Vertical4Pixels = 2u32 ; const Horizontal2Pixels = 4u32 ; const Horizontal4Pixels = 8u32 ; } }
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct RawAccessChainOperands : u32 { const None = 0u32 ; const RobustnessPerComponentNV = 1u32 ; const RobustnessPerElementNV = 2u32 ; } }
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
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AddressingModel {
    Logical = 0u32,
    Physical32 = 1u32,
    Physical64 = 2u32,
    #[doc = "Since SPIR-V 1.5"]
    PhysicalStorageBuffer64 = 5348u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MemoryModel {
    Simple = 0u32,
    GLSL450 = 1u32,
    OpenCL = 2u32,
    #[doc = "Since SPIR-V 1.5"]
    Vulkan = 3u32,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ExecutionMode {
    Invocations(#[doc = "Number of <<Invocation,invocations>>"] LiteralInteger),
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
        #[doc = "x size"] LiteralInteger,
        #[doc = "y size"] LiteralInteger,
        #[doc = "z size"] LiteralInteger,
    ),
    LocalSizeHint(
        #[doc = "x size"] LiteralInteger,
        #[doc = "y size"] LiteralInteger,
        #[doc = "z size"] LiteralInteger,
    ),
    InputPoints,
    InputLines,
    InputLinesAdjacency,
    Triangles,
    InputTrianglesAdjacency,
    Quads,
    Isolines,
    OutputVertices(#[doc = "Vertex count"] LiteralInteger),
    OutputPoints,
    OutputLineStrip,
    OutputTriangleStrip,
    VecTypeHint(#[doc = "Vector type"] LiteralInteger),
    ContractionOff,
    #[doc = "Since SPIR-V 1.1"]
    Initializer,
    #[doc = "Since SPIR-V 1.1"]
    Finalizer,
    #[doc = "Since SPIR-V 1.1"]
    SubgroupSize(#[doc = "Subgroup Size"] LiteralInteger),
    #[doc = "Since SPIR-V 1.1"]
    SubgroupsPerWorkgroup(#[doc = "Subgroups Per Workgroup"] LiteralInteger),
    #[doc = "Since SPIR-V 1.2"]
    SubgroupsPerWorkgroupId(#[doc = "Subgroups Per Workgroup"] IdRef),
    #[doc = "Since SPIR-V 1.2"]
    LocalSizeId(
        #[doc = "x size"] IdRef,
        #[doc = "y size"] IdRef,
        #[doc = "z size"] IdRef,
    ),
    #[doc = "Since SPIR-V 1.2"]
    LocalSizeHintId(
        #[doc = "x size hint"] IdRef,
        #[doc = "y size hint"] IdRef,
        #[doc = "z size hint"] IdRef,
    ),
    NonCoherentColorAttachmentReadEXT,
    NonCoherentDepthAttachmentReadEXT,
    NonCoherentStencilAttachmentReadEXT,
    SubgroupUniformControlFlowKHR,
    PostDepthCoverage,
    #[doc = "Since SPIR-V 1.4"]
    DenormPreserve(#[doc = "Target Width"] LiteralInteger),
    #[doc = "Since SPIR-V 1.4"]
    DenormFlushToZero(#[doc = "Target Width"] LiteralInteger),
    #[doc = "Since SPIR-V 1.4"]
    SignedZeroInfNanPreserve(#[doc = "Target Width"] LiteralInteger),
    #[doc = "Since SPIR-V 1.4"]
    RoundingModeRTE(#[doc = "Target Width"] LiteralInteger),
    #[doc = "Since SPIR-V 1.4"]
    RoundingModeRTZ(#[doc = "Target Width"] LiteralInteger),
    NonCoherentTileAttachmentReadQCOM,
    TileShadingRateQCOM(
        #[doc = "x rate"] LiteralInteger,
        #[doc = "y rate"] LiteralInteger,
        #[doc = "z rate"] LiteralInteger,
    ),
    EarlyAndLateFragmentTestsAMD,
    StencilRefReplacingEXT,
    CoalescingAMDX,
    IsApiEntryAMDX(#[doc = "Is Entry"] IdRef),
    MaxNodeRecursionAMDX(#[doc = "Number of recursions"] IdRef),
    StaticNumWorkgroupsAMDX(
        #[doc = "x size"] IdRef,
        #[doc = "y size"] IdRef,
        #[doc = "z size"] IdRef,
    ),
    ShaderIndexAMDX(#[doc = "Shader Index"] IdRef),
    MaxNumWorkgroupsAMDX(
        #[doc = "x size"] IdRef,
        #[doc = "y size"] IdRef,
        #[doc = "z size"] IdRef,
    ),
    StencilRefUnchangedFrontAMD,
    StencilRefGreaterFrontAMD,
    StencilRefLessFrontAMD,
    StencilRefUnchangedBackAMD,
    StencilRefGreaterBackAMD,
    StencilRefLessBackAMD,
    QuadDerivativesKHR,
    RequireFullQuadsKHR,
    SharesInputWithAMDX(#[doc = "Node Name"] IdRef, #[doc = "Shader Index"] IdRef),
    OutputLinesEXT,
    OutputPrimitivesEXT(#[doc = "Primitive count"] LiteralInteger),
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
    SharedLocalMemorySizeINTEL(#[doc = "Size"] LiteralInteger),
    RoundingModeRTPINTEL(#[doc = "Target Width"] LiteralInteger),
    RoundingModeRTNINTEL(#[doc = "Target Width"] LiteralInteger),
    FloatingPointModeALTINTEL(#[doc = "Target Width"] LiteralInteger),
    FloatingPointModeIEEEINTEL(#[doc = "Target Width"] LiteralInteger),
    MaxWorkgroupSizeINTEL(
        #[doc = "max_x_size"] LiteralInteger,
        #[doc = "max_y_size"] LiteralInteger,
        #[doc = "max_z_size"] LiteralInteger,
    ),
    MaxWorkDimINTEL(#[doc = "max_dimensions"] LiteralInteger),
    NoGlobalOffsetINTEL,
    NumSIMDWorkitemsINTEL(#[doc = "vector_width"] LiteralInteger),
    SchedulerTargetFmaxMhzINTEL(#[doc = "target_fmax"] LiteralInteger),
    MaximallyReconvergesKHR,
    FPFastMathDefault(
        #[doc = "Target Type"] IdRef,
        #[doc = "Fast-Math Mode"] IdRef,
    ),
    StreamingInterfaceINTEL(#[doc = "StallFreeReturn"] LiteralInteger),
    RegisterMapInterfaceINTEL(#[doc = "WaitForDoneWrite"] LiteralInteger),
    NamedBarrierCountINTEL(#[doc = "Barrier Count"] LiteralInteger),
    MaximumRegistersINTEL(#[doc = "Number of Registers"] LiteralInteger),
    MaximumRegistersIdINTEL(#[doc = "Number of Registers"] IdRef),
    NamedMaximumRegistersINTEL(
        #[doc = "Named Maximum Number of Registers"] NamedMaximumNumberOfRegisters,
    ),
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
    #[doc = "Since SPIR-V 1.3"]
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
    #[doc = "Since SPIR-V 1.5"]
    PhysicalStorageBuffer = 5349u32,
    HitObjectAttributeNV = 5385u32,
    #[doc = "Since SPIR-V 1.4"]
    TaskPayloadWorkgroupEXT = 5402u32,
    HitObjectAttributeEXT = 5411u32,
    CodeSectionINTEL = 5605u32,
    DeviceOnlyALTERA = 5936u32,
    HostOnlyALTERA = 5937u32,
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
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SamplerAddressingMode {
    None = 0u32,
    ClampToEdge = 1u32,
    Clamp = 2u32,
    Repeat = 3u32,
    RepeatMirrored = 4u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SamplerFilterMode {
    Nearest = 0u32,
    Linear = 1u32,
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
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FPRoundingMode {
    RTE = 0u32,
    RTZ = 1u32,
    RTP = 2u32,
    RTN = 3u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FPDenormMode {
    Preserve = 0u32,
    FlushToZero = 1u32,
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
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FPOperationMode {
    IEEE = 0u32,
    ALT = 1u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OverflowModes {
    WRAP = 0u32,
    SAT = 1u32,
    SAT_ZERO = 2u32,
    SAT_SYM = 3u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LinkageType {
    Export = 0u32,
    Import = 1u32,
    LinkOnceODR = 2u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AccessQualifier {
    ReadOnly = 0u32,
    WriteOnly = 1u32,
    ReadWrite = 2u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HostAccessQualifier {
    NoneINTEL = 0u32,
    ReadINTEL = 1u32,
    WriteINTEL = 2u32,
    ReadWriteINTEL = 3u32,
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
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Decoration {
    RelaxedPrecision,
    SpecId(#[doc = "Specialization Constant ID"] LiteralInteger),
    Block,
    #[doc = "Deprecated in SPIR-V 1.3"]
    #[deprecated]
    BufferBlock,
    RowMajor,
    ColMajor,
    ArrayStride(#[doc = "Array Stride"] LiteralInteger),
    MatrixStride(#[doc = "Matrix Stride"] LiteralInteger),
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
    #[doc = "Since SPIR-V 1.4"]
    UniformId(#[doc = "Execution"] IdScope),
    SaturatedConversion,
    Stream(#[doc = "Stream Number"] LiteralInteger),
    Location(#[doc = "Location"] LiteralInteger),
    Component(#[doc = "Component"] LiteralInteger),
    Index(#[doc = "Index"] LiteralInteger),
    Binding(#[doc = "Binding Point"] LiteralInteger),
    DescriptorSet(#[doc = "Descriptor Set"] LiteralInteger),
    Offset(#[doc = "Byte Offset"] LiteralInteger),
    XfbBuffer(#[doc = "XFB Buffer Number"] LiteralInteger),
    XfbStride(#[doc = "XFB Stride"] LiteralInteger),
    FuncParamAttr(#[doc = "Function Parameter Attribute"] FunctionParameterAttribute),
    FPRoundingMode(#[doc = "Floating-Point Rounding Mode"] FPRoundingMode),
    FPFastMathMode(#[doc = "Fast-Math Mode"] FPFastMathMode),
    LinkageAttributes(
        #[doc = "Name"] LiteralString,
        #[doc = "Linkage Type"] LinkageType,
    ),
    NoContraction,
    InputAttachmentIndex(#[doc = "Attachment Index"] LiteralInteger),
    Alignment(#[doc = "Alignment"] LiteralInteger),
    #[doc = "Since SPIR-V 1.1"]
    MaxByteOffset(#[doc = "Max Byte Offset"] LiteralInteger),
    #[doc = "Since SPIR-V 1.2"]
    AlignmentId(#[doc = "Alignment"] IdRef),
    #[doc = "Since SPIR-V 1.2"]
    MaxByteOffsetId(#[doc = "Max Byte Offset"] IdRef),
    SaturatedToLargestFloat8NormalConversionEXT,
    #[doc = "Since SPIR-V 1.4"]
    NoSignedWrap,
    #[doc = "Since SPIR-V 1.4"]
    NoUnsignedWrap,
    WeightTextureQCOM,
    BlockMatchTextureQCOM,
    BlockMatchSamplerQCOM,
    ExplicitInterpAMD,
    NodeSharesPayloadLimitsWithAMDX(#[doc = "Payload Type"] IdRef),
    NodeMaxPayloadsAMDX(#[doc = "Max number of payloads"] IdRef),
    TrackFinishWritingAMDX,
    PayloadNodeNameAMDX(#[doc = "Node Name"] IdRef),
    PayloadNodeBaseIndexAMDX(#[doc = "Base Index"] IdRef),
    PayloadNodeSparseArrayAMDX,
    PayloadNodeArraySizeAMDX(#[doc = "Array Size"] IdRef),
    PayloadDispatchIndirectAMDX,
    ArrayStrideIdEXT(#[doc = "Array Stride"] IdRef),
    OffsetIdEXT(#[doc = "Byte Offset"] IdRef),
    OverrideCoverageNV,
    PassthroughNV,
    ViewportRelativeNV,
    SecondaryViewportRelativeNV(#[doc = "Offset"] LiteralInteger),
    PerPrimitiveEXT,
    PerViewNV,
    PerTaskNV,
    PerVertexKHR,
    #[doc = "Since SPIR-V 1.5"]
    NonUniform,
    #[doc = "Since SPIR-V 1.5"]
    RestrictPointer,
    #[doc = "Since SPIR-V 1.5"]
    AliasedPointer,
    MemberOffsetNV(#[doc = "memberOffset"] LiteralInteger),
    HitObjectShaderRecordBufferNV,
    HitObjectShaderRecordBufferEXT,
    BankNV(#[doc = "Bank"] LiteralInteger),
    BindlessSamplerNV,
    BindlessImageNV,
    BoundSamplerNV,
    BoundImageNV,
    SIMTCallINTEL(#[doc = "N"] LiteralInteger),
    ReferencedIndirectlyINTEL,
    ClobberINTEL(#[doc = "Register"] LiteralString),
    SideEffectsINTEL,
    VectorComputeVariableINTEL,
    FuncParamIOKindINTEL(#[doc = "Kind"] LiteralInteger),
    VectorComputeFunctionINTEL,
    StackCallINTEL,
    GlobalVariableOffsetINTEL(#[doc = "Offset"] LiteralInteger),
    #[doc = "Since SPIR-V 1.4"]
    CounterBuffer(#[doc = "Counter Buffer"] IdRef),
    #[doc = "Since SPIR-V 1.4"]
    UserSemantic(#[doc = "Semantic"] LiteralString),
    UserTypeGOOGLE(#[doc = "User Type"] LiteralString),
    FunctionRoundingModeINTEL(
        #[doc = "Target Width"] LiteralInteger,
        #[doc = "FP Rounding Mode"] FPRoundingMode,
    ),
    FunctionDenormModeINTEL(
        #[doc = "Target Width"] LiteralInteger,
        #[doc = "FP Denorm Mode"] FPDenormMode,
    ),
    RegisterALTERA,
    MemoryALTERA(#[doc = "Memory Type"] LiteralString),
    NumbanksALTERA(#[doc = "Banks"] LiteralInteger),
    BankwidthALTERA(#[doc = "Bank Width"] LiteralInteger),
    MaxPrivateCopiesALTERA(#[doc = "Maximum Copies"] LiteralInteger),
    SinglepumpALTERA,
    DoublepumpALTERA,
    MaxReplicatesALTERA(#[doc = "Maximum Replicates"] LiteralInteger),
    SimpleDualPortALTERA,
    MergeALTERA(
        #[doc = "Merge Key"] LiteralString,
        #[doc = "Merge Type"] LiteralString,
    ),
    BankBitsALTERA(#[doc = "Bank Bits"] Vec<LiteralInteger>),
    ForcePow2DepthALTERA(#[doc = "Force Key"] LiteralInteger),
    StridesizeALTERA(#[doc = "Stride Size"] LiteralInteger),
    WordsizeALTERA(#[doc = "Word Size"] LiteralInteger),
    TrueDualPortALTERA,
    BurstCoalesceALTERA,
    CacheSizeALTERA(#[doc = "Cache Size in bytes"] LiteralInteger),
    DontStaticallyCoalesceALTERA,
    PrefetchALTERA(#[doc = "Prefetcher Size in bytes"] LiteralInteger),
    StallEnableALTERA,
    FuseLoopsInFunctionALTERA,
    MathOpDSPModeALTERA(
        #[doc = "Mode"] LiteralInteger,
        #[doc = "Propagate"] LiteralInteger,
    ),
    AliasScopeINTEL(#[doc = "Aliasing Scopes List"] IdRef),
    NoAliasINTEL(#[doc = "Aliasing Scopes List"] IdRef),
    InitiationIntervalALTERA(#[doc = "Cycles"] LiteralInteger),
    MaxConcurrencyALTERA(#[doc = "Invocations"] LiteralInteger),
    PipelineEnableALTERA(#[doc = "Enable"] LiteralInteger),
    BufferLocationALTERA(#[doc = "Buffer Location ID"] LiteralInteger),
    IOPipeStorageALTERA(#[doc = "IO Pipe ID"] LiteralInteger),
    FunctionFloatingPointModeINTEL(
        #[doc = "Target Width"] LiteralInteger,
        #[doc = "FP Operation Mode"] FPOperationMode,
    ),
    SingleElementVectorINTEL,
    VectorComputeCallableFunctionINTEL,
    MediaBlockIOINTEL,
    StallFreeALTERA,
    FPMaxErrorDecorationINTEL(#[doc = "Max Error"] LiteralFloat),
    LatencyControlLabelALTERA(#[doc = "Latency Label"] LiteralInteger),
    LatencyControlConstraintALTERA(
        #[doc = "Relative To"] LiteralInteger,
        #[doc = "Control Type"] LiteralInteger,
        #[doc = "Relative Cycle"] LiteralInteger,
    ),
    ConduitKernelArgumentALTERA,
    RegisterMapKernelArgumentALTERA,
    MMHostInterfaceAddressWidthALTERA(#[doc = "AddressWidth"] LiteralInteger),
    MMHostInterfaceDataWidthALTERA(#[doc = "DataWidth"] LiteralInteger),
    MMHostInterfaceLatencyALTERA(#[doc = "Latency"] LiteralInteger),
    MMHostInterfaceReadWriteModeALTERA(#[doc = "ReadWriteMode"] AccessQualifier),
    MMHostInterfaceMaxBurstALTERA(#[doc = "MaxBurstCount"] LiteralInteger),
    MMHostInterfaceWaitRequestALTERA(#[doc = "Waitrequest"] LiteralInteger),
    StableKernelArgumentALTERA,
    HostAccessINTEL(
        #[doc = "Access"] HostAccessQualifier,
        #[doc = "Name"] LiteralString,
    ),
    InitModeALTERA(#[doc = "Trigger"] InitializationModeQualifier),
    ImplementInRegisterMapALTERA(#[doc = "Value"] LiteralInteger),
    ConditionalINTEL(#[doc = "Condition"] IdRef),
    CacheControlLoadINTEL(
        #[doc = "Cache Level"] LiteralInteger,
        #[doc = "Cache Control"] LoadCacheControl,
    ),
    CacheControlStoreINTEL(
        #[doc = "Cache Level"] LiteralInteger,
        #[doc = "Cache Control"] StoreCacheControl,
    ),
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
    #[doc = "Since SPIR-V 1.3"]
    SubgroupEqMask = 4416u32,
    #[doc = "Since SPIR-V 1.3"]
    SubgroupGeMask = 4417u32,
    #[doc = "Since SPIR-V 1.3"]
    SubgroupGtMask = 4418u32,
    #[doc = "Since SPIR-V 1.3"]
    SubgroupLeMask = 4419u32,
    #[doc = "Since SPIR-V 1.3"]
    SubgroupLtMask = 4420u32,
    #[doc = "Since SPIR-V 1.3"]
    BaseVertex = 4424u32,
    #[doc = "Since SPIR-V 1.3"]
    BaseInstance = 4425u32,
    #[doc = "Since SPIR-V 1.3"]
    DrawIndex = 4426u32,
    PrimitiveShadingRateKHR = 4432u32,
    #[doc = "Since SPIR-V 1.3"]
    DeviceIndex = 4438u32,
    #[doc = "Since SPIR-V 1.3"]
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
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Scope {
    CrossDevice = 0u32,
    Device = 1u32,
    Workgroup = 2u32,
    Subgroup = 3u32,
    Invocation = 4u32,
    #[doc = "Since SPIR-V 1.5"]
    QueueFamily = 5u32,
    ShaderCallKHR = 6u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GroupOperation {
    Reduce = 0u32,
    InclusiveScan = 1u32,
    ExclusiveScan = 2u32,
    #[doc = "Since SPIR-V 1.3"]
    ClusteredReduce = 3u32,
    PartitionedReduceEXT = 6u32,
    PartitionedInclusiveScanEXT = 7u32,
    PartitionedExclusiveScanEXT = 8u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum KernelEnqueueFlags {
    NoWait = 0u32,
    WaitKernel = 1u32,
    WaitWorkGroup = 2u32,
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
    #[doc = "Since SPIR-V 1.1"]
    SubgroupDispatch = 58u32,
    #[doc = "Since SPIR-V 1.1"]
    NamedBarrier = 59u32,
    #[doc = "Since SPIR-V 1.1"]
    PipeStorage = 60u32,
    #[doc = "Since SPIR-V 1.3"]
    GroupNonUniform = 61u32,
    #[doc = "Since SPIR-V 1.3"]
    GroupNonUniformVote = 62u32,
    #[doc = "Since SPIR-V 1.3"]
    GroupNonUniformArithmetic = 63u32,
    #[doc = "Since SPIR-V 1.3"]
    GroupNonUniformBallot = 64u32,
    #[doc = "Since SPIR-V 1.3"]
    GroupNonUniformShuffle = 65u32,
    #[doc = "Since SPIR-V 1.3"]
    GroupNonUniformShuffleRelative = 66u32,
    #[doc = "Since SPIR-V 1.3"]
    GroupNonUniformClustered = 67u32,
    #[doc = "Since SPIR-V 1.3"]
    GroupNonUniformQuad = 68u32,
    #[doc = "Since SPIR-V 1.5"]
    ShaderLayer = 69u32,
    #[doc = "Since SPIR-V 1.5"]
    ShaderViewportIndex = 70u32,
    #[doc = "Since SPIR-V 1.6"]
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
    #[doc = "Since SPIR-V 1.3"]
    DrawParameters = 4427u32,
    WorkgroupMemoryExplicitLayoutKHR = 4428u32,
    WorkgroupMemoryExplicitLayout8BitAccessKHR = 4429u32,
    WorkgroupMemoryExplicitLayout16BitAccessKHR = 4430u32,
    SubgroupVoteKHR = 4431u32,
    #[doc = "Since SPIR-V 1.3"]
    StorageBuffer16BitAccess = 4433u32,
    #[doc = "Since SPIR-V 1.3"]
    UniformAndStorageBuffer16BitAccess = 4434u32,
    #[doc = "Since SPIR-V 1.3"]
    StoragePushConstant16 = 4435u32,
    #[doc = "Since SPIR-V 1.3"]
    StorageInputOutput16 = 4436u32,
    #[doc = "Since SPIR-V 1.3"]
    DeviceGroup = 4437u32,
    #[doc = "Since SPIR-V 1.3"]
    MultiView = 4439u32,
    #[doc = "Since SPIR-V 1.3"]
    VariablePointersStorageBuffer = 4441u32,
    #[doc = "Since SPIR-V 1.3"]
    VariablePointers = 4442u32,
    AtomicStorageOps = 4445u32,
    SampleMaskPostDepthCoverage = 4447u32,
    #[doc = "Since SPIR-V 1.5"]
    StorageBuffer8BitAccess = 4448u32,
    #[doc = "Since SPIR-V 1.5"]
    UniformAndStorageBuffer8BitAccess = 4449u32,
    #[doc = "Since SPIR-V 1.5"]
    StoragePushConstant8 = 4450u32,
    #[doc = "Since SPIR-V 1.4"]
    DenormPreserve = 4464u32,
    #[doc = "Since SPIR-V 1.4"]
    DenormFlushToZero = 4465u32,
    #[doc = "Since SPIR-V 1.4"]
    SignedZeroInfNanPreserve = 4466u32,
    #[doc = "Since SPIR-V 1.4"]
    RoundingModeRTE = 4467u32,
    #[doc = "Since SPIR-V 1.4"]
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
    #[doc = "Since SPIR-V 1.5"]
    ShaderNonUniform = 5301u32,
    #[doc = "Since SPIR-V 1.5"]
    RuntimeDescriptorArray = 5302u32,
    #[doc = "Since SPIR-V 1.5"]
    InputAttachmentArrayDynamicIndexing = 5303u32,
    #[doc = "Since SPIR-V 1.5"]
    UniformTexelBufferArrayDynamicIndexing = 5304u32,
    #[doc = "Since SPIR-V 1.5"]
    StorageTexelBufferArrayDynamicIndexing = 5305u32,
    #[doc = "Since SPIR-V 1.5"]
    UniformBufferArrayNonUniformIndexing = 5306u32,
    #[doc = "Since SPIR-V 1.5"]
    SampledImageArrayNonUniformIndexing = 5307u32,
    #[doc = "Since SPIR-V 1.5"]
    StorageBufferArrayNonUniformIndexing = 5308u32,
    #[doc = "Since SPIR-V 1.5"]
    StorageImageArrayNonUniformIndexing = 5309u32,
    #[doc = "Since SPIR-V 1.5"]
    InputAttachmentArrayNonUniformIndexing = 5310u32,
    #[doc = "Since SPIR-V 1.5"]
    UniformTexelBufferArrayNonUniformIndexing = 5311u32,
    #[doc = "Since SPIR-V 1.5"]
    StorageTexelBufferArrayNonUniformIndexing = 5312u32,
    RayTracingPositionFetchKHR = 5336u32,
    RayTracingNV = 5340u32,
    RayTracingMotionBlurNV = 5341u32,
    #[doc = "Since SPIR-V 1.5"]
    VulkanMemoryModel = 5345u32,
    #[doc = "Since SPIR-V 1.5"]
    VulkanMemoryModelDeviceScope = 5346u32,
    #[doc = "Since SPIR-V 1.5"]
    PhysicalStorageBufferAddresses = 5347u32,
    ComputeDerivativeGroupLinearKHR = 5350u32,
    RayTracingProvisionalKHR = 5353u32,
    CooperativeMatrixNV = 5357u32,
    FragmentShaderSampleInterlockEXT = 5363u32,
    FragmentShaderShadingRateInterlockEXT = 5372u32,
    ShaderSMBuiltinsNV = 5373u32,
    FragmentShaderPixelInterlockEXT = 5378u32,
    #[doc = "Since SPIR-V 1.6"]
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
    #[doc = "Since SPIR-V 1.6"]
    DotProductInputAll = 6016u32,
    #[doc = "Since SPIR-V 1.6"]
    DotProductInput4x8Bit = 6017u32,
    #[doc = "Since SPIR-V 1.6"]
    DotProductInput4x8BitPacked = 6018u32,
    #[doc = "Since SPIR-V 1.6"]
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
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RayQueryIntersection {
    RayQueryCandidateIntersectionKHR = 0u32,
    RayQueryCommittedIntersectionKHR = 1u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RayQueryCommittedIntersectionType {
    RayQueryCommittedIntersectionNoneKHR = 0u32,
    RayQueryCommittedIntersectionTriangleKHR = 1u32,
    RayQueryCommittedIntersectionGeneratedKHR = 2u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RayQueryCandidateIntersectionType {
    RayQueryCandidateIntersectionTriangleKHR = 0u32,
    RayQueryCandidateIntersectionAABBKHR = 1u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PackedVectorFormat {
    #[doc = "Since SPIR-V 1.6"]
    PackedVectorFormat4x8Bit = 0u32,
}
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct CooperativeMatrixOperands : u32 { const NoneKHR = 0u32 ; const MatrixASignedComponentsKHR = 1u32 ; const MatrixBSignedComponentsKHR = 2u32 ; const MatrixCSignedComponentsKHR = 4u32 ; const MatrixResultSignedComponentsKHR = 8u32 ; const SaturatingAccumulationKHR = 16u32 ; } }
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CooperativeMatrixLayout {
    RowMajorKHR = 0u32,
    ColumnMajorKHR = 1u32,
    RowBlockedInterleavedARM = 4202u32,
    ColumnBlockedInterleavedARM = 4203u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CooperativeMatrixUse {
    MatrixAKHR = 0u32,
    MatrixBKHR = 1u32,
    MatrixAccumulatorKHR = 2u32,
}
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct CooperativeMatrixReduce : u32 { const Row = 1u32 ; const Column = 2u32 ; const TwoByTwo = 4u32 ; } }
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TensorClampMode {
    Undefined = 0u32,
    Constant = 1u32,
    ClampToEdge = 2u32,
    Repeat = 3u32,
    RepeatMirrored = 4u32,
}
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct TensorAddressingOperands : u32 { const None = 0u32 ; const TensorView = 1u32 ; const DecodeFunc = 2u32 ; } }
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InitializationModeQualifier {
    InitOnDeviceReprogramALTERA = 0u32,
    InitOnDeviceResetALTERA = 1u32,
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
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StoreCacheControl {
    UncachedINTEL = 0u32,
    WriteThroughINTEL = 1u32,
    WriteBackINTEL = 2u32,
    StreamingINTEL = 3u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NamedMaximumNumberOfRegisters {
    AutoINTEL = 0u32,
}
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct MatrixMultiplyAccumulateOperands : u32 { const None = 0u32 ; const MatrixASignedComponentsINTEL = 1u32 ; const MatrixBSignedComponentsINTEL = 2u32 ; const MatrixCBFloat16INTEL = 4u32 ; const MatrixResultBFloat16INTEL = 8u32 ; const MatrixAPackedInt8INTEL = 16u32 ; const MatrixBPackedInt8INTEL = 32u32 ; const MatrixAPackedInt4INTEL = 64u32 ; const MatrixBPackedInt4INTEL = 128u32 ; const MatrixATF32INTEL = 256u32 ; const MatrixBTF32INTEL = 512u32 ; const MatrixAPackedFloat16INTEL = 1024u32 ; const MatrixBPackedFloat16INTEL = 2048u32 ; const MatrixAPackedBFloat16INTEL = 4096u32 ; const MatrixBPackedBFloat16INTEL = 8192u32 ; } }
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FPEncoding {
    BFloat16KHR = 0u32,
    Float8E4M3EXT = 4214u32,
    Float8E5M2EXT = 4215u32,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CooperativeVectorMatrixLayout {
    RowMajorNV = 0u32,
    ColumnMajorNV = 1u32,
    InferencingOptimalNV = 2u32,
    TrainingOptimalNV = 3u32,
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
bitflags! { # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct TensorOperands : u32 { const NoneARM = 0u32 ; const NontemporalARM = 1u32 ; const OutOfBoundsValueARM = 2u32 ; const MakeElementAvailableARM = 4u32 ; const MakeElementVisibleARM = 8u32 ; const NonPrivateElementARM = 16u32 ; } }
