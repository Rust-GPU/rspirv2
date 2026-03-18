use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum CoreInstSet {
    Nop(OpNop),
    Undef(OpUndef),
    SourceContinued(OpSourceContinued),
    Source(OpSource),
    SourceExtension(OpSourceExtension),
    Name(OpName),
    MemberName(OpMemberName),
    String(OpString),
    Line(OpLine),
    Extension(OpExtension),
    ExtInstImport(OpExtInstImport),
    ExtInst(OpExtInst),
    MemoryModel(OpMemoryModel),
    EntryPoint(OpEntryPoint),
    ExecutionMode(OpExecutionMode),
    Capability(OpCapability),
    TypeVoid(OpTypeVoid),
    TypeBool(OpTypeBool),
    TypeInt(OpTypeInt),
    TypeFloat(OpTypeFloat),
    TypeVector(OpTypeVector),
    TypeMatrix(OpTypeMatrix),
    TypeImage(OpTypeImage),
    TypeSampler(OpTypeSampler),
    TypeSampledImage(OpTypeSampledImage),
    TypeArray(OpTypeArray),
    TypeRuntimeArray(OpTypeRuntimeArray),
    TypeStruct(OpTypeStruct),
    TypeOpaque(OpTypeOpaque),
    TypePointer(OpTypePointer),
    TypeFunction(OpTypeFunction),
    TypeEvent(OpTypeEvent),
    TypeDeviceEvent(OpTypeDeviceEvent),
    TypeReserveId(OpTypeReserveId),
    TypeQueue(OpTypeQueue),
    TypePipe(OpTypePipe),
    TypeForwardPointer(OpTypeForwardPointer),
    ConstantTrue(OpConstantTrue),
    ConstantFalse(OpConstantFalse),
    Constant(OpConstant),
    ConstantComposite(OpConstantComposite),
    ConstantSampler(OpConstantSampler),
    ConstantNull(OpConstantNull),
    SpecConstantTrue(OpSpecConstantTrue),
    SpecConstantFalse(OpSpecConstantFalse),
    SpecConstant(OpSpecConstant),
    SpecConstantComposite(OpSpecConstantComposite),
    SpecConstantOp(OpSpecConstantOp),
    Function(OpFunction),
    FunctionParameter(OpFunctionParameter),
    FunctionEnd(OpFunctionEnd),
    FunctionCall(OpFunctionCall),
    Variable(OpVariable),
    ImageTexelPointer(OpImageTexelPointer),
    Load(OpLoad),
    Store(OpStore),
    CopyMemory(OpCopyMemory),
    CopyMemorySized(OpCopyMemorySized),
    AccessChain(OpAccessChain),
    InBoundsAccessChain(OpInBoundsAccessChain),
    PtrAccessChain(OpPtrAccessChain),
    ArrayLength(OpArrayLength),
    GenericPtrMemSemantics(OpGenericPtrMemSemantics),
    InBoundsPtrAccessChain(OpInBoundsPtrAccessChain),
    Decorate(OpDecorate),
    MemberDecorate(OpMemberDecorate),
    DecorationGroup(OpDecorationGroup),
    GroupDecorate(OpGroupDecorate),
    GroupMemberDecorate(OpGroupMemberDecorate),
    VectorExtractDynamic(OpVectorExtractDynamic),
    VectorInsertDynamic(OpVectorInsertDynamic),
    VectorShuffle(OpVectorShuffle),
    CompositeConstruct(OpCompositeConstruct),
    CompositeExtract(OpCompositeExtract),
    CompositeInsert(OpCompositeInsert),
    CopyObject(OpCopyObject),
    Transpose(OpTranspose),
    SampledImage(OpSampledImage),
    ImageSampleImplicitLod(OpImageSampleImplicitLod),
    ImageSampleExplicitLod(OpImageSampleExplicitLod),
    ImageSampleDrefImplicitLod(OpImageSampleDrefImplicitLod),
    ImageSampleDrefExplicitLod(OpImageSampleDrefExplicitLod),
    ImageSampleProjImplicitLod(OpImageSampleProjImplicitLod),
    ImageSampleProjExplicitLod(OpImageSampleProjExplicitLod),
    ImageSampleProjDrefImplicitLod(OpImageSampleProjDrefImplicitLod),
    ImageSampleProjDrefExplicitLod(OpImageSampleProjDrefExplicitLod),
    ImageFetch(OpImageFetch),
    ImageGather(OpImageGather),
    ImageDrefGather(OpImageDrefGather),
    ImageRead(OpImageRead),
    ImageWrite(OpImageWrite),
    Image(OpImage),
    ImageQueryFormat(OpImageQueryFormat),
    ImageQueryOrder(OpImageQueryOrder),
    ImageQuerySizeLod(OpImageQuerySizeLod),
    ImageQuerySize(OpImageQuerySize),
    ImageQueryLod(OpImageQueryLod),
    ImageQueryLevels(OpImageQueryLevels),
    ImageQuerySamples(OpImageQuerySamples),
    ConvertFToU(OpConvertFToU),
    ConvertFToS(OpConvertFToS),
    ConvertSToF(OpConvertSToF),
    ConvertUToF(OpConvertUToF),
    UConvert(OpUConvert),
    SConvert(OpSConvert),
    FConvert(OpFConvert),
    QuantizeToF16(OpQuantizeToF16),
    ConvertPtrToU(OpConvertPtrToU),
    SatConvertSToU(OpSatConvertSToU),
    SatConvertUToS(OpSatConvertUToS),
    ConvertUToPtr(OpConvertUToPtr),
    PtrCastToGeneric(OpPtrCastToGeneric),
    GenericCastToPtr(OpGenericCastToPtr),
    GenericCastToPtrExplicit(OpGenericCastToPtrExplicit),
    Bitcast(OpBitcast),
    SNegate(OpSNegate),
    FNegate(OpFNegate),
    IAdd(OpIAdd),
    FAdd(OpFAdd),
    ISub(OpISub),
    FSub(OpFSub),
    IMul(OpIMul),
    FMul(OpFMul),
    UDiv(OpUDiv),
    SDiv(OpSDiv),
    FDiv(OpFDiv),
    UMod(OpUMod),
    SRem(OpSRem),
    SMod(OpSMod),
    FRem(OpFRem),
    FMod(OpFMod),
    VectorTimesScalar(OpVectorTimesScalar),
    MatrixTimesScalar(OpMatrixTimesScalar),
    VectorTimesMatrix(OpVectorTimesMatrix),
    MatrixTimesVector(OpMatrixTimesVector),
    MatrixTimesMatrix(OpMatrixTimesMatrix),
    OuterProduct(OpOuterProduct),
    Dot(OpDot),
    IAddCarry(OpIAddCarry),
    ISubBorrow(OpISubBorrow),
    UMulExtended(OpUMulExtended),
    SMulExtended(OpSMulExtended),
    Any(OpAny),
    All(OpAll),
    IsNan(OpIsNan),
    IsInf(OpIsInf),
    IsFinite(OpIsFinite),
    IsNormal(OpIsNormal),
    SignBitSet(OpSignBitSet),
    LessOrGreater(OpLessOrGreater),
    Ordered(OpOrdered),
    Unordered(OpUnordered),
    LogicalEqual(OpLogicalEqual),
    LogicalNotEqual(OpLogicalNotEqual),
    LogicalOr(OpLogicalOr),
    LogicalAnd(OpLogicalAnd),
    LogicalNot(OpLogicalNot),
    Select(OpSelect),
    IEqual(OpIEqual),
    INotEqual(OpINotEqual),
    UGreaterThan(OpUGreaterThan),
    SGreaterThan(OpSGreaterThan),
    UGreaterThanEqual(OpUGreaterThanEqual),
    SGreaterThanEqual(OpSGreaterThanEqual),
    ULessThan(OpULessThan),
    SLessThan(OpSLessThan),
    ULessThanEqual(OpULessThanEqual),
    SLessThanEqual(OpSLessThanEqual),
    FOrdEqual(OpFOrdEqual),
    FUnordEqual(OpFUnordEqual),
    FOrdNotEqual(OpFOrdNotEqual),
    FUnordNotEqual(OpFUnordNotEqual),
    FOrdLessThan(OpFOrdLessThan),
    FUnordLessThan(OpFUnordLessThan),
    FOrdGreaterThan(OpFOrdGreaterThan),
    FUnordGreaterThan(OpFUnordGreaterThan),
    FOrdLessThanEqual(OpFOrdLessThanEqual),
    FUnordLessThanEqual(OpFUnordLessThanEqual),
    FOrdGreaterThanEqual(OpFOrdGreaterThanEqual),
    FUnordGreaterThanEqual(OpFUnordGreaterThanEqual),
    ShiftRightLogical(OpShiftRightLogical),
    ShiftRightArithmetic(OpShiftRightArithmetic),
    ShiftLeftLogical(OpShiftLeftLogical),
    BitwiseOr(OpBitwiseOr),
    BitwiseXor(OpBitwiseXor),
    BitwiseAnd(OpBitwiseAnd),
    Not(OpNot),
    BitFieldInsert(OpBitFieldInsert),
    BitFieldSExtract(OpBitFieldSExtract),
    BitFieldUExtract(OpBitFieldUExtract),
    BitReverse(OpBitReverse),
    BitCount(OpBitCount),
    DPdx(OpDPdx),
    DPdy(OpDPdy),
    Fwidth(OpFwidth),
    DPdxFine(OpDPdxFine),
    DPdyFine(OpDPdyFine),
    FwidthFine(OpFwidthFine),
    DPdxCoarse(OpDPdxCoarse),
    DPdyCoarse(OpDPdyCoarse),
    FwidthCoarse(OpFwidthCoarse),
    EmitVertex(OpEmitVertex),
    EndPrimitive(OpEndPrimitive),
    EmitStreamVertex(OpEmitStreamVertex),
    EndStreamPrimitive(OpEndStreamPrimitive),
    ControlBarrier(OpControlBarrier),
    MemoryBarrier(OpMemoryBarrier),
    AtomicLoad(OpAtomicLoad),
    AtomicStore(OpAtomicStore),
    AtomicExchange(OpAtomicExchange),
    AtomicCompareExchange(OpAtomicCompareExchange),
    AtomicCompareExchangeWeak(OpAtomicCompareExchangeWeak),
    AtomicIIncrement(OpAtomicIIncrement),
    AtomicIDecrement(OpAtomicIDecrement),
    AtomicIAdd(OpAtomicIAdd),
    AtomicISub(OpAtomicISub),
    AtomicSMin(OpAtomicSMin),
    AtomicUMin(OpAtomicUMin),
    AtomicSMax(OpAtomicSMax),
    AtomicUMax(OpAtomicUMax),
    AtomicAnd(OpAtomicAnd),
    AtomicOr(OpAtomicOr),
    AtomicXor(OpAtomicXor),
    Phi(OpPhi),
    LoopMerge(OpLoopMerge),
    SelectionMerge(OpSelectionMerge),
    Label(OpLabel),
    Branch(OpBranch),
    BranchConditional(OpBranchConditional),
    Switch(OpSwitch),
    Kill(OpKill),
    Return(OpReturn),
    ReturnValue(OpReturnValue),
    Unreachable(OpUnreachable),
    LifetimeStart(OpLifetimeStart),
    LifetimeStop(OpLifetimeStop),
    GroupAsyncCopy(OpGroupAsyncCopy),
    GroupWaitEvents(OpGroupWaitEvents),
    GroupAll(OpGroupAll),
    GroupAny(OpGroupAny),
    GroupBroadcast(OpGroupBroadcast),
    GroupIAdd(OpGroupIAdd),
    GroupFAdd(OpGroupFAdd),
    GroupFMin(OpGroupFMin),
    GroupUMin(OpGroupUMin),
    GroupSMin(OpGroupSMin),
    GroupFMax(OpGroupFMax),
    GroupUMax(OpGroupUMax),
    GroupSMax(OpGroupSMax),
    ReadPipe(OpReadPipe),
    WritePipe(OpWritePipe),
    ReservedReadPipe(OpReservedReadPipe),
    ReservedWritePipe(OpReservedWritePipe),
    ReserveReadPipePackets(OpReserveReadPipePackets),
    ReserveWritePipePackets(OpReserveWritePipePackets),
    CommitReadPipe(OpCommitReadPipe),
    CommitWritePipe(OpCommitWritePipe),
    IsValidReserveId(OpIsValidReserveId),
    GetNumPipePackets(OpGetNumPipePackets),
    GetMaxPipePackets(OpGetMaxPipePackets),
    GroupReserveReadPipePackets(OpGroupReserveReadPipePackets),
    GroupReserveWritePipePackets(OpGroupReserveWritePipePackets),
    GroupCommitReadPipe(OpGroupCommitReadPipe),
    GroupCommitWritePipe(OpGroupCommitWritePipe),
    EnqueueMarker(OpEnqueueMarker),
    EnqueueKernel(OpEnqueueKernel),
    GetKernelNDrangeSubGroupCount(OpGetKernelNDrangeSubGroupCount),
    GetKernelNDrangeMaxSubGroupSize(OpGetKernelNDrangeMaxSubGroupSize),
    GetKernelWorkGroupSize(OpGetKernelWorkGroupSize),
    GetKernelPreferredWorkGroupSizeMultiple(OpGetKernelPreferredWorkGroupSizeMultiple),
    RetainEvent(OpRetainEvent),
    ReleaseEvent(OpReleaseEvent),
    CreateUserEvent(OpCreateUserEvent),
    IsValidEvent(OpIsValidEvent),
    SetUserEventStatus(OpSetUserEventStatus),
    CaptureEventProfilingInfo(OpCaptureEventProfilingInfo),
    GetDefaultQueue(OpGetDefaultQueue),
    BuildNDRange(OpBuildNDRange),
    ImageSparseSampleImplicitLod(OpImageSparseSampleImplicitLod),
    ImageSparseSampleExplicitLod(OpImageSparseSampleExplicitLod),
    ImageSparseSampleDrefImplicitLod(OpImageSparseSampleDrefImplicitLod),
    ImageSparseSampleDrefExplicitLod(OpImageSparseSampleDrefExplicitLod),
    ImageSparseSampleProjImplicitLod(OpImageSparseSampleProjImplicitLod),
    ImageSparseSampleProjExplicitLod(OpImageSparseSampleProjExplicitLod),
    ImageSparseSampleProjDrefImplicitLod(OpImageSparseSampleProjDrefImplicitLod),
    ImageSparseSampleProjDrefExplicitLod(OpImageSparseSampleProjDrefExplicitLod),
    ImageSparseFetch(OpImageSparseFetch),
    ImageSparseGather(OpImageSparseGather),
    ImageSparseDrefGather(OpImageSparseDrefGather),
    ImageSparseTexelsResident(OpImageSparseTexelsResident),
    NoLine(OpNoLine),
    AtomicFlagTestAndSet(OpAtomicFlagTestAndSet),
    AtomicFlagClear(OpAtomicFlagClear),
    ImageSparseRead(OpImageSparseRead),
    SizeOf(OpSizeOf),
    TypePipeStorage(OpTypePipeStorage),
    ConstantPipeStorage(OpConstantPipeStorage),
    CreatePipeFromPipeStorage(OpCreatePipeFromPipeStorage),
    GetKernelLocalSizeForSubgroupCount(OpGetKernelLocalSizeForSubgroupCount),
    GetKernelMaxNumSubgroups(OpGetKernelMaxNumSubgroups),
    TypeNamedBarrier(OpTypeNamedBarrier),
    NamedBarrierInitialize(OpNamedBarrierInitialize),
    MemoryNamedBarrier(OpMemoryNamedBarrier),
    ModuleProcessed(OpModuleProcessed),
    ExecutionModeId(OpExecutionModeId),
    DecorateId(OpDecorateId),
    GroupNonUniformElect(OpGroupNonUniformElect),
    GroupNonUniformAll(OpGroupNonUniformAll),
    GroupNonUniformAny(OpGroupNonUniformAny),
    GroupNonUniformAllEqual(OpGroupNonUniformAllEqual),
    GroupNonUniformBroadcast(OpGroupNonUniformBroadcast),
    GroupNonUniformBroadcastFirst(OpGroupNonUniformBroadcastFirst),
    GroupNonUniformBallot(OpGroupNonUniformBallot),
    GroupNonUniformInverseBallot(OpGroupNonUniformInverseBallot),
    GroupNonUniformBallotBitExtract(OpGroupNonUniformBallotBitExtract),
    GroupNonUniformBallotBitCount(OpGroupNonUniformBallotBitCount),
    GroupNonUniformBallotFindLSB(OpGroupNonUniformBallotFindLSB),
    GroupNonUniformBallotFindMSB(OpGroupNonUniformBallotFindMSB),
    GroupNonUniformShuffle(OpGroupNonUniformShuffle),
    GroupNonUniformShuffleXor(OpGroupNonUniformShuffleXor),
    GroupNonUniformShuffleUp(OpGroupNonUniformShuffleUp),
    GroupNonUniformShuffleDown(OpGroupNonUniformShuffleDown),
    GroupNonUniformIAdd(OpGroupNonUniformIAdd),
    GroupNonUniformFAdd(OpGroupNonUniformFAdd),
    GroupNonUniformIMul(OpGroupNonUniformIMul),
    GroupNonUniformFMul(OpGroupNonUniformFMul),
    GroupNonUniformSMin(OpGroupNonUniformSMin),
    GroupNonUniformUMin(OpGroupNonUniformUMin),
    GroupNonUniformFMin(OpGroupNonUniformFMin),
    GroupNonUniformSMax(OpGroupNonUniformSMax),
    GroupNonUniformUMax(OpGroupNonUniformUMax),
    GroupNonUniformFMax(OpGroupNonUniformFMax),
    GroupNonUniformBitwiseAnd(OpGroupNonUniformBitwiseAnd),
    GroupNonUniformBitwiseOr(OpGroupNonUniformBitwiseOr),
    GroupNonUniformBitwiseXor(OpGroupNonUniformBitwiseXor),
    GroupNonUniformLogicalAnd(OpGroupNonUniformLogicalAnd),
    GroupNonUniformLogicalOr(OpGroupNonUniformLogicalOr),
    GroupNonUniformLogicalXor(OpGroupNonUniformLogicalXor),
    GroupNonUniformQuadBroadcast(OpGroupNonUniformQuadBroadcast),
    GroupNonUniformQuadSwap(OpGroupNonUniformQuadSwap),
    CopyLogical(OpCopyLogical),
    PtrEqual(OpPtrEqual),
    PtrNotEqual(OpPtrNotEqual),
    PtrDiff(OpPtrDiff),
    ColorAttachmentReadEXT(OpColorAttachmentReadEXT),
    DepthAttachmentReadEXT(OpDepthAttachmentReadEXT),
    StencilAttachmentReadEXT(OpStencilAttachmentReadEXT),
    TypeTensorARM(OpTypeTensorARM),
    TensorReadARM(OpTensorReadARM),
    TensorWriteARM(OpTensorWriteARM),
    TensorQuerySizeARM(OpTensorQuerySizeARM),
    GraphConstantARM(OpGraphConstantARM),
    GraphEntryPointARM(OpGraphEntryPointARM),
    GraphARM(OpGraphARM),
    GraphInputARM(OpGraphInputARM),
    GraphSetOutputARM(OpGraphSetOutputARM),
    GraphEndARM(OpGraphEndARM),
    TypeGraphARM(OpTypeGraphARM),
    TerminateInvocation(OpTerminateInvocation),
    TypeUntypedPointerKHR(OpTypeUntypedPointerKHR),
    UntypedVariableKHR(OpUntypedVariableKHR),
    UntypedAccessChainKHR(OpUntypedAccessChainKHR),
    UntypedInBoundsAccessChainKHR(OpUntypedInBoundsAccessChainKHR),
    SubgroupBallotKHR(OpSubgroupBallotKHR),
    SubgroupFirstInvocationKHR(OpSubgroupFirstInvocationKHR),
    UntypedPtrAccessChainKHR(OpUntypedPtrAccessChainKHR),
    UntypedInBoundsPtrAccessChainKHR(OpUntypedInBoundsPtrAccessChainKHR),
    UntypedArrayLengthKHR(OpUntypedArrayLengthKHR),
    UntypedPrefetchKHR(OpUntypedPrefetchKHR),
    FmaKHR(OpFmaKHR),
    SubgroupAllKHR(OpSubgroupAllKHR),
    SubgroupAnyKHR(OpSubgroupAnyKHR),
    SubgroupAllEqualKHR(OpSubgroupAllEqualKHR),
    GroupNonUniformRotateKHR(OpGroupNonUniformRotateKHR),
    SubgroupReadInvocationKHR(OpSubgroupReadInvocationKHR),
    ExtInstWithForwardRefsKHR(OpExtInstWithForwardRefsKHR),
    UntypedGroupAsyncCopyKHR(OpUntypedGroupAsyncCopyKHR),
    TraceRayKHR(OpTraceRayKHR),
    ExecuteCallableKHR(OpExecuteCallableKHR),
    ConvertUToAccelerationStructureKHR(OpConvertUToAccelerationStructureKHR),
    IgnoreIntersectionKHR(OpIgnoreIntersectionKHR),
    TerminateRayKHR(OpTerminateRayKHR),
    SDot(OpSDot),
    UDot(OpUDot),
    SUDot(OpSUDot),
    SDotAccSat(OpSDotAccSat),
    UDotAccSat(OpUDotAccSat),
    SUDotAccSat(OpSUDotAccSat),
    TypeCooperativeMatrixKHR(OpTypeCooperativeMatrixKHR),
    CooperativeMatrixLoadKHR(OpCooperativeMatrixLoadKHR),
    CooperativeMatrixStoreKHR(OpCooperativeMatrixStoreKHR),
    CooperativeMatrixMulAddKHR(OpCooperativeMatrixMulAddKHR),
    CooperativeMatrixLengthKHR(OpCooperativeMatrixLengthKHR),
    ConstantCompositeReplicateEXT(OpConstantCompositeReplicateEXT),
    SpecConstantCompositeReplicateEXT(OpSpecConstantCompositeReplicateEXT),
    CompositeConstructReplicateEXT(OpCompositeConstructReplicateEXT),
    TypeRayQueryKHR(OpTypeRayQueryKHR),
    RayQueryInitializeKHR(OpRayQueryInitializeKHR),
    RayQueryTerminateKHR(OpRayQueryTerminateKHR),
    RayQueryGenerateIntersectionKHR(OpRayQueryGenerateIntersectionKHR),
    RayQueryConfirmIntersectionKHR(OpRayQueryConfirmIntersectionKHR),
    RayQueryProceedKHR(OpRayQueryProceedKHR),
    RayQueryGetIntersectionTypeKHR(OpRayQueryGetIntersectionTypeKHR),
    ImageSampleWeightedQCOM(OpImageSampleWeightedQCOM),
    ImageBoxFilterQCOM(OpImageBoxFilterQCOM),
    ImageBlockMatchSSDQCOM(OpImageBlockMatchSSDQCOM),
    ImageBlockMatchSADQCOM(OpImageBlockMatchSADQCOM),
    BitCastArrayQCOM(OpBitCastArrayQCOM),
    ImageBlockMatchWindowSSDQCOM(OpImageBlockMatchWindowSSDQCOM),
    ImageBlockMatchWindowSADQCOM(OpImageBlockMatchWindowSADQCOM),
    ImageBlockMatchGatherSSDQCOM(OpImageBlockMatchGatherSSDQCOM),
    ImageBlockMatchGatherSADQCOM(OpImageBlockMatchGatherSADQCOM),
    CompositeConstructCoopMatQCOM(OpCompositeConstructCoopMatQCOM),
    CompositeExtractCoopMatQCOM(OpCompositeExtractCoopMatQCOM),
    ExtractSubArrayQCOM(OpExtractSubArrayQCOM),
    GroupIAddNonUniformAMD(OpGroupIAddNonUniformAMD),
    GroupFAddNonUniformAMD(OpGroupFAddNonUniformAMD),
    GroupFMinNonUniformAMD(OpGroupFMinNonUniformAMD),
    GroupUMinNonUniformAMD(OpGroupUMinNonUniformAMD),
    GroupSMinNonUniformAMD(OpGroupSMinNonUniformAMD),
    GroupFMaxNonUniformAMD(OpGroupFMaxNonUniformAMD),
    GroupUMaxNonUniformAMD(OpGroupUMaxNonUniformAMD),
    GroupSMaxNonUniformAMD(OpGroupSMaxNonUniformAMD),
    FragmentMaskFetchAMD(OpFragmentMaskFetchAMD),
    FragmentFetchAMD(OpFragmentFetchAMD),
    ReadClockKHR(OpReadClockKHR),
    AllocateNodePayloadsAMDX(OpAllocateNodePayloadsAMDX),
    EnqueueNodePayloadsAMDX(OpEnqueueNodePayloadsAMDX),
    TypeNodePayloadArrayAMDX(OpTypeNodePayloadArrayAMDX),
    FinishWritingNodePayloadAMDX(OpFinishWritingNodePayloadAMDX),
    NodePayloadArrayLengthAMDX(OpNodePayloadArrayLengthAMDX),
    IsNodePayloadValidAMDX(OpIsNodePayloadValidAMDX),
    ConstantStringAMDX(OpConstantStringAMDX),
    SpecConstantStringAMDX(OpSpecConstantStringAMDX),
    GroupNonUniformQuadAllKHR(OpGroupNonUniformQuadAllKHR),
    GroupNonUniformQuadAnyKHR(OpGroupNonUniformQuadAnyKHR),
    TypeBufferEXT(OpTypeBufferEXT),
    BufferPointerEXT(OpBufferPointerEXT),
    UntypedImageTexelPointerEXT(OpUntypedImageTexelPointerEXT),
    MemberDecorateIdEXT(OpMemberDecorateIdEXT),
    ConstantSizeOfEXT(OpConstantSizeOfEXT),
    HitObjectRecordHitMotionNV(OpHitObjectRecordHitMotionNV),
    HitObjectRecordHitWithIndexMotionNV(OpHitObjectRecordHitWithIndexMotionNV),
    HitObjectRecordMissMotionNV(OpHitObjectRecordMissMotionNV),
    HitObjectGetWorldToObjectNV(OpHitObjectGetWorldToObjectNV),
    HitObjectGetObjectToWorldNV(OpHitObjectGetObjectToWorldNV),
    HitObjectGetObjectRayDirectionNV(OpHitObjectGetObjectRayDirectionNV),
    HitObjectGetObjectRayOriginNV(OpHitObjectGetObjectRayOriginNV),
    HitObjectTraceRayMotionNV(OpHitObjectTraceRayMotionNV),
    HitObjectGetShaderRecordBufferHandleNV(OpHitObjectGetShaderRecordBufferHandleNV),
    HitObjectGetShaderBindingTableRecordIndexNV(OpHitObjectGetShaderBindingTableRecordIndexNV),
    HitObjectRecordEmptyNV(OpHitObjectRecordEmptyNV),
    HitObjectTraceRayNV(OpHitObjectTraceRayNV),
    HitObjectRecordHitNV(OpHitObjectRecordHitNV),
    HitObjectRecordHitWithIndexNV(OpHitObjectRecordHitWithIndexNV),
    HitObjectRecordMissNV(OpHitObjectRecordMissNV),
    HitObjectExecuteShaderNV(OpHitObjectExecuteShaderNV),
    HitObjectGetCurrentTimeNV(OpHitObjectGetCurrentTimeNV),
    HitObjectGetAttributesNV(OpHitObjectGetAttributesNV),
    HitObjectGetHitKindNV(OpHitObjectGetHitKindNV),
    HitObjectGetPrimitiveIndexNV(OpHitObjectGetPrimitiveIndexNV),
    HitObjectGetGeometryIndexNV(OpHitObjectGetGeometryIndexNV),
    HitObjectGetInstanceIdNV(OpHitObjectGetInstanceIdNV),
    HitObjectGetInstanceCustomIndexNV(OpHitObjectGetInstanceCustomIndexNV),
    HitObjectGetWorldRayDirectionNV(OpHitObjectGetWorldRayDirectionNV),
    HitObjectGetWorldRayOriginNV(OpHitObjectGetWorldRayOriginNV),
    HitObjectGetRayTMaxNV(OpHitObjectGetRayTMaxNV),
    HitObjectGetRayTMinNV(OpHitObjectGetRayTMinNV),
    HitObjectIsEmptyNV(OpHitObjectIsEmptyNV),
    HitObjectIsHitNV(OpHitObjectIsHitNV),
    HitObjectIsMissNV(OpHitObjectIsMissNV),
    ReorderThreadWithHitObjectNV(OpReorderThreadWithHitObjectNV),
    ReorderThreadWithHintNV(OpReorderThreadWithHintNV),
    TypeHitObjectNV(OpTypeHitObjectNV),
    ImageSampleFootprintNV(OpImageSampleFootprintNV),
    TypeVectorIdEXT(OpTypeVectorIdEXT),
    CooperativeVectorMatrixMulNV(OpCooperativeVectorMatrixMulNV),
    CooperativeVectorOuterProductAccumulateNV(OpCooperativeVectorOuterProductAccumulateNV),
    CooperativeVectorReduceSumAccumulateNV(OpCooperativeVectorReduceSumAccumulateNV),
    CooperativeVectorMatrixMulAddNV(OpCooperativeVectorMatrixMulAddNV),
    CooperativeMatrixConvertNV(OpCooperativeMatrixConvertNV),
    EmitMeshTasksEXT(OpEmitMeshTasksEXT),
    SetMeshOutputsEXT(OpSetMeshOutputsEXT),
    GroupNonUniformPartitionEXT(OpGroupNonUniformPartitionEXT),
    WritePackedPrimitiveIndices4x8NV(OpWritePackedPrimitiveIndices4x8NV),
    FetchMicroTriangleVertexPositionNV(OpFetchMicroTriangleVertexPositionNV),
    FetchMicroTriangleVertexBarycentricNV(OpFetchMicroTriangleVertexBarycentricNV),
    CooperativeVectorLoadNV(OpCooperativeVectorLoadNV),
    CooperativeVectorStoreNV(OpCooperativeVectorStoreNV),
    HitObjectRecordFromQueryEXT(OpHitObjectRecordFromQueryEXT),
    HitObjectRecordMissEXT(OpHitObjectRecordMissEXT),
    HitObjectRecordMissMotionEXT(OpHitObjectRecordMissMotionEXT),
    HitObjectGetIntersectionTriangleVertexPositionsEXT(
        OpHitObjectGetIntersectionTriangleVertexPositionsEXT,
    ),
    HitObjectGetRayFlagsEXT(OpHitObjectGetRayFlagsEXT),
    HitObjectSetShaderBindingTableRecordIndexEXT(OpHitObjectSetShaderBindingTableRecordIndexEXT),
    HitObjectReorderExecuteShaderEXT(OpHitObjectReorderExecuteShaderEXT),
    HitObjectTraceReorderExecuteEXT(OpHitObjectTraceReorderExecuteEXT),
    HitObjectTraceMotionReorderExecuteEXT(OpHitObjectTraceMotionReorderExecuteEXT),
    TypeHitObjectEXT(OpTypeHitObjectEXT),
    ReorderThreadWithHintEXT(OpReorderThreadWithHintEXT),
    ReorderThreadWithHitObjectEXT(OpReorderThreadWithHitObjectEXT),
    HitObjectTraceRayEXT(OpHitObjectTraceRayEXT),
    HitObjectTraceRayMotionEXT(OpHitObjectTraceRayMotionEXT),
    HitObjectRecordEmptyEXT(OpHitObjectRecordEmptyEXT),
    HitObjectExecuteShaderEXT(OpHitObjectExecuteShaderEXT),
    HitObjectGetCurrentTimeEXT(OpHitObjectGetCurrentTimeEXT),
    HitObjectGetAttributesEXT(OpHitObjectGetAttributesEXT),
    HitObjectGetHitKindEXT(OpHitObjectGetHitKindEXT),
    HitObjectGetPrimitiveIndexEXT(OpHitObjectGetPrimitiveIndexEXT),
    HitObjectGetGeometryIndexEXT(OpHitObjectGetGeometryIndexEXT),
    HitObjectGetInstanceIdEXT(OpHitObjectGetInstanceIdEXT),
    HitObjectGetInstanceCustomIndexEXT(OpHitObjectGetInstanceCustomIndexEXT),
    HitObjectGetObjectRayOriginEXT(OpHitObjectGetObjectRayOriginEXT),
    HitObjectGetObjectRayDirectionEXT(OpHitObjectGetObjectRayDirectionEXT),
    HitObjectGetWorldRayDirectionEXT(OpHitObjectGetWorldRayDirectionEXT),
    HitObjectGetWorldRayOriginEXT(OpHitObjectGetWorldRayOriginEXT),
    HitObjectGetObjectToWorldEXT(OpHitObjectGetObjectToWorldEXT),
    HitObjectGetWorldToObjectEXT(OpHitObjectGetWorldToObjectEXT),
    HitObjectGetRayTMaxEXT(OpHitObjectGetRayTMaxEXT),
    ReportIntersectionKHR(OpReportIntersectionKHR),
    IgnoreIntersectionNV(OpIgnoreIntersectionNV),
    TerminateRayNV(OpTerminateRayNV),
    TraceNV(OpTraceNV),
    TraceMotionNV(OpTraceMotionNV),
    TraceRayMotionNV(OpTraceRayMotionNV),
    RayQueryGetIntersectionTriangleVertexPositionsKHR(
        OpRayQueryGetIntersectionTriangleVertexPositionsKHR,
    ),
    TypeAccelerationStructureKHR(OpTypeAccelerationStructureKHR),
    ExecuteCallableNV(OpExecuteCallableNV),
    RayQueryGetIntersectionClusterIdNV(OpRayQueryGetIntersectionClusterIdNV),
    HitObjectGetClusterIdNV(OpHitObjectGetClusterIdNV),
    HitObjectGetRayTMinEXT(OpHitObjectGetRayTMinEXT),
    HitObjectGetShaderBindingTableRecordIndexEXT(OpHitObjectGetShaderBindingTableRecordIndexEXT),
    HitObjectGetShaderRecordBufferHandleEXT(OpHitObjectGetShaderRecordBufferHandleEXT),
    HitObjectIsEmptyEXT(OpHitObjectIsEmptyEXT),
    HitObjectIsHitEXT(OpHitObjectIsHitEXT),
    HitObjectIsMissEXT(OpHitObjectIsMissEXT),
    TypeCooperativeMatrixNV(OpTypeCooperativeMatrixNV),
    CooperativeMatrixLoadNV(OpCooperativeMatrixLoadNV),
    CooperativeMatrixStoreNV(OpCooperativeMatrixStoreNV),
    CooperativeMatrixMulAddNV(OpCooperativeMatrixMulAddNV),
    CooperativeMatrixLengthNV(OpCooperativeMatrixLengthNV),
    BeginInvocationInterlockEXT(OpBeginInvocationInterlockEXT),
    EndInvocationInterlockEXT(OpEndInvocationInterlockEXT),
    CooperativeMatrixReduceNV(OpCooperativeMatrixReduceNV),
    CooperativeMatrixLoadTensorNV(OpCooperativeMatrixLoadTensorNV),
    CooperativeMatrixStoreTensorNV(OpCooperativeMatrixStoreTensorNV),
    CooperativeMatrixPerElementOpNV(OpCooperativeMatrixPerElementOpNV),
    TypeTensorLayoutNV(OpTypeTensorLayoutNV),
    TypeTensorViewNV(OpTypeTensorViewNV),
    CreateTensorLayoutNV(OpCreateTensorLayoutNV),
    TensorLayoutSetDimensionNV(OpTensorLayoutSetDimensionNV),
    TensorLayoutSetStrideNV(OpTensorLayoutSetStrideNV),
    TensorLayoutSliceNV(OpTensorLayoutSliceNV),
    TensorLayoutSetClampValueNV(OpTensorLayoutSetClampValueNV),
    CreateTensorViewNV(OpCreateTensorViewNV),
    TensorViewSetDimensionNV(OpTensorViewSetDimensionNV),
    TensorViewSetStrideNV(OpTensorViewSetStrideNV),
    DemoteToHelperInvocation(OpDemoteToHelperInvocation),
    IsHelperInvocationEXT(OpIsHelperInvocationEXT),
    TensorViewSetClipNV(OpTensorViewSetClipNV),
    TensorLayoutSetBlockSizeNV(OpTensorLayoutSetBlockSizeNV),
    CooperativeMatrixTransposeNV(OpCooperativeMatrixTransposeNV),
    ConvertUToImageNV(OpConvertUToImageNV),
    ConvertUToSamplerNV(OpConvertUToSamplerNV),
    ConvertImageToUNV(OpConvertImageToUNV),
    ConvertSamplerToUNV(OpConvertSamplerToUNV),
    ConvertUToSampledImageNV(OpConvertUToSampledImageNV),
    ConvertSampledImageToUNV(OpConvertSampledImageToUNV),
    SamplerImageAddressingModeNV(OpSamplerImageAddressingModeNV),
    RawAccessChainNV(OpRawAccessChainNV),
    RayQueryGetIntersectionSpherePositionNV(OpRayQueryGetIntersectionSpherePositionNV),
    RayQueryGetIntersectionSphereRadiusNV(OpRayQueryGetIntersectionSphereRadiusNV),
    RayQueryGetIntersectionLSSPositionsNV(OpRayQueryGetIntersectionLSSPositionsNV),
    RayQueryGetIntersectionLSSRadiiNV(OpRayQueryGetIntersectionLSSRadiiNV),
    RayQueryGetIntersectionLSSHitValueNV(OpRayQueryGetIntersectionLSSHitValueNV),
    HitObjectGetSpherePositionNV(OpHitObjectGetSpherePositionNV),
    HitObjectGetSphereRadiusNV(OpHitObjectGetSphereRadiusNV),
    HitObjectGetLSSPositionsNV(OpHitObjectGetLSSPositionsNV),
    HitObjectGetLSSRadiiNV(OpHitObjectGetLSSRadiiNV),
    HitObjectIsSphereHitNV(OpHitObjectIsSphereHitNV),
    HitObjectIsLSSHitNV(OpHitObjectIsLSSHitNV),
    RayQueryIsSphereHitNV(OpRayQueryIsSphereHitNV),
    RayQueryIsLSSHitNV(OpRayQueryIsLSSHitNV),
    SubgroupShuffleINTEL(OpSubgroupShuffleINTEL),
    SubgroupShuffleDownINTEL(OpSubgroupShuffleDownINTEL),
    SubgroupShuffleUpINTEL(OpSubgroupShuffleUpINTEL),
    SubgroupShuffleXorINTEL(OpSubgroupShuffleXorINTEL),
    SubgroupBlockReadINTEL(OpSubgroupBlockReadINTEL),
    SubgroupBlockWriteINTEL(OpSubgroupBlockWriteINTEL),
    SubgroupImageBlockReadINTEL(OpSubgroupImageBlockReadINTEL),
    SubgroupImageBlockWriteINTEL(OpSubgroupImageBlockWriteINTEL),
    SubgroupImageMediaBlockReadINTEL(OpSubgroupImageMediaBlockReadINTEL),
    SubgroupImageMediaBlockWriteINTEL(OpSubgroupImageMediaBlockWriteINTEL),
    UCountLeadingZerosINTEL(OpUCountLeadingZerosINTEL),
    UCountTrailingZerosINTEL(OpUCountTrailingZerosINTEL),
    AbsISubINTEL(OpAbsISubINTEL),
    AbsUSubINTEL(OpAbsUSubINTEL),
    IAddSatINTEL(OpIAddSatINTEL),
    UAddSatINTEL(OpUAddSatINTEL),
    IAverageINTEL(OpIAverageINTEL),
    UAverageINTEL(OpUAverageINTEL),
    IAverageRoundedINTEL(OpIAverageRoundedINTEL),
    UAverageRoundedINTEL(OpUAverageRoundedINTEL),
    ISubSatINTEL(OpISubSatINTEL),
    USubSatINTEL(OpUSubSatINTEL),
    IMul32x16INTEL(OpIMul32x16INTEL),
    UMul32x16INTEL(OpUMul32x16INTEL),
    ConstantFunctionPointerINTEL(OpConstantFunctionPointerINTEL),
    FunctionPointerCallINTEL(OpFunctionPointerCallINTEL),
    AsmTargetINTEL(OpAsmTargetINTEL),
    AsmINTEL(OpAsmINTEL),
    AsmCallINTEL(OpAsmCallINTEL),
    AtomicFMinEXT(OpAtomicFMinEXT),
    AtomicFMaxEXT(OpAtomicFMaxEXT),
    AssumeTrueKHR(OpAssumeTrueKHR),
    ExpectKHR(OpExpectKHR),
    DecorateString(OpDecorateString),
    MemberDecorateString(OpMemberDecorateString),
    VmeImageINTEL(OpVmeImageINTEL),
    TypeVmeImageINTEL(OpTypeVmeImageINTEL),
    TypeAvcImePayloadINTEL(OpTypeAvcImePayloadINTEL),
    TypeAvcRefPayloadINTEL(OpTypeAvcRefPayloadINTEL),
    TypeAvcSicPayloadINTEL(OpTypeAvcSicPayloadINTEL),
    TypeAvcMcePayloadINTEL(OpTypeAvcMcePayloadINTEL),
    TypeAvcMceResultINTEL(OpTypeAvcMceResultINTEL),
    TypeAvcImeResultINTEL(OpTypeAvcImeResultINTEL),
    TypeAvcImeResultSingleReferenceStreamoutINTEL(OpTypeAvcImeResultSingleReferenceStreamoutINTEL),
    TypeAvcImeResultDualReferenceStreamoutINTEL(OpTypeAvcImeResultDualReferenceStreamoutINTEL),
    TypeAvcImeSingleReferenceStreaminINTEL(OpTypeAvcImeSingleReferenceStreaminINTEL),
    TypeAvcImeDualReferenceStreaminINTEL(OpTypeAvcImeDualReferenceStreaminINTEL),
    TypeAvcRefResultINTEL(OpTypeAvcRefResultINTEL),
    TypeAvcSicResultINTEL(OpTypeAvcSicResultINTEL),
    SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(
        OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL,
    ),
    SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(
        OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL,
    ),
    SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(
        OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL,
    ),
    SubgroupAvcMceSetInterShapePenaltyINTEL(OpSubgroupAvcMceSetInterShapePenaltyINTEL),
    SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(
        OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL,
    ),
    SubgroupAvcMceSetInterDirectionPenaltyINTEL(OpSubgroupAvcMceSetInterDirectionPenaltyINTEL),
    SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(
        OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL,
    ),
    SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(
        OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL,
    ),
    SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(
        OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL,
    ),
    SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(
        OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL,
    ),
    SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(
        OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL,
    ),
    SubgroupAvcMceSetMotionVectorCostFunctionINTEL(
        OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL,
    ),
    SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(
        OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL,
    ),
    SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(
        OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL,
    ),
    SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(
        OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL,
    ),
    SubgroupAvcMceSetAcOnlyHaarINTEL(OpSubgroupAvcMceSetAcOnlyHaarINTEL),
    SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(
        OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL,
    ),
    SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(
        OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL,
    ),
    SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(
        OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL,
    ),
    SubgroupAvcMceConvertToImePayloadINTEL(OpSubgroupAvcMceConvertToImePayloadINTEL),
    SubgroupAvcMceConvertToImeResultINTEL(OpSubgroupAvcMceConvertToImeResultINTEL),
    SubgroupAvcMceConvertToRefPayloadINTEL(OpSubgroupAvcMceConvertToRefPayloadINTEL),
    SubgroupAvcMceConvertToRefResultINTEL(OpSubgroupAvcMceConvertToRefResultINTEL),
    SubgroupAvcMceConvertToSicPayloadINTEL(OpSubgroupAvcMceConvertToSicPayloadINTEL),
    SubgroupAvcMceConvertToSicResultINTEL(OpSubgroupAvcMceConvertToSicResultINTEL),
    SubgroupAvcMceGetMotionVectorsINTEL(OpSubgroupAvcMceGetMotionVectorsINTEL),
    SubgroupAvcMceGetInterDistortionsINTEL(OpSubgroupAvcMceGetInterDistortionsINTEL),
    SubgroupAvcMceGetBestInterDistortionsINTEL(OpSubgroupAvcMceGetBestInterDistortionsINTEL),
    SubgroupAvcMceGetInterMajorShapeINTEL(OpSubgroupAvcMceGetInterMajorShapeINTEL),
    SubgroupAvcMceGetInterMinorShapeINTEL(OpSubgroupAvcMceGetInterMinorShapeINTEL),
    SubgroupAvcMceGetInterDirectionsINTEL(OpSubgroupAvcMceGetInterDirectionsINTEL),
    SubgroupAvcMceGetInterMotionVectorCountINTEL(OpSubgroupAvcMceGetInterMotionVectorCountINTEL),
    SubgroupAvcMceGetInterReferenceIdsINTEL(OpSubgroupAvcMceGetInterReferenceIdsINTEL),
    SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(
        OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL,
    ),
    SubgroupAvcImeInitializeINTEL(OpSubgroupAvcImeInitializeINTEL),
    SubgroupAvcImeSetSingleReferenceINTEL(OpSubgroupAvcImeSetSingleReferenceINTEL),
    SubgroupAvcImeSetDualReferenceINTEL(OpSubgroupAvcImeSetDualReferenceINTEL),
    SubgroupAvcImeRefWindowSizeINTEL(OpSubgroupAvcImeRefWindowSizeINTEL),
    SubgroupAvcImeAdjustRefOffsetINTEL(OpSubgroupAvcImeAdjustRefOffsetINTEL),
    SubgroupAvcImeConvertToMcePayloadINTEL(OpSubgroupAvcImeConvertToMcePayloadINTEL),
    SubgroupAvcImeSetMaxMotionVectorCountINTEL(OpSubgroupAvcImeSetMaxMotionVectorCountINTEL),
    SubgroupAvcImeSetUnidirectionalMixDisableINTEL(
        OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL,
    ),
    SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(
        OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL,
    ),
    SubgroupAvcImeSetWeightedSadINTEL(OpSubgroupAvcImeSetWeightedSadINTEL),
    SubgroupAvcImeEvaluateWithSingleReferenceINTEL(
        OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL,
    ),
    SubgroupAvcImeEvaluateWithDualReferenceINTEL(OpSubgroupAvcImeEvaluateWithDualReferenceINTEL),
    SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(
        OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL,
    ),
    SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(
        OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL,
    ),
    SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(
        OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL,
    ),
    SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(
        OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL,
    ),
    SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(
        OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL,
    ),
    SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(
        OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL,
    ),
    SubgroupAvcImeConvertToMceResultINTEL(OpSubgroupAvcImeConvertToMceResultINTEL),
    SubgroupAvcImeGetSingleReferenceStreaminINTEL(OpSubgroupAvcImeGetSingleReferenceStreaminINTEL),
    SubgroupAvcImeGetDualReferenceStreaminINTEL(OpSubgroupAvcImeGetDualReferenceStreaminINTEL),
    SubgroupAvcImeStripSingleReferenceStreamoutINTEL(
        OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL,
    ),
    SubgroupAvcImeStripDualReferenceStreamoutINTEL(
        OpSubgroupAvcImeStripDualReferenceStreamoutINTEL,
    ),
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
        OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL,
    ),
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(
        OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL,
    ),
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
        OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL,
    ),
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
        OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL,
    ),
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
        OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL,
    ),
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
        OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL,
    ),
    SubgroupAvcImeGetBorderReachedINTEL(OpSubgroupAvcImeGetBorderReachedINTEL),
    SubgroupAvcImeGetTruncatedSearchIndicationINTEL(
        OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL,
    ),
    SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(
        OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL,
    ),
    SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(
        OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL,
    ),
    SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(
        OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL,
    ),
    SubgroupAvcFmeInitializeINTEL(OpSubgroupAvcFmeInitializeINTEL),
    SubgroupAvcBmeInitializeINTEL(OpSubgroupAvcBmeInitializeINTEL),
    SubgroupAvcRefConvertToMcePayloadINTEL(OpSubgroupAvcRefConvertToMcePayloadINTEL),
    SubgroupAvcRefSetBidirectionalMixDisableINTEL(OpSubgroupAvcRefSetBidirectionalMixDisableINTEL),
    SubgroupAvcRefSetBilinearFilterEnableINTEL(OpSubgroupAvcRefSetBilinearFilterEnableINTEL),
    SubgroupAvcRefEvaluateWithSingleReferenceINTEL(
        OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL,
    ),
    SubgroupAvcRefEvaluateWithDualReferenceINTEL(OpSubgroupAvcRefEvaluateWithDualReferenceINTEL),
    SubgroupAvcRefEvaluateWithMultiReferenceINTEL(OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL),
    SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(
        OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL,
    ),
    SubgroupAvcRefConvertToMceResultINTEL(OpSubgroupAvcRefConvertToMceResultINTEL),
    SubgroupAvcSicInitializeINTEL(OpSubgroupAvcSicInitializeINTEL),
    SubgroupAvcSicConfigureSkcINTEL(OpSubgroupAvcSicConfigureSkcINTEL),
    SubgroupAvcSicConfigureIpeLumaINTEL(OpSubgroupAvcSicConfigureIpeLumaINTEL),
    SubgroupAvcSicConfigureIpeLumaChromaINTEL(OpSubgroupAvcSicConfigureIpeLumaChromaINTEL),
    SubgroupAvcSicGetMotionVectorMaskINTEL(OpSubgroupAvcSicGetMotionVectorMaskINTEL),
    SubgroupAvcSicConvertToMcePayloadINTEL(OpSubgroupAvcSicConvertToMcePayloadINTEL),
    SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL),
    SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(
        OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL,
    ),
    SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(
        OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL,
    ),
    SubgroupAvcSicSetBilinearFilterEnableINTEL(OpSubgroupAvcSicSetBilinearFilterEnableINTEL),
    SubgroupAvcSicSetSkcForwardTransformEnableINTEL(
        OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL,
    ),
    SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL),
    SubgroupAvcSicEvaluateIpeINTEL(OpSubgroupAvcSicEvaluateIpeINTEL),
    SubgroupAvcSicEvaluateWithSingleReferenceINTEL(
        OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL,
    ),
    SubgroupAvcSicEvaluateWithDualReferenceINTEL(OpSubgroupAvcSicEvaluateWithDualReferenceINTEL),
    SubgroupAvcSicEvaluateWithMultiReferenceINTEL(OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL),
    SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(
        OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL,
    ),
    SubgroupAvcSicConvertToMceResultINTEL(OpSubgroupAvcSicConvertToMceResultINTEL),
    SubgroupAvcSicGetIpeLumaShapeINTEL(OpSubgroupAvcSicGetIpeLumaShapeINTEL),
    SubgroupAvcSicGetBestIpeLumaDistortionINTEL(OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL),
    SubgroupAvcSicGetBestIpeChromaDistortionINTEL(OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL),
    SubgroupAvcSicGetPackedIpeLumaModesINTEL(OpSubgroupAvcSicGetPackedIpeLumaModesINTEL),
    SubgroupAvcSicGetIpeChromaModeINTEL(OpSubgroupAvcSicGetIpeChromaModeINTEL),
    SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(
        OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL,
    ),
    SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(
        OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL,
    ),
    SubgroupAvcSicGetInterRawSadsINTEL(OpSubgroupAvcSicGetInterRawSadsINTEL),
    VariableLengthArrayINTEL(OpVariableLengthArrayINTEL),
    SaveMemoryINTEL(OpSaveMemoryINTEL),
    RestoreMemoryINTEL(OpRestoreMemoryINTEL),
    ArbitraryFloatSinCosPiALTERA(OpArbitraryFloatSinCosPiALTERA),
    ArbitraryFloatCastALTERA(OpArbitraryFloatCastALTERA),
    ArbitraryFloatCastFromIntALTERA(OpArbitraryFloatCastFromIntALTERA),
    ArbitraryFloatCastToIntALTERA(OpArbitraryFloatCastToIntALTERA),
    ArbitraryFloatAddALTERA(OpArbitraryFloatAddALTERA),
    ArbitraryFloatSubALTERA(OpArbitraryFloatSubALTERA),
    ArbitraryFloatMulALTERA(OpArbitraryFloatMulALTERA),
    ArbitraryFloatDivALTERA(OpArbitraryFloatDivALTERA),
    ArbitraryFloatGTALTERA(OpArbitraryFloatGTALTERA),
    ArbitraryFloatGEALTERA(OpArbitraryFloatGEALTERA),
    ArbitraryFloatLTALTERA(OpArbitraryFloatLTALTERA),
    ArbitraryFloatLEALTERA(OpArbitraryFloatLEALTERA),
    ArbitraryFloatEQALTERA(OpArbitraryFloatEQALTERA),
    ArbitraryFloatRecipALTERA(OpArbitraryFloatRecipALTERA),
    ArbitraryFloatRSqrtALTERA(OpArbitraryFloatRSqrtALTERA),
    ArbitraryFloatCbrtALTERA(OpArbitraryFloatCbrtALTERA),
    ArbitraryFloatHypotALTERA(OpArbitraryFloatHypotALTERA),
    ArbitraryFloatSqrtALTERA(OpArbitraryFloatSqrtALTERA),
    ArbitraryFloatLogINTEL(OpArbitraryFloatLogINTEL),
    ArbitraryFloatLog2INTEL(OpArbitraryFloatLog2INTEL),
    ArbitraryFloatLog10INTEL(OpArbitraryFloatLog10INTEL),
    ArbitraryFloatLog1pINTEL(OpArbitraryFloatLog1pINTEL),
    ArbitraryFloatExpINTEL(OpArbitraryFloatExpINTEL),
    ArbitraryFloatExp2INTEL(OpArbitraryFloatExp2INTEL),
    ArbitraryFloatExp10INTEL(OpArbitraryFloatExp10INTEL),
    ArbitraryFloatExpm1INTEL(OpArbitraryFloatExpm1INTEL),
    ArbitraryFloatSinINTEL(OpArbitraryFloatSinINTEL),
    ArbitraryFloatCosINTEL(OpArbitraryFloatCosINTEL),
    ArbitraryFloatSinCosINTEL(OpArbitraryFloatSinCosINTEL),
    ArbitraryFloatSinPiINTEL(OpArbitraryFloatSinPiINTEL),
    ArbitraryFloatCosPiINTEL(OpArbitraryFloatCosPiINTEL),
    ArbitraryFloatASinINTEL(OpArbitraryFloatASinINTEL),
    ArbitraryFloatASinPiINTEL(OpArbitraryFloatASinPiINTEL),
    ArbitraryFloatACosINTEL(OpArbitraryFloatACosINTEL),
    ArbitraryFloatACosPiINTEL(OpArbitraryFloatACosPiINTEL),
    ArbitraryFloatATanINTEL(OpArbitraryFloatATanINTEL),
    ArbitraryFloatATanPiINTEL(OpArbitraryFloatATanPiINTEL),
    ArbitraryFloatATan2INTEL(OpArbitraryFloatATan2INTEL),
    ArbitraryFloatPowINTEL(OpArbitraryFloatPowINTEL),
    ArbitraryFloatPowRINTEL(OpArbitraryFloatPowRINTEL),
    ArbitraryFloatPowNINTEL(OpArbitraryFloatPowNINTEL),
    LoopControlINTEL(OpLoopControlINTEL),
    AliasDomainDeclINTEL(OpAliasDomainDeclINTEL),
    AliasScopeDeclINTEL(OpAliasScopeDeclINTEL),
    AliasScopeListDeclINTEL(OpAliasScopeListDeclINTEL),
    FixedSqrtALTERA(OpFixedSqrtALTERA),
    FixedRecipALTERA(OpFixedRecipALTERA),
    FixedRsqrtALTERA(OpFixedRsqrtALTERA),
    FixedSinALTERA(OpFixedSinALTERA),
    FixedCosALTERA(OpFixedCosALTERA),
    FixedSinCosALTERA(OpFixedSinCosALTERA),
    FixedSinPiALTERA(OpFixedSinPiALTERA),
    FixedCosPiALTERA(OpFixedCosPiALTERA),
    FixedSinCosPiALTERA(OpFixedSinCosPiALTERA),
    FixedLogALTERA(OpFixedLogALTERA),
    FixedExpALTERA(OpFixedExpALTERA),
    PtrCastToCrossWorkgroupALTERA(OpPtrCastToCrossWorkgroupALTERA),
    CrossWorkgroupCastToPtrALTERA(OpCrossWorkgroupCastToPtrALTERA),
    ReadPipeBlockingALTERA(OpReadPipeBlockingALTERA),
    WritePipeBlockingALTERA(OpWritePipeBlockingALTERA),
    FPGARegALTERA(OpFPGARegALTERA),
    RayQueryGetRayTMinKHR(OpRayQueryGetRayTMinKHR),
    RayQueryGetRayFlagsKHR(OpRayQueryGetRayFlagsKHR),
    RayQueryGetIntersectionTKHR(OpRayQueryGetIntersectionTKHR),
    RayQueryGetIntersectionInstanceCustomIndexKHR(OpRayQueryGetIntersectionInstanceCustomIndexKHR),
    RayQueryGetIntersectionInstanceIdKHR(OpRayQueryGetIntersectionInstanceIdKHR),
    RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(
        OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR,
    ),
    RayQueryGetIntersectionGeometryIndexKHR(OpRayQueryGetIntersectionGeometryIndexKHR),
    RayQueryGetIntersectionPrimitiveIndexKHR(OpRayQueryGetIntersectionPrimitiveIndexKHR),
    RayQueryGetIntersectionBarycentricsKHR(OpRayQueryGetIntersectionBarycentricsKHR),
    RayQueryGetIntersectionFrontFaceKHR(OpRayQueryGetIntersectionFrontFaceKHR),
    RayQueryGetIntersectionCandidateAABBOpaqueKHR(OpRayQueryGetIntersectionCandidateAABBOpaqueKHR),
    RayQueryGetIntersectionObjectRayDirectionKHR(OpRayQueryGetIntersectionObjectRayDirectionKHR),
    RayQueryGetIntersectionObjectRayOriginKHR(OpRayQueryGetIntersectionObjectRayOriginKHR),
    RayQueryGetWorldRayDirectionKHR(OpRayQueryGetWorldRayDirectionKHR),
    RayQueryGetWorldRayOriginKHR(OpRayQueryGetWorldRayOriginKHR),
    RayQueryGetIntersectionObjectToWorldKHR(OpRayQueryGetIntersectionObjectToWorldKHR),
    RayQueryGetIntersectionWorldToObjectKHR(OpRayQueryGetIntersectionWorldToObjectKHR),
    AtomicFAddEXT(OpAtomicFAddEXT),
    TypeBufferSurfaceINTEL(OpTypeBufferSurfaceINTEL),
    TypeStructContinuedINTEL(OpTypeStructContinuedINTEL),
    ConstantCompositeContinuedINTEL(OpConstantCompositeContinuedINTEL),
    SpecConstantCompositeContinuedINTEL(OpSpecConstantCompositeContinuedINTEL),
    CompositeConstructContinuedINTEL(OpCompositeConstructContinuedINTEL),
    ConvertFToBF16INTEL(OpConvertFToBF16INTEL),
    ConvertBF16ToFINTEL(OpConvertBF16ToFINTEL),
    ControlBarrierArriveINTEL(OpControlBarrierArriveINTEL),
    ControlBarrierWaitINTEL(OpControlBarrierWaitINTEL),
    ArithmeticFenceEXT(OpArithmeticFenceEXT),
    TaskSequenceCreateALTERA(OpTaskSequenceCreateALTERA),
    TaskSequenceAsyncALTERA(OpTaskSequenceAsyncALTERA),
    TaskSequenceGetALTERA(OpTaskSequenceGetALTERA),
    TaskSequenceReleaseALTERA(OpTaskSequenceReleaseALTERA),
    TypeTaskSequenceALTERA(OpTypeTaskSequenceALTERA),
    SubgroupBlockPrefetchINTEL(OpSubgroupBlockPrefetchINTEL),
    Subgroup2DBlockLoadINTEL(OpSubgroup2DBlockLoadINTEL),
    Subgroup2DBlockLoadTransformINTEL(OpSubgroup2DBlockLoadTransformINTEL),
    Subgroup2DBlockLoadTransposeINTEL(OpSubgroup2DBlockLoadTransposeINTEL),
    Subgroup2DBlockPrefetchINTEL(OpSubgroup2DBlockPrefetchINTEL),
    Subgroup2DBlockStoreINTEL(OpSubgroup2DBlockStoreINTEL),
    SubgroupMatrixMultiplyAccumulateINTEL(OpSubgroupMatrixMultiplyAccumulateINTEL),
    BitwiseFunctionINTEL(OpBitwiseFunctionINTEL),
    UntypedVariableLengthArrayINTEL(OpUntypedVariableLengthArrayINTEL),
    ConditionalExtensionINTEL(OpConditionalExtensionINTEL),
    ConditionalEntryPointINTEL(OpConditionalEntryPointINTEL),
    ConditionalCapabilityINTEL(OpConditionalCapabilityINTEL),
    SpecConstantTargetINTEL(OpSpecConstantTargetINTEL),
    SpecConstantArchitectureINTEL(OpSpecConstantArchitectureINTEL),
    SpecConstantCapabilitiesINTEL(OpSpecConstantCapabilitiesINTEL),
    ConditionalCopyObjectINTEL(OpConditionalCopyObjectINTEL),
    GroupIMulKHR(OpGroupIMulKHR),
    GroupFMulKHR(OpGroupFMulKHR),
    GroupBitwiseAndKHR(OpGroupBitwiseAndKHR),
    GroupBitwiseOrKHR(OpGroupBitwiseOrKHR),
    GroupBitwiseXorKHR(OpGroupBitwiseXorKHR),
    GroupLogicalAndKHR(OpGroupLogicalAndKHR),
    GroupLogicalOrKHR(OpGroupLogicalOrKHR),
    GroupLogicalXorKHR(OpGroupLogicalXorKHR),
    RoundFToTF32INTEL(OpRoundFToTF32INTEL),
    MaskedGatherINTEL(OpMaskedGatherINTEL),
    MaskedScatterINTEL(OpMaskedScatterINTEL),
    ConvertHandleToImageINTEL(OpConvertHandleToImageINTEL),
    ConvertHandleToSamplerINTEL(OpConvertHandleToSamplerINTEL),
    ConvertHandleToSampledImageINTEL(OpConvertHandleToSampledImageINTEL),
}
impl InstEncoding for CoreInstSet {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        match self {
            Self::Nop(inst) => InstEncoding::encode(inst, writer),
            Self::Undef(inst) => InstEncoding::encode(inst, writer),
            Self::SourceContinued(inst) => InstEncoding::encode(inst, writer),
            Self::Source(inst) => InstEncoding::encode(inst, writer),
            Self::SourceExtension(inst) => InstEncoding::encode(inst, writer),
            Self::Name(inst) => InstEncoding::encode(inst, writer),
            Self::MemberName(inst) => InstEncoding::encode(inst, writer),
            Self::String(inst) => InstEncoding::encode(inst, writer),
            Self::Line(inst) => InstEncoding::encode(inst, writer),
            Self::Extension(inst) => InstEncoding::encode(inst, writer),
            Self::ExtInstImport(inst) => InstEncoding::encode(inst, writer),
            Self::ExtInst(inst) => InstEncoding::encode(inst, writer),
            Self::MemoryModel(inst) => InstEncoding::encode(inst, writer),
            Self::EntryPoint(inst) => InstEncoding::encode(inst, writer),
            Self::ExecutionMode(inst) => InstEncoding::encode(inst, writer),
            Self::Capability(inst) => InstEncoding::encode(inst, writer),
            Self::TypeVoid(inst) => InstEncoding::encode(inst, writer),
            Self::TypeBool(inst) => InstEncoding::encode(inst, writer),
            Self::TypeInt(inst) => InstEncoding::encode(inst, writer),
            Self::TypeFloat(inst) => InstEncoding::encode(inst, writer),
            Self::TypeVector(inst) => InstEncoding::encode(inst, writer),
            Self::TypeMatrix(inst) => InstEncoding::encode(inst, writer),
            Self::TypeImage(inst) => InstEncoding::encode(inst, writer),
            Self::TypeSampler(inst) => InstEncoding::encode(inst, writer),
            Self::TypeSampledImage(inst) => InstEncoding::encode(inst, writer),
            Self::TypeArray(inst) => InstEncoding::encode(inst, writer),
            Self::TypeRuntimeArray(inst) => InstEncoding::encode(inst, writer),
            Self::TypeStruct(inst) => InstEncoding::encode(inst, writer),
            Self::TypeOpaque(inst) => InstEncoding::encode(inst, writer),
            Self::TypePointer(inst) => InstEncoding::encode(inst, writer),
            Self::TypeFunction(inst) => InstEncoding::encode(inst, writer),
            Self::TypeEvent(inst) => InstEncoding::encode(inst, writer),
            Self::TypeDeviceEvent(inst) => InstEncoding::encode(inst, writer),
            Self::TypeReserveId(inst) => InstEncoding::encode(inst, writer),
            Self::TypeQueue(inst) => InstEncoding::encode(inst, writer),
            Self::TypePipe(inst) => InstEncoding::encode(inst, writer),
            Self::TypeForwardPointer(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantTrue(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantFalse(inst) => InstEncoding::encode(inst, writer),
            Self::Constant(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantComposite(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantSampler(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantNull(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstantTrue(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstantFalse(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstant(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstantComposite(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstantOp(inst) => InstEncoding::encode(inst, writer),
            Self::Function(inst) => InstEncoding::encode(inst, writer),
            Self::FunctionParameter(inst) => InstEncoding::encode(inst, writer),
            Self::FunctionEnd(inst) => InstEncoding::encode(inst, writer),
            Self::FunctionCall(inst) => InstEncoding::encode(inst, writer),
            Self::Variable(inst) => InstEncoding::encode(inst, writer),
            Self::ImageTexelPointer(inst) => InstEncoding::encode(inst, writer),
            Self::Load(inst) => InstEncoding::encode(inst, writer),
            Self::Store(inst) => InstEncoding::encode(inst, writer),
            Self::CopyMemory(inst) => InstEncoding::encode(inst, writer),
            Self::CopyMemorySized(inst) => InstEncoding::encode(inst, writer),
            Self::AccessChain(inst) => InstEncoding::encode(inst, writer),
            Self::InBoundsAccessChain(inst) => InstEncoding::encode(inst, writer),
            Self::PtrAccessChain(inst) => InstEncoding::encode(inst, writer),
            Self::ArrayLength(inst) => InstEncoding::encode(inst, writer),
            Self::GenericPtrMemSemantics(inst) => InstEncoding::encode(inst, writer),
            Self::InBoundsPtrAccessChain(inst) => InstEncoding::encode(inst, writer),
            Self::Decorate(inst) => InstEncoding::encode(inst, writer),
            Self::MemberDecorate(inst) => InstEncoding::encode(inst, writer),
            Self::DecorationGroup(inst) => InstEncoding::encode(inst, writer),
            Self::GroupDecorate(inst) => InstEncoding::encode(inst, writer),
            Self::GroupMemberDecorate(inst) => InstEncoding::encode(inst, writer),
            Self::VectorExtractDynamic(inst) => InstEncoding::encode(inst, writer),
            Self::VectorInsertDynamic(inst) => InstEncoding::encode(inst, writer),
            Self::VectorShuffle(inst) => InstEncoding::encode(inst, writer),
            Self::CompositeConstruct(inst) => InstEncoding::encode(inst, writer),
            Self::CompositeExtract(inst) => InstEncoding::encode(inst, writer),
            Self::CompositeInsert(inst) => InstEncoding::encode(inst, writer),
            Self::CopyObject(inst) => InstEncoding::encode(inst, writer),
            Self::Transpose(inst) => InstEncoding::encode(inst, writer),
            Self::SampledImage(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSampleImplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSampleExplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSampleDrefImplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSampleDrefExplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSampleProjImplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSampleProjExplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSampleProjDrefImplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSampleProjDrefExplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageFetch(inst) => InstEncoding::encode(inst, writer),
            Self::ImageGather(inst) => InstEncoding::encode(inst, writer),
            Self::ImageDrefGather(inst) => InstEncoding::encode(inst, writer),
            Self::ImageRead(inst) => InstEncoding::encode(inst, writer),
            Self::ImageWrite(inst) => InstEncoding::encode(inst, writer),
            Self::Image(inst) => InstEncoding::encode(inst, writer),
            Self::ImageQueryFormat(inst) => InstEncoding::encode(inst, writer),
            Self::ImageQueryOrder(inst) => InstEncoding::encode(inst, writer),
            Self::ImageQuerySizeLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageQuerySize(inst) => InstEncoding::encode(inst, writer),
            Self::ImageQueryLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageQueryLevels(inst) => InstEncoding::encode(inst, writer),
            Self::ImageQuerySamples(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertFToU(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertFToS(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertSToF(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertUToF(inst) => InstEncoding::encode(inst, writer),
            Self::UConvert(inst) => InstEncoding::encode(inst, writer),
            Self::SConvert(inst) => InstEncoding::encode(inst, writer),
            Self::FConvert(inst) => InstEncoding::encode(inst, writer),
            Self::QuantizeToF16(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertPtrToU(inst) => InstEncoding::encode(inst, writer),
            Self::SatConvertSToU(inst) => InstEncoding::encode(inst, writer),
            Self::SatConvertUToS(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertUToPtr(inst) => InstEncoding::encode(inst, writer),
            Self::PtrCastToGeneric(inst) => InstEncoding::encode(inst, writer),
            Self::GenericCastToPtr(inst) => InstEncoding::encode(inst, writer),
            Self::GenericCastToPtrExplicit(inst) => InstEncoding::encode(inst, writer),
            Self::Bitcast(inst) => InstEncoding::encode(inst, writer),
            Self::SNegate(inst) => InstEncoding::encode(inst, writer),
            Self::FNegate(inst) => InstEncoding::encode(inst, writer),
            Self::IAdd(inst) => InstEncoding::encode(inst, writer),
            Self::FAdd(inst) => InstEncoding::encode(inst, writer),
            Self::ISub(inst) => InstEncoding::encode(inst, writer),
            Self::FSub(inst) => InstEncoding::encode(inst, writer),
            Self::IMul(inst) => InstEncoding::encode(inst, writer),
            Self::FMul(inst) => InstEncoding::encode(inst, writer),
            Self::UDiv(inst) => InstEncoding::encode(inst, writer),
            Self::SDiv(inst) => InstEncoding::encode(inst, writer),
            Self::FDiv(inst) => InstEncoding::encode(inst, writer),
            Self::UMod(inst) => InstEncoding::encode(inst, writer),
            Self::SRem(inst) => InstEncoding::encode(inst, writer),
            Self::SMod(inst) => InstEncoding::encode(inst, writer),
            Self::FRem(inst) => InstEncoding::encode(inst, writer),
            Self::FMod(inst) => InstEncoding::encode(inst, writer),
            Self::VectorTimesScalar(inst) => InstEncoding::encode(inst, writer),
            Self::MatrixTimesScalar(inst) => InstEncoding::encode(inst, writer),
            Self::VectorTimesMatrix(inst) => InstEncoding::encode(inst, writer),
            Self::MatrixTimesVector(inst) => InstEncoding::encode(inst, writer),
            Self::MatrixTimesMatrix(inst) => InstEncoding::encode(inst, writer),
            Self::OuterProduct(inst) => InstEncoding::encode(inst, writer),
            Self::Dot(inst) => InstEncoding::encode(inst, writer),
            Self::IAddCarry(inst) => InstEncoding::encode(inst, writer),
            Self::ISubBorrow(inst) => InstEncoding::encode(inst, writer),
            Self::UMulExtended(inst) => InstEncoding::encode(inst, writer),
            Self::SMulExtended(inst) => InstEncoding::encode(inst, writer),
            Self::Any(inst) => InstEncoding::encode(inst, writer),
            Self::All(inst) => InstEncoding::encode(inst, writer),
            Self::IsNan(inst) => InstEncoding::encode(inst, writer),
            Self::IsInf(inst) => InstEncoding::encode(inst, writer),
            Self::IsFinite(inst) => InstEncoding::encode(inst, writer),
            Self::IsNormal(inst) => InstEncoding::encode(inst, writer),
            Self::SignBitSet(inst) => InstEncoding::encode(inst, writer),
            Self::LessOrGreater(inst) => InstEncoding::encode(inst, writer),
            Self::Ordered(inst) => InstEncoding::encode(inst, writer),
            Self::Unordered(inst) => InstEncoding::encode(inst, writer),
            Self::LogicalEqual(inst) => InstEncoding::encode(inst, writer),
            Self::LogicalNotEqual(inst) => InstEncoding::encode(inst, writer),
            Self::LogicalOr(inst) => InstEncoding::encode(inst, writer),
            Self::LogicalAnd(inst) => InstEncoding::encode(inst, writer),
            Self::LogicalNot(inst) => InstEncoding::encode(inst, writer),
            Self::Select(inst) => InstEncoding::encode(inst, writer),
            Self::IEqual(inst) => InstEncoding::encode(inst, writer),
            Self::INotEqual(inst) => InstEncoding::encode(inst, writer),
            Self::UGreaterThan(inst) => InstEncoding::encode(inst, writer),
            Self::SGreaterThan(inst) => InstEncoding::encode(inst, writer),
            Self::UGreaterThanEqual(inst) => InstEncoding::encode(inst, writer),
            Self::SGreaterThanEqual(inst) => InstEncoding::encode(inst, writer),
            Self::ULessThan(inst) => InstEncoding::encode(inst, writer),
            Self::SLessThan(inst) => InstEncoding::encode(inst, writer),
            Self::ULessThanEqual(inst) => InstEncoding::encode(inst, writer),
            Self::SLessThanEqual(inst) => InstEncoding::encode(inst, writer),
            Self::FOrdEqual(inst) => InstEncoding::encode(inst, writer),
            Self::FUnordEqual(inst) => InstEncoding::encode(inst, writer),
            Self::FOrdNotEqual(inst) => InstEncoding::encode(inst, writer),
            Self::FUnordNotEqual(inst) => InstEncoding::encode(inst, writer),
            Self::FOrdLessThan(inst) => InstEncoding::encode(inst, writer),
            Self::FUnordLessThan(inst) => InstEncoding::encode(inst, writer),
            Self::FOrdGreaterThan(inst) => InstEncoding::encode(inst, writer),
            Self::FUnordGreaterThan(inst) => InstEncoding::encode(inst, writer),
            Self::FOrdLessThanEqual(inst) => InstEncoding::encode(inst, writer),
            Self::FUnordLessThanEqual(inst) => InstEncoding::encode(inst, writer),
            Self::FOrdGreaterThanEqual(inst) => InstEncoding::encode(inst, writer),
            Self::FUnordGreaterThanEqual(inst) => InstEncoding::encode(inst, writer),
            Self::ShiftRightLogical(inst) => InstEncoding::encode(inst, writer),
            Self::ShiftRightArithmetic(inst) => InstEncoding::encode(inst, writer),
            Self::ShiftLeftLogical(inst) => InstEncoding::encode(inst, writer),
            Self::BitwiseOr(inst) => InstEncoding::encode(inst, writer),
            Self::BitwiseXor(inst) => InstEncoding::encode(inst, writer),
            Self::BitwiseAnd(inst) => InstEncoding::encode(inst, writer),
            Self::Not(inst) => InstEncoding::encode(inst, writer),
            Self::BitFieldInsert(inst) => InstEncoding::encode(inst, writer),
            Self::BitFieldSExtract(inst) => InstEncoding::encode(inst, writer),
            Self::BitFieldUExtract(inst) => InstEncoding::encode(inst, writer),
            Self::BitReverse(inst) => InstEncoding::encode(inst, writer),
            Self::BitCount(inst) => InstEncoding::encode(inst, writer),
            Self::DPdx(inst) => InstEncoding::encode(inst, writer),
            Self::DPdy(inst) => InstEncoding::encode(inst, writer),
            Self::Fwidth(inst) => InstEncoding::encode(inst, writer),
            Self::DPdxFine(inst) => InstEncoding::encode(inst, writer),
            Self::DPdyFine(inst) => InstEncoding::encode(inst, writer),
            Self::FwidthFine(inst) => InstEncoding::encode(inst, writer),
            Self::DPdxCoarse(inst) => InstEncoding::encode(inst, writer),
            Self::DPdyCoarse(inst) => InstEncoding::encode(inst, writer),
            Self::FwidthCoarse(inst) => InstEncoding::encode(inst, writer),
            Self::EmitVertex(inst) => InstEncoding::encode(inst, writer),
            Self::EndPrimitive(inst) => InstEncoding::encode(inst, writer),
            Self::EmitStreamVertex(inst) => InstEncoding::encode(inst, writer),
            Self::EndStreamPrimitive(inst) => InstEncoding::encode(inst, writer),
            Self::ControlBarrier(inst) => InstEncoding::encode(inst, writer),
            Self::MemoryBarrier(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicLoad(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicStore(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicExchange(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicCompareExchange(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicCompareExchangeWeak(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicIIncrement(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicIDecrement(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicIAdd(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicISub(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicSMin(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicUMin(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicSMax(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicUMax(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicAnd(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicOr(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicXor(inst) => InstEncoding::encode(inst, writer),
            Self::Phi(inst) => InstEncoding::encode(inst, writer),
            Self::LoopMerge(inst) => InstEncoding::encode(inst, writer),
            Self::SelectionMerge(inst) => InstEncoding::encode(inst, writer),
            Self::Label(inst) => InstEncoding::encode(inst, writer),
            Self::Branch(inst) => InstEncoding::encode(inst, writer),
            Self::BranchConditional(inst) => InstEncoding::encode(inst, writer),
            Self::Switch(inst) => InstEncoding::encode(inst, writer),
            Self::Kill(inst) => InstEncoding::encode(inst, writer),
            Self::Return(inst) => InstEncoding::encode(inst, writer),
            Self::ReturnValue(inst) => InstEncoding::encode(inst, writer),
            Self::Unreachable(inst) => InstEncoding::encode(inst, writer),
            Self::LifetimeStart(inst) => InstEncoding::encode(inst, writer),
            Self::LifetimeStop(inst) => InstEncoding::encode(inst, writer),
            Self::GroupAsyncCopy(inst) => InstEncoding::encode(inst, writer),
            Self::GroupWaitEvents(inst) => InstEncoding::encode(inst, writer),
            Self::GroupAll(inst) => InstEncoding::encode(inst, writer),
            Self::GroupAny(inst) => InstEncoding::encode(inst, writer),
            Self::GroupBroadcast(inst) => InstEncoding::encode(inst, writer),
            Self::GroupIAdd(inst) => InstEncoding::encode(inst, writer),
            Self::GroupFAdd(inst) => InstEncoding::encode(inst, writer),
            Self::GroupFMin(inst) => InstEncoding::encode(inst, writer),
            Self::GroupUMin(inst) => InstEncoding::encode(inst, writer),
            Self::GroupSMin(inst) => InstEncoding::encode(inst, writer),
            Self::GroupFMax(inst) => InstEncoding::encode(inst, writer),
            Self::GroupUMax(inst) => InstEncoding::encode(inst, writer),
            Self::GroupSMax(inst) => InstEncoding::encode(inst, writer),
            Self::ReadPipe(inst) => InstEncoding::encode(inst, writer),
            Self::WritePipe(inst) => InstEncoding::encode(inst, writer),
            Self::ReservedReadPipe(inst) => InstEncoding::encode(inst, writer),
            Self::ReservedWritePipe(inst) => InstEncoding::encode(inst, writer),
            Self::ReserveReadPipePackets(inst) => InstEncoding::encode(inst, writer),
            Self::ReserveWritePipePackets(inst) => InstEncoding::encode(inst, writer),
            Self::CommitReadPipe(inst) => InstEncoding::encode(inst, writer),
            Self::CommitWritePipe(inst) => InstEncoding::encode(inst, writer),
            Self::IsValidReserveId(inst) => InstEncoding::encode(inst, writer),
            Self::GetNumPipePackets(inst) => InstEncoding::encode(inst, writer),
            Self::GetMaxPipePackets(inst) => InstEncoding::encode(inst, writer),
            Self::GroupReserveReadPipePackets(inst) => InstEncoding::encode(inst, writer),
            Self::GroupReserveWritePipePackets(inst) => InstEncoding::encode(inst, writer),
            Self::GroupCommitReadPipe(inst) => InstEncoding::encode(inst, writer),
            Self::GroupCommitWritePipe(inst) => InstEncoding::encode(inst, writer),
            Self::EnqueueMarker(inst) => InstEncoding::encode(inst, writer),
            Self::EnqueueKernel(inst) => InstEncoding::encode(inst, writer),
            Self::GetKernelNDrangeSubGroupCount(inst) => InstEncoding::encode(inst, writer),
            Self::GetKernelNDrangeMaxSubGroupSize(inst) => InstEncoding::encode(inst, writer),
            Self::GetKernelWorkGroupSize(inst) => InstEncoding::encode(inst, writer),
            Self::GetKernelPreferredWorkGroupSizeMultiple(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RetainEvent(inst) => InstEncoding::encode(inst, writer),
            Self::ReleaseEvent(inst) => InstEncoding::encode(inst, writer),
            Self::CreateUserEvent(inst) => InstEncoding::encode(inst, writer),
            Self::IsValidEvent(inst) => InstEncoding::encode(inst, writer),
            Self::SetUserEventStatus(inst) => InstEncoding::encode(inst, writer),
            Self::CaptureEventProfilingInfo(inst) => InstEncoding::encode(inst, writer),
            Self::GetDefaultQueue(inst) => InstEncoding::encode(inst, writer),
            Self::BuildNDRange(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseSampleImplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseSampleExplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseSampleDrefImplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseSampleDrefExplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseSampleProjImplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseSampleProjExplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseSampleProjDrefImplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseSampleProjDrefExplicitLod(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseFetch(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseGather(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseDrefGather(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseTexelsResident(inst) => InstEncoding::encode(inst, writer),
            Self::NoLine(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicFlagTestAndSet(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicFlagClear(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSparseRead(inst) => InstEncoding::encode(inst, writer),
            Self::SizeOf(inst) => InstEncoding::encode(inst, writer),
            Self::TypePipeStorage(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantPipeStorage(inst) => InstEncoding::encode(inst, writer),
            Self::CreatePipeFromPipeStorage(inst) => InstEncoding::encode(inst, writer),
            Self::GetKernelLocalSizeForSubgroupCount(inst) => InstEncoding::encode(inst, writer),
            Self::GetKernelMaxNumSubgroups(inst) => InstEncoding::encode(inst, writer),
            Self::TypeNamedBarrier(inst) => InstEncoding::encode(inst, writer),
            Self::NamedBarrierInitialize(inst) => InstEncoding::encode(inst, writer),
            Self::MemoryNamedBarrier(inst) => InstEncoding::encode(inst, writer),
            Self::ModuleProcessed(inst) => InstEncoding::encode(inst, writer),
            Self::ExecutionModeId(inst) => InstEncoding::encode(inst, writer),
            Self::DecorateId(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformElect(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformAll(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformAny(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformAllEqual(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformBroadcast(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformBroadcastFirst(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformBallot(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformInverseBallot(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformBallotBitExtract(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformBallotBitCount(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformBallotFindLSB(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformBallotFindMSB(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformShuffle(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformShuffleXor(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformShuffleUp(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformShuffleDown(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformIAdd(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformFAdd(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformIMul(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformFMul(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformSMin(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformUMin(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformFMin(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformSMax(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformUMax(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformFMax(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformBitwiseAnd(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformBitwiseOr(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformBitwiseXor(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformLogicalAnd(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformLogicalOr(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformLogicalXor(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformQuadBroadcast(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformQuadSwap(inst) => InstEncoding::encode(inst, writer),
            Self::CopyLogical(inst) => InstEncoding::encode(inst, writer),
            Self::PtrEqual(inst) => InstEncoding::encode(inst, writer),
            Self::PtrNotEqual(inst) => InstEncoding::encode(inst, writer),
            Self::PtrDiff(inst) => InstEncoding::encode(inst, writer),
            Self::ColorAttachmentReadEXT(inst) => InstEncoding::encode(inst, writer),
            Self::DepthAttachmentReadEXT(inst) => InstEncoding::encode(inst, writer),
            Self::StencilAttachmentReadEXT(inst) => InstEncoding::encode(inst, writer),
            Self::TypeTensorARM(inst) => InstEncoding::encode(inst, writer),
            Self::TensorReadARM(inst) => InstEncoding::encode(inst, writer),
            Self::TensorWriteARM(inst) => InstEncoding::encode(inst, writer),
            Self::TensorQuerySizeARM(inst) => InstEncoding::encode(inst, writer),
            Self::GraphConstantARM(inst) => InstEncoding::encode(inst, writer),
            Self::GraphEntryPointARM(inst) => InstEncoding::encode(inst, writer),
            Self::GraphARM(inst) => InstEncoding::encode(inst, writer),
            Self::GraphInputARM(inst) => InstEncoding::encode(inst, writer),
            Self::GraphSetOutputARM(inst) => InstEncoding::encode(inst, writer),
            Self::GraphEndARM(inst) => InstEncoding::encode(inst, writer),
            Self::TypeGraphARM(inst) => InstEncoding::encode(inst, writer),
            Self::TerminateInvocation(inst) => InstEncoding::encode(inst, writer),
            Self::TypeUntypedPointerKHR(inst) => InstEncoding::encode(inst, writer),
            Self::UntypedVariableKHR(inst) => InstEncoding::encode(inst, writer),
            Self::UntypedAccessChainKHR(inst) => InstEncoding::encode(inst, writer),
            Self::UntypedInBoundsAccessChainKHR(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupBallotKHR(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupFirstInvocationKHR(inst) => InstEncoding::encode(inst, writer),
            Self::UntypedPtrAccessChainKHR(inst) => InstEncoding::encode(inst, writer),
            Self::UntypedInBoundsPtrAccessChainKHR(inst) => InstEncoding::encode(inst, writer),
            Self::UntypedArrayLengthKHR(inst) => InstEncoding::encode(inst, writer),
            Self::UntypedPrefetchKHR(inst) => InstEncoding::encode(inst, writer),
            Self::FmaKHR(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAllKHR(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAnyKHR(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAllEqualKHR(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformRotateKHR(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupReadInvocationKHR(inst) => InstEncoding::encode(inst, writer),
            Self::ExtInstWithForwardRefsKHR(inst) => InstEncoding::encode(inst, writer),
            Self::UntypedGroupAsyncCopyKHR(inst) => InstEncoding::encode(inst, writer),
            Self::TraceRayKHR(inst) => InstEncoding::encode(inst, writer),
            Self::ExecuteCallableKHR(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertUToAccelerationStructureKHR(inst) => InstEncoding::encode(inst, writer),
            Self::IgnoreIntersectionKHR(inst) => InstEncoding::encode(inst, writer),
            Self::TerminateRayKHR(inst) => InstEncoding::encode(inst, writer),
            Self::SDot(inst) => InstEncoding::encode(inst, writer),
            Self::UDot(inst) => InstEncoding::encode(inst, writer),
            Self::SUDot(inst) => InstEncoding::encode(inst, writer),
            Self::SDotAccSat(inst) => InstEncoding::encode(inst, writer),
            Self::UDotAccSat(inst) => InstEncoding::encode(inst, writer),
            Self::SUDotAccSat(inst) => InstEncoding::encode(inst, writer),
            Self::TypeCooperativeMatrixKHR(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixLoadKHR(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixStoreKHR(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixMulAddKHR(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixLengthKHR(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantCompositeReplicateEXT(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstantCompositeReplicateEXT(inst) => InstEncoding::encode(inst, writer),
            Self::CompositeConstructReplicateEXT(inst) => InstEncoding::encode(inst, writer),
            Self::TypeRayQueryKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryInitializeKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryTerminateKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGenerateIntersectionKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryConfirmIntersectionKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryProceedKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionTypeKHR(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSampleWeightedQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::ImageBoxFilterQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::ImageBlockMatchSSDQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::ImageBlockMatchSADQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::BitCastArrayQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::ImageBlockMatchWindowSSDQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::ImageBlockMatchWindowSADQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::ImageBlockMatchGatherSSDQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::ImageBlockMatchGatherSADQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::CompositeConstructCoopMatQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::CompositeExtractCoopMatQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::ExtractSubArrayQCOM(inst) => InstEncoding::encode(inst, writer),
            Self::GroupIAddNonUniformAMD(inst) => InstEncoding::encode(inst, writer),
            Self::GroupFAddNonUniformAMD(inst) => InstEncoding::encode(inst, writer),
            Self::GroupFMinNonUniformAMD(inst) => InstEncoding::encode(inst, writer),
            Self::GroupUMinNonUniformAMD(inst) => InstEncoding::encode(inst, writer),
            Self::GroupSMinNonUniformAMD(inst) => InstEncoding::encode(inst, writer),
            Self::GroupFMaxNonUniformAMD(inst) => InstEncoding::encode(inst, writer),
            Self::GroupUMaxNonUniformAMD(inst) => InstEncoding::encode(inst, writer),
            Self::GroupSMaxNonUniformAMD(inst) => InstEncoding::encode(inst, writer),
            Self::FragmentMaskFetchAMD(inst) => InstEncoding::encode(inst, writer),
            Self::FragmentFetchAMD(inst) => InstEncoding::encode(inst, writer),
            Self::ReadClockKHR(inst) => InstEncoding::encode(inst, writer),
            Self::AllocateNodePayloadsAMDX(inst) => InstEncoding::encode(inst, writer),
            Self::EnqueueNodePayloadsAMDX(inst) => InstEncoding::encode(inst, writer),
            Self::TypeNodePayloadArrayAMDX(inst) => InstEncoding::encode(inst, writer),
            Self::FinishWritingNodePayloadAMDX(inst) => InstEncoding::encode(inst, writer),
            Self::NodePayloadArrayLengthAMDX(inst) => InstEncoding::encode(inst, writer),
            Self::IsNodePayloadValidAMDX(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantStringAMDX(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstantStringAMDX(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformQuadAllKHR(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformQuadAnyKHR(inst) => InstEncoding::encode(inst, writer),
            Self::TypeBufferEXT(inst) => InstEncoding::encode(inst, writer),
            Self::BufferPointerEXT(inst) => InstEncoding::encode(inst, writer),
            Self::UntypedImageTexelPointerEXT(inst) => InstEncoding::encode(inst, writer),
            Self::MemberDecorateIdEXT(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantSizeOfEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectRecordHitMotionNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectRecordHitWithIndexMotionNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectRecordMissMotionNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetWorldToObjectNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetObjectToWorldNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetObjectRayDirectionNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetObjectRayOriginNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectTraceRayMotionNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetShaderRecordBufferHandleNV(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::HitObjectGetShaderBindingTableRecordIndexNV(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::HitObjectRecordEmptyNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectTraceRayNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectRecordHitNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectRecordHitWithIndexNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectRecordMissNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectExecuteShaderNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetCurrentTimeNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetAttributesNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetHitKindNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetPrimitiveIndexNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetGeometryIndexNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetInstanceIdNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetInstanceCustomIndexNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetWorldRayDirectionNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetWorldRayOriginNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetRayTMaxNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetRayTMinNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectIsEmptyNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectIsHitNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectIsMissNV(inst) => InstEncoding::encode(inst, writer),
            Self::ReorderThreadWithHitObjectNV(inst) => InstEncoding::encode(inst, writer),
            Self::ReorderThreadWithHintNV(inst) => InstEncoding::encode(inst, writer),
            Self::TypeHitObjectNV(inst) => InstEncoding::encode(inst, writer),
            Self::ImageSampleFootprintNV(inst) => InstEncoding::encode(inst, writer),
            Self::TypeVectorIdEXT(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeVectorMatrixMulNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeVectorOuterProductAccumulateNV(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::CooperativeVectorReduceSumAccumulateNV(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::CooperativeVectorMatrixMulAddNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixConvertNV(inst) => InstEncoding::encode(inst, writer),
            Self::EmitMeshTasksEXT(inst) => InstEncoding::encode(inst, writer),
            Self::SetMeshOutputsEXT(inst) => InstEncoding::encode(inst, writer),
            Self::GroupNonUniformPartitionEXT(inst) => InstEncoding::encode(inst, writer),
            Self::WritePackedPrimitiveIndices4x8NV(inst) => InstEncoding::encode(inst, writer),
            Self::FetchMicroTriangleVertexPositionNV(inst) => InstEncoding::encode(inst, writer),
            Self::FetchMicroTriangleVertexBarycentricNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeVectorLoadNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeVectorStoreNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectRecordFromQueryEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectRecordMissEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectRecordMissMotionEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetIntersectionTriangleVertexPositionsEXT(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::HitObjectGetRayFlagsEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectSetShaderBindingTableRecordIndexEXT(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::HitObjectReorderExecuteShaderEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectTraceReorderExecuteEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectTraceMotionReorderExecuteEXT(inst) => InstEncoding::encode(inst, writer),
            Self::TypeHitObjectEXT(inst) => InstEncoding::encode(inst, writer),
            Self::ReorderThreadWithHintEXT(inst) => InstEncoding::encode(inst, writer),
            Self::ReorderThreadWithHitObjectEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectTraceRayEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectTraceRayMotionEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectRecordEmptyEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectExecuteShaderEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetCurrentTimeEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetAttributesEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetHitKindEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetPrimitiveIndexEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetGeometryIndexEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetInstanceIdEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetInstanceCustomIndexEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetObjectRayOriginEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetObjectRayDirectionEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetWorldRayDirectionEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetWorldRayOriginEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetObjectToWorldEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetWorldToObjectEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetRayTMaxEXT(inst) => InstEncoding::encode(inst, writer),
            Self::ReportIntersectionKHR(inst) => InstEncoding::encode(inst, writer),
            Self::IgnoreIntersectionNV(inst) => InstEncoding::encode(inst, writer),
            Self::TerminateRayNV(inst) => InstEncoding::encode(inst, writer),
            Self::TraceNV(inst) => InstEncoding::encode(inst, writer),
            Self::TraceMotionNV(inst) => InstEncoding::encode(inst, writer),
            Self::TraceRayMotionNV(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionTriangleVertexPositionsKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::TypeAccelerationStructureKHR(inst) => InstEncoding::encode(inst, writer),
            Self::ExecuteCallableNV(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionClusterIdNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetClusterIdNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetRayTMinEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetShaderBindingTableRecordIndexEXT(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::HitObjectGetShaderRecordBufferHandleEXT(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::HitObjectIsEmptyEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectIsHitEXT(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectIsMissEXT(inst) => InstEncoding::encode(inst, writer),
            Self::TypeCooperativeMatrixNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixLoadNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixStoreNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixMulAddNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixLengthNV(inst) => InstEncoding::encode(inst, writer),
            Self::BeginInvocationInterlockEXT(inst) => InstEncoding::encode(inst, writer),
            Self::EndInvocationInterlockEXT(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixReduceNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixLoadTensorNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixStoreTensorNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixPerElementOpNV(inst) => InstEncoding::encode(inst, writer),
            Self::TypeTensorLayoutNV(inst) => InstEncoding::encode(inst, writer),
            Self::TypeTensorViewNV(inst) => InstEncoding::encode(inst, writer),
            Self::CreateTensorLayoutNV(inst) => InstEncoding::encode(inst, writer),
            Self::TensorLayoutSetDimensionNV(inst) => InstEncoding::encode(inst, writer),
            Self::TensorLayoutSetStrideNV(inst) => InstEncoding::encode(inst, writer),
            Self::TensorLayoutSliceNV(inst) => InstEncoding::encode(inst, writer),
            Self::TensorLayoutSetClampValueNV(inst) => InstEncoding::encode(inst, writer),
            Self::CreateTensorViewNV(inst) => InstEncoding::encode(inst, writer),
            Self::TensorViewSetDimensionNV(inst) => InstEncoding::encode(inst, writer),
            Self::TensorViewSetStrideNV(inst) => InstEncoding::encode(inst, writer),
            Self::DemoteToHelperInvocation(inst) => InstEncoding::encode(inst, writer),
            Self::IsHelperInvocationEXT(inst) => InstEncoding::encode(inst, writer),
            Self::TensorViewSetClipNV(inst) => InstEncoding::encode(inst, writer),
            Self::TensorLayoutSetBlockSizeNV(inst) => InstEncoding::encode(inst, writer),
            Self::CooperativeMatrixTransposeNV(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertUToImageNV(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertUToSamplerNV(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertImageToUNV(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertSamplerToUNV(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertUToSampledImageNV(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertSampledImageToUNV(inst) => InstEncoding::encode(inst, writer),
            Self::SamplerImageAddressingModeNV(inst) => InstEncoding::encode(inst, writer),
            Self::RawAccessChainNV(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionSpherePositionNV(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RayQueryGetIntersectionSphereRadiusNV(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionLSSPositionsNV(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionLSSRadiiNV(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionLSSHitValueNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetSpherePositionNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetSphereRadiusNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetLSSPositionsNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectGetLSSRadiiNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectIsSphereHitNV(inst) => InstEncoding::encode(inst, writer),
            Self::HitObjectIsLSSHitNV(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryIsSphereHitNV(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryIsLSSHitNV(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupShuffleINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupShuffleDownINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupShuffleUpINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupShuffleXorINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupBlockReadINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupBlockWriteINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupImageBlockReadINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupImageBlockWriteINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupImageMediaBlockReadINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupImageMediaBlockWriteINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::UCountLeadingZerosINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::UCountTrailingZerosINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::AbsISubINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::AbsUSubINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::IAddSatINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::UAddSatINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::IAverageINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::UAverageINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::IAverageRoundedINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::UAverageRoundedINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ISubSatINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::USubSatINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::IMul32x16INTEL(inst) => InstEncoding::encode(inst, writer),
            Self::UMul32x16INTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantFunctionPointerINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::FunctionPointerCallINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::AsmTargetINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::AsmINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::AsmCallINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicFMinEXT(inst) => InstEncoding::encode(inst, writer),
            Self::AtomicFMaxEXT(inst) => InstEncoding::encode(inst, writer),
            Self::AssumeTrueKHR(inst) => InstEncoding::encode(inst, writer),
            Self::ExpectKHR(inst) => InstEncoding::encode(inst, writer),
            Self::DecorateString(inst) => InstEncoding::encode(inst, writer),
            Self::MemberDecorateString(inst) => InstEncoding::encode(inst, writer),
            Self::VmeImageINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeVmeImageINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeAvcImePayloadINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeAvcRefPayloadINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeAvcSicPayloadINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeAvcMcePayloadINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeAvcMceResultINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeAvcImeResultINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeAvcImeResultSingleReferenceStreamoutINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::TypeAvcImeResultDualReferenceStreamoutINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::TypeAvcImeSingleReferenceStreaminINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::TypeAvcImeDualReferenceStreaminINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeAvcRefResultINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeAvcSicResultINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceSetInterShapePenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceSetInterDirectionPenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceSetMotionVectorCostFunctionINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceSetAcOnlyHaarINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceConvertToImePayloadINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceConvertToImeResultINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcMceConvertToRefPayloadINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceConvertToRefResultINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcMceConvertToSicPayloadINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceConvertToSicResultINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcMceGetMotionVectorsINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcMceGetInterDistortionsINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetBestInterDistortionsINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetInterMajorShapeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcMceGetInterMinorShapeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcMceGetInterDirectionsINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcMceGetInterMotionVectorCountINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetInterReferenceIdsINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeInitializeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcImeSetSingleReferenceINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcImeSetDualReferenceINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcImeRefWindowSizeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcImeAdjustRefOffsetINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcImeConvertToMcePayloadINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeSetMaxMotionVectorCountINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeSetUnidirectionalMixDisableINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeSetWeightedSadINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcImeEvaluateWithSingleReferenceINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeEvaluateWithDualReferenceINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeConvertToMceResultINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcImeGetSingleReferenceStreaminINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetDualReferenceStreaminINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeStripSingleReferenceStreamoutINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeStripDualReferenceStreamoutINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetBorderReachedINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcImeGetTruncatedSearchIndicationINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcFmeInitializeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcBmeInitializeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcRefConvertToMcePayloadINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcRefSetBidirectionalMixDisableINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcRefSetBilinearFilterEnableINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcRefEvaluateWithSingleReferenceINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcRefEvaluateWithDualReferenceINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcRefEvaluateWithMultiReferenceINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcRefConvertToMceResultINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcSicInitializeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcSicConfigureSkcINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcSicConfigureIpeLumaINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcSicConfigureIpeLumaChromaINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicGetMotionVectorMaskINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicConvertToMcePayloadINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicSetBilinearFilterEnableINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicSetSkcForwardTransformEnableINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicEvaluateIpeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcSicEvaluateWithSingleReferenceINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicEvaluateWithDualReferenceINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicEvaluateWithMultiReferenceINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicConvertToMceResultINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcSicGetIpeLumaShapeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcSicGetBestIpeLumaDistortionINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicGetBestIpeChromaDistortionINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicGetPackedIpeLumaModesINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicGetIpeChromaModeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::SubgroupAvcSicGetInterRawSadsINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::VariableLengthArrayINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SaveMemoryINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::RestoreMemoryINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatSinCosPiALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatCastALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatCastFromIntALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatCastToIntALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatAddALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatSubALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatMulALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatDivALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatGTALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatGEALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatLTALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatLEALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatEQALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatRecipALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatRSqrtALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatCbrtALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatHypotALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatSqrtALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatLogINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatLog2INTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatLog10INTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatLog1pINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatExpINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatExp2INTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatExp10INTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatExpm1INTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatSinINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatCosINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatSinCosINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatSinPiINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatCosPiINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatASinINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatASinPiINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatACosINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatACosPiINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatATanINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatATanPiINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatATan2INTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatPowINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatPowRINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArbitraryFloatPowNINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::LoopControlINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::AliasDomainDeclINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::AliasScopeDeclINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::AliasScopeListDeclINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::FixedSqrtALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FixedRecipALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FixedRsqrtALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FixedSinALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FixedCosALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FixedSinCosALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FixedSinPiALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FixedCosPiALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FixedSinCosPiALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FixedLogALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FixedExpALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::PtrCastToCrossWorkgroupALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::CrossWorkgroupCastToPtrALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::ReadPipeBlockingALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::WritePipeBlockingALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::FPGARegALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetRayTMinKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetRayFlagsKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionTKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionInstanceCustomIndexKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RayQueryGetIntersectionInstanceIdKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RayQueryGetIntersectionGeometryIndexKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RayQueryGetIntersectionPrimitiveIndexKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RayQueryGetIntersectionBarycentricsKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RayQueryGetIntersectionFrontFaceKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionCandidateAABBOpaqueKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RayQueryGetIntersectionObjectRayDirectionKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RayQueryGetIntersectionObjectRayOriginKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RayQueryGetWorldRayDirectionKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetWorldRayOriginKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RayQueryGetIntersectionObjectToWorldKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::RayQueryGetIntersectionWorldToObjectKHR(inst) => {
                InstEncoding::encode(inst, writer)
            }
            Self::AtomicFAddEXT(inst) => InstEncoding::encode(inst, writer),
            Self::TypeBufferSurfaceINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::TypeStructContinuedINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConstantCompositeContinuedINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstantCompositeContinuedINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::CompositeConstructContinuedINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertFToBF16INTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertBF16ToFINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ControlBarrierArriveINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ControlBarrierWaitINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ArithmeticFenceEXT(inst) => InstEncoding::encode(inst, writer),
            Self::TaskSequenceCreateALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::TaskSequenceAsyncALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::TaskSequenceGetALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::TaskSequenceReleaseALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::TypeTaskSequenceALTERA(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupBlockPrefetchINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::Subgroup2DBlockLoadINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::Subgroup2DBlockLoadTransformINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::Subgroup2DBlockLoadTransposeINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::Subgroup2DBlockPrefetchINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::Subgroup2DBlockStoreINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SubgroupMatrixMultiplyAccumulateINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::BitwiseFunctionINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::UntypedVariableLengthArrayINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConditionalExtensionINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConditionalEntryPointINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConditionalCapabilityINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstantTargetINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstantArchitectureINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::SpecConstantCapabilitiesINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConditionalCopyObjectINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::GroupIMulKHR(inst) => InstEncoding::encode(inst, writer),
            Self::GroupFMulKHR(inst) => InstEncoding::encode(inst, writer),
            Self::GroupBitwiseAndKHR(inst) => InstEncoding::encode(inst, writer),
            Self::GroupBitwiseOrKHR(inst) => InstEncoding::encode(inst, writer),
            Self::GroupBitwiseXorKHR(inst) => InstEncoding::encode(inst, writer),
            Self::GroupLogicalAndKHR(inst) => InstEncoding::encode(inst, writer),
            Self::GroupLogicalOrKHR(inst) => InstEncoding::encode(inst, writer),
            Self::GroupLogicalXorKHR(inst) => InstEncoding::encode(inst, writer),
            Self::RoundFToTF32INTEL(inst) => InstEncoding::encode(inst, writer),
            Self::MaskedGatherINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::MaskedScatterINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertHandleToImageINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertHandleToSamplerINTEL(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertHandleToSampledImageINTEL(inst) => InstEncoding::encode(inst, writer),
        }
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        let opcode = reader.opcode();
        Ok(
            match opcode {
                0u16 => Self::Nop(<OpNop as InstEncoding>::decode(reader)?),
                1u16 => Self::Undef(<OpUndef as InstEncoding>::decode(reader)?),
                2u16 => {
                    Self::SourceContinued(
                        <OpSourceContinued as InstEncoding>::decode(reader)?,
                    )
                }
                3u16 => Self::Source(<OpSource as InstEncoding>::decode(reader)?),
                4u16 => {
                    Self::SourceExtension(
                        <OpSourceExtension as InstEncoding>::decode(reader)?,
                    )
                }
                5u16 => Self::Name(<OpName as InstEncoding>::decode(reader)?),
                6u16 => Self::MemberName(<OpMemberName as InstEncoding>::decode(reader)?),
                7u16 => Self::String(<OpString as InstEncoding>::decode(reader)?),
                8u16 => Self::Line(<OpLine as InstEncoding>::decode(reader)?),
                10u16 => Self::Extension(<OpExtension as InstEncoding>::decode(reader)?),
                11u16 => {
                    Self::ExtInstImport(
                        <OpExtInstImport as InstEncoding>::decode(reader)?,
                    )
                }
                12u16 => Self::ExtInst(<OpExtInst as InstEncoding>::decode(reader)?),
                14u16 => {
                    Self::MemoryModel(<OpMemoryModel as InstEncoding>::decode(reader)?)
                }
                15u16 => {
                    Self::EntryPoint(<OpEntryPoint as InstEncoding>::decode(reader)?)
                }
                16u16 => {
                    Self::ExecutionMode(
                        <OpExecutionMode as InstEncoding>::decode(reader)?,
                    )
                }
                17u16 => {
                    Self::Capability(<OpCapability as InstEncoding>::decode(reader)?)
                }
                19u16 => Self::TypeVoid(<OpTypeVoid as InstEncoding>::decode(reader)?),
                20u16 => Self::TypeBool(<OpTypeBool as InstEncoding>::decode(reader)?),
                21u16 => Self::TypeInt(<OpTypeInt as InstEncoding>::decode(reader)?),
                22u16 => Self::TypeFloat(<OpTypeFloat as InstEncoding>::decode(reader)?),
                23u16 => {
                    Self::TypeVector(<OpTypeVector as InstEncoding>::decode(reader)?)
                }
                24u16 => {
                    Self::TypeMatrix(<OpTypeMatrix as InstEncoding>::decode(reader)?)
                }
                25u16 => Self::TypeImage(<OpTypeImage as InstEncoding>::decode(reader)?),
                26u16 => {
                    Self::TypeSampler(<OpTypeSampler as InstEncoding>::decode(reader)?)
                }
                27u16 => {
                    Self::TypeSampledImage(
                        <OpTypeSampledImage as InstEncoding>::decode(reader)?,
                    )
                }
                28u16 => Self::TypeArray(<OpTypeArray as InstEncoding>::decode(reader)?),
                29u16 => {
                    Self::TypeRuntimeArray(
                        <OpTypeRuntimeArray as InstEncoding>::decode(reader)?,
                    )
                }
                30u16 => {
                    Self::TypeStruct(<OpTypeStruct as InstEncoding>::decode(reader)?)
                }
                31u16 => {
                    Self::TypeOpaque(<OpTypeOpaque as InstEncoding>::decode(reader)?)
                }
                32u16 => {
                    Self::TypePointer(<OpTypePointer as InstEncoding>::decode(reader)?)
                }
                33u16 => {
                    Self::TypeFunction(<OpTypeFunction as InstEncoding>::decode(reader)?)
                }
                34u16 => Self::TypeEvent(<OpTypeEvent as InstEncoding>::decode(reader)?),
                35u16 => {
                    Self::TypeDeviceEvent(
                        <OpTypeDeviceEvent as InstEncoding>::decode(reader)?,
                    )
                }
                36u16 => {
                    Self::TypeReserveId(
                        <OpTypeReserveId as InstEncoding>::decode(reader)?,
                    )
                }
                37u16 => Self::TypeQueue(<OpTypeQueue as InstEncoding>::decode(reader)?),
                38u16 => Self::TypePipe(<OpTypePipe as InstEncoding>::decode(reader)?),
                39u16 => {
                    Self::TypeForwardPointer(
                        <OpTypeForwardPointer as InstEncoding>::decode(reader)?,
                    )
                }
                41u16 => {
                    Self::ConstantTrue(<OpConstantTrue as InstEncoding>::decode(reader)?)
                }
                42u16 => {
                    Self::ConstantFalse(
                        <OpConstantFalse as InstEncoding>::decode(reader)?,
                    )
                }
                43u16 => Self::Constant(<OpConstant as InstEncoding>::decode(reader)?),
                44u16 => {
                    Self::ConstantComposite(
                        <OpConstantComposite as InstEncoding>::decode(reader)?,
                    )
                }
                45u16 => {
                    Self::ConstantSampler(
                        <OpConstantSampler as InstEncoding>::decode(reader)?,
                    )
                }
                46u16 => {
                    Self::ConstantNull(<OpConstantNull as InstEncoding>::decode(reader)?)
                }
                48u16 => {
                    Self::SpecConstantTrue(
                        <OpSpecConstantTrue as InstEncoding>::decode(reader)?,
                    )
                }
                49u16 => {
                    Self::SpecConstantFalse(
                        <OpSpecConstantFalse as InstEncoding>::decode(reader)?,
                    )
                }
                50u16 => {
                    Self::SpecConstant(<OpSpecConstant as InstEncoding>::decode(reader)?)
                }
                51u16 => {
                    Self::SpecConstantComposite(
                        <OpSpecConstantComposite as InstEncoding>::decode(reader)?,
                    )
                }
                52u16 => {
                    Self::SpecConstantOp(
                        <OpSpecConstantOp as InstEncoding>::decode(reader)?,
                    )
                }
                54u16 => Self::Function(<OpFunction as InstEncoding>::decode(reader)?),
                55u16 => {
                    Self::FunctionParameter(
                        <OpFunctionParameter as InstEncoding>::decode(reader)?,
                    )
                }
                56u16 => {
                    Self::FunctionEnd(<OpFunctionEnd as InstEncoding>::decode(reader)?)
                }
                57u16 => {
                    Self::FunctionCall(<OpFunctionCall as InstEncoding>::decode(reader)?)
                }
                59u16 => Self::Variable(<OpVariable as InstEncoding>::decode(reader)?),
                60u16 => {
                    Self::ImageTexelPointer(
                        <OpImageTexelPointer as InstEncoding>::decode(reader)?,
                    )
                }
                61u16 => Self::Load(<OpLoad as InstEncoding>::decode(reader)?),
                62u16 => Self::Store(<OpStore as InstEncoding>::decode(reader)?),
                63u16 => {
                    Self::CopyMemory(<OpCopyMemory as InstEncoding>::decode(reader)?)
                }
                64u16 => {
                    Self::CopyMemorySized(
                        <OpCopyMemorySized as InstEncoding>::decode(reader)?,
                    )
                }
                65u16 => {
                    Self::AccessChain(<OpAccessChain as InstEncoding>::decode(reader)?)
                }
                66u16 => {
                    Self::InBoundsAccessChain(
                        <OpInBoundsAccessChain as InstEncoding>::decode(reader)?,
                    )
                }
                67u16 => {
                    Self::PtrAccessChain(
                        <OpPtrAccessChain as InstEncoding>::decode(reader)?,
                    )
                }
                68u16 => {
                    Self::ArrayLength(<OpArrayLength as InstEncoding>::decode(reader)?)
                }
                69u16 => {
                    Self::GenericPtrMemSemantics(
                        <OpGenericPtrMemSemantics as InstEncoding>::decode(reader)?,
                    )
                }
                70u16 => {
                    Self::InBoundsPtrAccessChain(
                        <OpInBoundsPtrAccessChain as InstEncoding>::decode(reader)?,
                    )
                }
                71u16 => Self::Decorate(<OpDecorate as InstEncoding>::decode(reader)?),
                72u16 => {
                    Self::MemberDecorate(
                        <OpMemberDecorate as InstEncoding>::decode(reader)?,
                    )
                }
                73u16 => {
                    Self::DecorationGroup(
                        <OpDecorationGroup as InstEncoding>::decode(reader)?,
                    )
                }
                74u16 => {
                    Self::GroupDecorate(
                        <OpGroupDecorate as InstEncoding>::decode(reader)?,
                    )
                }
                75u16 => {
                    Self::GroupMemberDecorate(
                        <OpGroupMemberDecorate as InstEncoding>::decode(reader)?,
                    )
                }
                77u16 => {
                    Self::VectorExtractDynamic(
                        <OpVectorExtractDynamic as InstEncoding>::decode(reader)?,
                    )
                }
                78u16 => {
                    Self::VectorInsertDynamic(
                        <OpVectorInsertDynamic as InstEncoding>::decode(reader)?,
                    )
                }
                79u16 => {
                    Self::VectorShuffle(
                        <OpVectorShuffle as InstEncoding>::decode(reader)?,
                    )
                }
                80u16 => {
                    Self::CompositeConstruct(
                        <OpCompositeConstruct as InstEncoding>::decode(reader)?,
                    )
                }
                81u16 => {
                    Self::CompositeExtract(
                        <OpCompositeExtract as InstEncoding>::decode(reader)?,
                    )
                }
                82u16 => {
                    Self::CompositeInsert(
                        <OpCompositeInsert as InstEncoding>::decode(reader)?,
                    )
                }
                83u16 => {
                    Self::CopyObject(<OpCopyObject as InstEncoding>::decode(reader)?)
                }
                84u16 => Self::Transpose(<OpTranspose as InstEncoding>::decode(reader)?),
                86u16 => {
                    Self::SampledImage(<OpSampledImage as InstEncoding>::decode(reader)?)
                }
                87u16 => {
                    Self::ImageSampleImplicitLod(
                        <OpImageSampleImplicitLod as InstEncoding>::decode(reader)?,
                    )
                }
                88u16 => {
                    Self::ImageSampleExplicitLod(
                        <OpImageSampleExplicitLod as InstEncoding>::decode(reader)?,
                    )
                }
                89u16 => {
                    Self::ImageSampleDrefImplicitLod(
                        <OpImageSampleDrefImplicitLod as InstEncoding>::decode(reader)?,
                    )
                }
                90u16 => {
                    Self::ImageSampleDrefExplicitLod(
                        <OpImageSampleDrefExplicitLod as InstEncoding>::decode(reader)?,
                    )
                }
                91u16 => {
                    Self::ImageSampleProjImplicitLod(
                        <OpImageSampleProjImplicitLod as InstEncoding>::decode(reader)?,
                    )
                }
                92u16 => {
                    Self::ImageSampleProjExplicitLod(
                        <OpImageSampleProjExplicitLod as InstEncoding>::decode(reader)?,
                    )
                }
                93u16 => {
                    Self::ImageSampleProjDrefImplicitLod(
                        <OpImageSampleProjDrefImplicitLod as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                94u16 => {
                    Self::ImageSampleProjDrefExplicitLod(
                        <OpImageSampleProjDrefExplicitLod as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                95u16 => {
                    Self::ImageFetch(<OpImageFetch as InstEncoding>::decode(reader)?)
                }
                96u16 => {
                    Self::ImageGather(<OpImageGather as InstEncoding>::decode(reader)?)
                }
                97u16 => {
                    Self::ImageDrefGather(
                        <OpImageDrefGather as InstEncoding>::decode(reader)?,
                    )
                }
                98u16 => Self::ImageRead(<OpImageRead as InstEncoding>::decode(reader)?),
                99u16 => {
                    Self::ImageWrite(<OpImageWrite as InstEncoding>::decode(reader)?)
                }
                100u16 => Self::Image(<OpImage as InstEncoding>::decode(reader)?),
                101u16 => {
                    Self::ImageQueryFormat(
                        <OpImageQueryFormat as InstEncoding>::decode(reader)?,
                    )
                }
                102u16 => {
                    Self::ImageQueryOrder(
                        <OpImageQueryOrder as InstEncoding>::decode(reader)?,
                    )
                }
                103u16 => {
                    Self::ImageQuerySizeLod(
                        <OpImageQuerySizeLod as InstEncoding>::decode(reader)?,
                    )
                }
                104u16 => {
                    Self::ImageQuerySize(
                        <OpImageQuerySize as InstEncoding>::decode(reader)?,
                    )
                }
                105u16 => {
                    Self::ImageQueryLod(
                        <OpImageQueryLod as InstEncoding>::decode(reader)?,
                    )
                }
                106u16 => {
                    Self::ImageQueryLevels(
                        <OpImageQueryLevels as InstEncoding>::decode(reader)?,
                    )
                }
                107u16 => {
                    Self::ImageQuerySamples(
                        <OpImageQuerySamples as InstEncoding>::decode(reader)?,
                    )
                }
                109u16 => {
                    Self::ConvertFToU(<OpConvertFToU as InstEncoding>::decode(reader)?)
                }
                110u16 => {
                    Self::ConvertFToS(<OpConvertFToS as InstEncoding>::decode(reader)?)
                }
                111u16 => {
                    Self::ConvertSToF(<OpConvertSToF as InstEncoding>::decode(reader)?)
                }
                112u16 => {
                    Self::ConvertUToF(<OpConvertUToF as InstEncoding>::decode(reader)?)
                }
                113u16 => Self::UConvert(<OpUConvert as InstEncoding>::decode(reader)?),
                114u16 => Self::SConvert(<OpSConvert as InstEncoding>::decode(reader)?),
                115u16 => Self::FConvert(<OpFConvert as InstEncoding>::decode(reader)?),
                116u16 => {
                    Self::QuantizeToF16(
                        <OpQuantizeToF16 as InstEncoding>::decode(reader)?,
                    )
                }
                117u16 => {
                    Self::ConvertPtrToU(
                        <OpConvertPtrToU as InstEncoding>::decode(reader)?,
                    )
                }
                118u16 => {
                    Self::SatConvertSToU(
                        <OpSatConvertSToU as InstEncoding>::decode(reader)?,
                    )
                }
                119u16 => {
                    Self::SatConvertUToS(
                        <OpSatConvertUToS as InstEncoding>::decode(reader)?,
                    )
                }
                120u16 => {
                    Self::ConvertUToPtr(
                        <OpConvertUToPtr as InstEncoding>::decode(reader)?,
                    )
                }
                121u16 => {
                    Self::PtrCastToGeneric(
                        <OpPtrCastToGeneric as InstEncoding>::decode(reader)?,
                    )
                }
                122u16 => {
                    Self::GenericCastToPtr(
                        <OpGenericCastToPtr as InstEncoding>::decode(reader)?,
                    )
                }
                123u16 => {
                    Self::GenericCastToPtrExplicit(
                        <OpGenericCastToPtrExplicit as InstEncoding>::decode(reader)?,
                    )
                }
                124u16 => Self::Bitcast(<OpBitcast as InstEncoding>::decode(reader)?),
                126u16 => Self::SNegate(<OpSNegate as InstEncoding>::decode(reader)?),
                127u16 => Self::FNegate(<OpFNegate as InstEncoding>::decode(reader)?),
                128u16 => Self::IAdd(<OpIAdd as InstEncoding>::decode(reader)?),
                129u16 => Self::FAdd(<OpFAdd as InstEncoding>::decode(reader)?),
                130u16 => Self::ISub(<OpISub as InstEncoding>::decode(reader)?),
                131u16 => Self::FSub(<OpFSub as InstEncoding>::decode(reader)?),
                132u16 => Self::IMul(<OpIMul as InstEncoding>::decode(reader)?),
                133u16 => Self::FMul(<OpFMul as InstEncoding>::decode(reader)?),
                134u16 => Self::UDiv(<OpUDiv as InstEncoding>::decode(reader)?),
                135u16 => Self::SDiv(<OpSDiv as InstEncoding>::decode(reader)?),
                136u16 => Self::FDiv(<OpFDiv as InstEncoding>::decode(reader)?),
                137u16 => Self::UMod(<OpUMod as InstEncoding>::decode(reader)?),
                138u16 => Self::SRem(<OpSRem as InstEncoding>::decode(reader)?),
                139u16 => Self::SMod(<OpSMod as InstEncoding>::decode(reader)?),
                140u16 => Self::FRem(<OpFRem as InstEncoding>::decode(reader)?),
                141u16 => Self::FMod(<OpFMod as InstEncoding>::decode(reader)?),
                142u16 => {
                    Self::VectorTimesScalar(
                        <OpVectorTimesScalar as InstEncoding>::decode(reader)?,
                    )
                }
                143u16 => {
                    Self::MatrixTimesScalar(
                        <OpMatrixTimesScalar as InstEncoding>::decode(reader)?,
                    )
                }
                144u16 => {
                    Self::VectorTimesMatrix(
                        <OpVectorTimesMatrix as InstEncoding>::decode(reader)?,
                    )
                }
                145u16 => {
                    Self::MatrixTimesVector(
                        <OpMatrixTimesVector as InstEncoding>::decode(reader)?,
                    )
                }
                146u16 => {
                    Self::MatrixTimesMatrix(
                        <OpMatrixTimesMatrix as InstEncoding>::decode(reader)?,
                    )
                }
                147u16 => {
                    Self::OuterProduct(<OpOuterProduct as InstEncoding>::decode(reader)?)
                }
                148u16 => Self::Dot(<OpDot as InstEncoding>::decode(reader)?),
                149u16 => Self::IAddCarry(<OpIAddCarry as InstEncoding>::decode(reader)?),
                150u16 => {
                    Self::ISubBorrow(<OpISubBorrow as InstEncoding>::decode(reader)?)
                }
                151u16 => {
                    Self::UMulExtended(<OpUMulExtended as InstEncoding>::decode(reader)?)
                }
                152u16 => {
                    Self::SMulExtended(<OpSMulExtended as InstEncoding>::decode(reader)?)
                }
                154u16 => Self::Any(<OpAny as InstEncoding>::decode(reader)?),
                155u16 => Self::All(<OpAll as InstEncoding>::decode(reader)?),
                156u16 => Self::IsNan(<OpIsNan as InstEncoding>::decode(reader)?),
                157u16 => Self::IsInf(<OpIsInf as InstEncoding>::decode(reader)?),
                158u16 => Self::IsFinite(<OpIsFinite as InstEncoding>::decode(reader)?),
                159u16 => Self::IsNormal(<OpIsNormal as InstEncoding>::decode(reader)?),
                160u16 => {
                    Self::SignBitSet(<OpSignBitSet as InstEncoding>::decode(reader)?)
                }
                161u16 => {
                    Self::LessOrGreater(
                        <OpLessOrGreater as InstEncoding>::decode(reader)?,
                    )
                }
                162u16 => Self::Ordered(<OpOrdered as InstEncoding>::decode(reader)?),
                163u16 => Self::Unordered(<OpUnordered as InstEncoding>::decode(reader)?),
                164u16 => {
                    Self::LogicalEqual(<OpLogicalEqual as InstEncoding>::decode(reader)?)
                }
                165u16 => {
                    Self::LogicalNotEqual(
                        <OpLogicalNotEqual as InstEncoding>::decode(reader)?,
                    )
                }
                166u16 => Self::LogicalOr(<OpLogicalOr as InstEncoding>::decode(reader)?),
                167u16 => {
                    Self::LogicalAnd(<OpLogicalAnd as InstEncoding>::decode(reader)?)
                }
                168u16 => {
                    Self::LogicalNot(<OpLogicalNot as InstEncoding>::decode(reader)?)
                }
                169u16 => Self::Select(<OpSelect as InstEncoding>::decode(reader)?),
                170u16 => Self::IEqual(<OpIEqual as InstEncoding>::decode(reader)?),
                171u16 => Self::INotEqual(<OpINotEqual as InstEncoding>::decode(reader)?),
                172u16 => {
                    Self::UGreaterThan(<OpUGreaterThan as InstEncoding>::decode(reader)?)
                }
                173u16 => {
                    Self::SGreaterThan(<OpSGreaterThan as InstEncoding>::decode(reader)?)
                }
                174u16 => {
                    Self::UGreaterThanEqual(
                        <OpUGreaterThanEqual as InstEncoding>::decode(reader)?,
                    )
                }
                175u16 => {
                    Self::SGreaterThanEqual(
                        <OpSGreaterThanEqual as InstEncoding>::decode(reader)?,
                    )
                }
                176u16 => Self::ULessThan(<OpULessThan as InstEncoding>::decode(reader)?),
                177u16 => Self::SLessThan(<OpSLessThan as InstEncoding>::decode(reader)?),
                178u16 => {
                    Self::ULessThanEqual(
                        <OpULessThanEqual as InstEncoding>::decode(reader)?,
                    )
                }
                179u16 => {
                    Self::SLessThanEqual(
                        <OpSLessThanEqual as InstEncoding>::decode(reader)?,
                    )
                }
                180u16 => Self::FOrdEqual(<OpFOrdEqual as InstEncoding>::decode(reader)?),
                181u16 => {
                    Self::FUnordEqual(<OpFUnordEqual as InstEncoding>::decode(reader)?)
                }
                182u16 => {
                    Self::FOrdNotEqual(<OpFOrdNotEqual as InstEncoding>::decode(reader)?)
                }
                183u16 => {
                    Self::FUnordNotEqual(
                        <OpFUnordNotEqual as InstEncoding>::decode(reader)?,
                    )
                }
                184u16 => {
                    Self::FOrdLessThan(<OpFOrdLessThan as InstEncoding>::decode(reader)?)
                }
                185u16 => {
                    Self::FUnordLessThan(
                        <OpFUnordLessThan as InstEncoding>::decode(reader)?,
                    )
                }
                186u16 => {
                    Self::FOrdGreaterThan(
                        <OpFOrdGreaterThan as InstEncoding>::decode(reader)?,
                    )
                }
                187u16 => {
                    Self::FUnordGreaterThan(
                        <OpFUnordGreaterThan as InstEncoding>::decode(reader)?,
                    )
                }
                188u16 => {
                    Self::FOrdLessThanEqual(
                        <OpFOrdLessThanEqual as InstEncoding>::decode(reader)?,
                    )
                }
                189u16 => {
                    Self::FUnordLessThanEqual(
                        <OpFUnordLessThanEqual as InstEncoding>::decode(reader)?,
                    )
                }
                190u16 => {
                    Self::FOrdGreaterThanEqual(
                        <OpFOrdGreaterThanEqual as InstEncoding>::decode(reader)?,
                    )
                }
                191u16 => {
                    Self::FUnordGreaterThanEqual(
                        <OpFUnordGreaterThanEqual as InstEncoding>::decode(reader)?,
                    )
                }
                194u16 => {
                    Self::ShiftRightLogical(
                        <OpShiftRightLogical as InstEncoding>::decode(reader)?,
                    )
                }
                195u16 => {
                    Self::ShiftRightArithmetic(
                        <OpShiftRightArithmetic as InstEncoding>::decode(reader)?,
                    )
                }
                196u16 => {
                    Self::ShiftLeftLogical(
                        <OpShiftLeftLogical as InstEncoding>::decode(reader)?,
                    )
                }
                197u16 => Self::BitwiseOr(<OpBitwiseOr as InstEncoding>::decode(reader)?),
                198u16 => {
                    Self::BitwiseXor(<OpBitwiseXor as InstEncoding>::decode(reader)?)
                }
                199u16 => {
                    Self::BitwiseAnd(<OpBitwiseAnd as InstEncoding>::decode(reader)?)
                }
                200u16 => Self::Not(<OpNot as InstEncoding>::decode(reader)?),
                201u16 => {
                    Self::BitFieldInsert(
                        <OpBitFieldInsert as InstEncoding>::decode(reader)?,
                    )
                }
                202u16 => {
                    Self::BitFieldSExtract(
                        <OpBitFieldSExtract as InstEncoding>::decode(reader)?,
                    )
                }
                203u16 => {
                    Self::BitFieldUExtract(
                        <OpBitFieldUExtract as InstEncoding>::decode(reader)?,
                    )
                }
                204u16 => {
                    Self::BitReverse(<OpBitReverse as InstEncoding>::decode(reader)?)
                }
                205u16 => Self::BitCount(<OpBitCount as InstEncoding>::decode(reader)?),
                207u16 => Self::DPdx(<OpDPdx as InstEncoding>::decode(reader)?),
                208u16 => Self::DPdy(<OpDPdy as InstEncoding>::decode(reader)?),
                209u16 => Self::Fwidth(<OpFwidth as InstEncoding>::decode(reader)?),
                210u16 => Self::DPdxFine(<OpDPdxFine as InstEncoding>::decode(reader)?),
                211u16 => Self::DPdyFine(<OpDPdyFine as InstEncoding>::decode(reader)?),
                212u16 => {
                    Self::FwidthFine(<OpFwidthFine as InstEncoding>::decode(reader)?)
                }
                213u16 => {
                    Self::DPdxCoarse(<OpDPdxCoarse as InstEncoding>::decode(reader)?)
                }
                214u16 => {
                    Self::DPdyCoarse(<OpDPdyCoarse as InstEncoding>::decode(reader)?)
                }
                215u16 => {
                    Self::FwidthCoarse(<OpFwidthCoarse as InstEncoding>::decode(reader)?)
                }
                218u16 => {
                    Self::EmitVertex(<OpEmitVertex as InstEncoding>::decode(reader)?)
                }
                219u16 => {
                    Self::EndPrimitive(<OpEndPrimitive as InstEncoding>::decode(reader)?)
                }
                220u16 => {
                    Self::EmitStreamVertex(
                        <OpEmitStreamVertex as InstEncoding>::decode(reader)?,
                    )
                }
                221u16 => {
                    Self::EndStreamPrimitive(
                        <OpEndStreamPrimitive as InstEncoding>::decode(reader)?,
                    )
                }
                224u16 => {
                    Self::ControlBarrier(
                        <OpControlBarrier as InstEncoding>::decode(reader)?,
                    )
                }
                225u16 => {
                    Self::MemoryBarrier(
                        <OpMemoryBarrier as InstEncoding>::decode(reader)?,
                    )
                }
                227u16 => {
                    Self::AtomicLoad(<OpAtomicLoad as InstEncoding>::decode(reader)?)
                }
                228u16 => {
                    Self::AtomicStore(<OpAtomicStore as InstEncoding>::decode(reader)?)
                }
                229u16 => {
                    Self::AtomicExchange(
                        <OpAtomicExchange as InstEncoding>::decode(reader)?,
                    )
                }
                230u16 => {
                    Self::AtomicCompareExchange(
                        <OpAtomicCompareExchange as InstEncoding>::decode(reader)?,
                    )
                }
                231u16 => {
                    Self::AtomicCompareExchangeWeak(
                        <OpAtomicCompareExchangeWeak as InstEncoding>::decode(reader)?,
                    )
                }
                232u16 => {
                    Self::AtomicIIncrement(
                        <OpAtomicIIncrement as InstEncoding>::decode(reader)?,
                    )
                }
                233u16 => {
                    Self::AtomicIDecrement(
                        <OpAtomicIDecrement as InstEncoding>::decode(reader)?,
                    )
                }
                234u16 => {
                    Self::AtomicIAdd(<OpAtomicIAdd as InstEncoding>::decode(reader)?)
                }
                235u16 => {
                    Self::AtomicISub(<OpAtomicISub as InstEncoding>::decode(reader)?)
                }
                236u16 => {
                    Self::AtomicSMin(<OpAtomicSMin as InstEncoding>::decode(reader)?)
                }
                237u16 => {
                    Self::AtomicUMin(<OpAtomicUMin as InstEncoding>::decode(reader)?)
                }
                238u16 => {
                    Self::AtomicSMax(<OpAtomicSMax as InstEncoding>::decode(reader)?)
                }
                239u16 => {
                    Self::AtomicUMax(<OpAtomicUMax as InstEncoding>::decode(reader)?)
                }
                240u16 => Self::AtomicAnd(<OpAtomicAnd as InstEncoding>::decode(reader)?),
                241u16 => Self::AtomicOr(<OpAtomicOr as InstEncoding>::decode(reader)?),
                242u16 => Self::AtomicXor(<OpAtomicXor as InstEncoding>::decode(reader)?),
                245u16 => Self::Phi(<OpPhi as InstEncoding>::decode(reader)?),
                246u16 => Self::LoopMerge(<OpLoopMerge as InstEncoding>::decode(reader)?),
                247u16 => {
                    Self::SelectionMerge(
                        <OpSelectionMerge as InstEncoding>::decode(reader)?,
                    )
                }
                248u16 => Self::Label(<OpLabel as InstEncoding>::decode(reader)?),
                249u16 => Self::Branch(<OpBranch as InstEncoding>::decode(reader)?),
                250u16 => {
                    Self::BranchConditional(
                        <OpBranchConditional as InstEncoding>::decode(reader)?,
                    )
                }
                251u16 => Self::Switch(<OpSwitch as InstEncoding>::decode(reader)?),
                252u16 => Self::Kill(<OpKill as InstEncoding>::decode(reader)?),
                253u16 => Self::Return(<OpReturn as InstEncoding>::decode(reader)?),
                254u16 => {
                    Self::ReturnValue(<OpReturnValue as InstEncoding>::decode(reader)?)
                }
                255u16 => {
                    Self::Unreachable(<OpUnreachable as InstEncoding>::decode(reader)?)
                }
                256u16 => {
                    Self::LifetimeStart(
                        <OpLifetimeStart as InstEncoding>::decode(reader)?,
                    )
                }
                257u16 => {
                    Self::LifetimeStop(<OpLifetimeStop as InstEncoding>::decode(reader)?)
                }
                259u16 => {
                    Self::GroupAsyncCopy(
                        <OpGroupAsyncCopy as InstEncoding>::decode(reader)?,
                    )
                }
                260u16 => {
                    Self::GroupWaitEvents(
                        <OpGroupWaitEvents as InstEncoding>::decode(reader)?,
                    )
                }
                261u16 => Self::GroupAll(<OpGroupAll as InstEncoding>::decode(reader)?),
                262u16 => Self::GroupAny(<OpGroupAny as InstEncoding>::decode(reader)?),
                263u16 => {
                    Self::GroupBroadcast(
                        <OpGroupBroadcast as InstEncoding>::decode(reader)?,
                    )
                }
                264u16 => Self::GroupIAdd(<OpGroupIAdd as InstEncoding>::decode(reader)?),
                265u16 => Self::GroupFAdd(<OpGroupFAdd as InstEncoding>::decode(reader)?),
                266u16 => Self::GroupFMin(<OpGroupFMin as InstEncoding>::decode(reader)?),
                267u16 => Self::GroupUMin(<OpGroupUMin as InstEncoding>::decode(reader)?),
                268u16 => Self::GroupSMin(<OpGroupSMin as InstEncoding>::decode(reader)?),
                269u16 => Self::GroupFMax(<OpGroupFMax as InstEncoding>::decode(reader)?),
                270u16 => Self::GroupUMax(<OpGroupUMax as InstEncoding>::decode(reader)?),
                271u16 => Self::GroupSMax(<OpGroupSMax as InstEncoding>::decode(reader)?),
                274u16 => Self::ReadPipe(<OpReadPipe as InstEncoding>::decode(reader)?),
                275u16 => Self::WritePipe(<OpWritePipe as InstEncoding>::decode(reader)?),
                276u16 => {
                    Self::ReservedReadPipe(
                        <OpReservedReadPipe as InstEncoding>::decode(reader)?,
                    )
                }
                277u16 => {
                    Self::ReservedWritePipe(
                        <OpReservedWritePipe as InstEncoding>::decode(reader)?,
                    )
                }
                278u16 => {
                    Self::ReserveReadPipePackets(
                        <OpReserveReadPipePackets as InstEncoding>::decode(reader)?,
                    )
                }
                279u16 => {
                    Self::ReserveWritePipePackets(
                        <OpReserveWritePipePackets as InstEncoding>::decode(reader)?,
                    )
                }
                280u16 => {
                    Self::CommitReadPipe(
                        <OpCommitReadPipe as InstEncoding>::decode(reader)?,
                    )
                }
                281u16 => {
                    Self::CommitWritePipe(
                        <OpCommitWritePipe as InstEncoding>::decode(reader)?,
                    )
                }
                282u16 => {
                    Self::IsValidReserveId(
                        <OpIsValidReserveId as InstEncoding>::decode(reader)?,
                    )
                }
                283u16 => {
                    Self::GetNumPipePackets(
                        <OpGetNumPipePackets as InstEncoding>::decode(reader)?,
                    )
                }
                284u16 => {
                    Self::GetMaxPipePackets(
                        <OpGetMaxPipePackets as InstEncoding>::decode(reader)?,
                    )
                }
                285u16 => {
                    Self::GroupReserveReadPipePackets(
                        <OpGroupReserveReadPipePackets as InstEncoding>::decode(reader)?,
                    )
                }
                286u16 => {
                    Self::GroupReserveWritePipePackets(
                        <OpGroupReserveWritePipePackets as InstEncoding>::decode(reader)?,
                    )
                }
                287u16 => {
                    Self::GroupCommitReadPipe(
                        <OpGroupCommitReadPipe as InstEncoding>::decode(reader)?,
                    )
                }
                288u16 => {
                    Self::GroupCommitWritePipe(
                        <OpGroupCommitWritePipe as InstEncoding>::decode(reader)?,
                    )
                }
                291u16 => {
                    Self::EnqueueMarker(
                        <OpEnqueueMarker as InstEncoding>::decode(reader)?,
                    )
                }
                292u16 => {
                    Self::EnqueueKernel(
                        <OpEnqueueKernel as InstEncoding>::decode(reader)?,
                    )
                }
                293u16 => {
                    Self::GetKernelNDrangeSubGroupCount(
                        <OpGetKernelNDrangeSubGroupCount as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                294u16 => {
                    Self::GetKernelNDrangeMaxSubGroupSize(
                        <OpGetKernelNDrangeMaxSubGroupSize as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                295u16 => {
                    Self::GetKernelWorkGroupSize(
                        <OpGetKernelWorkGroupSize as InstEncoding>::decode(reader)?,
                    )
                }
                296u16 => {
                    Self::GetKernelPreferredWorkGroupSizeMultiple(
                        <OpGetKernelPreferredWorkGroupSizeMultiple as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                297u16 => {
                    Self::RetainEvent(<OpRetainEvent as InstEncoding>::decode(reader)?)
                }
                298u16 => {
                    Self::ReleaseEvent(<OpReleaseEvent as InstEncoding>::decode(reader)?)
                }
                299u16 => {
                    Self::CreateUserEvent(
                        <OpCreateUserEvent as InstEncoding>::decode(reader)?,
                    )
                }
                300u16 => {
                    Self::IsValidEvent(<OpIsValidEvent as InstEncoding>::decode(reader)?)
                }
                301u16 => {
                    Self::SetUserEventStatus(
                        <OpSetUserEventStatus as InstEncoding>::decode(reader)?,
                    )
                }
                302u16 => {
                    Self::CaptureEventProfilingInfo(
                        <OpCaptureEventProfilingInfo as InstEncoding>::decode(reader)?,
                    )
                }
                303u16 => {
                    Self::GetDefaultQueue(
                        <OpGetDefaultQueue as InstEncoding>::decode(reader)?,
                    )
                }
                304u16 => {
                    Self::BuildNDRange(<OpBuildNDRange as InstEncoding>::decode(reader)?)
                }
                305u16 => {
                    Self::ImageSparseSampleImplicitLod(
                        <OpImageSparseSampleImplicitLod as InstEncoding>::decode(reader)?,
                    )
                }
                306u16 => {
                    Self::ImageSparseSampleExplicitLod(
                        <OpImageSparseSampleExplicitLod as InstEncoding>::decode(reader)?,
                    )
                }
                307u16 => {
                    Self::ImageSparseSampleDrefImplicitLod(
                        <OpImageSparseSampleDrefImplicitLod as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                308u16 => {
                    Self::ImageSparseSampleDrefExplicitLod(
                        <OpImageSparseSampleDrefExplicitLod as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                309u16 => {
                    Self::ImageSparseSampleProjImplicitLod(
                        <OpImageSparseSampleProjImplicitLod as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                310u16 => {
                    Self::ImageSparseSampleProjExplicitLod(
                        <OpImageSparseSampleProjExplicitLod as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                311u16 => {
                    Self::ImageSparseSampleProjDrefImplicitLod(
                        <OpImageSparseSampleProjDrefImplicitLod as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                312u16 => {
                    Self::ImageSparseSampleProjDrefExplicitLod(
                        <OpImageSparseSampleProjDrefExplicitLod as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                313u16 => {
                    Self::ImageSparseFetch(
                        <OpImageSparseFetch as InstEncoding>::decode(reader)?,
                    )
                }
                314u16 => {
                    Self::ImageSparseGather(
                        <OpImageSparseGather as InstEncoding>::decode(reader)?,
                    )
                }
                315u16 => {
                    Self::ImageSparseDrefGather(
                        <OpImageSparseDrefGather as InstEncoding>::decode(reader)?,
                    )
                }
                316u16 => {
                    Self::ImageSparseTexelsResident(
                        <OpImageSparseTexelsResident as InstEncoding>::decode(reader)?,
                    )
                }
                317u16 => Self::NoLine(<OpNoLine as InstEncoding>::decode(reader)?),
                318u16 => {
                    Self::AtomicFlagTestAndSet(
                        <OpAtomicFlagTestAndSet as InstEncoding>::decode(reader)?,
                    )
                }
                319u16 => {
                    Self::AtomicFlagClear(
                        <OpAtomicFlagClear as InstEncoding>::decode(reader)?,
                    )
                }
                320u16 => {
                    Self::ImageSparseRead(
                        <OpImageSparseRead as InstEncoding>::decode(reader)?,
                    )
                }
                321u16 => Self::SizeOf(<OpSizeOf as InstEncoding>::decode(reader)?),
                322u16 => {
                    Self::TypePipeStorage(
                        <OpTypePipeStorage as InstEncoding>::decode(reader)?,
                    )
                }
                323u16 => {
                    Self::ConstantPipeStorage(
                        <OpConstantPipeStorage as InstEncoding>::decode(reader)?,
                    )
                }
                324u16 => {
                    Self::CreatePipeFromPipeStorage(
                        <OpCreatePipeFromPipeStorage as InstEncoding>::decode(reader)?,
                    )
                }
                325u16 => {
                    Self::GetKernelLocalSizeForSubgroupCount(
                        <OpGetKernelLocalSizeForSubgroupCount as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                326u16 => {
                    Self::GetKernelMaxNumSubgroups(
                        <OpGetKernelMaxNumSubgroups as InstEncoding>::decode(reader)?,
                    )
                }
                327u16 => {
                    Self::TypeNamedBarrier(
                        <OpTypeNamedBarrier as InstEncoding>::decode(reader)?,
                    )
                }
                328u16 => {
                    Self::NamedBarrierInitialize(
                        <OpNamedBarrierInitialize as InstEncoding>::decode(reader)?,
                    )
                }
                329u16 => {
                    Self::MemoryNamedBarrier(
                        <OpMemoryNamedBarrier as InstEncoding>::decode(reader)?,
                    )
                }
                330u16 => {
                    Self::ModuleProcessed(
                        <OpModuleProcessed as InstEncoding>::decode(reader)?,
                    )
                }
                331u16 => {
                    Self::ExecutionModeId(
                        <OpExecutionModeId as InstEncoding>::decode(reader)?,
                    )
                }
                332u16 => {
                    Self::DecorateId(<OpDecorateId as InstEncoding>::decode(reader)?)
                }
                333u16 => {
                    Self::GroupNonUniformElect(
                        <OpGroupNonUniformElect as InstEncoding>::decode(reader)?,
                    )
                }
                334u16 => {
                    Self::GroupNonUniformAll(
                        <OpGroupNonUniformAll as InstEncoding>::decode(reader)?,
                    )
                }
                335u16 => {
                    Self::GroupNonUniformAny(
                        <OpGroupNonUniformAny as InstEncoding>::decode(reader)?,
                    )
                }
                336u16 => {
                    Self::GroupNonUniformAllEqual(
                        <OpGroupNonUniformAllEqual as InstEncoding>::decode(reader)?,
                    )
                }
                337u16 => {
                    Self::GroupNonUniformBroadcast(
                        <OpGroupNonUniformBroadcast as InstEncoding>::decode(reader)?,
                    )
                }
                338u16 => {
                    Self::GroupNonUniformBroadcastFirst(
                        <OpGroupNonUniformBroadcastFirst as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                339u16 => {
                    Self::GroupNonUniformBallot(
                        <OpGroupNonUniformBallot as InstEncoding>::decode(reader)?,
                    )
                }
                340u16 => {
                    Self::GroupNonUniformInverseBallot(
                        <OpGroupNonUniformInverseBallot as InstEncoding>::decode(reader)?,
                    )
                }
                341u16 => {
                    Self::GroupNonUniformBallotBitExtract(
                        <OpGroupNonUniformBallotBitExtract as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                342u16 => {
                    Self::GroupNonUniformBallotBitCount(
                        <OpGroupNonUniformBallotBitCount as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                343u16 => {
                    Self::GroupNonUniformBallotFindLSB(
                        <OpGroupNonUniformBallotFindLSB as InstEncoding>::decode(reader)?,
                    )
                }
                344u16 => {
                    Self::GroupNonUniformBallotFindMSB(
                        <OpGroupNonUniformBallotFindMSB as InstEncoding>::decode(reader)?,
                    )
                }
                345u16 => {
                    Self::GroupNonUniformShuffle(
                        <OpGroupNonUniformShuffle as InstEncoding>::decode(reader)?,
                    )
                }
                346u16 => {
                    Self::GroupNonUniformShuffleXor(
                        <OpGroupNonUniformShuffleXor as InstEncoding>::decode(reader)?,
                    )
                }
                347u16 => {
                    Self::GroupNonUniformShuffleUp(
                        <OpGroupNonUniformShuffleUp as InstEncoding>::decode(reader)?,
                    )
                }
                348u16 => {
                    Self::GroupNonUniformShuffleDown(
                        <OpGroupNonUniformShuffleDown as InstEncoding>::decode(reader)?,
                    )
                }
                349u16 => {
                    Self::GroupNonUniformIAdd(
                        <OpGroupNonUniformIAdd as InstEncoding>::decode(reader)?,
                    )
                }
                350u16 => {
                    Self::GroupNonUniformFAdd(
                        <OpGroupNonUniformFAdd as InstEncoding>::decode(reader)?,
                    )
                }
                351u16 => {
                    Self::GroupNonUniformIMul(
                        <OpGroupNonUniformIMul as InstEncoding>::decode(reader)?,
                    )
                }
                352u16 => {
                    Self::GroupNonUniformFMul(
                        <OpGroupNonUniformFMul as InstEncoding>::decode(reader)?,
                    )
                }
                353u16 => {
                    Self::GroupNonUniformSMin(
                        <OpGroupNonUniformSMin as InstEncoding>::decode(reader)?,
                    )
                }
                354u16 => {
                    Self::GroupNonUniformUMin(
                        <OpGroupNonUniformUMin as InstEncoding>::decode(reader)?,
                    )
                }
                355u16 => {
                    Self::GroupNonUniformFMin(
                        <OpGroupNonUniformFMin as InstEncoding>::decode(reader)?,
                    )
                }
                356u16 => {
                    Self::GroupNonUniformSMax(
                        <OpGroupNonUniformSMax as InstEncoding>::decode(reader)?,
                    )
                }
                357u16 => {
                    Self::GroupNonUniformUMax(
                        <OpGroupNonUniformUMax as InstEncoding>::decode(reader)?,
                    )
                }
                358u16 => {
                    Self::GroupNonUniformFMax(
                        <OpGroupNonUniformFMax as InstEncoding>::decode(reader)?,
                    )
                }
                359u16 => {
                    Self::GroupNonUniformBitwiseAnd(
                        <OpGroupNonUniformBitwiseAnd as InstEncoding>::decode(reader)?,
                    )
                }
                360u16 => {
                    Self::GroupNonUniformBitwiseOr(
                        <OpGroupNonUniformBitwiseOr as InstEncoding>::decode(reader)?,
                    )
                }
                361u16 => {
                    Self::GroupNonUniformBitwiseXor(
                        <OpGroupNonUniformBitwiseXor as InstEncoding>::decode(reader)?,
                    )
                }
                362u16 => {
                    Self::GroupNonUniformLogicalAnd(
                        <OpGroupNonUniformLogicalAnd as InstEncoding>::decode(reader)?,
                    )
                }
                363u16 => {
                    Self::GroupNonUniformLogicalOr(
                        <OpGroupNonUniformLogicalOr as InstEncoding>::decode(reader)?,
                    )
                }
                364u16 => {
                    Self::GroupNonUniformLogicalXor(
                        <OpGroupNonUniformLogicalXor as InstEncoding>::decode(reader)?,
                    )
                }
                365u16 => {
                    Self::GroupNonUniformQuadBroadcast(
                        <OpGroupNonUniformQuadBroadcast as InstEncoding>::decode(reader)?,
                    )
                }
                366u16 => {
                    Self::GroupNonUniformQuadSwap(
                        <OpGroupNonUniformQuadSwap as InstEncoding>::decode(reader)?,
                    )
                }
                400u16 => {
                    Self::CopyLogical(<OpCopyLogical as InstEncoding>::decode(reader)?)
                }
                401u16 => Self::PtrEqual(<OpPtrEqual as InstEncoding>::decode(reader)?),
                402u16 => {
                    Self::PtrNotEqual(<OpPtrNotEqual as InstEncoding>::decode(reader)?)
                }
                403u16 => Self::PtrDiff(<OpPtrDiff as InstEncoding>::decode(reader)?),
                4160u16 => {
                    Self::ColorAttachmentReadEXT(
                        <OpColorAttachmentReadEXT as InstEncoding>::decode(reader)?,
                    )
                }
                4161u16 => {
                    Self::DepthAttachmentReadEXT(
                        <OpDepthAttachmentReadEXT as InstEncoding>::decode(reader)?,
                    )
                }
                4162u16 => {
                    Self::StencilAttachmentReadEXT(
                        <OpStencilAttachmentReadEXT as InstEncoding>::decode(reader)?,
                    )
                }
                4163u16 => {
                    Self::TypeTensorARM(
                        <OpTypeTensorARM as InstEncoding>::decode(reader)?,
                    )
                }
                4164u16 => {
                    Self::TensorReadARM(
                        <OpTensorReadARM as InstEncoding>::decode(reader)?,
                    )
                }
                4165u16 => {
                    Self::TensorWriteARM(
                        <OpTensorWriteARM as InstEncoding>::decode(reader)?,
                    )
                }
                4166u16 => {
                    Self::TensorQuerySizeARM(
                        <OpTensorQuerySizeARM as InstEncoding>::decode(reader)?,
                    )
                }
                4181u16 => {
                    Self::GraphConstantARM(
                        <OpGraphConstantARM as InstEncoding>::decode(reader)?,
                    )
                }
                4182u16 => {
                    Self::GraphEntryPointARM(
                        <OpGraphEntryPointARM as InstEncoding>::decode(reader)?,
                    )
                }
                4183u16 => Self::GraphARM(<OpGraphARM as InstEncoding>::decode(reader)?),
                4184u16 => {
                    Self::GraphInputARM(
                        <OpGraphInputARM as InstEncoding>::decode(reader)?,
                    )
                }
                4185u16 => {
                    Self::GraphSetOutputARM(
                        <OpGraphSetOutputARM as InstEncoding>::decode(reader)?,
                    )
                }
                4186u16 => {
                    Self::GraphEndARM(<OpGraphEndARM as InstEncoding>::decode(reader)?)
                }
                4190u16 => {
                    Self::TypeGraphARM(<OpTypeGraphARM as InstEncoding>::decode(reader)?)
                }
                4416u16 => {
                    Self::TerminateInvocation(
                        <OpTerminateInvocation as InstEncoding>::decode(reader)?,
                    )
                }
                4417u16 => {
                    Self::TypeUntypedPointerKHR(
                        <OpTypeUntypedPointerKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4418u16 => {
                    Self::UntypedVariableKHR(
                        <OpUntypedVariableKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4419u16 => {
                    Self::UntypedAccessChainKHR(
                        <OpUntypedAccessChainKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4420u16 => {
                    Self::UntypedInBoundsAccessChainKHR(
                        <OpUntypedInBoundsAccessChainKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                4421u16 => {
                    Self::SubgroupBallotKHR(
                        <OpSubgroupBallotKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4422u16 => {
                    Self::SubgroupFirstInvocationKHR(
                        <OpSubgroupFirstInvocationKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4423u16 => {
                    Self::UntypedPtrAccessChainKHR(
                        <OpUntypedPtrAccessChainKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4424u16 => {
                    Self::UntypedInBoundsPtrAccessChainKHR(
                        <OpUntypedInBoundsPtrAccessChainKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                4425u16 => {
                    Self::UntypedArrayLengthKHR(
                        <OpUntypedArrayLengthKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4426u16 => {
                    Self::UntypedPrefetchKHR(
                        <OpUntypedPrefetchKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4427u16 => Self::FmaKHR(<OpFmaKHR as InstEncoding>::decode(reader)?),
                4428u16 => {
                    Self::SubgroupAllKHR(
                        <OpSubgroupAllKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4429u16 => {
                    Self::SubgroupAnyKHR(
                        <OpSubgroupAnyKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4430u16 => {
                    Self::SubgroupAllEqualKHR(
                        <OpSubgroupAllEqualKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4431u16 => {
                    Self::GroupNonUniformRotateKHR(
                        <OpGroupNonUniformRotateKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4432u16 => {
                    Self::SubgroupReadInvocationKHR(
                        <OpSubgroupReadInvocationKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4433u16 => {
                    Self::ExtInstWithForwardRefsKHR(
                        <OpExtInstWithForwardRefsKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4434u16 => {
                    Self::UntypedGroupAsyncCopyKHR(
                        <OpUntypedGroupAsyncCopyKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4445u16 => {
                    Self::TraceRayKHR(<OpTraceRayKHR as InstEncoding>::decode(reader)?)
                }
                4446u16 => {
                    Self::ExecuteCallableKHR(
                        <OpExecuteCallableKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4447u16 => {
                    Self::ConvertUToAccelerationStructureKHR(
                        <OpConvertUToAccelerationStructureKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                4448u16 => {
                    Self::IgnoreIntersectionKHR(
                        <OpIgnoreIntersectionKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4449u16 => {
                    Self::TerminateRayKHR(
                        <OpTerminateRayKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4450u16 => Self::SDot(<OpSDot as InstEncoding>::decode(reader)?),
                4451u16 => Self::UDot(<OpUDot as InstEncoding>::decode(reader)?),
                4452u16 => Self::SUDot(<OpSUDot as InstEncoding>::decode(reader)?),
                4453u16 => {
                    Self::SDotAccSat(<OpSDotAccSat as InstEncoding>::decode(reader)?)
                }
                4454u16 => {
                    Self::UDotAccSat(<OpUDotAccSat as InstEncoding>::decode(reader)?)
                }
                4455u16 => {
                    Self::SUDotAccSat(<OpSUDotAccSat as InstEncoding>::decode(reader)?)
                }
                4456u16 => {
                    Self::TypeCooperativeMatrixKHR(
                        <OpTypeCooperativeMatrixKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4457u16 => {
                    Self::CooperativeMatrixLoadKHR(
                        <OpCooperativeMatrixLoadKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4458u16 => {
                    Self::CooperativeMatrixStoreKHR(
                        <OpCooperativeMatrixStoreKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4459u16 => {
                    Self::CooperativeMatrixMulAddKHR(
                        <OpCooperativeMatrixMulAddKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4460u16 => {
                    Self::CooperativeMatrixLengthKHR(
                        <OpCooperativeMatrixLengthKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4461u16 => {
                    Self::ConstantCompositeReplicateEXT(
                        <OpConstantCompositeReplicateEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                4462u16 => {
                    Self::SpecConstantCompositeReplicateEXT(
                        <OpSpecConstantCompositeReplicateEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                4463u16 => {
                    Self::CompositeConstructReplicateEXT(
                        <OpCompositeConstructReplicateEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                4472u16 => {
                    Self::TypeRayQueryKHR(
                        <OpTypeRayQueryKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4473u16 => {
                    Self::RayQueryInitializeKHR(
                        <OpRayQueryInitializeKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4474u16 => {
                    Self::RayQueryTerminateKHR(
                        <OpRayQueryTerminateKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4475u16 => {
                    Self::RayQueryGenerateIntersectionKHR(
                        <OpRayQueryGenerateIntersectionKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                4476u16 => {
                    Self::RayQueryConfirmIntersectionKHR(
                        <OpRayQueryConfirmIntersectionKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                4477u16 => {
                    Self::RayQueryProceedKHR(
                        <OpRayQueryProceedKHR as InstEncoding>::decode(reader)?,
                    )
                }
                4479u16 => {
                    Self::RayQueryGetIntersectionTypeKHR(
                        <OpRayQueryGetIntersectionTypeKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                4480u16 => {
                    Self::ImageSampleWeightedQCOM(
                        <OpImageSampleWeightedQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                4481u16 => {
                    Self::ImageBoxFilterQCOM(
                        <OpImageBoxFilterQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                4482u16 => {
                    Self::ImageBlockMatchSSDQCOM(
                        <OpImageBlockMatchSSDQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                4483u16 => {
                    Self::ImageBlockMatchSADQCOM(
                        <OpImageBlockMatchSADQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                4497u16 => {
                    Self::BitCastArrayQCOM(
                        <OpBitCastArrayQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                4500u16 => {
                    Self::ImageBlockMatchWindowSSDQCOM(
                        <OpImageBlockMatchWindowSSDQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                4501u16 => {
                    Self::ImageBlockMatchWindowSADQCOM(
                        <OpImageBlockMatchWindowSADQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                4502u16 => {
                    Self::ImageBlockMatchGatherSSDQCOM(
                        <OpImageBlockMatchGatherSSDQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                4503u16 => {
                    Self::ImageBlockMatchGatherSADQCOM(
                        <OpImageBlockMatchGatherSADQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                4540u16 => {
                    Self::CompositeConstructCoopMatQCOM(
                        <OpCompositeConstructCoopMatQCOM as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                4541u16 => {
                    Self::CompositeExtractCoopMatQCOM(
                        <OpCompositeExtractCoopMatQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                4542u16 => {
                    Self::ExtractSubArrayQCOM(
                        <OpExtractSubArrayQCOM as InstEncoding>::decode(reader)?,
                    )
                }
                5000u16 => {
                    Self::GroupIAddNonUniformAMD(
                        <OpGroupIAddNonUniformAMD as InstEncoding>::decode(reader)?,
                    )
                }
                5001u16 => {
                    Self::GroupFAddNonUniformAMD(
                        <OpGroupFAddNonUniformAMD as InstEncoding>::decode(reader)?,
                    )
                }
                5002u16 => {
                    Self::GroupFMinNonUniformAMD(
                        <OpGroupFMinNonUniformAMD as InstEncoding>::decode(reader)?,
                    )
                }
                5003u16 => {
                    Self::GroupUMinNonUniformAMD(
                        <OpGroupUMinNonUniformAMD as InstEncoding>::decode(reader)?,
                    )
                }
                5004u16 => {
                    Self::GroupSMinNonUniformAMD(
                        <OpGroupSMinNonUniformAMD as InstEncoding>::decode(reader)?,
                    )
                }
                5005u16 => {
                    Self::GroupFMaxNonUniformAMD(
                        <OpGroupFMaxNonUniformAMD as InstEncoding>::decode(reader)?,
                    )
                }
                5006u16 => {
                    Self::GroupUMaxNonUniformAMD(
                        <OpGroupUMaxNonUniformAMD as InstEncoding>::decode(reader)?,
                    )
                }
                5007u16 => {
                    Self::GroupSMaxNonUniformAMD(
                        <OpGroupSMaxNonUniformAMD as InstEncoding>::decode(reader)?,
                    )
                }
                5011u16 => {
                    Self::FragmentMaskFetchAMD(
                        <OpFragmentMaskFetchAMD as InstEncoding>::decode(reader)?,
                    )
                }
                5012u16 => {
                    Self::FragmentFetchAMD(
                        <OpFragmentFetchAMD as InstEncoding>::decode(reader)?,
                    )
                }
                5056u16 => {
                    Self::ReadClockKHR(<OpReadClockKHR as InstEncoding>::decode(reader)?)
                }
                5074u16 => {
                    Self::AllocateNodePayloadsAMDX(
                        <OpAllocateNodePayloadsAMDX as InstEncoding>::decode(reader)?,
                    )
                }
                5075u16 => {
                    Self::EnqueueNodePayloadsAMDX(
                        <OpEnqueueNodePayloadsAMDX as InstEncoding>::decode(reader)?,
                    )
                }
                5076u16 => {
                    Self::TypeNodePayloadArrayAMDX(
                        <OpTypeNodePayloadArrayAMDX as InstEncoding>::decode(reader)?,
                    )
                }
                5078u16 => {
                    Self::FinishWritingNodePayloadAMDX(
                        <OpFinishWritingNodePayloadAMDX as InstEncoding>::decode(reader)?,
                    )
                }
                5090u16 => {
                    Self::NodePayloadArrayLengthAMDX(
                        <OpNodePayloadArrayLengthAMDX as InstEncoding>::decode(reader)?,
                    )
                }
                5101u16 => {
                    Self::IsNodePayloadValidAMDX(
                        <OpIsNodePayloadValidAMDX as InstEncoding>::decode(reader)?,
                    )
                }
                5103u16 => {
                    Self::ConstantStringAMDX(
                        <OpConstantStringAMDX as InstEncoding>::decode(reader)?,
                    )
                }
                5104u16 => {
                    Self::SpecConstantStringAMDX(
                        <OpSpecConstantStringAMDX as InstEncoding>::decode(reader)?,
                    )
                }
                5110u16 => {
                    Self::GroupNonUniformQuadAllKHR(
                        <OpGroupNonUniformQuadAllKHR as InstEncoding>::decode(reader)?,
                    )
                }
                5111u16 => {
                    Self::GroupNonUniformQuadAnyKHR(
                        <OpGroupNonUniformQuadAnyKHR as InstEncoding>::decode(reader)?,
                    )
                }
                5115u16 => {
                    Self::TypeBufferEXT(
                        <OpTypeBufferEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5119u16 => {
                    Self::BufferPointerEXT(
                        <OpBufferPointerEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5126u16 => {
                    Self::UntypedImageTexelPointerEXT(
                        <OpUntypedImageTexelPointerEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5127u16 => {
                    Self::MemberDecorateIdEXT(
                        <OpMemberDecorateIdEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5129u16 => {
                    Self::ConstantSizeOfEXT(
                        <OpConstantSizeOfEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5249u16 => {
                    Self::HitObjectRecordHitMotionNV(
                        <OpHitObjectRecordHitMotionNV as InstEncoding>::decode(reader)?,
                    )
                }
                5250u16 => {
                    Self::HitObjectRecordHitWithIndexMotionNV(
                        <OpHitObjectRecordHitWithIndexMotionNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5251u16 => {
                    Self::HitObjectRecordMissMotionNV(
                        <OpHitObjectRecordMissMotionNV as InstEncoding>::decode(reader)?,
                    )
                }
                5252u16 => {
                    Self::HitObjectGetWorldToObjectNV(
                        <OpHitObjectGetWorldToObjectNV as InstEncoding>::decode(reader)?,
                    )
                }
                5253u16 => {
                    Self::HitObjectGetObjectToWorldNV(
                        <OpHitObjectGetObjectToWorldNV as InstEncoding>::decode(reader)?,
                    )
                }
                5254u16 => {
                    Self::HitObjectGetObjectRayDirectionNV(
                        <OpHitObjectGetObjectRayDirectionNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5255u16 => {
                    Self::HitObjectGetObjectRayOriginNV(
                        <OpHitObjectGetObjectRayOriginNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5256u16 => {
                    Self::HitObjectTraceRayMotionNV(
                        <OpHitObjectTraceRayMotionNV as InstEncoding>::decode(reader)?,
                    )
                }
                5257u16 => {
                    Self::HitObjectGetShaderRecordBufferHandleNV(
                        <OpHitObjectGetShaderRecordBufferHandleNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5258u16 => {
                    Self::HitObjectGetShaderBindingTableRecordIndexNV(
                        <OpHitObjectGetShaderBindingTableRecordIndexNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5259u16 => {
                    Self::HitObjectRecordEmptyNV(
                        <OpHitObjectRecordEmptyNV as InstEncoding>::decode(reader)?,
                    )
                }
                5260u16 => {
                    Self::HitObjectTraceRayNV(
                        <OpHitObjectTraceRayNV as InstEncoding>::decode(reader)?,
                    )
                }
                5261u16 => {
                    Self::HitObjectRecordHitNV(
                        <OpHitObjectRecordHitNV as InstEncoding>::decode(reader)?,
                    )
                }
                5262u16 => {
                    Self::HitObjectRecordHitWithIndexNV(
                        <OpHitObjectRecordHitWithIndexNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5263u16 => {
                    Self::HitObjectRecordMissNV(
                        <OpHitObjectRecordMissNV as InstEncoding>::decode(reader)?,
                    )
                }
                5264u16 => {
                    Self::HitObjectExecuteShaderNV(
                        <OpHitObjectExecuteShaderNV as InstEncoding>::decode(reader)?,
                    )
                }
                5265u16 => {
                    Self::HitObjectGetCurrentTimeNV(
                        <OpHitObjectGetCurrentTimeNV as InstEncoding>::decode(reader)?,
                    )
                }
                5266u16 => {
                    Self::HitObjectGetAttributesNV(
                        <OpHitObjectGetAttributesNV as InstEncoding>::decode(reader)?,
                    )
                }
                5267u16 => {
                    Self::HitObjectGetHitKindNV(
                        <OpHitObjectGetHitKindNV as InstEncoding>::decode(reader)?,
                    )
                }
                5268u16 => {
                    Self::HitObjectGetPrimitiveIndexNV(
                        <OpHitObjectGetPrimitiveIndexNV as InstEncoding>::decode(reader)?,
                    )
                }
                5269u16 => {
                    Self::HitObjectGetGeometryIndexNV(
                        <OpHitObjectGetGeometryIndexNV as InstEncoding>::decode(reader)?,
                    )
                }
                5270u16 => {
                    Self::HitObjectGetInstanceIdNV(
                        <OpHitObjectGetInstanceIdNV as InstEncoding>::decode(reader)?,
                    )
                }
                5271u16 => {
                    Self::HitObjectGetInstanceCustomIndexNV(
                        <OpHitObjectGetInstanceCustomIndexNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5272u16 => {
                    Self::HitObjectGetWorldRayDirectionNV(
                        <OpHitObjectGetWorldRayDirectionNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5273u16 => {
                    Self::HitObjectGetWorldRayOriginNV(
                        <OpHitObjectGetWorldRayOriginNV as InstEncoding>::decode(reader)?,
                    )
                }
                5274u16 => {
                    Self::HitObjectGetRayTMaxNV(
                        <OpHitObjectGetRayTMaxNV as InstEncoding>::decode(reader)?,
                    )
                }
                5275u16 => {
                    Self::HitObjectGetRayTMinNV(
                        <OpHitObjectGetRayTMinNV as InstEncoding>::decode(reader)?,
                    )
                }
                5276u16 => {
                    Self::HitObjectIsEmptyNV(
                        <OpHitObjectIsEmptyNV as InstEncoding>::decode(reader)?,
                    )
                }
                5277u16 => {
                    Self::HitObjectIsHitNV(
                        <OpHitObjectIsHitNV as InstEncoding>::decode(reader)?,
                    )
                }
                5278u16 => {
                    Self::HitObjectIsMissNV(
                        <OpHitObjectIsMissNV as InstEncoding>::decode(reader)?,
                    )
                }
                5279u16 => {
                    Self::ReorderThreadWithHitObjectNV(
                        <OpReorderThreadWithHitObjectNV as InstEncoding>::decode(reader)?,
                    )
                }
                5280u16 => {
                    Self::ReorderThreadWithHintNV(
                        <OpReorderThreadWithHintNV as InstEncoding>::decode(reader)?,
                    )
                }
                5281u16 => {
                    Self::TypeHitObjectNV(
                        <OpTypeHitObjectNV as InstEncoding>::decode(reader)?,
                    )
                }
                5283u16 => {
                    Self::ImageSampleFootprintNV(
                        <OpImageSampleFootprintNV as InstEncoding>::decode(reader)?,
                    )
                }
                5288u16 => {
                    Self::TypeVectorIdEXT(
                        <OpTypeVectorIdEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5289u16 => {
                    Self::CooperativeVectorMatrixMulNV(
                        <OpCooperativeVectorMatrixMulNV as InstEncoding>::decode(reader)?,
                    )
                }
                5290u16 => {
                    Self::CooperativeVectorOuterProductAccumulateNV(
                        <OpCooperativeVectorOuterProductAccumulateNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5291u16 => {
                    Self::CooperativeVectorReduceSumAccumulateNV(
                        <OpCooperativeVectorReduceSumAccumulateNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5292u16 => {
                    Self::CooperativeVectorMatrixMulAddNV(
                        <OpCooperativeVectorMatrixMulAddNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5293u16 => {
                    Self::CooperativeMatrixConvertNV(
                        <OpCooperativeMatrixConvertNV as InstEncoding>::decode(reader)?,
                    )
                }
                5294u16 => {
                    Self::EmitMeshTasksEXT(
                        <OpEmitMeshTasksEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5295u16 => {
                    Self::SetMeshOutputsEXT(
                        <OpSetMeshOutputsEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5296u16 => {
                    Self::GroupNonUniformPartitionEXT(
                        <OpGroupNonUniformPartitionEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5299u16 => {
                    Self::WritePackedPrimitiveIndices4x8NV(
                        <OpWritePackedPrimitiveIndices4x8NV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5300u16 => {
                    Self::FetchMicroTriangleVertexPositionNV(
                        <OpFetchMicroTriangleVertexPositionNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5301u16 => {
                    Self::FetchMicroTriangleVertexBarycentricNV(
                        <OpFetchMicroTriangleVertexBarycentricNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5302u16 => {
                    Self::CooperativeVectorLoadNV(
                        <OpCooperativeVectorLoadNV as InstEncoding>::decode(reader)?,
                    )
                }
                5303u16 => {
                    Self::CooperativeVectorStoreNV(
                        <OpCooperativeVectorStoreNV as InstEncoding>::decode(reader)?,
                    )
                }
                5304u16 => {
                    Self::HitObjectRecordFromQueryEXT(
                        <OpHitObjectRecordFromQueryEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5305u16 => {
                    Self::HitObjectRecordMissEXT(
                        <OpHitObjectRecordMissEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5306u16 => {
                    Self::HitObjectRecordMissMotionEXT(
                        <OpHitObjectRecordMissMotionEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5307u16 => {
                    Self::HitObjectGetIntersectionTriangleVertexPositionsEXT(
                        <OpHitObjectGetIntersectionTriangleVertexPositionsEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5308u16 => {
                    Self::HitObjectGetRayFlagsEXT(
                        <OpHitObjectGetRayFlagsEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5309u16 => {
                    Self::HitObjectSetShaderBindingTableRecordIndexEXT(
                        <OpHitObjectSetShaderBindingTableRecordIndexEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5310u16 => {
                    Self::HitObjectReorderExecuteShaderEXT(
                        <OpHitObjectReorderExecuteShaderEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5311u16 => {
                    Self::HitObjectTraceReorderExecuteEXT(
                        <OpHitObjectTraceReorderExecuteEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5312u16 => {
                    Self::HitObjectTraceMotionReorderExecuteEXT(
                        <OpHitObjectTraceMotionReorderExecuteEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5313u16 => {
                    Self::TypeHitObjectEXT(
                        <OpTypeHitObjectEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5314u16 => {
                    Self::ReorderThreadWithHintEXT(
                        <OpReorderThreadWithHintEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5315u16 => {
                    Self::ReorderThreadWithHitObjectEXT(
                        <OpReorderThreadWithHitObjectEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5316u16 => {
                    Self::HitObjectTraceRayEXT(
                        <OpHitObjectTraceRayEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5317u16 => {
                    Self::HitObjectTraceRayMotionEXT(
                        <OpHitObjectTraceRayMotionEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5318u16 => {
                    Self::HitObjectRecordEmptyEXT(
                        <OpHitObjectRecordEmptyEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5319u16 => {
                    Self::HitObjectExecuteShaderEXT(
                        <OpHitObjectExecuteShaderEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5320u16 => {
                    Self::HitObjectGetCurrentTimeEXT(
                        <OpHitObjectGetCurrentTimeEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5321u16 => {
                    Self::HitObjectGetAttributesEXT(
                        <OpHitObjectGetAttributesEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5322u16 => {
                    Self::HitObjectGetHitKindEXT(
                        <OpHitObjectGetHitKindEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5323u16 => {
                    Self::HitObjectGetPrimitiveIndexEXT(
                        <OpHitObjectGetPrimitiveIndexEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5324u16 => {
                    Self::HitObjectGetGeometryIndexEXT(
                        <OpHitObjectGetGeometryIndexEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5325u16 => {
                    Self::HitObjectGetInstanceIdEXT(
                        <OpHitObjectGetInstanceIdEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5326u16 => {
                    Self::HitObjectGetInstanceCustomIndexEXT(
                        <OpHitObjectGetInstanceCustomIndexEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5327u16 => {
                    Self::HitObjectGetObjectRayOriginEXT(
                        <OpHitObjectGetObjectRayOriginEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5328u16 => {
                    Self::HitObjectGetObjectRayDirectionEXT(
                        <OpHitObjectGetObjectRayDirectionEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5329u16 => {
                    Self::HitObjectGetWorldRayDirectionEXT(
                        <OpHitObjectGetWorldRayDirectionEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5330u16 => {
                    Self::HitObjectGetWorldRayOriginEXT(
                        <OpHitObjectGetWorldRayOriginEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5331u16 => {
                    Self::HitObjectGetObjectToWorldEXT(
                        <OpHitObjectGetObjectToWorldEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5332u16 => {
                    Self::HitObjectGetWorldToObjectEXT(
                        <OpHitObjectGetWorldToObjectEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5333u16 => {
                    Self::HitObjectGetRayTMaxEXT(
                        <OpHitObjectGetRayTMaxEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5334u16 => {
                    Self::ReportIntersectionKHR(
                        <OpReportIntersectionKHR as InstEncoding>::decode(reader)?,
                    )
                }
                5335u16 => {
                    Self::IgnoreIntersectionNV(
                        <OpIgnoreIntersectionNV as InstEncoding>::decode(reader)?,
                    )
                }
                5336u16 => {
                    Self::TerminateRayNV(
                        <OpTerminateRayNV as InstEncoding>::decode(reader)?,
                    )
                }
                5337u16 => Self::TraceNV(<OpTraceNV as InstEncoding>::decode(reader)?),
                5338u16 => {
                    Self::TraceMotionNV(
                        <OpTraceMotionNV as InstEncoding>::decode(reader)?,
                    )
                }
                5339u16 => {
                    Self::TraceRayMotionNV(
                        <OpTraceRayMotionNV as InstEncoding>::decode(reader)?,
                    )
                }
                5340u16 => {
                    Self::RayQueryGetIntersectionTriangleVertexPositionsKHR(
                        <OpRayQueryGetIntersectionTriangleVertexPositionsKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5341u16 => {
                    Self::TypeAccelerationStructureKHR(
                        <OpTypeAccelerationStructureKHR as InstEncoding>::decode(reader)?,
                    )
                }
                5344u16 => {
                    Self::ExecuteCallableNV(
                        <OpExecuteCallableNV as InstEncoding>::decode(reader)?,
                    )
                }
                5345u16 => {
                    Self::RayQueryGetIntersectionClusterIdNV(
                        <OpRayQueryGetIntersectionClusterIdNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5346u16 => {
                    Self::HitObjectGetClusterIdNV(
                        <OpHitObjectGetClusterIdNV as InstEncoding>::decode(reader)?,
                    )
                }
                5347u16 => {
                    Self::HitObjectGetRayTMinEXT(
                        <OpHitObjectGetRayTMinEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5348u16 => {
                    Self::HitObjectGetShaderBindingTableRecordIndexEXT(
                        <OpHitObjectGetShaderBindingTableRecordIndexEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5349u16 => {
                    Self::HitObjectGetShaderRecordBufferHandleEXT(
                        <OpHitObjectGetShaderRecordBufferHandleEXT as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5350u16 => {
                    Self::HitObjectIsEmptyEXT(
                        <OpHitObjectIsEmptyEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5351u16 => {
                    Self::HitObjectIsHitEXT(
                        <OpHitObjectIsHitEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5352u16 => {
                    Self::HitObjectIsMissEXT(
                        <OpHitObjectIsMissEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5358u16 => {
                    Self::TypeCooperativeMatrixNV(
                        <OpTypeCooperativeMatrixNV as InstEncoding>::decode(reader)?,
                    )
                }
                5359u16 => {
                    Self::CooperativeMatrixLoadNV(
                        <OpCooperativeMatrixLoadNV as InstEncoding>::decode(reader)?,
                    )
                }
                5360u16 => {
                    Self::CooperativeMatrixStoreNV(
                        <OpCooperativeMatrixStoreNV as InstEncoding>::decode(reader)?,
                    )
                }
                5361u16 => {
                    Self::CooperativeMatrixMulAddNV(
                        <OpCooperativeMatrixMulAddNV as InstEncoding>::decode(reader)?,
                    )
                }
                5362u16 => {
                    Self::CooperativeMatrixLengthNV(
                        <OpCooperativeMatrixLengthNV as InstEncoding>::decode(reader)?,
                    )
                }
                5364u16 => {
                    Self::BeginInvocationInterlockEXT(
                        <OpBeginInvocationInterlockEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5365u16 => {
                    Self::EndInvocationInterlockEXT(
                        <OpEndInvocationInterlockEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5366u16 => {
                    Self::CooperativeMatrixReduceNV(
                        <OpCooperativeMatrixReduceNV as InstEncoding>::decode(reader)?,
                    )
                }
                5367u16 => {
                    Self::CooperativeMatrixLoadTensorNV(
                        <OpCooperativeMatrixLoadTensorNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5368u16 => {
                    Self::CooperativeMatrixStoreTensorNV(
                        <OpCooperativeMatrixStoreTensorNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5369u16 => {
                    Self::CooperativeMatrixPerElementOpNV(
                        <OpCooperativeMatrixPerElementOpNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5370u16 => {
                    Self::TypeTensorLayoutNV(
                        <OpTypeTensorLayoutNV as InstEncoding>::decode(reader)?,
                    )
                }
                5371u16 => {
                    Self::TypeTensorViewNV(
                        <OpTypeTensorViewNV as InstEncoding>::decode(reader)?,
                    )
                }
                5372u16 => {
                    Self::CreateTensorLayoutNV(
                        <OpCreateTensorLayoutNV as InstEncoding>::decode(reader)?,
                    )
                }
                5373u16 => {
                    Self::TensorLayoutSetDimensionNV(
                        <OpTensorLayoutSetDimensionNV as InstEncoding>::decode(reader)?,
                    )
                }
                5374u16 => {
                    Self::TensorLayoutSetStrideNV(
                        <OpTensorLayoutSetStrideNV as InstEncoding>::decode(reader)?,
                    )
                }
                5375u16 => {
                    Self::TensorLayoutSliceNV(
                        <OpTensorLayoutSliceNV as InstEncoding>::decode(reader)?,
                    )
                }
                5376u16 => {
                    Self::TensorLayoutSetClampValueNV(
                        <OpTensorLayoutSetClampValueNV as InstEncoding>::decode(reader)?,
                    )
                }
                5377u16 => {
                    Self::CreateTensorViewNV(
                        <OpCreateTensorViewNV as InstEncoding>::decode(reader)?,
                    )
                }
                5378u16 => {
                    Self::TensorViewSetDimensionNV(
                        <OpTensorViewSetDimensionNV as InstEncoding>::decode(reader)?,
                    )
                }
                5379u16 => {
                    Self::TensorViewSetStrideNV(
                        <OpTensorViewSetStrideNV as InstEncoding>::decode(reader)?,
                    )
                }
                5380u16 => {
                    Self::DemoteToHelperInvocation(
                        <OpDemoteToHelperInvocation as InstEncoding>::decode(reader)?,
                    )
                }
                5381u16 => {
                    Self::IsHelperInvocationEXT(
                        <OpIsHelperInvocationEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5382u16 => {
                    Self::TensorViewSetClipNV(
                        <OpTensorViewSetClipNV as InstEncoding>::decode(reader)?,
                    )
                }
                5384u16 => {
                    Self::TensorLayoutSetBlockSizeNV(
                        <OpTensorLayoutSetBlockSizeNV as InstEncoding>::decode(reader)?,
                    )
                }
                5390u16 => {
                    Self::CooperativeMatrixTransposeNV(
                        <OpCooperativeMatrixTransposeNV as InstEncoding>::decode(reader)?,
                    )
                }
                5391u16 => {
                    Self::ConvertUToImageNV(
                        <OpConvertUToImageNV as InstEncoding>::decode(reader)?,
                    )
                }
                5392u16 => {
                    Self::ConvertUToSamplerNV(
                        <OpConvertUToSamplerNV as InstEncoding>::decode(reader)?,
                    )
                }
                5393u16 => {
                    Self::ConvertImageToUNV(
                        <OpConvertImageToUNV as InstEncoding>::decode(reader)?,
                    )
                }
                5394u16 => {
                    Self::ConvertSamplerToUNV(
                        <OpConvertSamplerToUNV as InstEncoding>::decode(reader)?,
                    )
                }
                5395u16 => {
                    Self::ConvertUToSampledImageNV(
                        <OpConvertUToSampledImageNV as InstEncoding>::decode(reader)?,
                    )
                }
                5396u16 => {
                    Self::ConvertSampledImageToUNV(
                        <OpConvertSampledImageToUNV as InstEncoding>::decode(reader)?,
                    )
                }
                5397u16 => {
                    Self::SamplerImageAddressingModeNV(
                        <OpSamplerImageAddressingModeNV as InstEncoding>::decode(reader)?,
                    )
                }
                5398u16 => {
                    Self::RawAccessChainNV(
                        <OpRawAccessChainNV as InstEncoding>::decode(reader)?,
                    )
                }
                5427u16 => {
                    Self::RayQueryGetIntersectionSpherePositionNV(
                        <OpRayQueryGetIntersectionSpherePositionNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5428u16 => {
                    Self::RayQueryGetIntersectionSphereRadiusNV(
                        <OpRayQueryGetIntersectionSphereRadiusNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5429u16 => {
                    Self::RayQueryGetIntersectionLSSPositionsNV(
                        <OpRayQueryGetIntersectionLSSPositionsNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5430u16 => {
                    Self::RayQueryGetIntersectionLSSRadiiNV(
                        <OpRayQueryGetIntersectionLSSRadiiNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5431u16 => {
                    Self::RayQueryGetIntersectionLSSHitValueNV(
                        <OpRayQueryGetIntersectionLSSHitValueNV as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5432u16 => {
                    Self::HitObjectGetSpherePositionNV(
                        <OpHitObjectGetSpherePositionNV as InstEncoding>::decode(reader)?,
                    )
                }
                5433u16 => {
                    Self::HitObjectGetSphereRadiusNV(
                        <OpHitObjectGetSphereRadiusNV as InstEncoding>::decode(reader)?,
                    )
                }
                5434u16 => {
                    Self::HitObjectGetLSSPositionsNV(
                        <OpHitObjectGetLSSPositionsNV as InstEncoding>::decode(reader)?,
                    )
                }
                5435u16 => {
                    Self::HitObjectGetLSSRadiiNV(
                        <OpHitObjectGetLSSRadiiNV as InstEncoding>::decode(reader)?,
                    )
                }
                5436u16 => {
                    Self::HitObjectIsSphereHitNV(
                        <OpHitObjectIsSphereHitNV as InstEncoding>::decode(reader)?,
                    )
                }
                5437u16 => {
                    Self::HitObjectIsLSSHitNV(
                        <OpHitObjectIsLSSHitNV as InstEncoding>::decode(reader)?,
                    )
                }
                5438u16 => {
                    Self::RayQueryIsSphereHitNV(
                        <OpRayQueryIsSphereHitNV as InstEncoding>::decode(reader)?,
                    )
                }
                5439u16 => {
                    Self::RayQueryIsLSSHitNV(
                        <OpRayQueryIsLSSHitNV as InstEncoding>::decode(reader)?,
                    )
                }
                5571u16 => {
                    Self::SubgroupShuffleINTEL(
                        <OpSubgroupShuffleINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5572u16 => {
                    Self::SubgroupShuffleDownINTEL(
                        <OpSubgroupShuffleDownINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5573u16 => {
                    Self::SubgroupShuffleUpINTEL(
                        <OpSubgroupShuffleUpINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5574u16 => {
                    Self::SubgroupShuffleXorINTEL(
                        <OpSubgroupShuffleXorINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5575u16 => {
                    Self::SubgroupBlockReadINTEL(
                        <OpSubgroupBlockReadINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5576u16 => {
                    Self::SubgroupBlockWriteINTEL(
                        <OpSubgroupBlockWriteINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5577u16 => {
                    Self::SubgroupImageBlockReadINTEL(
                        <OpSubgroupImageBlockReadINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5578u16 => {
                    Self::SubgroupImageBlockWriteINTEL(
                        <OpSubgroupImageBlockWriteINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5580u16 => {
                    Self::SubgroupImageMediaBlockReadINTEL(
                        <OpSubgroupImageMediaBlockReadINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5581u16 => {
                    Self::SubgroupImageMediaBlockWriteINTEL(
                        <OpSubgroupImageMediaBlockWriteINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5585u16 => {
                    Self::UCountLeadingZerosINTEL(
                        <OpUCountLeadingZerosINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5586u16 => {
                    Self::UCountTrailingZerosINTEL(
                        <OpUCountTrailingZerosINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5587u16 => {
                    Self::AbsISubINTEL(<OpAbsISubINTEL as InstEncoding>::decode(reader)?)
                }
                5588u16 => {
                    Self::AbsUSubINTEL(<OpAbsUSubINTEL as InstEncoding>::decode(reader)?)
                }
                5589u16 => {
                    Self::IAddSatINTEL(<OpIAddSatINTEL as InstEncoding>::decode(reader)?)
                }
                5590u16 => {
                    Self::UAddSatINTEL(<OpUAddSatINTEL as InstEncoding>::decode(reader)?)
                }
                5591u16 => {
                    Self::IAverageINTEL(
                        <OpIAverageINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5592u16 => {
                    Self::UAverageINTEL(
                        <OpUAverageINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5593u16 => {
                    Self::IAverageRoundedINTEL(
                        <OpIAverageRoundedINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5594u16 => {
                    Self::UAverageRoundedINTEL(
                        <OpUAverageRoundedINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5595u16 => {
                    Self::ISubSatINTEL(<OpISubSatINTEL as InstEncoding>::decode(reader)?)
                }
                5596u16 => {
                    Self::USubSatINTEL(<OpUSubSatINTEL as InstEncoding>::decode(reader)?)
                }
                5597u16 => {
                    Self::IMul32x16INTEL(
                        <OpIMul32x16INTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5598u16 => {
                    Self::UMul32x16INTEL(
                        <OpUMul32x16INTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5600u16 => {
                    Self::ConstantFunctionPointerINTEL(
                        <OpConstantFunctionPointerINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5601u16 => {
                    Self::FunctionPointerCallINTEL(
                        <OpFunctionPointerCallINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5609u16 => {
                    Self::AsmTargetINTEL(
                        <OpAsmTargetINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5610u16 => Self::AsmINTEL(<OpAsmINTEL as InstEncoding>::decode(reader)?),
                5611u16 => {
                    Self::AsmCallINTEL(<OpAsmCallINTEL as InstEncoding>::decode(reader)?)
                }
                5614u16 => {
                    Self::AtomicFMinEXT(
                        <OpAtomicFMinEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5615u16 => {
                    Self::AtomicFMaxEXT(
                        <OpAtomicFMaxEXT as InstEncoding>::decode(reader)?,
                    )
                }
                5630u16 => {
                    Self::AssumeTrueKHR(
                        <OpAssumeTrueKHR as InstEncoding>::decode(reader)?,
                    )
                }
                5631u16 => {
                    Self::ExpectKHR(<OpExpectKHR as InstEncoding>::decode(reader)?)
                }
                5632u16 => {
                    Self::DecorateString(
                        <OpDecorateString as InstEncoding>::decode(reader)?,
                    )
                }
                5633u16 => {
                    Self::MemberDecorateString(
                        <OpMemberDecorateString as InstEncoding>::decode(reader)?,
                    )
                }
                5699u16 => {
                    Self::VmeImageINTEL(
                        <OpVmeImageINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5700u16 => {
                    Self::TypeVmeImageINTEL(
                        <OpTypeVmeImageINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5701u16 => {
                    Self::TypeAvcImePayloadINTEL(
                        <OpTypeAvcImePayloadINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5702u16 => {
                    Self::TypeAvcRefPayloadINTEL(
                        <OpTypeAvcRefPayloadINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5703u16 => {
                    Self::TypeAvcSicPayloadINTEL(
                        <OpTypeAvcSicPayloadINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5704u16 => {
                    Self::TypeAvcMcePayloadINTEL(
                        <OpTypeAvcMcePayloadINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5705u16 => {
                    Self::TypeAvcMceResultINTEL(
                        <OpTypeAvcMceResultINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5706u16 => {
                    Self::TypeAvcImeResultINTEL(
                        <OpTypeAvcImeResultINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5707u16 => {
                    Self::TypeAvcImeResultSingleReferenceStreamoutINTEL(
                        <OpTypeAvcImeResultSingleReferenceStreamoutINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5708u16 => {
                    Self::TypeAvcImeResultDualReferenceStreamoutINTEL(
                        <OpTypeAvcImeResultDualReferenceStreamoutINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5709u16 => {
                    Self::TypeAvcImeSingleReferenceStreaminINTEL(
                        <OpTypeAvcImeSingleReferenceStreaminINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5710u16 => {
                    Self::TypeAvcImeDualReferenceStreaminINTEL(
                        <OpTypeAvcImeDualReferenceStreaminINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5711u16 => {
                    Self::TypeAvcRefResultINTEL(
                        <OpTypeAvcRefResultINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5712u16 => {
                    Self::TypeAvcSicResultINTEL(
                        <OpTypeAvcSicResultINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5713u16 => {
                    Self::SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(
                        <OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5714u16 => {
                    Self::SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(
                        <OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5715u16 => {
                    Self::SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(
                        <OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5716u16 => {
                    Self::SubgroupAvcMceSetInterShapePenaltyINTEL(
                        <OpSubgroupAvcMceSetInterShapePenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5717u16 => {
                    Self::SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(
                        <OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5718u16 => {
                    Self::SubgroupAvcMceSetInterDirectionPenaltyINTEL(
                        <OpSubgroupAvcMceSetInterDirectionPenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5719u16 => {
                    Self::SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(
                        <OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5720u16 => {
                    Self::SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(
                        <OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5721u16 => {
                    Self::SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(
                        <OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5722u16 => {
                    Self::SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(
                        <OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5723u16 => {
                    Self::SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(
                        <OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5724u16 => {
                    Self::SubgroupAvcMceSetMotionVectorCostFunctionINTEL(
                        <OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5725u16 => {
                    Self::SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(
                        <OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5726u16 => {
                    Self::SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(
                        <OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5727u16 => {
                    Self::SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(
                        <OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5728u16 => {
                    Self::SubgroupAvcMceSetAcOnlyHaarINTEL(
                        <OpSubgroupAvcMceSetAcOnlyHaarINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5729u16 => {
                    Self::SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(
                        <OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5730u16 => {
                    Self::SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(
                        <OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5731u16 => {
                    Self::SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(
                        <OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5732u16 => {
                    Self::SubgroupAvcMceConvertToImePayloadINTEL(
                        <OpSubgroupAvcMceConvertToImePayloadINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5733u16 => {
                    Self::SubgroupAvcMceConvertToImeResultINTEL(
                        <OpSubgroupAvcMceConvertToImeResultINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5734u16 => {
                    Self::SubgroupAvcMceConvertToRefPayloadINTEL(
                        <OpSubgroupAvcMceConvertToRefPayloadINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5735u16 => {
                    Self::SubgroupAvcMceConvertToRefResultINTEL(
                        <OpSubgroupAvcMceConvertToRefResultINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5736u16 => {
                    Self::SubgroupAvcMceConvertToSicPayloadINTEL(
                        <OpSubgroupAvcMceConvertToSicPayloadINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5737u16 => {
                    Self::SubgroupAvcMceConvertToSicResultINTEL(
                        <OpSubgroupAvcMceConvertToSicResultINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5738u16 => {
                    Self::SubgroupAvcMceGetMotionVectorsINTEL(
                        <OpSubgroupAvcMceGetMotionVectorsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5739u16 => {
                    Self::SubgroupAvcMceGetInterDistortionsINTEL(
                        <OpSubgroupAvcMceGetInterDistortionsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5740u16 => {
                    Self::SubgroupAvcMceGetBestInterDistortionsINTEL(
                        <OpSubgroupAvcMceGetBestInterDistortionsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5741u16 => {
                    Self::SubgroupAvcMceGetInterMajorShapeINTEL(
                        <OpSubgroupAvcMceGetInterMajorShapeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5742u16 => {
                    Self::SubgroupAvcMceGetInterMinorShapeINTEL(
                        <OpSubgroupAvcMceGetInterMinorShapeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5743u16 => {
                    Self::SubgroupAvcMceGetInterDirectionsINTEL(
                        <OpSubgroupAvcMceGetInterDirectionsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5744u16 => {
                    Self::SubgroupAvcMceGetInterMotionVectorCountINTEL(
                        <OpSubgroupAvcMceGetInterMotionVectorCountINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5745u16 => {
                    Self::SubgroupAvcMceGetInterReferenceIdsINTEL(
                        <OpSubgroupAvcMceGetInterReferenceIdsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5746u16 => {
                    Self::SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(
                        <OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5747u16 => {
                    Self::SubgroupAvcImeInitializeINTEL(
                        <OpSubgroupAvcImeInitializeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5748u16 => {
                    Self::SubgroupAvcImeSetSingleReferenceINTEL(
                        <OpSubgroupAvcImeSetSingleReferenceINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5749u16 => {
                    Self::SubgroupAvcImeSetDualReferenceINTEL(
                        <OpSubgroupAvcImeSetDualReferenceINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5750u16 => {
                    Self::SubgroupAvcImeRefWindowSizeINTEL(
                        <OpSubgroupAvcImeRefWindowSizeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5751u16 => {
                    Self::SubgroupAvcImeAdjustRefOffsetINTEL(
                        <OpSubgroupAvcImeAdjustRefOffsetINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5752u16 => {
                    Self::SubgroupAvcImeConvertToMcePayloadINTEL(
                        <OpSubgroupAvcImeConvertToMcePayloadINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5753u16 => {
                    Self::SubgroupAvcImeSetMaxMotionVectorCountINTEL(
                        <OpSubgroupAvcImeSetMaxMotionVectorCountINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5754u16 => {
                    Self::SubgroupAvcImeSetUnidirectionalMixDisableINTEL(
                        <OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5755u16 => {
                    Self::SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(
                        <OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5756u16 => {
                    Self::SubgroupAvcImeSetWeightedSadINTEL(
                        <OpSubgroupAvcImeSetWeightedSadINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5757u16 => {
                    Self::SubgroupAvcImeEvaluateWithSingleReferenceINTEL(
                        <OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5758u16 => {
                    Self::SubgroupAvcImeEvaluateWithDualReferenceINTEL(
                        <OpSubgroupAvcImeEvaluateWithDualReferenceINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5759u16 => {
                    Self::SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(
                        <OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5760u16 => {
                    Self::SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(
                        <OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5761u16 => {
                    Self::SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(
                        <OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5762u16 => {
                    Self::SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(
                        <OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5763u16 => {
                    Self::SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(
                        <OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5764u16 => {
                    Self::SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(
                        <OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5765u16 => {
                    Self::SubgroupAvcImeConvertToMceResultINTEL(
                        <OpSubgroupAvcImeConvertToMceResultINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5766u16 => {
                    Self::SubgroupAvcImeGetSingleReferenceStreaminINTEL(
                        <OpSubgroupAvcImeGetSingleReferenceStreaminINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5767u16 => {
                    Self::SubgroupAvcImeGetDualReferenceStreaminINTEL(
                        <OpSubgroupAvcImeGetDualReferenceStreaminINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5768u16 => {
                    Self::SubgroupAvcImeStripSingleReferenceStreamoutINTEL(
                        <OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5769u16 => {
                    Self::SubgroupAvcImeStripDualReferenceStreamoutINTEL(
                        <OpSubgroupAvcImeStripDualReferenceStreamoutINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5770u16 => {
                    Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
                        <OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5771u16 => {
                    Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(
                        <OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5772u16 => {
                    Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
                        <OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5773u16 => {
                    Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
                        <OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5774u16 => {
                    Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
                        <OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5775u16 => {
                    Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
                        <OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5776u16 => {
                    Self::SubgroupAvcImeGetBorderReachedINTEL(
                        <OpSubgroupAvcImeGetBorderReachedINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5777u16 => {
                    Self::SubgroupAvcImeGetTruncatedSearchIndicationINTEL(
                        <OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5778u16 => {
                    Self::SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(
                        <OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5779u16 => {
                    Self::SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(
                        <OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5780u16 => {
                    Self::SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(
                        <OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5781u16 => {
                    Self::SubgroupAvcFmeInitializeINTEL(
                        <OpSubgroupAvcFmeInitializeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5782u16 => {
                    Self::SubgroupAvcBmeInitializeINTEL(
                        <OpSubgroupAvcBmeInitializeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5783u16 => {
                    Self::SubgroupAvcRefConvertToMcePayloadINTEL(
                        <OpSubgroupAvcRefConvertToMcePayloadINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5784u16 => {
                    Self::SubgroupAvcRefSetBidirectionalMixDisableINTEL(
                        <OpSubgroupAvcRefSetBidirectionalMixDisableINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5785u16 => {
                    Self::SubgroupAvcRefSetBilinearFilterEnableINTEL(
                        <OpSubgroupAvcRefSetBilinearFilterEnableINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5786u16 => {
                    Self::SubgroupAvcRefEvaluateWithSingleReferenceINTEL(
                        <OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5787u16 => {
                    Self::SubgroupAvcRefEvaluateWithDualReferenceINTEL(
                        <OpSubgroupAvcRefEvaluateWithDualReferenceINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5788u16 => {
                    Self::SubgroupAvcRefEvaluateWithMultiReferenceINTEL(
                        <OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5789u16 => {
                    Self::SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(
                        <OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5790u16 => {
                    Self::SubgroupAvcRefConvertToMceResultINTEL(
                        <OpSubgroupAvcRefConvertToMceResultINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5791u16 => {
                    Self::SubgroupAvcSicInitializeINTEL(
                        <OpSubgroupAvcSicInitializeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5792u16 => {
                    Self::SubgroupAvcSicConfigureSkcINTEL(
                        <OpSubgroupAvcSicConfigureSkcINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5793u16 => {
                    Self::SubgroupAvcSicConfigureIpeLumaINTEL(
                        <OpSubgroupAvcSicConfigureIpeLumaINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5794u16 => {
                    Self::SubgroupAvcSicConfigureIpeLumaChromaINTEL(
                        <OpSubgroupAvcSicConfigureIpeLumaChromaINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5795u16 => {
                    Self::SubgroupAvcSicGetMotionVectorMaskINTEL(
                        <OpSubgroupAvcSicGetMotionVectorMaskINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5796u16 => {
                    Self::SubgroupAvcSicConvertToMcePayloadINTEL(
                        <OpSubgroupAvcSicConvertToMcePayloadINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5797u16 => {
                    Self::SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(
                        <OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5798u16 => {
                    Self::SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(
                        <OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5799u16 => {
                    Self::SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(
                        <OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5800u16 => {
                    Self::SubgroupAvcSicSetBilinearFilterEnableINTEL(
                        <OpSubgroupAvcSicSetBilinearFilterEnableINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5801u16 => {
                    Self::SubgroupAvcSicSetSkcForwardTransformEnableINTEL(
                        <OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5802u16 => {
                    Self::SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(
                        <OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5803u16 => {
                    Self::SubgroupAvcSicEvaluateIpeINTEL(
                        <OpSubgroupAvcSicEvaluateIpeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5804u16 => {
                    Self::SubgroupAvcSicEvaluateWithSingleReferenceINTEL(
                        <OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5805u16 => {
                    Self::SubgroupAvcSicEvaluateWithDualReferenceINTEL(
                        <OpSubgroupAvcSicEvaluateWithDualReferenceINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5806u16 => {
                    Self::SubgroupAvcSicEvaluateWithMultiReferenceINTEL(
                        <OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5807u16 => {
                    Self::SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(
                        <OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5808u16 => {
                    Self::SubgroupAvcSicConvertToMceResultINTEL(
                        <OpSubgroupAvcSicConvertToMceResultINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5809u16 => {
                    Self::SubgroupAvcSicGetIpeLumaShapeINTEL(
                        <OpSubgroupAvcSicGetIpeLumaShapeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5810u16 => {
                    Self::SubgroupAvcSicGetBestIpeLumaDistortionINTEL(
                        <OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5811u16 => {
                    Self::SubgroupAvcSicGetBestIpeChromaDistortionINTEL(
                        <OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5812u16 => {
                    Self::SubgroupAvcSicGetPackedIpeLumaModesINTEL(
                        <OpSubgroupAvcSicGetPackedIpeLumaModesINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5813u16 => {
                    Self::SubgroupAvcSicGetIpeChromaModeINTEL(
                        <OpSubgroupAvcSicGetIpeChromaModeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5814u16 => {
                    Self::SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(
                        <OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5815u16 => {
                    Self::SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(
                        <OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5816u16 => {
                    Self::SubgroupAvcSicGetInterRawSadsINTEL(
                        <OpSubgroupAvcSicGetInterRawSadsINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5818u16 => {
                    Self::VariableLengthArrayINTEL(
                        <OpVariableLengthArrayINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5819u16 => {
                    Self::SaveMemoryINTEL(
                        <OpSaveMemoryINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5820u16 => {
                    Self::RestoreMemoryINTEL(
                        <OpRestoreMemoryINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5840u16 => {
                    Self::ArbitraryFloatSinCosPiALTERA(
                        <OpArbitraryFloatSinCosPiALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5841u16 => {
                    Self::ArbitraryFloatCastALTERA(
                        <OpArbitraryFloatCastALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5842u16 => {
                    Self::ArbitraryFloatCastFromIntALTERA(
                        <OpArbitraryFloatCastFromIntALTERA as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5843u16 => {
                    Self::ArbitraryFloatCastToIntALTERA(
                        <OpArbitraryFloatCastToIntALTERA as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5846u16 => {
                    Self::ArbitraryFloatAddALTERA(
                        <OpArbitraryFloatAddALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5847u16 => {
                    Self::ArbitraryFloatSubALTERA(
                        <OpArbitraryFloatSubALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5848u16 => {
                    Self::ArbitraryFloatMulALTERA(
                        <OpArbitraryFloatMulALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5849u16 => {
                    Self::ArbitraryFloatDivALTERA(
                        <OpArbitraryFloatDivALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5850u16 => {
                    Self::ArbitraryFloatGTALTERA(
                        <OpArbitraryFloatGTALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5851u16 => {
                    Self::ArbitraryFloatGEALTERA(
                        <OpArbitraryFloatGEALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5852u16 => {
                    Self::ArbitraryFloatLTALTERA(
                        <OpArbitraryFloatLTALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5853u16 => {
                    Self::ArbitraryFloatLEALTERA(
                        <OpArbitraryFloatLEALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5854u16 => {
                    Self::ArbitraryFloatEQALTERA(
                        <OpArbitraryFloatEQALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5855u16 => {
                    Self::ArbitraryFloatRecipALTERA(
                        <OpArbitraryFloatRecipALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5856u16 => {
                    Self::ArbitraryFloatRSqrtALTERA(
                        <OpArbitraryFloatRSqrtALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5857u16 => {
                    Self::ArbitraryFloatCbrtALTERA(
                        <OpArbitraryFloatCbrtALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5858u16 => {
                    Self::ArbitraryFloatHypotALTERA(
                        <OpArbitraryFloatHypotALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5859u16 => {
                    Self::ArbitraryFloatSqrtALTERA(
                        <OpArbitraryFloatSqrtALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5860u16 => {
                    Self::ArbitraryFloatLogINTEL(
                        <OpArbitraryFloatLogINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5861u16 => {
                    Self::ArbitraryFloatLog2INTEL(
                        <OpArbitraryFloatLog2INTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5862u16 => {
                    Self::ArbitraryFloatLog10INTEL(
                        <OpArbitraryFloatLog10INTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5863u16 => {
                    Self::ArbitraryFloatLog1pINTEL(
                        <OpArbitraryFloatLog1pINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5864u16 => {
                    Self::ArbitraryFloatExpINTEL(
                        <OpArbitraryFloatExpINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5865u16 => {
                    Self::ArbitraryFloatExp2INTEL(
                        <OpArbitraryFloatExp2INTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5866u16 => {
                    Self::ArbitraryFloatExp10INTEL(
                        <OpArbitraryFloatExp10INTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5867u16 => {
                    Self::ArbitraryFloatExpm1INTEL(
                        <OpArbitraryFloatExpm1INTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5868u16 => {
                    Self::ArbitraryFloatSinINTEL(
                        <OpArbitraryFloatSinINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5869u16 => {
                    Self::ArbitraryFloatCosINTEL(
                        <OpArbitraryFloatCosINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5870u16 => {
                    Self::ArbitraryFloatSinCosINTEL(
                        <OpArbitraryFloatSinCosINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5871u16 => {
                    Self::ArbitraryFloatSinPiINTEL(
                        <OpArbitraryFloatSinPiINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5872u16 => {
                    Self::ArbitraryFloatCosPiINTEL(
                        <OpArbitraryFloatCosPiINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5873u16 => {
                    Self::ArbitraryFloatASinINTEL(
                        <OpArbitraryFloatASinINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5874u16 => {
                    Self::ArbitraryFloatASinPiINTEL(
                        <OpArbitraryFloatASinPiINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5875u16 => {
                    Self::ArbitraryFloatACosINTEL(
                        <OpArbitraryFloatACosINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5876u16 => {
                    Self::ArbitraryFloatACosPiINTEL(
                        <OpArbitraryFloatACosPiINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5877u16 => {
                    Self::ArbitraryFloatATanINTEL(
                        <OpArbitraryFloatATanINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5878u16 => {
                    Self::ArbitraryFloatATanPiINTEL(
                        <OpArbitraryFloatATanPiINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5879u16 => {
                    Self::ArbitraryFloatATan2INTEL(
                        <OpArbitraryFloatATan2INTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5880u16 => {
                    Self::ArbitraryFloatPowINTEL(
                        <OpArbitraryFloatPowINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5881u16 => {
                    Self::ArbitraryFloatPowRINTEL(
                        <OpArbitraryFloatPowRINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5882u16 => {
                    Self::ArbitraryFloatPowNINTEL(
                        <OpArbitraryFloatPowNINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5887u16 => {
                    Self::LoopControlINTEL(
                        <OpLoopControlINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5911u16 => {
                    Self::AliasDomainDeclINTEL(
                        <OpAliasDomainDeclINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5912u16 => {
                    Self::AliasScopeDeclINTEL(
                        <OpAliasScopeDeclINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5913u16 => {
                    Self::AliasScopeListDeclINTEL(
                        <OpAliasScopeListDeclINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                5923u16 => {
                    Self::FixedSqrtALTERA(
                        <OpFixedSqrtALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5924u16 => {
                    Self::FixedRecipALTERA(
                        <OpFixedRecipALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5925u16 => {
                    Self::FixedRsqrtALTERA(
                        <OpFixedRsqrtALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5926u16 => {
                    Self::FixedSinALTERA(
                        <OpFixedSinALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5927u16 => {
                    Self::FixedCosALTERA(
                        <OpFixedCosALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5928u16 => {
                    Self::FixedSinCosALTERA(
                        <OpFixedSinCosALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5929u16 => {
                    Self::FixedSinPiALTERA(
                        <OpFixedSinPiALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5930u16 => {
                    Self::FixedCosPiALTERA(
                        <OpFixedCosPiALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5931u16 => {
                    Self::FixedSinCosPiALTERA(
                        <OpFixedSinCosPiALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5932u16 => {
                    Self::FixedLogALTERA(
                        <OpFixedLogALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5933u16 => {
                    Self::FixedExpALTERA(
                        <OpFixedExpALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5934u16 => {
                    Self::PtrCastToCrossWorkgroupALTERA(
                        <OpPtrCastToCrossWorkgroupALTERA as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5938u16 => {
                    Self::CrossWorkgroupCastToPtrALTERA(
                        <OpCrossWorkgroupCastToPtrALTERA as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                5946u16 => {
                    Self::ReadPipeBlockingALTERA(
                        <OpReadPipeBlockingALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5947u16 => {
                    Self::WritePipeBlockingALTERA(
                        <OpWritePipeBlockingALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                5949u16 => {
                    Self::FPGARegALTERA(
                        <OpFPGARegALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                6016u16 => {
                    Self::RayQueryGetRayTMinKHR(
                        <OpRayQueryGetRayTMinKHR as InstEncoding>::decode(reader)?,
                    )
                }
                6017u16 => {
                    Self::RayQueryGetRayFlagsKHR(
                        <OpRayQueryGetRayFlagsKHR as InstEncoding>::decode(reader)?,
                    )
                }
                6018u16 => {
                    Self::RayQueryGetIntersectionTKHR(
                        <OpRayQueryGetIntersectionTKHR as InstEncoding>::decode(reader)?,
                    )
                }
                6019u16 => {
                    Self::RayQueryGetIntersectionInstanceCustomIndexKHR(
                        <OpRayQueryGetIntersectionInstanceCustomIndexKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6020u16 => {
                    Self::RayQueryGetIntersectionInstanceIdKHR(
                        <OpRayQueryGetIntersectionInstanceIdKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6021u16 => {
                    Self::RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(
                        <OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6022u16 => {
                    Self::RayQueryGetIntersectionGeometryIndexKHR(
                        <OpRayQueryGetIntersectionGeometryIndexKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6023u16 => {
                    Self::RayQueryGetIntersectionPrimitiveIndexKHR(
                        <OpRayQueryGetIntersectionPrimitiveIndexKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6024u16 => {
                    Self::RayQueryGetIntersectionBarycentricsKHR(
                        <OpRayQueryGetIntersectionBarycentricsKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6025u16 => {
                    Self::RayQueryGetIntersectionFrontFaceKHR(
                        <OpRayQueryGetIntersectionFrontFaceKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6026u16 => {
                    Self::RayQueryGetIntersectionCandidateAABBOpaqueKHR(
                        <OpRayQueryGetIntersectionCandidateAABBOpaqueKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6027u16 => {
                    Self::RayQueryGetIntersectionObjectRayDirectionKHR(
                        <OpRayQueryGetIntersectionObjectRayDirectionKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6028u16 => {
                    Self::RayQueryGetIntersectionObjectRayOriginKHR(
                        <OpRayQueryGetIntersectionObjectRayOriginKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6029u16 => {
                    Self::RayQueryGetWorldRayDirectionKHR(
                        <OpRayQueryGetWorldRayDirectionKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6030u16 => {
                    Self::RayQueryGetWorldRayOriginKHR(
                        <OpRayQueryGetWorldRayOriginKHR as InstEncoding>::decode(reader)?,
                    )
                }
                6031u16 => {
                    Self::RayQueryGetIntersectionObjectToWorldKHR(
                        <OpRayQueryGetIntersectionObjectToWorldKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6032u16 => {
                    Self::RayQueryGetIntersectionWorldToObjectKHR(
                        <OpRayQueryGetIntersectionWorldToObjectKHR as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6035u16 => {
                    Self::AtomicFAddEXT(
                        <OpAtomicFAddEXT as InstEncoding>::decode(reader)?,
                    )
                }
                6086u16 => {
                    Self::TypeBufferSurfaceINTEL(
                        <OpTypeBufferSurfaceINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6090u16 => {
                    Self::TypeStructContinuedINTEL(
                        <OpTypeStructContinuedINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6091u16 => {
                    Self::ConstantCompositeContinuedINTEL(
                        <OpConstantCompositeContinuedINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6092u16 => {
                    Self::SpecConstantCompositeContinuedINTEL(
                        <OpSpecConstantCompositeContinuedINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6096u16 => {
                    Self::CompositeConstructContinuedINTEL(
                        <OpCompositeConstructContinuedINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6116u16 => {
                    Self::ConvertFToBF16INTEL(
                        <OpConvertFToBF16INTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6117u16 => {
                    Self::ConvertBF16ToFINTEL(
                        <OpConvertBF16ToFINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6142u16 => {
                    Self::ControlBarrierArriveINTEL(
                        <OpControlBarrierArriveINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6143u16 => {
                    Self::ControlBarrierWaitINTEL(
                        <OpControlBarrierWaitINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6145u16 => {
                    Self::ArithmeticFenceEXT(
                        <OpArithmeticFenceEXT as InstEncoding>::decode(reader)?,
                    )
                }
                6163u16 => {
                    Self::TaskSequenceCreateALTERA(
                        <OpTaskSequenceCreateALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                6164u16 => {
                    Self::TaskSequenceAsyncALTERA(
                        <OpTaskSequenceAsyncALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                6165u16 => {
                    Self::TaskSequenceGetALTERA(
                        <OpTaskSequenceGetALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                6166u16 => {
                    Self::TaskSequenceReleaseALTERA(
                        <OpTaskSequenceReleaseALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                6199u16 => {
                    Self::TypeTaskSequenceALTERA(
                        <OpTypeTaskSequenceALTERA as InstEncoding>::decode(reader)?,
                    )
                }
                6221u16 => {
                    Self::SubgroupBlockPrefetchINTEL(
                        <OpSubgroupBlockPrefetchINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6231u16 => {
                    Self::Subgroup2DBlockLoadINTEL(
                        <OpSubgroup2DBlockLoadINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6232u16 => {
                    Self::Subgroup2DBlockLoadTransformINTEL(
                        <OpSubgroup2DBlockLoadTransformINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6233u16 => {
                    Self::Subgroup2DBlockLoadTransposeINTEL(
                        <OpSubgroup2DBlockLoadTransposeINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6234u16 => {
                    Self::Subgroup2DBlockPrefetchINTEL(
                        <OpSubgroup2DBlockPrefetchINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6235u16 => {
                    Self::Subgroup2DBlockStoreINTEL(
                        <OpSubgroup2DBlockStoreINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6237u16 => {
                    Self::SubgroupMatrixMultiplyAccumulateINTEL(
                        <OpSubgroupMatrixMultiplyAccumulateINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6242u16 => {
                    Self::BitwiseFunctionINTEL(
                        <OpBitwiseFunctionINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6244u16 => {
                    Self::UntypedVariableLengthArrayINTEL(
                        <OpUntypedVariableLengthArrayINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6248u16 => {
                    Self::ConditionalExtensionINTEL(
                        <OpConditionalExtensionINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6249u16 => {
                    Self::ConditionalEntryPointINTEL(
                        <OpConditionalEntryPointINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6250u16 => {
                    Self::ConditionalCapabilityINTEL(
                        <OpConditionalCapabilityINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6251u16 => {
                    Self::SpecConstantTargetINTEL(
                        <OpSpecConstantTargetINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6252u16 => {
                    Self::SpecConstantArchitectureINTEL(
                        <OpSpecConstantArchitectureINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6253u16 => {
                    Self::SpecConstantCapabilitiesINTEL(
                        <OpSpecConstantCapabilitiesINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                6254u16 => {
                    Self::ConditionalCopyObjectINTEL(
                        <OpConditionalCopyObjectINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6401u16 => {
                    Self::GroupIMulKHR(<OpGroupIMulKHR as InstEncoding>::decode(reader)?)
                }
                6402u16 => {
                    Self::GroupFMulKHR(<OpGroupFMulKHR as InstEncoding>::decode(reader)?)
                }
                6403u16 => {
                    Self::GroupBitwiseAndKHR(
                        <OpGroupBitwiseAndKHR as InstEncoding>::decode(reader)?,
                    )
                }
                6404u16 => {
                    Self::GroupBitwiseOrKHR(
                        <OpGroupBitwiseOrKHR as InstEncoding>::decode(reader)?,
                    )
                }
                6405u16 => {
                    Self::GroupBitwiseXorKHR(
                        <OpGroupBitwiseXorKHR as InstEncoding>::decode(reader)?,
                    )
                }
                6406u16 => {
                    Self::GroupLogicalAndKHR(
                        <OpGroupLogicalAndKHR as InstEncoding>::decode(reader)?,
                    )
                }
                6407u16 => {
                    Self::GroupLogicalOrKHR(
                        <OpGroupLogicalOrKHR as InstEncoding>::decode(reader)?,
                    )
                }
                6408u16 => {
                    Self::GroupLogicalXorKHR(
                        <OpGroupLogicalXorKHR as InstEncoding>::decode(reader)?,
                    )
                }
                6426u16 => {
                    Self::RoundFToTF32INTEL(
                        <OpRoundFToTF32INTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6428u16 => {
                    Self::MaskedGatherINTEL(
                        <OpMaskedGatherINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6429u16 => {
                    Self::MaskedScatterINTEL(
                        <OpMaskedScatterINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6529u16 => {
                    Self::ConvertHandleToImageINTEL(
                        <OpConvertHandleToImageINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6530u16 => {
                    Self::ConvertHandleToSamplerINTEL(
                        <OpConvertHandleToSamplerINTEL as InstEncoding>::decode(reader)?,
                    )
                }
                6531u16 => {
                    Self::ConvertHandleToSampledImageINTEL(
                        <OpConvertHandleToSampledImageINTEL as InstEncoding>::decode(
                            reader,
                        )?,
                    )
                }
                _ => {
                    return Err(DecodeError::UnknownOpCode {
                        opcode,
                    });
                }
            },
        )
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        match self {
            Self::Nop(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Undef(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SourceContinued(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Source(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SourceExtension(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Name(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MemberName(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::String(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Line(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Extension(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ExtInstImport(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ExtInst(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MemoryModel(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::EntryPoint(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ExecutionMode(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Capability(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeVoid(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeBool(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeInt(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeFloat(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeVector(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeMatrix(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeImage(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeSampler(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeSampledImage(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeArray(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeRuntimeArray(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeStruct(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeOpaque(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypePointer(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeFunction(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeEvent(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeDeviceEvent(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeReserveId(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeQueue(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypePipe(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeForwardPointer(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantTrue(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantFalse(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Constant(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantComposite(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantSampler(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantNull(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstantTrue(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstantFalse(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstant(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstantComposite(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstantOp(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Function(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FunctionParameter(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FunctionEnd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FunctionCall(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Variable(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageTexelPointer(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Load(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Store(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CopyMemory(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CopyMemorySized(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AccessChain(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::InBoundsAccessChain(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PtrAccessChain(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArrayLength(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GenericPtrMemSemantics(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::InBoundsPtrAccessChain(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Decorate(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MemberDecorate(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DecorationGroup(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupDecorate(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupMemberDecorate(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::VectorExtractDynamic(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::VectorInsertDynamic(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::VectorShuffle(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CompositeConstruct(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CompositeExtract(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CompositeInsert(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CopyObject(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Transpose(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SampledImage(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSampleImplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSampleExplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSampleDrefImplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSampleDrefExplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSampleProjImplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSampleProjExplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSampleProjDrefImplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSampleProjDrefExplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageFetch(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageGather(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageDrefGather(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageRead(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageWrite(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Image(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageQueryFormat(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageQueryOrder(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageQuerySizeLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageQuerySize(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageQueryLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageQueryLevels(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageQuerySamples(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertFToU(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertFToS(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertSToF(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertUToF(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UConvert(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SConvert(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FConvert(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::QuantizeToF16(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertPtrToU(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SatConvertSToU(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SatConvertUToS(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertUToPtr(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PtrCastToGeneric(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GenericCastToPtr(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GenericCastToPtrExplicit(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Bitcast(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SNegate(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FNegate(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IAdd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FAdd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ISub(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FSub(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IMul(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FMul(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UDiv(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SDiv(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FDiv(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UMod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SRem(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SMod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FRem(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FMod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::VectorTimesScalar(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MatrixTimesScalar(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::VectorTimesMatrix(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MatrixTimesVector(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MatrixTimesMatrix(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::OuterProduct(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Dot(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IAddCarry(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ISubBorrow(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UMulExtended(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SMulExtended(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Any(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::All(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IsNan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IsInf(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IsFinite(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IsNormal(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SignBitSet(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::LessOrGreater(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Ordered(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Unordered(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::LogicalEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::LogicalNotEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::LogicalOr(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::LogicalAnd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::LogicalNot(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Select(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::INotEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UGreaterThan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SGreaterThan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UGreaterThanEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SGreaterThanEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ULessThan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SLessThan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ULessThanEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SLessThanEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FOrdEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FUnordEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FOrdNotEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FUnordNotEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FOrdLessThan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FUnordLessThan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FOrdGreaterThan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FUnordGreaterThan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FOrdLessThanEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FUnordLessThanEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FOrdGreaterThanEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FUnordGreaterThanEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ShiftRightLogical(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ShiftRightArithmetic(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ShiftLeftLogical(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitwiseOr(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitwiseXor(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitwiseAnd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Not(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitFieldInsert(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitFieldSExtract(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitFieldUExtract(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitReverse(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitCount(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DPdx(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DPdy(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Fwidth(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DPdxFine(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DPdyFine(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FwidthFine(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DPdxCoarse(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DPdyCoarse(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FwidthCoarse(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::EmitVertex(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::EndPrimitive(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::EmitStreamVertex(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::EndStreamPrimitive(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ControlBarrier(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MemoryBarrier(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicLoad(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicStore(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicExchange(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicCompareExchange(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicCompareExchangeWeak(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicIIncrement(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicIDecrement(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicIAdd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicISub(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicSMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicUMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicSMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicUMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicAnd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicOr(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicXor(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Phi(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::LoopMerge(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SelectionMerge(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Label(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Branch(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BranchConditional(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Switch(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Kill(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Return(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReturnValue(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Unreachable(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::LifetimeStart(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::LifetimeStop(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupAsyncCopy(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupWaitEvents(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupAll(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupAny(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupBroadcast(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupIAdd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupFAdd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupFMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupUMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupSMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupFMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupUMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupSMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReadPipe(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::WritePipe(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReservedReadPipe(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReservedWritePipe(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReserveReadPipePackets(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReserveWritePipePackets(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CommitReadPipe(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CommitWritePipe(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IsValidReserveId(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GetNumPipePackets(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GetMaxPipePackets(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupReserveReadPipePackets(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupReserveWritePipePackets(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupCommitReadPipe(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupCommitWritePipe(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::EnqueueMarker(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::EnqueueKernel(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GetKernelNDrangeSubGroupCount(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GetKernelNDrangeMaxSubGroupSize(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GetKernelWorkGroupSize(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GetKernelPreferredWorkGroupSizeMultiple(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RetainEvent(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReleaseEvent(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CreateUserEvent(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IsValidEvent(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SetUserEventStatus(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CaptureEventProfilingInfo(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GetDefaultQueue(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BuildNDRange(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseSampleImplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseSampleExplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseSampleDrefImplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseSampleDrefExplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseSampleProjImplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseSampleProjExplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseSampleProjDrefImplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseSampleProjDrefExplicitLod(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseFetch(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseGather(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseDrefGather(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseTexelsResident(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::NoLine(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicFlagTestAndSet(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicFlagClear(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSparseRead(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SizeOf(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypePipeStorage(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantPipeStorage(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CreatePipeFromPipeStorage(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GetKernelLocalSizeForSubgroupCount(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GetKernelMaxNumSubgroups(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeNamedBarrier(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::NamedBarrierInitialize(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MemoryNamedBarrier(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ModuleProcessed(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ExecutionModeId(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DecorateId(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformElect(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformAll(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformAny(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformAllEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformBroadcast(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformBroadcastFirst(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformBallot(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformInverseBallot(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformBallotBitExtract(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformBallotBitCount(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformBallotFindLSB(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformBallotFindMSB(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformShuffle(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformShuffleXor(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformShuffleUp(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformShuffleDown(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformIAdd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformFAdd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformIMul(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformFMul(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformSMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformUMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformFMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformSMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformUMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformFMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformBitwiseAnd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformBitwiseOr(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformBitwiseXor(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformLogicalAnd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformLogicalOr(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformLogicalXor(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformQuadBroadcast(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformQuadSwap(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CopyLogical(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PtrEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PtrNotEqual(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PtrDiff(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ColorAttachmentReadEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DepthAttachmentReadEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::StencilAttachmentReadEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeTensorARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorReadARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorWriteARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorQuerySizeARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GraphConstantARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GraphEntryPointARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GraphARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GraphInputARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GraphSetOutputARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GraphEndARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeGraphARM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TerminateInvocation(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeUntypedPointerKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UntypedVariableKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UntypedAccessChainKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UntypedInBoundsAccessChainKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupBallotKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupFirstInvocationKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UntypedPtrAccessChainKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UntypedInBoundsPtrAccessChainKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UntypedArrayLengthKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UntypedPrefetchKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FmaKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAllKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAnyKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAllEqualKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformRotateKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupReadInvocationKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ExtInstWithForwardRefsKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UntypedGroupAsyncCopyKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TraceRayKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ExecuteCallableKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertUToAccelerationStructureKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IgnoreIntersectionKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TerminateRayKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SDot(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UDot(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SUDot(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SDotAccSat(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UDotAccSat(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SUDotAccSat(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeCooperativeMatrixKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixLoadKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixStoreKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixMulAddKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixLengthKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantCompositeReplicateEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstantCompositeReplicateEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CompositeConstructReplicateEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeRayQueryKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryInitializeKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryTerminateKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGenerateIntersectionKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryConfirmIntersectionKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryProceedKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetIntersectionTypeKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSampleWeightedQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageBoxFilterQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageBlockMatchSSDQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageBlockMatchSADQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitCastArrayQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageBlockMatchWindowSSDQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageBlockMatchWindowSADQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageBlockMatchGatherSSDQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageBlockMatchGatherSADQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CompositeConstructCoopMatQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CompositeExtractCoopMatQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ExtractSubArrayQCOM(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupIAddNonUniformAMD(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupFAddNonUniformAMD(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupFMinNonUniformAMD(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupUMinNonUniformAMD(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupSMinNonUniformAMD(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupFMaxNonUniformAMD(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupUMaxNonUniformAMD(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupSMaxNonUniformAMD(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FragmentMaskFetchAMD(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FragmentFetchAMD(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReadClockKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AllocateNodePayloadsAMDX(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::EnqueueNodePayloadsAMDX(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeNodePayloadArrayAMDX(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FinishWritingNodePayloadAMDX(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::NodePayloadArrayLengthAMDX(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IsNodePayloadValidAMDX(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantStringAMDX(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstantStringAMDX(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformQuadAllKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformQuadAnyKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeBufferEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BufferPointerEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UntypedImageTexelPointerEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MemberDecorateIdEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantSizeOfEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectRecordHitMotionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectRecordHitWithIndexMotionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectRecordMissMotionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetWorldToObjectNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetObjectToWorldNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetObjectRayDirectionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetObjectRayOriginNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectTraceRayMotionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetShaderRecordBufferHandleNV(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::HitObjectGetShaderBindingTableRecordIndexNV(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::HitObjectRecordEmptyNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectTraceRayNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectRecordHitNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectRecordHitWithIndexNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectRecordMissNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectExecuteShaderNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetCurrentTimeNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetAttributesNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetHitKindNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetPrimitiveIndexNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetGeometryIndexNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetInstanceIdNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetInstanceCustomIndexNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetWorldRayDirectionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetWorldRayOriginNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetRayTMaxNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetRayTMinNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectIsEmptyNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectIsHitNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectIsMissNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReorderThreadWithHitObjectNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReorderThreadWithHintNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeHitObjectNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ImageSampleFootprintNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeVectorIdEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeVectorMatrixMulNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeVectorOuterProductAccumulateNV(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::CooperativeVectorReduceSumAccumulateNV(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::CooperativeVectorMatrixMulAddNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixConvertNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::EmitMeshTasksEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SetMeshOutputsEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupNonUniformPartitionEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::WritePackedPrimitiveIndices4x8NV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FetchMicroTriangleVertexPositionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FetchMicroTriangleVertexBarycentricNV(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::CooperativeVectorLoadNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeVectorStoreNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectRecordFromQueryEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectRecordMissEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectRecordMissMotionEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetIntersectionTriangleVertexPositionsEXT(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::HitObjectGetRayFlagsEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectSetShaderBindingTableRecordIndexEXT(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::HitObjectReorderExecuteShaderEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectTraceReorderExecuteEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectTraceMotionReorderExecuteEXT(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::TypeHitObjectEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReorderThreadWithHintEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReorderThreadWithHitObjectEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectTraceRayEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectTraceRayMotionEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectRecordEmptyEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectExecuteShaderEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetCurrentTimeEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetAttributesEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetHitKindEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetPrimitiveIndexEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetGeometryIndexEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetInstanceIdEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetInstanceCustomIndexEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetObjectRayOriginEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetObjectRayDirectionEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetWorldRayDirectionEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetWorldRayOriginEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetObjectToWorldEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetWorldToObjectEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetRayTMaxEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReportIntersectionKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IgnoreIntersectionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TerminateRayNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TraceNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TraceMotionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TraceRayMotionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetIntersectionTriangleVertexPositionsKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::TypeAccelerationStructureKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ExecuteCallableNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetIntersectionClusterIdNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetClusterIdNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetRayTMinEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetShaderBindingTableRecordIndexEXT(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::HitObjectGetShaderRecordBufferHandleEXT(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::HitObjectIsEmptyEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectIsHitEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectIsMissEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeCooperativeMatrixNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixLoadNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixStoreNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixMulAddNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixLengthNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BeginInvocationInterlockEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::EndInvocationInterlockEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixReduceNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixLoadTensorNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixStoreTensorNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixPerElementOpNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeTensorLayoutNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeTensorViewNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CreateTensorLayoutNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorLayoutSetDimensionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorLayoutSetStrideNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorLayoutSliceNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorLayoutSetClampValueNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CreateTensorViewNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorViewSetDimensionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorViewSetStrideNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DemoteToHelperInvocation(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IsHelperInvocationEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorViewSetClipNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TensorLayoutSetBlockSizeNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CooperativeMatrixTransposeNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertUToImageNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertUToSamplerNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertImageToUNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertSamplerToUNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertUToSampledImageNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertSampledImageToUNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SamplerImageAddressingModeNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RawAccessChainNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetIntersectionSpherePositionNV(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionSphereRadiusNV(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionLSSPositionsNV(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionLSSRadiiNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetIntersectionLSSHitValueNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetSpherePositionNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetSphereRadiusNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetLSSPositionsNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectGetLSSRadiiNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectIsSphereHitNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::HitObjectIsLSSHitNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryIsSphereHitNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryIsLSSHitNV(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupShuffleINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupShuffleDownINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupShuffleUpINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupShuffleXorINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupBlockReadINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupBlockWriteINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupImageBlockReadINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupImageBlockWriteINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupImageMediaBlockReadINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupImageMediaBlockWriteINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UCountLeadingZerosINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UCountTrailingZerosINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AbsISubINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AbsUSubINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IAddSatINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UAddSatINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IAverageINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UAverageINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IAverageRoundedINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UAverageRoundedINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ISubSatINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::USubSatINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IMul32x16INTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UMul32x16INTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantFunctionPointerINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FunctionPointerCallINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AsmTargetINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AsmINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AsmCallINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicFMinEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AtomicFMaxEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AssumeTrueKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ExpectKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::DecorateString(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MemberDecorateString(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::VmeImageINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeVmeImageINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeAvcImePayloadINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeAvcRefPayloadINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeAvcSicPayloadINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeAvcMcePayloadINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeAvcMceResultINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeAvcImeResultINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeAvcImeResultSingleReferenceStreamoutINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::TypeAvcImeResultDualReferenceStreamoutINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::TypeAvcImeSingleReferenceStreaminINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::TypeAvcImeDualReferenceStreaminINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeAvcRefResultINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeAvcSicResultINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceSetInterShapePenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceSetInterDirectionPenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceSetMotionVectorCostFunctionINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceSetAcOnlyHaarINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceConvertToImePayloadINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceConvertToImeResultINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceConvertToRefPayloadINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceConvertToRefResultINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceConvertToSicPayloadINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceConvertToSicResultINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetMotionVectorsINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcMceGetInterDistortionsINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetBestInterDistortionsINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetInterMajorShapeINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetInterMinorShapeINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetInterDirectionsINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetInterMotionVectorCountINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetInterReferenceIdsINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeInitializeINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcImeSetSingleReferenceINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeSetDualReferenceINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcImeRefWindowSizeINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcImeAdjustRefOffsetINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcImeConvertToMcePayloadINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeSetMaxMotionVectorCountINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeSetUnidirectionalMixDisableINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeSetWeightedSadINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcImeEvaluateWithSingleReferenceINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeEvaluateWithDualReferenceINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeConvertToMceResultINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetSingleReferenceStreaminINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetDualReferenceStreaminINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeStripSingleReferenceStreamoutINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeStripDualReferenceStreamoutINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetBorderReachedINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcImeGetTruncatedSearchIndicationINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcFmeInitializeINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcBmeInitializeINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcRefConvertToMcePayloadINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcRefSetBidirectionalMixDisableINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcRefSetBilinearFilterEnableINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcRefEvaluateWithSingleReferenceINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcRefEvaluateWithDualReferenceINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcRefEvaluateWithMultiReferenceINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcRefConvertToMceResultINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicInitializeINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcSicConfigureSkcINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcSicConfigureIpeLumaINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcSicConfigureIpeLumaChromaINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicGetMotionVectorMaskINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicConvertToMcePayloadINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicSetBilinearFilterEnableINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicSetSkcForwardTransformEnableINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicEvaluateIpeINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcSicEvaluateWithSingleReferenceINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicEvaluateWithDualReferenceINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicEvaluateWithMultiReferenceINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicConvertToMceResultINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicGetIpeLumaShapeINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcSicGetBestIpeLumaDistortionINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicGetBestIpeChromaDistortionINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicGetPackedIpeLumaModesINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicGetIpeChromaModeINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::SubgroupAvcSicGetInterRawSadsINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::VariableLengthArrayINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SaveMemoryINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RestoreMemoryINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatSinCosPiALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatCastALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatCastFromIntALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatCastToIntALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatAddALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatSubALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatMulALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatDivALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatGTALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatGEALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatLTALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatLEALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatEQALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatRecipALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatRSqrtALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatCbrtALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatHypotALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatSqrtALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatLogINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatLog2INTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatLog10INTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatLog1pINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatExpINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatExp2INTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatExp10INTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatExpm1INTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatSinINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatCosINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatSinCosINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatSinPiINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatCosPiINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatASinINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatASinPiINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatACosINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatACosPiINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatATanINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatATanPiINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatATan2INTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatPowINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatPowRINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArbitraryFloatPowNINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::LoopControlINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AliasDomainDeclINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AliasScopeDeclINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::AliasScopeListDeclINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedSqrtALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedRecipALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedRsqrtALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedSinALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedCosALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedSinCosALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedSinPiALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedCosPiALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedSinCosPiALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedLogALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FixedExpALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PtrCastToCrossWorkgroupALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CrossWorkgroupCastToPtrALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ReadPipeBlockingALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::WritePipeBlockingALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FPGARegALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetRayTMinKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetRayFlagsKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetIntersectionTKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetIntersectionInstanceCustomIndexKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionInstanceIdKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionGeometryIndexKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionPrimitiveIndexKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionBarycentricsKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionFrontFaceKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetIntersectionCandidateAABBOpaqueKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionObjectRayDirectionKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionObjectRayOriginKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetWorldRayDirectionKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetWorldRayOriginKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RayQueryGetIntersectionObjectToWorldKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::RayQueryGetIntersectionWorldToObjectKHR(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::AtomicFAddEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeBufferSurfaceINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeStructContinuedINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConstantCompositeContinuedINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstantCompositeContinuedINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CompositeConstructContinuedINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertFToBF16INTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertBF16ToFINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ControlBarrierArriveINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ControlBarrierWaitINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ArithmeticFenceEXT(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TaskSequenceCreateALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TaskSequenceAsyncALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TaskSequenceGetALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TaskSequenceReleaseALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::TypeTaskSequenceALTERA(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupBlockPrefetchINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Subgroup2DBlockLoadINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Subgroup2DBlockLoadTransformINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Subgroup2DBlockLoadTransposeINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Subgroup2DBlockPrefetchINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Subgroup2DBlockStoreINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SubgroupMatrixMultiplyAccumulateINTEL(inst) => {
                InstEncoding::dis_fmt(inst, f, ctx)
            }
            Self::BitwiseFunctionINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UntypedVariableLengthArrayINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConditionalExtensionINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConditionalEntryPointINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConditionalCapabilityINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstantTargetINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstantArchitectureINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SpecConstantCapabilitiesINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConditionalCopyObjectINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupIMulKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupFMulKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupBitwiseAndKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupBitwiseOrKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupBitwiseXorKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupLogicalAndKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupLogicalOrKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GroupLogicalXorKHR(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RoundFToTF32INTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MaskedGatherINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MaskedScatterINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertHandleToImageINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertHandleToSamplerINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertHandleToSampledImageINTEL(inst) => InstEncoding::dis_fmt(inst, f, ctx),
        }
    }
}
impl From<OpNop> for CoreInstSet {
    fn from(inst: OpNop) -> Self {
        Self::Nop(inst)
    }
}
impl From<OpUndef> for CoreInstSet {
    fn from(inst: OpUndef) -> Self {
        Self::Undef(inst)
    }
}
impl From<OpSourceContinued> for CoreInstSet {
    fn from(inst: OpSourceContinued) -> Self {
        Self::SourceContinued(inst)
    }
}
impl From<OpSource> for CoreInstSet {
    fn from(inst: OpSource) -> Self {
        Self::Source(inst)
    }
}
impl From<OpSourceExtension> for CoreInstSet {
    fn from(inst: OpSourceExtension) -> Self {
        Self::SourceExtension(inst)
    }
}
impl From<OpName> for CoreInstSet {
    fn from(inst: OpName) -> Self {
        Self::Name(inst)
    }
}
impl From<OpMemberName> for CoreInstSet {
    fn from(inst: OpMemberName) -> Self {
        Self::MemberName(inst)
    }
}
impl From<OpString> for CoreInstSet {
    fn from(inst: OpString) -> Self {
        Self::String(inst)
    }
}
impl From<OpLine> for CoreInstSet {
    fn from(inst: OpLine) -> Self {
        Self::Line(inst)
    }
}
impl From<OpExtension> for CoreInstSet {
    fn from(inst: OpExtension) -> Self {
        Self::Extension(inst)
    }
}
impl From<OpExtInstImport> for CoreInstSet {
    fn from(inst: OpExtInstImport) -> Self {
        Self::ExtInstImport(inst)
    }
}
impl From<OpExtInst> for CoreInstSet {
    fn from(inst: OpExtInst) -> Self {
        Self::ExtInst(inst)
    }
}
impl From<OpMemoryModel> for CoreInstSet {
    fn from(inst: OpMemoryModel) -> Self {
        Self::MemoryModel(inst)
    }
}
impl From<OpEntryPoint> for CoreInstSet {
    fn from(inst: OpEntryPoint) -> Self {
        Self::EntryPoint(inst)
    }
}
impl From<OpExecutionMode> for CoreInstSet {
    fn from(inst: OpExecutionMode) -> Self {
        Self::ExecutionMode(inst)
    }
}
impl From<OpCapability> for CoreInstSet {
    fn from(inst: OpCapability) -> Self {
        Self::Capability(inst)
    }
}
impl From<OpTypeVoid> for CoreInstSet {
    fn from(inst: OpTypeVoid) -> Self {
        Self::TypeVoid(inst)
    }
}
impl From<OpTypeBool> for CoreInstSet {
    fn from(inst: OpTypeBool) -> Self {
        Self::TypeBool(inst)
    }
}
impl From<OpTypeInt> for CoreInstSet {
    fn from(inst: OpTypeInt) -> Self {
        Self::TypeInt(inst)
    }
}
impl From<OpTypeFloat> for CoreInstSet {
    fn from(inst: OpTypeFloat) -> Self {
        Self::TypeFloat(inst)
    }
}
impl From<OpTypeVector> for CoreInstSet {
    fn from(inst: OpTypeVector) -> Self {
        Self::TypeVector(inst)
    }
}
impl From<OpTypeMatrix> for CoreInstSet {
    fn from(inst: OpTypeMatrix) -> Self {
        Self::TypeMatrix(inst)
    }
}
impl From<OpTypeImage> for CoreInstSet {
    fn from(inst: OpTypeImage) -> Self {
        Self::TypeImage(inst)
    }
}
impl From<OpTypeSampler> for CoreInstSet {
    fn from(inst: OpTypeSampler) -> Self {
        Self::TypeSampler(inst)
    }
}
impl From<OpTypeSampledImage> for CoreInstSet {
    fn from(inst: OpTypeSampledImage) -> Self {
        Self::TypeSampledImage(inst)
    }
}
impl From<OpTypeArray> for CoreInstSet {
    fn from(inst: OpTypeArray) -> Self {
        Self::TypeArray(inst)
    }
}
impl From<OpTypeRuntimeArray> for CoreInstSet {
    fn from(inst: OpTypeRuntimeArray) -> Self {
        Self::TypeRuntimeArray(inst)
    }
}
impl From<OpTypeStruct> for CoreInstSet {
    fn from(inst: OpTypeStruct) -> Self {
        Self::TypeStruct(inst)
    }
}
impl From<OpTypeOpaque> for CoreInstSet {
    fn from(inst: OpTypeOpaque) -> Self {
        Self::TypeOpaque(inst)
    }
}
impl From<OpTypePointer> for CoreInstSet {
    fn from(inst: OpTypePointer) -> Self {
        Self::TypePointer(inst)
    }
}
impl From<OpTypeFunction> for CoreInstSet {
    fn from(inst: OpTypeFunction) -> Self {
        Self::TypeFunction(inst)
    }
}
impl From<OpTypeEvent> for CoreInstSet {
    fn from(inst: OpTypeEvent) -> Self {
        Self::TypeEvent(inst)
    }
}
impl From<OpTypeDeviceEvent> for CoreInstSet {
    fn from(inst: OpTypeDeviceEvent) -> Self {
        Self::TypeDeviceEvent(inst)
    }
}
impl From<OpTypeReserveId> for CoreInstSet {
    fn from(inst: OpTypeReserveId) -> Self {
        Self::TypeReserveId(inst)
    }
}
impl From<OpTypeQueue> for CoreInstSet {
    fn from(inst: OpTypeQueue) -> Self {
        Self::TypeQueue(inst)
    }
}
impl From<OpTypePipe> for CoreInstSet {
    fn from(inst: OpTypePipe) -> Self {
        Self::TypePipe(inst)
    }
}
impl From<OpTypeForwardPointer> for CoreInstSet {
    fn from(inst: OpTypeForwardPointer) -> Self {
        Self::TypeForwardPointer(inst)
    }
}
impl From<OpConstantTrue> for CoreInstSet {
    fn from(inst: OpConstantTrue) -> Self {
        Self::ConstantTrue(inst)
    }
}
impl From<OpConstantFalse> for CoreInstSet {
    fn from(inst: OpConstantFalse) -> Self {
        Self::ConstantFalse(inst)
    }
}
impl From<OpConstant> for CoreInstSet {
    fn from(inst: OpConstant) -> Self {
        Self::Constant(inst)
    }
}
impl From<OpConstantComposite> for CoreInstSet {
    fn from(inst: OpConstantComposite) -> Self {
        Self::ConstantComposite(inst)
    }
}
impl From<OpConstantSampler> for CoreInstSet {
    fn from(inst: OpConstantSampler) -> Self {
        Self::ConstantSampler(inst)
    }
}
impl From<OpConstantNull> for CoreInstSet {
    fn from(inst: OpConstantNull) -> Self {
        Self::ConstantNull(inst)
    }
}
impl From<OpSpecConstantTrue> for CoreInstSet {
    fn from(inst: OpSpecConstantTrue) -> Self {
        Self::SpecConstantTrue(inst)
    }
}
impl From<OpSpecConstantFalse> for CoreInstSet {
    fn from(inst: OpSpecConstantFalse) -> Self {
        Self::SpecConstantFalse(inst)
    }
}
impl From<OpSpecConstant> for CoreInstSet {
    fn from(inst: OpSpecConstant) -> Self {
        Self::SpecConstant(inst)
    }
}
impl From<OpSpecConstantComposite> for CoreInstSet {
    fn from(inst: OpSpecConstantComposite) -> Self {
        Self::SpecConstantComposite(inst)
    }
}
impl From<OpSpecConstantOp> for CoreInstSet {
    fn from(inst: OpSpecConstantOp) -> Self {
        Self::SpecConstantOp(inst)
    }
}
impl From<OpFunction> for CoreInstSet {
    fn from(inst: OpFunction) -> Self {
        Self::Function(inst)
    }
}
impl From<OpFunctionParameter> for CoreInstSet {
    fn from(inst: OpFunctionParameter) -> Self {
        Self::FunctionParameter(inst)
    }
}
impl From<OpFunctionEnd> for CoreInstSet {
    fn from(inst: OpFunctionEnd) -> Self {
        Self::FunctionEnd(inst)
    }
}
impl From<OpFunctionCall> for CoreInstSet {
    fn from(inst: OpFunctionCall) -> Self {
        Self::FunctionCall(inst)
    }
}
impl From<OpVariable> for CoreInstSet {
    fn from(inst: OpVariable) -> Self {
        Self::Variable(inst)
    }
}
impl From<OpImageTexelPointer> for CoreInstSet {
    fn from(inst: OpImageTexelPointer) -> Self {
        Self::ImageTexelPointer(inst)
    }
}
impl From<OpLoad> for CoreInstSet {
    fn from(inst: OpLoad) -> Self {
        Self::Load(inst)
    }
}
impl From<OpStore> for CoreInstSet {
    fn from(inst: OpStore) -> Self {
        Self::Store(inst)
    }
}
impl From<OpCopyMemory> for CoreInstSet {
    fn from(inst: OpCopyMemory) -> Self {
        Self::CopyMemory(inst)
    }
}
impl From<OpCopyMemorySized> for CoreInstSet {
    fn from(inst: OpCopyMemorySized) -> Self {
        Self::CopyMemorySized(inst)
    }
}
impl From<OpAccessChain> for CoreInstSet {
    fn from(inst: OpAccessChain) -> Self {
        Self::AccessChain(inst)
    }
}
impl From<OpInBoundsAccessChain> for CoreInstSet {
    fn from(inst: OpInBoundsAccessChain) -> Self {
        Self::InBoundsAccessChain(inst)
    }
}
impl From<OpPtrAccessChain> for CoreInstSet {
    fn from(inst: OpPtrAccessChain) -> Self {
        Self::PtrAccessChain(inst)
    }
}
impl From<OpArrayLength> for CoreInstSet {
    fn from(inst: OpArrayLength) -> Self {
        Self::ArrayLength(inst)
    }
}
impl From<OpGenericPtrMemSemantics> for CoreInstSet {
    fn from(inst: OpGenericPtrMemSemantics) -> Self {
        Self::GenericPtrMemSemantics(inst)
    }
}
impl From<OpInBoundsPtrAccessChain> for CoreInstSet {
    fn from(inst: OpInBoundsPtrAccessChain) -> Self {
        Self::InBoundsPtrAccessChain(inst)
    }
}
impl From<OpDecorate> for CoreInstSet {
    fn from(inst: OpDecorate) -> Self {
        Self::Decorate(inst)
    }
}
impl From<OpMemberDecorate> for CoreInstSet {
    fn from(inst: OpMemberDecorate) -> Self {
        Self::MemberDecorate(inst)
    }
}
impl From<OpDecorationGroup> for CoreInstSet {
    fn from(inst: OpDecorationGroup) -> Self {
        Self::DecorationGroup(inst)
    }
}
impl From<OpGroupDecorate> for CoreInstSet {
    fn from(inst: OpGroupDecorate) -> Self {
        Self::GroupDecorate(inst)
    }
}
impl From<OpGroupMemberDecorate> for CoreInstSet {
    fn from(inst: OpGroupMemberDecorate) -> Self {
        Self::GroupMemberDecorate(inst)
    }
}
impl From<OpVectorExtractDynamic> for CoreInstSet {
    fn from(inst: OpVectorExtractDynamic) -> Self {
        Self::VectorExtractDynamic(inst)
    }
}
impl From<OpVectorInsertDynamic> for CoreInstSet {
    fn from(inst: OpVectorInsertDynamic) -> Self {
        Self::VectorInsertDynamic(inst)
    }
}
impl From<OpVectorShuffle> for CoreInstSet {
    fn from(inst: OpVectorShuffle) -> Self {
        Self::VectorShuffle(inst)
    }
}
impl From<OpCompositeConstruct> for CoreInstSet {
    fn from(inst: OpCompositeConstruct) -> Self {
        Self::CompositeConstruct(inst)
    }
}
impl From<OpCompositeExtract> for CoreInstSet {
    fn from(inst: OpCompositeExtract) -> Self {
        Self::CompositeExtract(inst)
    }
}
impl From<OpCompositeInsert> for CoreInstSet {
    fn from(inst: OpCompositeInsert) -> Self {
        Self::CompositeInsert(inst)
    }
}
impl From<OpCopyObject> for CoreInstSet {
    fn from(inst: OpCopyObject) -> Self {
        Self::CopyObject(inst)
    }
}
impl From<OpTranspose> for CoreInstSet {
    fn from(inst: OpTranspose) -> Self {
        Self::Transpose(inst)
    }
}
impl From<OpSampledImage> for CoreInstSet {
    fn from(inst: OpSampledImage) -> Self {
        Self::SampledImage(inst)
    }
}
impl From<OpImageSampleImplicitLod> for CoreInstSet {
    fn from(inst: OpImageSampleImplicitLod) -> Self {
        Self::ImageSampleImplicitLod(inst)
    }
}
impl From<OpImageSampleExplicitLod> for CoreInstSet {
    fn from(inst: OpImageSampleExplicitLod) -> Self {
        Self::ImageSampleExplicitLod(inst)
    }
}
impl From<OpImageSampleDrefImplicitLod> for CoreInstSet {
    fn from(inst: OpImageSampleDrefImplicitLod) -> Self {
        Self::ImageSampleDrefImplicitLod(inst)
    }
}
impl From<OpImageSampleDrefExplicitLod> for CoreInstSet {
    fn from(inst: OpImageSampleDrefExplicitLod) -> Self {
        Self::ImageSampleDrefExplicitLod(inst)
    }
}
impl From<OpImageSampleProjImplicitLod> for CoreInstSet {
    fn from(inst: OpImageSampleProjImplicitLod) -> Self {
        Self::ImageSampleProjImplicitLod(inst)
    }
}
impl From<OpImageSampleProjExplicitLod> for CoreInstSet {
    fn from(inst: OpImageSampleProjExplicitLod) -> Self {
        Self::ImageSampleProjExplicitLod(inst)
    }
}
impl From<OpImageSampleProjDrefImplicitLod> for CoreInstSet {
    fn from(inst: OpImageSampleProjDrefImplicitLod) -> Self {
        Self::ImageSampleProjDrefImplicitLod(inst)
    }
}
impl From<OpImageSampleProjDrefExplicitLod> for CoreInstSet {
    fn from(inst: OpImageSampleProjDrefExplicitLod) -> Self {
        Self::ImageSampleProjDrefExplicitLod(inst)
    }
}
impl From<OpImageFetch> for CoreInstSet {
    fn from(inst: OpImageFetch) -> Self {
        Self::ImageFetch(inst)
    }
}
impl From<OpImageGather> for CoreInstSet {
    fn from(inst: OpImageGather) -> Self {
        Self::ImageGather(inst)
    }
}
impl From<OpImageDrefGather> for CoreInstSet {
    fn from(inst: OpImageDrefGather) -> Self {
        Self::ImageDrefGather(inst)
    }
}
impl From<OpImageRead> for CoreInstSet {
    fn from(inst: OpImageRead) -> Self {
        Self::ImageRead(inst)
    }
}
impl From<OpImageWrite> for CoreInstSet {
    fn from(inst: OpImageWrite) -> Self {
        Self::ImageWrite(inst)
    }
}
impl From<OpImage> for CoreInstSet {
    fn from(inst: OpImage) -> Self {
        Self::Image(inst)
    }
}
impl From<OpImageQueryFormat> for CoreInstSet {
    fn from(inst: OpImageQueryFormat) -> Self {
        Self::ImageQueryFormat(inst)
    }
}
impl From<OpImageQueryOrder> for CoreInstSet {
    fn from(inst: OpImageQueryOrder) -> Self {
        Self::ImageQueryOrder(inst)
    }
}
impl From<OpImageQuerySizeLod> for CoreInstSet {
    fn from(inst: OpImageQuerySizeLod) -> Self {
        Self::ImageQuerySizeLod(inst)
    }
}
impl From<OpImageQuerySize> for CoreInstSet {
    fn from(inst: OpImageQuerySize) -> Self {
        Self::ImageQuerySize(inst)
    }
}
impl From<OpImageQueryLod> for CoreInstSet {
    fn from(inst: OpImageQueryLod) -> Self {
        Self::ImageQueryLod(inst)
    }
}
impl From<OpImageQueryLevels> for CoreInstSet {
    fn from(inst: OpImageQueryLevels) -> Self {
        Self::ImageQueryLevels(inst)
    }
}
impl From<OpImageQuerySamples> for CoreInstSet {
    fn from(inst: OpImageQuerySamples) -> Self {
        Self::ImageQuerySamples(inst)
    }
}
impl From<OpConvertFToU> for CoreInstSet {
    fn from(inst: OpConvertFToU) -> Self {
        Self::ConvertFToU(inst)
    }
}
impl From<OpConvertFToS> for CoreInstSet {
    fn from(inst: OpConvertFToS) -> Self {
        Self::ConvertFToS(inst)
    }
}
impl From<OpConvertSToF> for CoreInstSet {
    fn from(inst: OpConvertSToF) -> Self {
        Self::ConvertSToF(inst)
    }
}
impl From<OpConvertUToF> for CoreInstSet {
    fn from(inst: OpConvertUToF) -> Self {
        Self::ConvertUToF(inst)
    }
}
impl From<OpUConvert> for CoreInstSet {
    fn from(inst: OpUConvert) -> Self {
        Self::UConvert(inst)
    }
}
impl From<OpSConvert> for CoreInstSet {
    fn from(inst: OpSConvert) -> Self {
        Self::SConvert(inst)
    }
}
impl From<OpFConvert> for CoreInstSet {
    fn from(inst: OpFConvert) -> Self {
        Self::FConvert(inst)
    }
}
impl From<OpQuantizeToF16> for CoreInstSet {
    fn from(inst: OpQuantizeToF16) -> Self {
        Self::QuantizeToF16(inst)
    }
}
impl From<OpConvertPtrToU> for CoreInstSet {
    fn from(inst: OpConvertPtrToU) -> Self {
        Self::ConvertPtrToU(inst)
    }
}
impl From<OpSatConvertSToU> for CoreInstSet {
    fn from(inst: OpSatConvertSToU) -> Self {
        Self::SatConvertSToU(inst)
    }
}
impl From<OpSatConvertUToS> for CoreInstSet {
    fn from(inst: OpSatConvertUToS) -> Self {
        Self::SatConvertUToS(inst)
    }
}
impl From<OpConvertUToPtr> for CoreInstSet {
    fn from(inst: OpConvertUToPtr) -> Self {
        Self::ConvertUToPtr(inst)
    }
}
impl From<OpPtrCastToGeneric> for CoreInstSet {
    fn from(inst: OpPtrCastToGeneric) -> Self {
        Self::PtrCastToGeneric(inst)
    }
}
impl From<OpGenericCastToPtr> for CoreInstSet {
    fn from(inst: OpGenericCastToPtr) -> Self {
        Self::GenericCastToPtr(inst)
    }
}
impl From<OpGenericCastToPtrExplicit> for CoreInstSet {
    fn from(inst: OpGenericCastToPtrExplicit) -> Self {
        Self::GenericCastToPtrExplicit(inst)
    }
}
impl From<OpBitcast> for CoreInstSet {
    fn from(inst: OpBitcast) -> Self {
        Self::Bitcast(inst)
    }
}
impl From<OpSNegate> for CoreInstSet {
    fn from(inst: OpSNegate) -> Self {
        Self::SNegate(inst)
    }
}
impl From<OpFNegate> for CoreInstSet {
    fn from(inst: OpFNegate) -> Self {
        Self::FNegate(inst)
    }
}
impl From<OpIAdd> for CoreInstSet {
    fn from(inst: OpIAdd) -> Self {
        Self::IAdd(inst)
    }
}
impl From<OpFAdd> for CoreInstSet {
    fn from(inst: OpFAdd) -> Self {
        Self::FAdd(inst)
    }
}
impl From<OpISub> for CoreInstSet {
    fn from(inst: OpISub) -> Self {
        Self::ISub(inst)
    }
}
impl From<OpFSub> for CoreInstSet {
    fn from(inst: OpFSub) -> Self {
        Self::FSub(inst)
    }
}
impl From<OpIMul> for CoreInstSet {
    fn from(inst: OpIMul) -> Self {
        Self::IMul(inst)
    }
}
impl From<OpFMul> for CoreInstSet {
    fn from(inst: OpFMul) -> Self {
        Self::FMul(inst)
    }
}
impl From<OpUDiv> for CoreInstSet {
    fn from(inst: OpUDiv) -> Self {
        Self::UDiv(inst)
    }
}
impl From<OpSDiv> for CoreInstSet {
    fn from(inst: OpSDiv) -> Self {
        Self::SDiv(inst)
    }
}
impl From<OpFDiv> for CoreInstSet {
    fn from(inst: OpFDiv) -> Self {
        Self::FDiv(inst)
    }
}
impl From<OpUMod> for CoreInstSet {
    fn from(inst: OpUMod) -> Self {
        Self::UMod(inst)
    }
}
impl From<OpSRem> for CoreInstSet {
    fn from(inst: OpSRem) -> Self {
        Self::SRem(inst)
    }
}
impl From<OpSMod> for CoreInstSet {
    fn from(inst: OpSMod) -> Self {
        Self::SMod(inst)
    }
}
impl From<OpFRem> for CoreInstSet {
    fn from(inst: OpFRem) -> Self {
        Self::FRem(inst)
    }
}
impl From<OpFMod> for CoreInstSet {
    fn from(inst: OpFMod) -> Self {
        Self::FMod(inst)
    }
}
impl From<OpVectorTimesScalar> for CoreInstSet {
    fn from(inst: OpVectorTimesScalar) -> Self {
        Self::VectorTimesScalar(inst)
    }
}
impl From<OpMatrixTimesScalar> for CoreInstSet {
    fn from(inst: OpMatrixTimesScalar) -> Self {
        Self::MatrixTimesScalar(inst)
    }
}
impl From<OpVectorTimesMatrix> for CoreInstSet {
    fn from(inst: OpVectorTimesMatrix) -> Self {
        Self::VectorTimesMatrix(inst)
    }
}
impl From<OpMatrixTimesVector> for CoreInstSet {
    fn from(inst: OpMatrixTimesVector) -> Self {
        Self::MatrixTimesVector(inst)
    }
}
impl From<OpMatrixTimesMatrix> for CoreInstSet {
    fn from(inst: OpMatrixTimesMatrix) -> Self {
        Self::MatrixTimesMatrix(inst)
    }
}
impl From<OpOuterProduct> for CoreInstSet {
    fn from(inst: OpOuterProduct) -> Self {
        Self::OuterProduct(inst)
    }
}
impl From<OpDot> for CoreInstSet {
    fn from(inst: OpDot) -> Self {
        Self::Dot(inst)
    }
}
impl From<OpIAddCarry> for CoreInstSet {
    fn from(inst: OpIAddCarry) -> Self {
        Self::IAddCarry(inst)
    }
}
impl From<OpISubBorrow> for CoreInstSet {
    fn from(inst: OpISubBorrow) -> Self {
        Self::ISubBorrow(inst)
    }
}
impl From<OpUMulExtended> for CoreInstSet {
    fn from(inst: OpUMulExtended) -> Self {
        Self::UMulExtended(inst)
    }
}
impl From<OpSMulExtended> for CoreInstSet {
    fn from(inst: OpSMulExtended) -> Self {
        Self::SMulExtended(inst)
    }
}
impl From<OpAny> for CoreInstSet {
    fn from(inst: OpAny) -> Self {
        Self::Any(inst)
    }
}
impl From<OpAll> for CoreInstSet {
    fn from(inst: OpAll) -> Self {
        Self::All(inst)
    }
}
impl From<OpIsNan> for CoreInstSet {
    fn from(inst: OpIsNan) -> Self {
        Self::IsNan(inst)
    }
}
impl From<OpIsInf> for CoreInstSet {
    fn from(inst: OpIsInf) -> Self {
        Self::IsInf(inst)
    }
}
impl From<OpIsFinite> for CoreInstSet {
    fn from(inst: OpIsFinite) -> Self {
        Self::IsFinite(inst)
    }
}
impl From<OpIsNormal> for CoreInstSet {
    fn from(inst: OpIsNormal) -> Self {
        Self::IsNormal(inst)
    }
}
impl From<OpSignBitSet> for CoreInstSet {
    fn from(inst: OpSignBitSet) -> Self {
        Self::SignBitSet(inst)
    }
}
impl From<OpLessOrGreater> for CoreInstSet {
    fn from(inst: OpLessOrGreater) -> Self {
        Self::LessOrGreater(inst)
    }
}
impl From<OpOrdered> for CoreInstSet {
    fn from(inst: OpOrdered) -> Self {
        Self::Ordered(inst)
    }
}
impl From<OpUnordered> for CoreInstSet {
    fn from(inst: OpUnordered) -> Self {
        Self::Unordered(inst)
    }
}
impl From<OpLogicalEqual> for CoreInstSet {
    fn from(inst: OpLogicalEqual) -> Self {
        Self::LogicalEqual(inst)
    }
}
impl From<OpLogicalNotEqual> for CoreInstSet {
    fn from(inst: OpLogicalNotEqual) -> Self {
        Self::LogicalNotEqual(inst)
    }
}
impl From<OpLogicalOr> for CoreInstSet {
    fn from(inst: OpLogicalOr) -> Self {
        Self::LogicalOr(inst)
    }
}
impl From<OpLogicalAnd> for CoreInstSet {
    fn from(inst: OpLogicalAnd) -> Self {
        Self::LogicalAnd(inst)
    }
}
impl From<OpLogicalNot> for CoreInstSet {
    fn from(inst: OpLogicalNot) -> Self {
        Self::LogicalNot(inst)
    }
}
impl From<OpSelect> for CoreInstSet {
    fn from(inst: OpSelect) -> Self {
        Self::Select(inst)
    }
}
impl From<OpIEqual> for CoreInstSet {
    fn from(inst: OpIEqual) -> Self {
        Self::IEqual(inst)
    }
}
impl From<OpINotEqual> for CoreInstSet {
    fn from(inst: OpINotEqual) -> Self {
        Self::INotEqual(inst)
    }
}
impl From<OpUGreaterThan> for CoreInstSet {
    fn from(inst: OpUGreaterThan) -> Self {
        Self::UGreaterThan(inst)
    }
}
impl From<OpSGreaterThan> for CoreInstSet {
    fn from(inst: OpSGreaterThan) -> Self {
        Self::SGreaterThan(inst)
    }
}
impl From<OpUGreaterThanEqual> for CoreInstSet {
    fn from(inst: OpUGreaterThanEqual) -> Self {
        Self::UGreaterThanEqual(inst)
    }
}
impl From<OpSGreaterThanEqual> for CoreInstSet {
    fn from(inst: OpSGreaterThanEqual) -> Self {
        Self::SGreaterThanEqual(inst)
    }
}
impl From<OpULessThan> for CoreInstSet {
    fn from(inst: OpULessThan) -> Self {
        Self::ULessThan(inst)
    }
}
impl From<OpSLessThan> for CoreInstSet {
    fn from(inst: OpSLessThan) -> Self {
        Self::SLessThan(inst)
    }
}
impl From<OpULessThanEqual> for CoreInstSet {
    fn from(inst: OpULessThanEqual) -> Self {
        Self::ULessThanEqual(inst)
    }
}
impl From<OpSLessThanEqual> for CoreInstSet {
    fn from(inst: OpSLessThanEqual) -> Self {
        Self::SLessThanEqual(inst)
    }
}
impl From<OpFOrdEqual> for CoreInstSet {
    fn from(inst: OpFOrdEqual) -> Self {
        Self::FOrdEqual(inst)
    }
}
impl From<OpFUnordEqual> for CoreInstSet {
    fn from(inst: OpFUnordEqual) -> Self {
        Self::FUnordEqual(inst)
    }
}
impl From<OpFOrdNotEqual> for CoreInstSet {
    fn from(inst: OpFOrdNotEqual) -> Self {
        Self::FOrdNotEqual(inst)
    }
}
impl From<OpFUnordNotEqual> for CoreInstSet {
    fn from(inst: OpFUnordNotEqual) -> Self {
        Self::FUnordNotEqual(inst)
    }
}
impl From<OpFOrdLessThan> for CoreInstSet {
    fn from(inst: OpFOrdLessThan) -> Self {
        Self::FOrdLessThan(inst)
    }
}
impl From<OpFUnordLessThan> for CoreInstSet {
    fn from(inst: OpFUnordLessThan) -> Self {
        Self::FUnordLessThan(inst)
    }
}
impl From<OpFOrdGreaterThan> for CoreInstSet {
    fn from(inst: OpFOrdGreaterThan) -> Self {
        Self::FOrdGreaterThan(inst)
    }
}
impl From<OpFUnordGreaterThan> for CoreInstSet {
    fn from(inst: OpFUnordGreaterThan) -> Self {
        Self::FUnordGreaterThan(inst)
    }
}
impl From<OpFOrdLessThanEqual> for CoreInstSet {
    fn from(inst: OpFOrdLessThanEqual) -> Self {
        Self::FOrdLessThanEqual(inst)
    }
}
impl From<OpFUnordLessThanEqual> for CoreInstSet {
    fn from(inst: OpFUnordLessThanEqual) -> Self {
        Self::FUnordLessThanEqual(inst)
    }
}
impl From<OpFOrdGreaterThanEqual> for CoreInstSet {
    fn from(inst: OpFOrdGreaterThanEqual) -> Self {
        Self::FOrdGreaterThanEqual(inst)
    }
}
impl From<OpFUnordGreaterThanEqual> for CoreInstSet {
    fn from(inst: OpFUnordGreaterThanEqual) -> Self {
        Self::FUnordGreaterThanEqual(inst)
    }
}
impl From<OpShiftRightLogical> for CoreInstSet {
    fn from(inst: OpShiftRightLogical) -> Self {
        Self::ShiftRightLogical(inst)
    }
}
impl From<OpShiftRightArithmetic> for CoreInstSet {
    fn from(inst: OpShiftRightArithmetic) -> Self {
        Self::ShiftRightArithmetic(inst)
    }
}
impl From<OpShiftLeftLogical> for CoreInstSet {
    fn from(inst: OpShiftLeftLogical) -> Self {
        Self::ShiftLeftLogical(inst)
    }
}
impl From<OpBitwiseOr> for CoreInstSet {
    fn from(inst: OpBitwiseOr) -> Self {
        Self::BitwiseOr(inst)
    }
}
impl From<OpBitwiseXor> for CoreInstSet {
    fn from(inst: OpBitwiseXor) -> Self {
        Self::BitwiseXor(inst)
    }
}
impl From<OpBitwiseAnd> for CoreInstSet {
    fn from(inst: OpBitwiseAnd) -> Self {
        Self::BitwiseAnd(inst)
    }
}
impl From<OpNot> for CoreInstSet {
    fn from(inst: OpNot) -> Self {
        Self::Not(inst)
    }
}
impl From<OpBitFieldInsert> for CoreInstSet {
    fn from(inst: OpBitFieldInsert) -> Self {
        Self::BitFieldInsert(inst)
    }
}
impl From<OpBitFieldSExtract> for CoreInstSet {
    fn from(inst: OpBitFieldSExtract) -> Self {
        Self::BitFieldSExtract(inst)
    }
}
impl From<OpBitFieldUExtract> for CoreInstSet {
    fn from(inst: OpBitFieldUExtract) -> Self {
        Self::BitFieldUExtract(inst)
    }
}
impl From<OpBitReverse> for CoreInstSet {
    fn from(inst: OpBitReverse) -> Self {
        Self::BitReverse(inst)
    }
}
impl From<OpBitCount> for CoreInstSet {
    fn from(inst: OpBitCount) -> Self {
        Self::BitCount(inst)
    }
}
impl From<OpDPdx> for CoreInstSet {
    fn from(inst: OpDPdx) -> Self {
        Self::DPdx(inst)
    }
}
impl From<OpDPdy> for CoreInstSet {
    fn from(inst: OpDPdy) -> Self {
        Self::DPdy(inst)
    }
}
impl From<OpFwidth> for CoreInstSet {
    fn from(inst: OpFwidth) -> Self {
        Self::Fwidth(inst)
    }
}
impl From<OpDPdxFine> for CoreInstSet {
    fn from(inst: OpDPdxFine) -> Self {
        Self::DPdxFine(inst)
    }
}
impl From<OpDPdyFine> for CoreInstSet {
    fn from(inst: OpDPdyFine) -> Self {
        Self::DPdyFine(inst)
    }
}
impl From<OpFwidthFine> for CoreInstSet {
    fn from(inst: OpFwidthFine) -> Self {
        Self::FwidthFine(inst)
    }
}
impl From<OpDPdxCoarse> for CoreInstSet {
    fn from(inst: OpDPdxCoarse) -> Self {
        Self::DPdxCoarse(inst)
    }
}
impl From<OpDPdyCoarse> for CoreInstSet {
    fn from(inst: OpDPdyCoarse) -> Self {
        Self::DPdyCoarse(inst)
    }
}
impl From<OpFwidthCoarse> for CoreInstSet {
    fn from(inst: OpFwidthCoarse) -> Self {
        Self::FwidthCoarse(inst)
    }
}
impl From<OpEmitVertex> for CoreInstSet {
    fn from(inst: OpEmitVertex) -> Self {
        Self::EmitVertex(inst)
    }
}
impl From<OpEndPrimitive> for CoreInstSet {
    fn from(inst: OpEndPrimitive) -> Self {
        Self::EndPrimitive(inst)
    }
}
impl From<OpEmitStreamVertex> for CoreInstSet {
    fn from(inst: OpEmitStreamVertex) -> Self {
        Self::EmitStreamVertex(inst)
    }
}
impl From<OpEndStreamPrimitive> for CoreInstSet {
    fn from(inst: OpEndStreamPrimitive) -> Self {
        Self::EndStreamPrimitive(inst)
    }
}
impl From<OpControlBarrier> for CoreInstSet {
    fn from(inst: OpControlBarrier) -> Self {
        Self::ControlBarrier(inst)
    }
}
impl From<OpMemoryBarrier> for CoreInstSet {
    fn from(inst: OpMemoryBarrier) -> Self {
        Self::MemoryBarrier(inst)
    }
}
impl From<OpAtomicLoad> for CoreInstSet {
    fn from(inst: OpAtomicLoad) -> Self {
        Self::AtomicLoad(inst)
    }
}
impl From<OpAtomicStore> for CoreInstSet {
    fn from(inst: OpAtomicStore) -> Self {
        Self::AtomicStore(inst)
    }
}
impl From<OpAtomicExchange> for CoreInstSet {
    fn from(inst: OpAtomicExchange) -> Self {
        Self::AtomicExchange(inst)
    }
}
impl From<OpAtomicCompareExchange> for CoreInstSet {
    fn from(inst: OpAtomicCompareExchange) -> Self {
        Self::AtomicCompareExchange(inst)
    }
}
impl From<OpAtomicCompareExchangeWeak> for CoreInstSet {
    fn from(inst: OpAtomicCompareExchangeWeak) -> Self {
        Self::AtomicCompareExchangeWeak(inst)
    }
}
impl From<OpAtomicIIncrement> for CoreInstSet {
    fn from(inst: OpAtomicIIncrement) -> Self {
        Self::AtomicIIncrement(inst)
    }
}
impl From<OpAtomicIDecrement> for CoreInstSet {
    fn from(inst: OpAtomicIDecrement) -> Self {
        Self::AtomicIDecrement(inst)
    }
}
impl From<OpAtomicIAdd> for CoreInstSet {
    fn from(inst: OpAtomicIAdd) -> Self {
        Self::AtomicIAdd(inst)
    }
}
impl From<OpAtomicISub> for CoreInstSet {
    fn from(inst: OpAtomicISub) -> Self {
        Self::AtomicISub(inst)
    }
}
impl From<OpAtomicSMin> for CoreInstSet {
    fn from(inst: OpAtomicSMin) -> Self {
        Self::AtomicSMin(inst)
    }
}
impl From<OpAtomicUMin> for CoreInstSet {
    fn from(inst: OpAtomicUMin) -> Self {
        Self::AtomicUMin(inst)
    }
}
impl From<OpAtomicSMax> for CoreInstSet {
    fn from(inst: OpAtomicSMax) -> Self {
        Self::AtomicSMax(inst)
    }
}
impl From<OpAtomicUMax> for CoreInstSet {
    fn from(inst: OpAtomicUMax) -> Self {
        Self::AtomicUMax(inst)
    }
}
impl From<OpAtomicAnd> for CoreInstSet {
    fn from(inst: OpAtomicAnd) -> Self {
        Self::AtomicAnd(inst)
    }
}
impl From<OpAtomicOr> for CoreInstSet {
    fn from(inst: OpAtomicOr) -> Self {
        Self::AtomicOr(inst)
    }
}
impl From<OpAtomicXor> for CoreInstSet {
    fn from(inst: OpAtomicXor) -> Self {
        Self::AtomicXor(inst)
    }
}
impl From<OpPhi> for CoreInstSet {
    fn from(inst: OpPhi) -> Self {
        Self::Phi(inst)
    }
}
impl From<OpLoopMerge> for CoreInstSet {
    fn from(inst: OpLoopMerge) -> Self {
        Self::LoopMerge(inst)
    }
}
impl From<OpSelectionMerge> for CoreInstSet {
    fn from(inst: OpSelectionMerge) -> Self {
        Self::SelectionMerge(inst)
    }
}
impl From<OpLabel> for CoreInstSet {
    fn from(inst: OpLabel) -> Self {
        Self::Label(inst)
    }
}
impl From<OpBranch> for CoreInstSet {
    fn from(inst: OpBranch) -> Self {
        Self::Branch(inst)
    }
}
impl From<OpBranchConditional> for CoreInstSet {
    fn from(inst: OpBranchConditional) -> Self {
        Self::BranchConditional(inst)
    }
}
impl From<OpSwitch> for CoreInstSet {
    fn from(inst: OpSwitch) -> Self {
        Self::Switch(inst)
    }
}
impl From<OpKill> for CoreInstSet {
    fn from(inst: OpKill) -> Self {
        Self::Kill(inst)
    }
}
impl From<OpReturn> for CoreInstSet {
    fn from(inst: OpReturn) -> Self {
        Self::Return(inst)
    }
}
impl From<OpReturnValue> for CoreInstSet {
    fn from(inst: OpReturnValue) -> Self {
        Self::ReturnValue(inst)
    }
}
impl From<OpUnreachable> for CoreInstSet {
    fn from(inst: OpUnreachable) -> Self {
        Self::Unreachable(inst)
    }
}
impl From<OpLifetimeStart> for CoreInstSet {
    fn from(inst: OpLifetimeStart) -> Self {
        Self::LifetimeStart(inst)
    }
}
impl From<OpLifetimeStop> for CoreInstSet {
    fn from(inst: OpLifetimeStop) -> Self {
        Self::LifetimeStop(inst)
    }
}
impl From<OpGroupAsyncCopy> for CoreInstSet {
    fn from(inst: OpGroupAsyncCopy) -> Self {
        Self::GroupAsyncCopy(inst)
    }
}
impl From<OpGroupWaitEvents> for CoreInstSet {
    fn from(inst: OpGroupWaitEvents) -> Self {
        Self::GroupWaitEvents(inst)
    }
}
impl From<OpGroupAll> for CoreInstSet {
    fn from(inst: OpGroupAll) -> Self {
        Self::GroupAll(inst)
    }
}
impl From<OpGroupAny> for CoreInstSet {
    fn from(inst: OpGroupAny) -> Self {
        Self::GroupAny(inst)
    }
}
impl From<OpGroupBroadcast> for CoreInstSet {
    fn from(inst: OpGroupBroadcast) -> Self {
        Self::GroupBroadcast(inst)
    }
}
impl From<OpGroupIAdd> for CoreInstSet {
    fn from(inst: OpGroupIAdd) -> Self {
        Self::GroupIAdd(inst)
    }
}
impl From<OpGroupFAdd> for CoreInstSet {
    fn from(inst: OpGroupFAdd) -> Self {
        Self::GroupFAdd(inst)
    }
}
impl From<OpGroupFMin> for CoreInstSet {
    fn from(inst: OpGroupFMin) -> Self {
        Self::GroupFMin(inst)
    }
}
impl From<OpGroupUMin> for CoreInstSet {
    fn from(inst: OpGroupUMin) -> Self {
        Self::GroupUMin(inst)
    }
}
impl From<OpGroupSMin> for CoreInstSet {
    fn from(inst: OpGroupSMin) -> Self {
        Self::GroupSMin(inst)
    }
}
impl From<OpGroupFMax> for CoreInstSet {
    fn from(inst: OpGroupFMax) -> Self {
        Self::GroupFMax(inst)
    }
}
impl From<OpGroupUMax> for CoreInstSet {
    fn from(inst: OpGroupUMax) -> Self {
        Self::GroupUMax(inst)
    }
}
impl From<OpGroupSMax> for CoreInstSet {
    fn from(inst: OpGroupSMax) -> Self {
        Self::GroupSMax(inst)
    }
}
impl From<OpReadPipe> for CoreInstSet {
    fn from(inst: OpReadPipe) -> Self {
        Self::ReadPipe(inst)
    }
}
impl From<OpWritePipe> for CoreInstSet {
    fn from(inst: OpWritePipe) -> Self {
        Self::WritePipe(inst)
    }
}
impl From<OpReservedReadPipe> for CoreInstSet {
    fn from(inst: OpReservedReadPipe) -> Self {
        Self::ReservedReadPipe(inst)
    }
}
impl From<OpReservedWritePipe> for CoreInstSet {
    fn from(inst: OpReservedWritePipe) -> Self {
        Self::ReservedWritePipe(inst)
    }
}
impl From<OpReserveReadPipePackets> for CoreInstSet {
    fn from(inst: OpReserveReadPipePackets) -> Self {
        Self::ReserveReadPipePackets(inst)
    }
}
impl From<OpReserveWritePipePackets> for CoreInstSet {
    fn from(inst: OpReserveWritePipePackets) -> Self {
        Self::ReserveWritePipePackets(inst)
    }
}
impl From<OpCommitReadPipe> for CoreInstSet {
    fn from(inst: OpCommitReadPipe) -> Self {
        Self::CommitReadPipe(inst)
    }
}
impl From<OpCommitWritePipe> for CoreInstSet {
    fn from(inst: OpCommitWritePipe) -> Self {
        Self::CommitWritePipe(inst)
    }
}
impl From<OpIsValidReserveId> for CoreInstSet {
    fn from(inst: OpIsValidReserveId) -> Self {
        Self::IsValidReserveId(inst)
    }
}
impl From<OpGetNumPipePackets> for CoreInstSet {
    fn from(inst: OpGetNumPipePackets) -> Self {
        Self::GetNumPipePackets(inst)
    }
}
impl From<OpGetMaxPipePackets> for CoreInstSet {
    fn from(inst: OpGetMaxPipePackets) -> Self {
        Self::GetMaxPipePackets(inst)
    }
}
impl From<OpGroupReserveReadPipePackets> for CoreInstSet {
    fn from(inst: OpGroupReserveReadPipePackets) -> Self {
        Self::GroupReserveReadPipePackets(inst)
    }
}
impl From<OpGroupReserveWritePipePackets> for CoreInstSet {
    fn from(inst: OpGroupReserveWritePipePackets) -> Self {
        Self::GroupReserveWritePipePackets(inst)
    }
}
impl From<OpGroupCommitReadPipe> for CoreInstSet {
    fn from(inst: OpGroupCommitReadPipe) -> Self {
        Self::GroupCommitReadPipe(inst)
    }
}
impl From<OpGroupCommitWritePipe> for CoreInstSet {
    fn from(inst: OpGroupCommitWritePipe) -> Self {
        Self::GroupCommitWritePipe(inst)
    }
}
impl From<OpEnqueueMarker> for CoreInstSet {
    fn from(inst: OpEnqueueMarker) -> Self {
        Self::EnqueueMarker(inst)
    }
}
impl From<OpEnqueueKernel> for CoreInstSet {
    fn from(inst: OpEnqueueKernel) -> Self {
        Self::EnqueueKernel(inst)
    }
}
impl From<OpGetKernelNDrangeSubGroupCount> for CoreInstSet {
    fn from(inst: OpGetKernelNDrangeSubGroupCount) -> Self {
        Self::GetKernelNDrangeSubGroupCount(inst)
    }
}
impl From<OpGetKernelNDrangeMaxSubGroupSize> for CoreInstSet {
    fn from(inst: OpGetKernelNDrangeMaxSubGroupSize) -> Self {
        Self::GetKernelNDrangeMaxSubGroupSize(inst)
    }
}
impl From<OpGetKernelWorkGroupSize> for CoreInstSet {
    fn from(inst: OpGetKernelWorkGroupSize) -> Self {
        Self::GetKernelWorkGroupSize(inst)
    }
}
impl From<OpGetKernelPreferredWorkGroupSizeMultiple> for CoreInstSet {
    fn from(inst: OpGetKernelPreferredWorkGroupSizeMultiple) -> Self {
        Self::GetKernelPreferredWorkGroupSizeMultiple(inst)
    }
}
impl From<OpRetainEvent> for CoreInstSet {
    fn from(inst: OpRetainEvent) -> Self {
        Self::RetainEvent(inst)
    }
}
impl From<OpReleaseEvent> for CoreInstSet {
    fn from(inst: OpReleaseEvent) -> Self {
        Self::ReleaseEvent(inst)
    }
}
impl From<OpCreateUserEvent> for CoreInstSet {
    fn from(inst: OpCreateUserEvent) -> Self {
        Self::CreateUserEvent(inst)
    }
}
impl From<OpIsValidEvent> for CoreInstSet {
    fn from(inst: OpIsValidEvent) -> Self {
        Self::IsValidEvent(inst)
    }
}
impl From<OpSetUserEventStatus> for CoreInstSet {
    fn from(inst: OpSetUserEventStatus) -> Self {
        Self::SetUserEventStatus(inst)
    }
}
impl From<OpCaptureEventProfilingInfo> for CoreInstSet {
    fn from(inst: OpCaptureEventProfilingInfo) -> Self {
        Self::CaptureEventProfilingInfo(inst)
    }
}
impl From<OpGetDefaultQueue> for CoreInstSet {
    fn from(inst: OpGetDefaultQueue) -> Self {
        Self::GetDefaultQueue(inst)
    }
}
impl From<OpBuildNDRange> for CoreInstSet {
    fn from(inst: OpBuildNDRange) -> Self {
        Self::BuildNDRange(inst)
    }
}
impl From<OpImageSparseSampleImplicitLod> for CoreInstSet {
    fn from(inst: OpImageSparseSampleImplicitLod) -> Self {
        Self::ImageSparseSampleImplicitLod(inst)
    }
}
impl From<OpImageSparseSampleExplicitLod> for CoreInstSet {
    fn from(inst: OpImageSparseSampleExplicitLod) -> Self {
        Self::ImageSparseSampleExplicitLod(inst)
    }
}
impl From<OpImageSparseSampleDrefImplicitLod> for CoreInstSet {
    fn from(inst: OpImageSparseSampleDrefImplicitLod) -> Self {
        Self::ImageSparseSampleDrefImplicitLod(inst)
    }
}
impl From<OpImageSparseSampleDrefExplicitLod> for CoreInstSet {
    fn from(inst: OpImageSparseSampleDrefExplicitLod) -> Self {
        Self::ImageSparseSampleDrefExplicitLod(inst)
    }
}
impl From<OpImageSparseSampleProjImplicitLod> for CoreInstSet {
    fn from(inst: OpImageSparseSampleProjImplicitLod) -> Self {
        Self::ImageSparseSampleProjImplicitLod(inst)
    }
}
impl From<OpImageSparseSampleProjExplicitLod> for CoreInstSet {
    fn from(inst: OpImageSparseSampleProjExplicitLod) -> Self {
        Self::ImageSparseSampleProjExplicitLod(inst)
    }
}
impl From<OpImageSparseSampleProjDrefImplicitLod> for CoreInstSet {
    fn from(inst: OpImageSparseSampleProjDrefImplicitLod) -> Self {
        Self::ImageSparseSampleProjDrefImplicitLod(inst)
    }
}
impl From<OpImageSparseSampleProjDrefExplicitLod> for CoreInstSet {
    fn from(inst: OpImageSparseSampleProjDrefExplicitLod) -> Self {
        Self::ImageSparseSampleProjDrefExplicitLod(inst)
    }
}
impl From<OpImageSparseFetch> for CoreInstSet {
    fn from(inst: OpImageSparseFetch) -> Self {
        Self::ImageSparseFetch(inst)
    }
}
impl From<OpImageSparseGather> for CoreInstSet {
    fn from(inst: OpImageSparseGather) -> Self {
        Self::ImageSparseGather(inst)
    }
}
impl From<OpImageSparseDrefGather> for CoreInstSet {
    fn from(inst: OpImageSparseDrefGather) -> Self {
        Self::ImageSparseDrefGather(inst)
    }
}
impl From<OpImageSparseTexelsResident> for CoreInstSet {
    fn from(inst: OpImageSparseTexelsResident) -> Self {
        Self::ImageSparseTexelsResident(inst)
    }
}
impl From<OpNoLine> for CoreInstSet {
    fn from(inst: OpNoLine) -> Self {
        Self::NoLine(inst)
    }
}
impl From<OpAtomicFlagTestAndSet> for CoreInstSet {
    fn from(inst: OpAtomicFlagTestAndSet) -> Self {
        Self::AtomicFlagTestAndSet(inst)
    }
}
impl From<OpAtomicFlagClear> for CoreInstSet {
    fn from(inst: OpAtomicFlagClear) -> Self {
        Self::AtomicFlagClear(inst)
    }
}
impl From<OpImageSparseRead> for CoreInstSet {
    fn from(inst: OpImageSparseRead) -> Self {
        Self::ImageSparseRead(inst)
    }
}
impl From<OpSizeOf> for CoreInstSet {
    fn from(inst: OpSizeOf) -> Self {
        Self::SizeOf(inst)
    }
}
impl From<OpTypePipeStorage> for CoreInstSet {
    fn from(inst: OpTypePipeStorage) -> Self {
        Self::TypePipeStorage(inst)
    }
}
impl From<OpConstantPipeStorage> for CoreInstSet {
    fn from(inst: OpConstantPipeStorage) -> Self {
        Self::ConstantPipeStorage(inst)
    }
}
impl From<OpCreatePipeFromPipeStorage> for CoreInstSet {
    fn from(inst: OpCreatePipeFromPipeStorage) -> Self {
        Self::CreatePipeFromPipeStorage(inst)
    }
}
impl From<OpGetKernelLocalSizeForSubgroupCount> for CoreInstSet {
    fn from(inst: OpGetKernelLocalSizeForSubgroupCount) -> Self {
        Self::GetKernelLocalSizeForSubgroupCount(inst)
    }
}
impl From<OpGetKernelMaxNumSubgroups> for CoreInstSet {
    fn from(inst: OpGetKernelMaxNumSubgroups) -> Self {
        Self::GetKernelMaxNumSubgroups(inst)
    }
}
impl From<OpTypeNamedBarrier> for CoreInstSet {
    fn from(inst: OpTypeNamedBarrier) -> Self {
        Self::TypeNamedBarrier(inst)
    }
}
impl From<OpNamedBarrierInitialize> for CoreInstSet {
    fn from(inst: OpNamedBarrierInitialize) -> Self {
        Self::NamedBarrierInitialize(inst)
    }
}
impl From<OpMemoryNamedBarrier> for CoreInstSet {
    fn from(inst: OpMemoryNamedBarrier) -> Self {
        Self::MemoryNamedBarrier(inst)
    }
}
impl From<OpModuleProcessed> for CoreInstSet {
    fn from(inst: OpModuleProcessed) -> Self {
        Self::ModuleProcessed(inst)
    }
}
impl From<OpExecutionModeId> for CoreInstSet {
    fn from(inst: OpExecutionModeId) -> Self {
        Self::ExecutionModeId(inst)
    }
}
impl From<OpDecorateId> for CoreInstSet {
    fn from(inst: OpDecorateId) -> Self {
        Self::DecorateId(inst)
    }
}
impl From<OpGroupNonUniformElect> for CoreInstSet {
    fn from(inst: OpGroupNonUniformElect) -> Self {
        Self::GroupNonUniformElect(inst)
    }
}
impl From<OpGroupNonUniformAll> for CoreInstSet {
    fn from(inst: OpGroupNonUniformAll) -> Self {
        Self::GroupNonUniformAll(inst)
    }
}
impl From<OpGroupNonUniformAny> for CoreInstSet {
    fn from(inst: OpGroupNonUniformAny) -> Self {
        Self::GroupNonUniformAny(inst)
    }
}
impl From<OpGroupNonUniformAllEqual> for CoreInstSet {
    fn from(inst: OpGroupNonUniformAllEqual) -> Self {
        Self::GroupNonUniformAllEqual(inst)
    }
}
impl From<OpGroupNonUniformBroadcast> for CoreInstSet {
    fn from(inst: OpGroupNonUniformBroadcast) -> Self {
        Self::GroupNonUniformBroadcast(inst)
    }
}
impl From<OpGroupNonUniformBroadcastFirst> for CoreInstSet {
    fn from(inst: OpGroupNonUniformBroadcastFirst) -> Self {
        Self::GroupNonUniformBroadcastFirst(inst)
    }
}
impl From<OpGroupNonUniformBallot> for CoreInstSet {
    fn from(inst: OpGroupNonUniformBallot) -> Self {
        Self::GroupNonUniformBallot(inst)
    }
}
impl From<OpGroupNonUniformInverseBallot> for CoreInstSet {
    fn from(inst: OpGroupNonUniformInverseBallot) -> Self {
        Self::GroupNonUniformInverseBallot(inst)
    }
}
impl From<OpGroupNonUniformBallotBitExtract> for CoreInstSet {
    fn from(inst: OpGroupNonUniformBallotBitExtract) -> Self {
        Self::GroupNonUniformBallotBitExtract(inst)
    }
}
impl From<OpGroupNonUniformBallotBitCount> for CoreInstSet {
    fn from(inst: OpGroupNonUniformBallotBitCount) -> Self {
        Self::GroupNonUniformBallotBitCount(inst)
    }
}
impl From<OpGroupNonUniformBallotFindLSB> for CoreInstSet {
    fn from(inst: OpGroupNonUniformBallotFindLSB) -> Self {
        Self::GroupNonUniformBallotFindLSB(inst)
    }
}
impl From<OpGroupNonUniformBallotFindMSB> for CoreInstSet {
    fn from(inst: OpGroupNonUniformBallotFindMSB) -> Self {
        Self::GroupNonUniformBallotFindMSB(inst)
    }
}
impl From<OpGroupNonUniformShuffle> for CoreInstSet {
    fn from(inst: OpGroupNonUniformShuffle) -> Self {
        Self::GroupNonUniformShuffle(inst)
    }
}
impl From<OpGroupNonUniformShuffleXor> for CoreInstSet {
    fn from(inst: OpGroupNonUniformShuffleXor) -> Self {
        Self::GroupNonUniformShuffleXor(inst)
    }
}
impl From<OpGroupNonUniformShuffleUp> for CoreInstSet {
    fn from(inst: OpGroupNonUniformShuffleUp) -> Self {
        Self::GroupNonUniformShuffleUp(inst)
    }
}
impl From<OpGroupNonUniformShuffleDown> for CoreInstSet {
    fn from(inst: OpGroupNonUniformShuffleDown) -> Self {
        Self::GroupNonUniformShuffleDown(inst)
    }
}
impl From<OpGroupNonUniformIAdd> for CoreInstSet {
    fn from(inst: OpGroupNonUniformIAdd) -> Self {
        Self::GroupNonUniformIAdd(inst)
    }
}
impl From<OpGroupNonUniformFAdd> for CoreInstSet {
    fn from(inst: OpGroupNonUniformFAdd) -> Self {
        Self::GroupNonUniformFAdd(inst)
    }
}
impl From<OpGroupNonUniformIMul> for CoreInstSet {
    fn from(inst: OpGroupNonUniformIMul) -> Self {
        Self::GroupNonUniformIMul(inst)
    }
}
impl From<OpGroupNonUniformFMul> for CoreInstSet {
    fn from(inst: OpGroupNonUniformFMul) -> Self {
        Self::GroupNonUniformFMul(inst)
    }
}
impl From<OpGroupNonUniformSMin> for CoreInstSet {
    fn from(inst: OpGroupNonUniformSMin) -> Self {
        Self::GroupNonUniformSMin(inst)
    }
}
impl From<OpGroupNonUniformUMin> for CoreInstSet {
    fn from(inst: OpGroupNonUniformUMin) -> Self {
        Self::GroupNonUniformUMin(inst)
    }
}
impl From<OpGroupNonUniformFMin> for CoreInstSet {
    fn from(inst: OpGroupNonUniformFMin) -> Self {
        Self::GroupNonUniformFMin(inst)
    }
}
impl From<OpGroupNonUniformSMax> for CoreInstSet {
    fn from(inst: OpGroupNonUniformSMax) -> Self {
        Self::GroupNonUniformSMax(inst)
    }
}
impl From<OpGroupNonUniformUMax> for CoreInstSet {
    fn from(inst: OpGroupNonUniformUMax) -> Self {
        Self::GroupNonUniformUMax(inst)
    }
}
impl From<OpGroupNonUniformFMax> for CoreInstSet {
    fn from(inst: OpGroupNonUniformFMax) -> Self {
        Self::GroupNonUniformFMax(inst)
    }
}
impl From<OpGroupNonUniformBitwiseAnd> for CoreInstSet {
    fn from(inst: OpGroupNonUniformBitwiseAnd) -> Self {
        Self::GroupNonUniformBitwiseAnd(inst)
    }
}
impl From<OpGroupNonUniformBitwiseOr> for CoreInstSet {
    fn from(inst: OpGroupNonUniformBitwiseOr) -> Self {
        Self::GroupNonUniformBitwiseOr(inst)
    }
}
impl From<OpGroupNonUniformBitwiseXor> for CoreInstSet {
    fn from(inst: OpGroupNonUniformBitwiseXor) -> Self {
        Self::GroupNonUniformBitwiseXor(inst)
    }
}
impl From<OpGroupNonUniformLogicalAnd> for CoreInstSet {
    fn from(inst: OpGroupNonUniformLogicalAnd) -> Self {
        Self::GroupNonUniformLogicalAnd(inst)
    }
}
impl From<OpGroupNonUniformLogicalOr> for CoreInstSet {
    fn from(inst: OpGroupNonUniformLogicalOr) -> Self {
        Self::GroupNonUniformLogicalOr(inst)
    }
}
impl From<OpGroupNonUniformLogicalXor> for CoreInstSet {
    fn from(inst: OpGroupNonUniformLogicalXor) -> Self {
        Self::GroupNonUniformLogicalXor(inst)
    }
}
impl From<OpGroupNonUniformQuadBroadcast> for CoreInstSet {
    fn from(inst: OpGroupNonUniformQuadBroadcast) -> Self {
        Self::GroupNonUniformQuadBroadcast(inst)
    }
}
impl From<OpGroupNonUniformQuadSwap> for CoreInstSet {
    fn from(inst: OpGroupNonUniformQuadSwap) -> Self {
        Self::GroupNonUniformQuadSwap(inst)
    }
}
impl From<OpCopyLogical> for CoreInstSet {
    fn from(inst: OpCopyLogical) -> Self {
        Self::CopyLogical(inst)
    }
}
impl From<OpPtrEqual> for CoreInstSet {
    fn from(inst: OpPtrEqual) -> Self {
        Self::PtrEqual(inst)
    }
}
impl From<OpPtrNotEqual> for CoreInstSet {
    fn from(inst: OpPtrNotEqual) -> Self {
        Self::PtrNotEqual(inst)
    }
}
impl From<OpPtrDiff> for CoreInstSet {
    fn from(inst: OpPtrDiff) -> Self {
        Self::PtrDiff(inst)
    }
}
impl From<OpColorAttachmentReadEXT> for CoreInstSet {
    fn from(inst: OpColorAttachmentReadEXT) -> Self {
        Self::ColorAttachmentReadEXT(inst)
    }
}
impl From<OpDepthAttachmentReadEXT> for CoreInstSet {
    fn from(inst: OpDepthAttachmentReadEXT) -> Self {
        Self::DepthAttachmentReadEXT(inst)
    }
}
impl From<OpStencilAttachmentReadEXT> for CoreInstSet {
    fn from(inst: OpStencilAttachmentReadEXT) -> Self {
        Self::StencilAttachmentReadEXT(inst)
    }
}
impl From<OpTypeTensorARM> for CoreInstSet {
    fn from(inst: OpTypeTensorARM) -> Self {
        Self::TypeTensorARM(inst)
    }
}
impl From<OpTensorReadARM> for CoreInstSet {
    fn from(inst: OpTensorReadARM) -> Self {
        Self::TensorReadARM(inst)
    }
}
impl From<OpTensorWriteARM> for CoreInstSet {
    fn from(inst: OpTensorWriteARM) -> Self {
        Self::TensorWriteARM(inst)
    }
}
impl From<OpTensorQuerySizeARM> for CoreInstSet {
    fn from(inst: OpTensorQuerySizeARM) -> Self {
        Self::TensorQuerySizeARM(inst)
    }
}
impl From<OpGraphConstantARM> for CoreInstSet {
    fn from(inst: OpGraphConstantARM) -> Self {
        Self::GraphConstantARM(inst)
    }
}
impl From<OpGraphEntryPointARM> for CoreInstSet {
    fn from(inst: OpGraphEntryPointARM) -> Self {
        Self::GraphEntryPointARM(inst)
    }
}
impl From<OpGraphARM> for CoreInstSet {
    fn from(inst: OpGraphARM) -> Self {
        Self::GraphARM(inst)
    }
}
impl From<OpGraphInputARM> for CoreInstSet {
    fn from(inst: OpGraphInputARM) -> Self {
        Self::GraphInputARM(inst)
    }
}
impl From<OpGraphSetOutputARM> for CoreInstSet {
    fn from(inst: OpGraphSetOutputARM) -> Self {
        Self::GraphSetOutputARM(inst)
    }
}
impl From<OpGraphEndARM> for CoreInstSet {
    fn from(inst: OpGraphEndARM) -> Self {
        Self::GraphEndARM(inst)
    }
}
impl From<OpTypeGraphARM> for CoreInstSet {
    fn from(inst: OpTypeGraphARM) -> Self {
        Self::TypeGraphARM(inst)
    }
}
impl From<OpTerminateInvocation> for CoreInstSet {
    fn from(inst: OpTerminateInvocation) -> Self {
        Self::TerminateInvocation(inst)
    }
}
impl From<OpTypeUntypedPointerKHR> for CoreInstSet {
    fn from(inst: OpTypeUntypedPointerKHR) -> Self {
        Self::TypeUntypedPointerKHR(inst)
    }
}
impl From<OpUntypedVariableKHR> for CoreInstSet {
    fn from(inst: OpUntypedVariableKHR) -> Self {
        Self::UntypedVariableKHR(inst)
    }
}
impl From<OpUntypedAccessChainKHR> for CoreInstSet {
    fn from(inst: OpUntypedAccessChainKHR) -> Self {
        Self::UntypedAccessChainKHR(inst)
    }
}
impl From<OpUntypedInBoundsAccessChainKHR> for CoreInstSet {
    fn from(inst: OpUntypedInBoundsAccessChainKHR) -> Self {
        Self::UntypedInBoundsAccessChainKHR(inst)
    }
}
impl From<OpSubgroupBallotKHR> for CoreInstSet {
    fn from(inst: OpSubgroupBallotKHR) -> Self {
        Self::SubgroupBallotKHR(inst)
    }
}
impl From<OpSubgroupFirstInvocationKHR> for CoreInstSet {
    fn from(inst: OpSubgroupFirstInvocationKHR) -> Self {
        Self::SubgroupFirstInvocationKHR(inst)
    }
}
impl From<OpUntypedPtrAccessChainKHR> for CoreInstSet {
    fn from(inst: OpUntypedPtrAccessChainKHR) -> Self {
        Self::UntypedPtrAccessChainKHR(inst)
    }
}
impl From<OpUntypedInBoundsPtrAccessChainKHR> for CoreInstSet {
    fn from(inst: OpUntypedInBoundsPtrAccessChainKHR) -> Self {
        Self::UntypedInBoundsPtrAccessChainKHR(inst)
    }
}
impl From<OpUntypedArrayLengthKHR> for CoreInstSet {
    fn from(inst: OpUntypedArrayLengthKHR) -> Self {
        Self::UntypedArrayLengthKHR(inst)
    }
}
impl From<OpUntypedPrefetchKHR> for CoreInstSet {
    fn from(inst: OpUntypedPrefetchKHR) -> Self {
        Self::UntypedPrefetchKHR(inst)
    }
}
impl From<OpFmaKHR> for CoreInstSet {
    fn from(inst: OpFmaKHR) -> Self {
        Self::FmaKHR(inst)
    }
}
impl From<OpSubgroupAllKHR> for CoreInstSet {
    fn from(inst: OpSubgroupAllKHR) -> Self {
        Self::SubgroupAllKHR(inst)
    }
}
impl From<OpSubgroupAnyKHR> for CoreInstSet {
    fn from(inst: OpSubgroupAnyKHR) -> Self {
        Self::SubgroupAnyKHR(inst)
    }
}
impl From<OpSubgroupAllEqualKHR> for CoreInstSet {
    fn from(inst: OpSubgroupAllEqualKHR) -> Self {
        Self::SubgroupAllEqualKHR(inst)
    }
}
impl From<OpGroupNonUniformRotateKHR> for CoreInstSet {
    fn from(inst: OpGroupNonUniformRotateKHR) -> Self {
        Self::GroupNonUniformRotateKHR(inst)
    }
}
impl From<OpSubgroupReadInvocationKHR> for CoreInstSet {
    fn from(inst: OpSubgroupReadInvocationKHR) -> Self {
        Self::SubgroupReadInvocationKHR(inst)
    }
}
impl From<OpExtInstWithForwardRefsKHR> for CoreInstSet {
    fn from(inst: OpExtInstWithForwardRefsKHR) -> Self {
        Self::ExtInstWithForwardRefsKHR(inst)
    }
}
impl From<OpUntypedGroupAsyncCopyKHR> for CoreInstSet {
    fn from(inst: OpUntypedGroupAsyncCopyKHR) -> Self {
        Self::UntypedGroupAsyncCopyKHR(inst)
    }
}
impl From<OpTraceRayKHR> for CoreInstSet {
    fn from(inst: OpTraceRayKHR) -> Self {
        Self::TraceRayKHR(inst)
    }
}
impl From<OpExecuteCallableKHR> for CoreInstSet {
    fn from(inst: OpExecuteCallableKHR) -> Self {
        Self::ExecuteCallableKHR(inst)
    }
}
impl From<OpConvertUToAccelerationStructureKHR> for CoreInstSet {
    fn from(inst: OpConvertUToAccelerationStructureKHR) -> Self {
        Self::ConvertUToAccelerationStructureKHR(inst)
    }
}
impl From<OpIgnoreIntersectionKHR> for CoreInstSet {
    fn from(inst: OpIgnoreIntersectionKHR) -> Self {
        Self::IgnoreIntersectionKHR(inst)
    }
}
impl From<OpTerminateRayKHR> for CoreInstSet {
    fn from(inst: OpTerminateRayKHR) -> Self {
        Self::TerminateRayKHR(inst)
    }
}
impl From<OpSDot> for CoreInstSet {
    fn from(inst: OpSDot) -> Self {
        Self::SDot(inst)
    }
}
impl From<OpUDot> for CoreInstSet {
    fn from(inst: OpUDot) -> Self {
        Self::UDot(inst)
    }
}
impl From<OpSUDot> for CoreInstSet {
    fn from(inst: OpSUDot) -> Self {
        Self::SUDot(inst)
    }
}
impl From<OpSDotAccSat> for CoreInstSet {
    fn from(inst: OpSDotAccSat) -> Self {
        Self::SDotAccSat(inst)
    }
}
impl From<OpUDotAccSat> for CoreInstSet {
    fn from(inst: OpUDotAccSat) -> Self {
        Self::UDotAccSat(inst)
    }
}
impl From<OpSUDotAccSat> for CoreInstSet {
    fn from(inst: OpSUDotAccSat) -> Self {
        Self::SUDotAccSat(inst)
    }
}
impl From<OpTypeCooperativeMatrixKHR> for CoreInstSet {
    fn from(inst: OpTypeCooperativeMatrixKHR) -> Self {
        Self::TypeCooperativeMatrixKHR(inst)
    }
}
impl From<OpCooperativeMatrixLoadKHR> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixLoadKHR) -> Self {
        Self::CooperativeMatrixLoadKHR(inst)
    }
}
impl From<OpCooperativeMatrixStoreKHR> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixStoreKHR) -> Self {
        Self::CooperativeMatrixStoreKHR(inst)
    }
}
impl From<OpCooperativeMatrixMulAddKHR> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixMulAddKHR) -> Self {
        Self::CooperativeMatrixMulAddKHR(inst)
    }
}
impl From<OpCooperativeMatrixLengthKHR> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixLengthKHR) -> Self {
        Self::CooperativeMatrixLengthKHR(inst)
    }
}
impl From<OpConstantCompositeReplicateEXT> for CoreInstSet {
    fn from(inst: OpConstantCompositeReplicateEXT) -> Self {
        Self::ConstantCompositeReplicateEXT(inst)
    }
}
impl From<OpSpecConstantCompositeReplicateEXT> for CoreInstSet {
    fn from(inst: OpSpecConstantCompositeReplicateEXT) -> Self {
        Self::SpecConstantCompositeReplicateEXT(inst)
    }
}
impl From<OpCompositeConstructReplicateEXT> for CoreInstSet {
    fn from(inst: OpCompositeConstructReplicateEXT) -> Self {
        Self::CompositeConstructReplicateEXT(inst)
    }
}
impl From<OpTypeRayQueryKHR> for CoreInstSet {
    fn from(inst: OpTypeRayQueryKHR) -> Self {
        Self::TypeRayQueryKHR(inst)
    }
}
impl From<OpRayQueryInitializeKHR> for CoreInstSet {
    fn from(inst: OpRayQueryInitializeKHR) -> Self {
        Self::RayQueryInitializeKHR(inst)
    }
}
impl From<OpRayQueryTerminateKHR> for CoreInstSet {
    fn from(inst: OpRayQueryTerminateKHR) -> Self {
        Self::RayQueryTerminateKHR(inst)
    }
}
impl From<OpRayQueryGenerateIntersectionKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGenerateIntersectionKHR) -> Self {
        Self::RayQueryGenerateIntersectionKHR(inst)
    }
}
impl From<OpRayQueryConfirmIntersectionKHR> for CoreInstSet {
    fn from(inst: OpRayQueryConfirmIntersectionKHR) -> Self {
        Self::RayQueryConfirmIntersectionKHR(inst)
    }
}
impl From<OpRayQueryProceedKHR> for CoreInstSet {
    fn from(inst: OpRayQueryProceedKHR) -> Self {
        Self::RayQueryProceedKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionTypeKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionTypeKHR) -> Self {
        Self::RayQueryGetIntersectionTypeKHR(inst)
    }
}
impl From<OpImageSampleWeightedQCOM> for CoreInstSet {
    fn from(inst: OpImageSampleWeightedQCOM) -> Self {
        Self::ImageSampleWeightedQCOM(inst)
    }
}
impl From<OpImageBoxFilterQCOM> for CoreInstSet {
    fn from(inst: OpImageBoxFilterQCOM) -> Self {
        Self::ImageBoxFilterQCOM(inst)
    }
}
impl From<OpImageBlockMatchSSDQCOM> for CoreInstSet {
    fn from(inst: OpImageBlockMatchSSDQCOM) -> Self {
        Self::ImageBlockMatchSSDQCOM(inst)
    }
}
impl From<OpImageBlockMatchSADQCOM> for CoreInstSet {
    fn from(inst: OpImageBlockMatchSADQCOM) -> Self {
        Self::ImageBlockMatchSADQCOM(inst)
    }
}
impl From<OpBitCastArrayQCOM> for CoreInstSet {
    fn from(inst: OpBitCastArrayQCOM) -> Self {
        Self::BitCastArrayQCOM(inst)
    }
}
impl From<OpImageBlockMatchWindowSSDQCOM> for CoreInstSet {
    fn from(inst: OpImageBlockMatchWindowSSDQCOM) -> Self {
        Self::ImageBlockMatchWindowSSDQCOM(inst)
    }
}
impl From<OpImageBlockMatchWindowSADQCOM> for CoreInstSet {
    fn from(inst: OpImageBlockMatchWindowSADQCOM) -> Self {
        Self::ImageBlockMatchWindowSADQCOM(inst)
    }
}
impl From<OpImageBlockMatchGatherSSDQCOM> for CoreInstSet {
    fn from(inst: OpImageBlockMatchGatherSSDQCOM) -> Self {
        Self::ImageBlockMatchGatherSSDQCOM(inst)
    }
}
impl From<OpImageBlockMatchGatherSADQCOM> for CoreInstSet {
    fn from(inst: OpImageBlockMatchGatherSADQCOM) -> Self {
        Self::ImageBlockMatchGatherSADQCOM(inst)
    }
}
impl From<OpCompositeConstructCoopMatQCOM> for CoreInstSet {
    fn from(inst: OpCompositeConstructCoopMatQCOM) -> Self {
        Self::CompositeConstructCoopMatQCOM(inst)
    }
}
impl From<OpCompositeExtractCoopMatQCOM> for CoreInstSet {
    fn from(inst: OpCompositeExtractCoopMatQCOM) -> Self {
        Self::CompositeExtractCoopMatQCOM(inst)
    }
}
impl From<OpExtractSubArrayQCOM> for CoreInstSet {
    fn from(inst: OpExtractSubArrayQCOM) -> Self {
        Self::ExtractSubArrayQCOM(inst)
    }
}
impl From<OpGroupIAddNonUniformAMD> for CoreInstSet {
    fn from(inst: OpGroupIAddNonUniformAMD) -> Self {
        Self::GroupIAddNonUniformAMD(inst)
    }
}
impl From<OpGroupFAddNonUniformAMD> for CoreInstSet {
    fn from(inst: OpGroupFAddNonUniformAMD) -> Self {
        Self::GroupFAddNonUniformAMD(inst)
    }
}
impl From<OpGroupFMinNonUniformAMD> for CoreInstSet {
    fn from(inst: OpGroupFMinNonUniformAMD) -> Self {
        Self::GroupFMinNonUniformAMD(inst)
    }
}
impl From<OpGroupUMinNonUniformAMD> for CoreInstSet {
    fn from(inst: OpGroupUMinNonUniformAMD) -> Self {
        Self::GroupUMinNonUniformAMD(inst)
    }
}
impl From<OpGroupSMinNonUniformAMD> for CoreInstSet {
    fn from(inst: OpGroupSMinNonUniformAMD) -> Self {
        Self::GroupSMinNonUniformAMD(inst)
    }
}
impl From<OpGroupFMaxNonUniformAMD> for CoreInstSet {
    fn from(inst: OpGroupFMaxNonUniformAMD) -> Self {
        Self::GroupFMaxNonUniformAMD(inst)
    }
}
impl From<OpGroupUMaxNonUniformAMD> for CoreInstSet {
    fn from(inst: OpGroupUMaxNonUniformAMD) -> Self {
        Self::GroupUMaxNonUniformAMD(inst)
    }
}
impl From<OpGroupSMaxNonUniformAMD> for CoreInstSet {
    fn from(inst: OpGroupSMaxNonUniformAMD) -> Self {
        Self::GroupSMaxNonUniformAMD(inst)
    }
}
impl From<OpFragmentMaskFetchAMD> for CoreInstSet {
    fn from(inst: OpFragmentMaskFetchAMD) -> Self {
        Self::FragmentMaskFetchAMD(inst)
    }
}
impl From<OpFragmentFetchAMD> for CoreInstSet {
    fn from(inst: OpFragmentFetchAMD) -> Self {
        Self::FragmentFetchAMD(inst)
    }
}
impl From<OpReadClockKHR> for CoreInstSet {
    fn from(inst: OpReadClockKHR) -> Self {
        Self::ReadClockKHR(inst)
    }
}
impl From<OpAllocateNodePayloadsAMDX> for CoreInstSet {
    fn from(inst: OpAllocateNodePayloadsAMDX) -> Self {
        Self::AllocateNodePayloadsAMDX(inst)
    }
}
impl From<OpEnqueueNodePayloadsAMDX> for CoreInstSet {
    fn from(inst: OpEnqueueNodePayloadsAMDX) -> Self {
        Self::EnqueueNodePayloadsAMDX(inst)
    }
}
impl From<OpTypeNodePayloadArrayAMDX> for CoreInstSet {
    fn from(inst: OpTypeNodePayloadArrayAMDX) -> Self {
        Self::TypeNodePayloadArrayAMDX(inst)
    }
}
impl From<OpFinishWritingNodePayloadAMDX> for CoreInstSet {
    fn from(inst: OpFinishWritingNodePayloadAMDX) -> Self {
        Self::FinishWritingNodePayloadAMDX(inst)
    }
}
impl From<OpNodePayloadArrayLengthAMDX> for CoreInstSet {
    fn from(inst: OpNodePayloadArrayLengthAMDX) -> Self {
        Self::NodePayloadArrayLengthAMDX(inst)
    }
}
impl From<OpIsNodePayloadValidAMDX> for CoreInstSet {
    fn from(inst: OpIsNodePayloadValidAMDX) -> Self {
        Self::IsNodePayloadValidAMDX(inst)
    }
}
impl From<OpConstantStringAMDX> for CoreInstSet {
    fn from(inst: OpConstantStringAMDX) -> Self {
        Self::ConstantStringAMDX(inst)
    }
}
impl From<OpSpecConstantStringAMDX> for CoreInstSet {
    fn from(inst: OpSpecConstantStringAMDX) -> Self {
        Self::SpecConstantStringAMDX(inst)
    }
}
impl From<OpGroupNonUniformQuadAllKHR> for CoreInstSet {
    fn from(inst: OpGroupNonUniformQuadAllKHR) -> Self {
        Self::GroupNonUniformQuadAllKHR(inst)
    }
}
impl From<OpGroupNonUniformQuadAnyKHR> for CoreInstSet {
    fn from(inst: OpGroupNonUniformQuadAnyKHR) -> Self {
        Self::GroupNonUniformQuadAnyKHR(inst)
    }
}
impl From<OpTypeBufferEXT> for CoreInstSet {
    fn from(inst: OpTypeBufferEXT) -> Self {
        Self::TypeBufferEXT(inst)
    }
}
impl From<OpBufferPointerEXT> for CoreInstSet {
    fn from(inst: OpBufferPointerEXT) -> Self {
        Self::BufferPointerEXT(inst)
    }
}
impl From<OpUntypedImageTexelPointerEXT> for CoreInstSet {
    fn from(inst: OpUntypedImageTexelPointerEXT) -> Self {
        Self::UntypedImageTexelPointerEXT(inst)
    }
}
impl From<OpMemberDecorateIdEXT> for CoreInstSet {
    fn from(inst: OpMemberDecorateIdEXT) -> Self {
        Self::MemberDecorateIdEXT(inst)
    }
}
impl From<OpConstantSizeOfEXT> for CoreInstSet {
    fn from(inst: OpConstantSizeOfEXT) -> Self {
        Self::ConstantSizeOfEXT(inst)
    }
}
impl From<OpHitObjectRecordHitMotionNV> for CoreInstSet {
    fn from(inst: OpHitObjectRecordHitMotionNV) -> Self {
        Self::HitObjectRecordHitMotionNV(inst)
    }
}
impl From<OpHitObjectRecordHitWithIndexMotionNV> for CoreInstSet {
    fn from(inst: OpHitObjectRecordHitWithIndexMotionNV) -> Self {
        Self::HitObjectRecordHitWithIndexMotionNV(inst)
    }
}
impl From<OpHitObjectRecordMissMotionNV> for CoreInstSet {
    fn from(inst: OpHitObjectRecordMissMotionNV) -> Self {
        Self::HitObjectRecordMissMotionNV(inst)
    }
}
impl From<OpHitObjectGetWorldToObjectNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetWorldToObjectNV) -> Self {
        Self::HitObjectGetWorldToObjectNV(inst)
    }
}
impl From<OpHitObjectGetObjectToWorldNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetObjectToWorldNV) -> Self {
        Self::HitObjectGetObjectToWorldNV(inst)
    }
}
impl From<OpHitObjectGetObjectRayDirectionNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetObjectRayDirectionNV) -> Self {
        Self::HitObjectGetObjectRayDirectionNV(inst)
    }
}
impl From<OpHitObjectGetObjectRayOriginNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetObjectRayOriginNV) -> Self {
        Self::HitObjectGetObjectRayOriginNV(inst)
    }
}
impl From<OpHitObjectTraceRayMotionNV> for CoreInstSet {
    fn from(inst: OpHitObjectTraceRayMotionNV) -> Self {
        Self::HitObjectTraceRayMotionNV(inst)
    }
}
impl From<OpHitObjectGetShaderRecordBufferHandleNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetShaderRecordBufferHandleNV) -> Self {
        Self::HitObjectGetShaderRecordBufferHandleNV(inst)
    }
}
impl From<OpHitObjectGetShaderBindingTableRecordIndexNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetShaderBindingTableRecordIndexNV) -> Self {
        Self::HitObjectGetShaderBindingTableRecordIndexNV(inst)
    }
}
impl From<OpHitObjectRecordEmptyNV> for CoreInstSet {
    fn from(inst: OpHitObjectRecordEmptyNV) -> Self {
        Self::HitObjectRecordEmptyNV(inst)
    }
}
impl From<OpHitObjectTraceRayNV> for CoreInstSet {
    fn from(inst: OpHitObjectTraceRayNV) -> Self {
        Self::HitObjectTraceRayNV(inst)
    }
}
impl From<OpHitObjectRecordHitNV> for CoreInstSet {
    fn from(inst: OpHitObjectRecordHitNV) -> Self {
        Self::HitObjectRecordHitNV(inst)
    }
}
impl From<OpHitObjectRecordHitWithIndexNV> for CoreInstSet {
    fn from(inst: OpHitObjectRecordHitWithIndexNV) -> Self {
        Self::HitObjectRecordHitWithIndexNV(inst)
    }
}
impl From<OpHitObjectRecordMissNV> for CoreInstSet {
    fn from(inst: OpHitObjectRecordMissNV) -> Self {
        Self::HitObjectRecordMissNV(inst)
    }
}
impl From<OpHitObjectExecuteShaderNV> for CoreInstSet {
    fn from(inst: OpHitObjectExecuteShaderNV) -> Self {
        Self::HitObjectExecuteShaderNV(inst)
    }
}
impl From<OpHitObjectGetCurrentTimeNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetCurrentTimeNV) -> Self {
        Self::HitObjectGetCurrentTimeNV(inst)
    }
}
impl From<OpHitObjectGetAttributesNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetAttributesNV) -> Self {
        Self::HitObjectGetAttributesNV(inst)
    }
}
impl From<OpHitObjectGetHitKindNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetHitKindNV) -> Self {
        Self::HitObjectGetHitKindNV(inst)
    }
}
impl From<OpHitObjectGetPrimitiveIndexNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetPrimitiveIndexNV) -> Self {
        Self::HitObjectGetPrimitiveIndexNV(inst)
    }
}
impl From<OpHitObjectGetGeometryIndexNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetGeometryIndexNV) -> Self {
        Self::HitObjectGetGeometryIndexNV(inst)
    }
}
impl From<OpHitObjectGetInstanceIdNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetInstanceIdNV) -> Self {
        Self::HitObjectGetInstanceIdNV(inst)
    }
}
impl From<OpHitObjectGetInstanceCustomIndexNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetInstanceCustomIndexNV) -> Self {
        Self::HitObjectGetInstanceCustomIndexNV(inst)
    }
}
impl From<OpHitObjectGetWorldRayDirectionNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetWorldRayDirectionNV) -> Self {
        Self::HitObjectGetWorldRayDirectionNV(inst)
    }
}
impl From<OpHitObjectGetWorldRayOriginNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetWorldRayOriginNV) -> Self {
        Self::HitObjectGetWorldRayOriginNV(inst)
    }
}
impl From<OpHitObjectGetRayTMaxNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetRayTMaxNV) -> Self {
        Self::HitObjectGetRayTMaxNV(inst)
    }
}
impl From<OpHitObjectGetRayTMinNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetRayTMinNV) -> Self {
        Self::HitObjectGetRayTMinNV(inst)
    }
}
impl From<OpHitObjectIsEmptyNV> for CoreInstSet {
    fn from(inst: OpHitObjectIsEmptyNV) -> Self {
        Self::HitObjectIsEmptyNV(inst)
    }
}
impl From<OpHitObjectIsHitNV> for CoreInstSet {
    fn from(inst: OpHitObjectIsHitNV) -> Self {
        Self::HitObjectIsHitNV(inst)
    }
}
impl From<OpHitObjectIsMissNV> for CoreInstSet {
    fn from(inst: OpHitObjectIsMissNV) -> Self {
        Self::HitObjectIsMissNV(inst)
    }
}
impl From<OpReorderThreadWithHitObjectNV> for CoreInstSet {
    fn from(inst: OpReorderThreadWithHitObjectNV) -> Self {
        Self::ReorderThreadWithHitObjectNV(inst)
    }
}
impl From<OpReorderThreadWithHintNV> for CoreInstSet {
    fn from(inst: OpReorderThreadWithHintNV) -> Self {
        Self::ReorderThreadWithHintNV(inst)
    }
}
impl From<OpTypeHitObjectNV> for CoreInstSet {
    fn from(inst: OpTypeHitObjectNV) -> Self {
        Self::TypeHitObjectNV(inst)
    }
}
impl From<OpImageSampleFootprintNV> for CoreInstSet {
    fn from(inst: OpImageSampleFootprintNV) -> Self {
        Self::ImageSampleFootprintNV(inst)
    }
}
impl From<OpTypeVectorIdEXT> for CoreInstSet {
    fn from(inst: OpTypeVectorIdEXT) -> Self {
        Self::TypeVectorIdEXT(inst)
    }
}
impl From<OpCooperativeVectorMatrixMulNV> for CoreInstSet {
    fn from(inst: OpCooperativeVectorMatrixMulNV) -> Self {
        Self::CooperativeVectorMatrixMulNV(inst)
    }
}
impl From<OpCooperativeVectorOuterProductAccumulateNV> for CoreInstSet {
    fn from(inst: OpCooperativeVectorOuterProductAccumulateNV) -> Self {
        Self::CooperativeVectorOuterProductAccumulateNV(inst)
    }
}
impl From<OpCooperativeVectorReduceSumAccumulateNV> for CoreInstSet {
    fn from(inst: OpCooperativeVectorReduceSumAccumulateNV) -> Self {
        Self::CooperativeVectorReduceSumAccumulateNV(inst)
    }
}
impl From<OpCooperativeVectorMatrixMulAddNV> for CoreInstSet {
    fn from(inst: OpCooperativeVectorMatrixMulAddNV) -> Self {
        Self::CooperativeVectorMatrixMulAddNV(inst)
    }
}
impl From<OpCooperativeMatrixConvertNV> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixConvertNV) -> Self {
        Self::CooperativeMatrixConvertNV(inst)
    }
}
impl From<OpEmitMeshTasksEXT> for CoreInstSet {
    fn from(inst: OpEmitMeshTasksEXT) -> Self {
        Self::EmitMeshTasksEXT(inst)
    }
}
impl From<OpSetMeshOutputsEXT> for CoreInstSet {
    fn from(inst: OpSetMeshOutputsEXT) -> Self {
        Self::SetMeshOutputsEXT(inst)
    }
}
impl From<OpGroupNonUniformPartitionEXT> for CoreInstSet {
    fn from(inst: OpGroupNonUniformPartitionEXT) -> Self {
        Self::GroupNonUniformPartitionEXT(inst)
    }
}
impl From<OpWritePackedPrimitiveIndices4x8NV> for CoreInstSet {
    fn from(inst: OpWritePackedPrimitiveIndices4x8NV) -> Self {
        Self::WritePackedPrimitiveIndices4x8NV(inst)
    }
}
impl From<OpFetchMicroTriangleVertexPositionNV> for CoreInstSet {
    fn from(inst: OpFetchMicroTriangleVertexPositionNV) -> Self {
        Self::FetchMicroTriangleVertexPositionNV(inst)
    }
}
impl From<OpFetchMicroTriangleVertexBarycentricNV> for CoreInstSet {
    fn from(inst: OpFetchMicroTriangleVertexBarycentricNV) -> Self {
        Self::FetchMicroTriangleVertexBarycentricNV(inst)
    }
}
impl From<OpCooperativeVectorLoadNV> for CoreInstSet {
    fn from(inst: OpCooperativeVectorLoadNV) -> Self {
        Self::CooperativeVectorLoadNV(inst)
    }
}
impl From<OpCooperativeVectorStoreNV> for CoreInstSet {
    fn from(inst: OpCooperativeVectorStoreNV) -> Self {
        Self::CooperativeVectorStoreNV(inst)
    }
}
impl From<OpHitObjectRecordFromQueryEXT> for CoreInstSet {
    fn from(inst: OpHitObjectRecordFromQueryEXT) -> Self {
        Self::HitObjectRecordFromQueryEXT(inst)
    }
}
impl From<OpHitObjectRecordMissEXT> for CoreInstSet {
    fn from(inst: OpHitObjectRecordMissEXT) -> Self {
        Self::HitObjectRecordMissEXT(inst)
    }
}
impl From<OpHitObjectRecordMissMotionEXT> for CoreInstSet {
    fn from(inst: OpHitObjectRecordMissMotionEXT) -> Self {
        Self::HitObjectRecordMissMotionEXT(inst)
    }
}
impl From<OpHitObjectGetIntersectionTriangleVertexPositionsEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetIntersectionTriangleVertexPositionsEXT) -> Self {
        Self::HitObjectGetIntersectionTriangleVertexPositionsEXT(inst)
    }
}
impl From<OpHitObjectGetRayFlagsEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetRayFlagsEXT) -> Self {
        Self::HitObjectGetRayFlagsEXT(inst)
    }
}
impl From<OpHitObjectSetShaderBindingTableRecordIndexEXT> for CoreInstSet {
    fn from(inst: OpHitObjectSetShaderBindingTableRecordIndexEXT) -> Self {
        Self::HitObjectSetShaderBindingTableRecordIndexEXT(inst)
    }
}
impl From<OpHitObjectReorderExecuteShaderEXT> for CoreInstSet {
    fn from(inst: OpHitObjectReorderExecuteShaderEXT) -> Self {
        Self::HitObjectReorderExecuteShaderEXT(inst)
    }
}
impl From<OpHitObjectTraceReorderExecuteEXT> for CoreInstSet {
    fn from(inst: OpHitObjectTraceReorderExecuteEXT) -> Self {
        Self::HitObjectTraceReorderExecuteEXT(inst)
    }
}
impl From<OpHitObjectTraceMotionReorderExecuteEXT> for CoreInstSet {
    fn from(inst: OpHitObjectTraceMotionReorderExecuteEXT) -> Self {
        Self::HitObjectTraceMotionReorderExecuteEXT(inst)
    }
}
impl From<OpTypeHitObjectEXT> for CoreInstSet {
    fn from(inst: OpTypeHitObjectEXT) -> Self {
        Self::TypeHitObjectEXT(inst)
    }
}
impl From<OpReorderThreadWithHintEXT> for CoreInstSet {
    fn from(inst: OpReorderThreadWithHintEXT) -> Self {
        Self::ReorderThreadWithHintEXT(inst)
    }
}
impl From<OpReorderThreadWithHitObjectEXT> for CoreInstSet {
    fn from(inst: OpReorderThreadWithHitObjectEXT) -> Self {
        Self::ReorderThreadWithHitObjectEXT(inst)
    }
}
impl From<OpHitObjectTraceRayEXT> for CoreInstSet {
    fn from(inst: OpHitObjectTraceRayEXT) -> Self {
        Self::HitObjectTraceRayEXT(inst)
    }
}
impl From<OpHitObjectTraceRayMotionEXT> for CoreInstSet {
    fn from(inst: OpHitObjectTraceRayMotionEXT) -> Self {
        Self::HitObjectTraceRayMotionEXT(inst)
    }
}
impl From<OpHitObjectRecordEmptyEXT> for CoreInstSet {
    fn from(inst: OpHitObjectRecordEmptyEXT) -> Self {
        Self::HitObjectRecordEmptyEXT(inst)
    }
}
impl From<OpHitObjectExecuteShaderEXT> for CoreInstSet {
    fn from(inst: OpHitObjectExecuteShaderEXT) -> Self {
        Self::HitObjectExecuteShaderEXT(inst)
    }
}
impl From<OpHitObjectGetCurrentTimeEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetCurrentTimeEXT) -> Self {
        Self::HitObjectGetCurrentTimeEXT(inst)
    }
}
impl From<OpHitObjectGetAttributesEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetAttributesEXT) -> Self {
        Self::HitObjectGetAttributesEXT(inst)
    }
}
impl From<OpHitObjectGetHitKindEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetHitKindEXT) -> Self {
        Self::HitObjectGetHitKindEXT(inst)
    }
}
impl From<OpHitObjectGetPrimitiveIndexEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetPrimitiveIndexEXT) -> Self {
        Self::HitObjectGetPrimitiveIndexEXT(inst)
    }
}
impl From<OpHitObjectGetGeometryIndexEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetGeometryIndexEXT) -> Self {
        Self::HitObjectGetGeometryIndexEXT(inst)
    }
}
impl From<OpHitObjectGetInstanceIdEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetInstanceIdEXT) -> Self {
        Self::HitObjectGetInstanceIdEXT(inst)
    }
}
impl From<OpHitObjectGetInstanceCustomIndexEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetInstanceCustomIndexEXT) -> Self {
        Self::HitObjectGetInstanceCustomIndexEXT(inst)
    }
}
impl From<OpHitObjectGetObjectRayOriginEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetObjectRayOriginEXT) -> Self {
        Self::HitObjectGetObjectRayOriginEXT(inst)
    }
}
impl From<OpHitObjectGetObjectRayDirectionEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetObjectRayDirectionEXT) -> Self {
        Self::HitObjectGetObjectRayDirectionEXT(inst)
    }
}
impl From<OpHitObjectGetWorldRayDirectionEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetWorldRayDirectionEXT) -> Self {
        Self::HitObjectGetWorldRayDirectionEXT(inst)
    }
}
impl From<OpHitObjectGetWorldRayOriginEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetWorldRayOriginEXT) -> Self {
        Self::HitObjectGetWorldRayOriginEXT(inst)
    }
}
impl From<OpHitObjectGetObjectToWorldEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetObjectToWorldEXT) -> Self {
        Self::HitObjectGetObjectToWorldEXT(inst)
    }
}
impl From<OpHitObjectGetWorldToObjectEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetWorldToObjectEXT) -> Self {
        Self::HitObjectGetWorldToObjectEXT(inst)
    }
}
impl From<OpHitObjectGetRayTMaxEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetRayTMaxEXT) -> Self {
        Self::HitObjectGetRayTMaxEXT(inst)
    }
}
impl From<OpReportIntersectionKHR> for CoreInstSet {
    fn from(inst: OpReportIntersectionKHR) -> Self {
        Self::ReportIntersectionKHR(inst)
    }
}
impl From<OpIgnoreIntersectionNV> for CoreInstSet {
    fn from(inst: OpIgnoreIntersectionNV) -> Self {
        Self::IgnoreIntersectionNV(inst)
    }
}
impl From<OpTerminateRayNV> for CoreInstSet {
    fn from(inst: OpTerminateRayNV) -> Self {
        Self::TerminateRayNV(inst)
    }
}
impl From<OpTraceNV> for CoreInstSet {
    fn from(inst: OpTraceNV) -> Self {
        Self::TraceNV(inst)
    }
}
impl From<OpTraceMotionNV> for CoreInstSet {
    fn from(inst: OpTraceMotionNV) -> Self {
        Self::TraceMotionNV(inst)
    }
}
impl From<OpTraceRayMotionNV> for CoreInstSet {
    fn from(inst: OpTraceRayMotionNV) -> Self {
        Self::TraceRayMotionNV(inst)
    }
}
impl From<OpRayQueryGetIntersectionTriangleVertexPositionsKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionTriangleVertexPositionsKHR) -> Self {
        Self::RayQueryGetIntersectionTriangleVertexPositionsKHR(inst)
    }
}
impl From<OpTypeAccelerationStructureKHR> for CoreInstSet {
    fn from(inst: OpTypeAccelerationStructureKHR) -> Self {
        Self::TypeAccelerationStructureKHR(inst)
    }
}
impl From<OpExecuteCallableNV> for CoreInstSet {
    fn from(inst: OpExecuteCallableNV) -> Self {
        Self::ExecuteCallableNV(inst)
    }
}
impl From<OpRayQueryGetIntersectionClusterIdNV> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionClusterIdNV) -> Self {
        Self::RayQueryGetIntersectionClusterIdNV(inst)
    }
}
impl From<OpHitObjectGetClusterIdNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetClusterIdNV) -> Self {
        Self::HitObjectGetClusterIdNV(inst)
    }
}
impl From<OpHitObjectGetRayTMinEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetRayTMinEXT) -> Self {
        Self::HitObjectGetRayTMinEXT(inst)
    }
}
impl From<OpHitObjectGetShaderBindingTableRecordIndexEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetShaderBindingTableRecordIndexEXT) -> Self {
        Self::HitObjectGetShaderBindingTableRecordIndexEXT(inst)
    }
}
impl From<OpHitObjectGetShaderRecordBufferHandleEXT> for CoreInstSet {
    fn from(inst: OpHitObjectGetShaderRecordBufferHandleEXT) -> Self {
        Self::HitObjectGetShaderRecordBufferHandleEXT(inst)
    }
}
impl From<OpHitObjectIsEmptyEXT> for CoreInstSet {
    fn from(inst: OpHitObjectIsEmptyEXT) -> Self {
        Self::HitObjectIsEmptyEXT(inst)
    }
}
impl From<OpHitObjectIsHitEXT> for CoreInstSet {
    fn from(inst: OpHitObjectIsHitEXT) -> Self {
        Self::HitObjectIsHitEXT(inst)
    }
}
impl From<OpHitObjectIsMissEXT> for CoreInstSet {
    fn from(inst: OpHitObjectIsMissEXT) -> Self {
        Self::HitObjectIsMissEXT(inst)
    }
}
impl From<OpTypeCooperativeMatrixNV> for CoreInstSet {
    fn from(inst: OpTypeCooperativeMatrixNV) -> Self {
        Self::TypeCooperativeMatrixNV(inst)
    }
}
impl From<OpCooperativeMatrixLoadNV> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixLoadNV) -> Self {
        Self::CooperativeMatrixLoadNV(inst)
    }
}
impl From<OpCooperativeMatrixStoreNV> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixStoreNV) -> Self {
        Self::CooperativeMatrixStoreNV(inst)
    }
}
impl From<OpCooperativeMatrixMulAddNV> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixMulAddNV) -> Self {
        Self::CooperativeMatrixMulAddNV(inst)
    }
}
impl From<OpCooperativeMatrixLengthNV> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixLengthNV) -> Self {
        Self::CooperativeMatrixLengthNV(inst)
    }
}
impl From<OpBeginInvocationInterlockEXT> for CoreInstSet {
    fn from(inst: OpBeginInvocationInterlockEXT) -> Self {
        Self::BeginInvocationInterlockEXT(inst)
    }
}
impl From<OpEndInvocationInterlockEXT> for CoreInstSet {
    fn from(inst: OpEndInvocationInterlockEXT) -> Self {
        Self::EndInvocationInterlockEXT(inst)
    }
}
impl From<OpCooperativeMatrixReduceNV> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixReduceNV) -> Self {
        Self::CooperativeMatrixReduceNV(inst)
    }
}
impl From<OpCooperativeMatrixLoadTensorNV> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixLoadTensorNV) -> Self {
        Self::CooperativeMatrixLoadTensorNV(inst)
    }
}
impl From<OpCooperativeMatrixStoreTensorNV> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixStoreTensorNV) -> Self {
        Self::CooperativeMatrixStoreTensorNV(inst)
    }
}
impl From<OpCooperativeMatrixPerElementOpNV> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixPerElementOpNV) -> Self {
        Self::CooperativeMatrixPerElementOpNV(inst)
    }
}
impl From<OpTypeTensorLayoutNV> for CoreInstSet {
    fn from(inst: OpTypeTensorLayoutNV) -> Self {
        Self::TypeTensorLayoutNV(inst)
    }
}
impl From<OpTypeTensorViewNV> for CoreInstSet {
    fn from(inst: OpTypeTensorViewNV) -> Self {
        Self::TypeTensorViewNV(inst)
    }
}
impl From<OpCreateTensorLayoutNV> for CoreInstSet {
    fn from(inst: OpCreateTensorLayoutNV) -> Self {
        Self::CreateTensorLayoutNV(inst)
    }
}
impl From<OpTensorLayoutSetDimensionNV> for CoreInstSet {
    fn from(inst: OpTensorLayoutSetDimensionNV) -> Self {
        Self::TensorLayoutSetDimensionNV(inst)
    }
}
impl From<OpTensorLayoutSetStrideNV> for CoreInstSet {
    fn from(inst: OpTensorLayoutSetStrideNV) -> Self {
        Self::TensorLayoutSetStrideNV(inst)
    }
}
impl From<OpTensorLayoutSliceNV> for CoreInstSet {
    fn from(inst: OpTensorLayoutSliceNV) -> Self {
        Self::TensorLayoutSliceNV(inst)
    }
}
impl From<OpTensorLayoutSetClampValueNV> for CoreInstSet {
    fn from(inst: OpTensorLayoutSetClampValueNV) -> Self {
        Self::TensorLayoutSetClampValueNV(inst)
    }
}
impl From<OpCreateTensorViewNV> for CoreInstSet {
    fn from(inst: OpCreateTensorViewNV) -> Self {
        Self::CreateTensorViewNV(inst)
    }
}
impl From<OpTensorViewSetDimensionNV> for CoreInstSet {
    fn from(inst: OpTensorViewSetDimensionNV) -> Self {
        Self::TensorViewSetDimensionNV(inst)
    }
}
impl From<OpTensorViewSetStrideNV> for CoreInstSet {
    fn from(inst: OpTensorViewSetStrideNV) -> Self {
        Self::TensorViewSetStrideNV(inst)
    }
}
impl From<OpDemoteToHelperInvocation> for CoreInstSet {
    fn from(inst: OpDemoteToHelperInvocation) -> Self {
        Self::DemoteToHelperInvocation(inst)
    }
}
impl From<OpIsHelperInvocationEXT> for CoreInstSet {
    fn from(inst: OpIsHelperInvocationEXT) -> Self {
        Self::IsHelperInvocationEXT(inst)
    }
}
impl From<OpTensorViewSetClipNV> for CoreInstSet {
    fn from(inst: OpTensorViewSetClipNV) -> Self {
        Self::TensorViewSetClipNV(inst)
    }
}
impl From<OpTensorLayoutSetBlockSizeNV> for CoreInstSet {
    fn from(inst: OpTensorLayoutSetBlockSizeNV) -> Self {
        Self::TensorLayoutSetBlockSizeNV(inst)
    }
}
impl From<OpCooperativeMatrixTransposeNV> for CoreInstSet {
    fn from(inst: OpCooperativeMatrixTransposeNV) -> Self {
        Self::CooperativeMatrixTransposeNV(inst)
    }
}
impl From<OpConvertUToImageNV> for CoreInstSet {
    fn from(inst: OpConvertUToImageNV) -> Self {
        Self::ConvertUToImageNV(inst)
    }
}
impl From<OpConvertUToSamplerNV> for CoreInstSet {
    fn from(inst: OpConvertUToSamplerNV) -> Self {
        Self::ConvertUToSamplerNV(inst)
    }
}
impl From<OpConvertImageToUNV> for CoreInstSet {
    fn from(inst: OpConvertImageToUNV) -> Self {
        Self::ConvertImageToUNV(inst)
    }
}
impl From<OpConvertSamplerToUNV> for CoreInstSet {
    fn from(inst: OpConvertSamplerToUNV) -> Self {
        Self::ConvertSamplerToUNV(inst)
    }
}
impl From<OpConvertUToSampledImageNV> for CoreInstSet {
    fn from(inst: OpConvertUToSampledImageNV) -> Self {
        Self::ConvertUToSampledImageNV(inst)
    }
}
impl From<OpConvertSampledImageToUNV> for CoreInstSet {
    fn from(inst: OpConvertSampledImageToUNV) -> Self {
        Self::ConvertSampledImageToUNV(inst)
    }
}
impl From<OpSamplerImageAddressingModeNV> for CoreInstSet {
    fn from(inst: OpSamplerImageAddressingModeNV) -> Self {
        Self::SamplerImageAddressingModeNV(inst)
    }
}
impl From<OpRawAccessChainNV> for CoreInstSet {
    fn from(inst: OpRawAccessChainNV) -> Self {
        Self::RawAccessChainNV(inst)
    }
}
impl From<OpRayQueryGetIntersectionSpherePositionNV> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionSpherePositionNV) -> Self {
        Self::RayQueryGetIntersectionSpherePositionNV(inst)
    }
}
impl From<OpRayQueryGetIntersectionSphereRadiusNV> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionSphereRadiusNV) -> Self {
        Self::RayQueryGetIntersectionSphereRadiusNV(inst)
    }
}
impl From<OpRayQueryGetIntersectionLSSPositionsNV> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionLSSPositionsNV) -> Self {
        Self::RayQueryGetIntersectionLSSPositionsNV(inst)
    }
}
impl From<OpRayQueryGetIntersectionLSSRadiiNV> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionLSSRadiiNV) -> Self {
        Self::RayQueryGetIntersectionLSSRadiiNV(inst)
    }
}
impl From<OpRayQueryGetIntersectionLSSHitValueNV> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionLSSHitValueNV) -> Self {
        Self::RayQueryGetIntersectionLSSHitValueNV(inst)
    }
}
impl From<OpHitObjectGetSpherePositionNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetSpherePositionNV) -> Self {
        Self::HitObjectGetSpherePositionNV(inst)
    }
}
impl From<OpHitObjectGetSphereRadiusNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetSphereRadiusNV) -> Self {
        Self::HitObjectGetSphereRadiusNV(inst)
    }
}
impl From<OpHitObjectGetLSSPositionsNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetLSSPositionsNV) -> Self {
        Self::HitObjectGetLSSPositionsNV(inst)
    }
}
impl From<OpHitObjectGetLSSRadiiNV> for CoreInstSet {
    fn from(inst: OpHitObjectGetLSSRadiiNV) -> Self {
        Self::HitObjectGetLSSRadiiNV(inst)
    }
}
impl From<OpHitObjectIsSphereHitNV> for CoreInstSet {
    fn from(inst: OpHitObjectIsSphereHitNV) -> Self {
        Self::HitObjectIsSphereHitNV(inst)
    }
}
impl From<OpHitObjectIsLSSHitNV> for CoreInstSet {
    fn from(inst: OpHitObjectIsLSSHitNV) -> Self {
        Self::HitObjectIsLSSHitNV(inst)
    }
}
impl From<OpRayQueryIsSphereHitNV> for CoreInstSet {
    fn from(inst: OpRayQueryIsSphereHitNV) -> Self {
        Self::RayQueryIsSphereHitNV(inst)
    }
}
impl From<OpRayQueryIsLSSHitNV> for CoreInstSet {
    fn from(inst: OpRayQueryIsLSSHitNV) -> Self {
        Self::RayQueryIsLSSHitNV(inst)
    }
}
impl From<OpSubgroupShuffleINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupShuffleINTEL) -> Self {
        Self::SubgroupShuffleINTEL(inst)
    }
}
impl From<OpSubgroupShuffleDownINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupShuffleDownINTEL) -> Self {
        Self::SubgroupShuffleDownINTEL(inst)
    }
}
impl From<OpSubgroupShuffleUpINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupShuffleUpINTEL) -> Self {
        Self::SubgroupShuffleUpINTEL(inst)
    }
}
impl From<OpSubgroupShuffleXorINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupShuffleXorINTEL) -> Self {
        Self::SubgroupShuffleXorINTEL(inst)
    }
}
impl From<OpSubgroupBlockReadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupBlockReadINTEL) -> Self {
        Self::SubgroupBlockReadINTEL(inst)
    }
}
impl From<OpSubgroupBlockWriteINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupBlockWriteINTEL) -> Self {
        Self::SubgroupBlockWriteINTEL(inst)
    }
}
impl From<OpSubgroupImageBlockReadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupImageBlockReadINTEL) -> Self {
        Self::SubgroupImageBlockReadINTEL(inst)
    }
}
impl From<OpSubgroupImageBlockWriteINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupImageBlockWriteINTEL) -> Self {
        Self::SubgroupImageBlockWriteINTEL(inst)
    }
}
impl From<OpSubgroupImageMediaBlockReadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupImageMediaBlockReadINTEL) -> Self {
        Self::SubgroupImageMediaBlockReadINTEL(inst)
    }
}
impl From<OpSubgroupImageMediaBlockWriteINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupImageMediaBlockWriteINTEL) -> Self {
        Self::SubgroupImageMediaBlockWriteINTEL(inst)
    }
}
impl From<OpUCountLeadingZerosINTEL> for CoreInstSet {
    fn from(inst: OpUCountLeadingZerosINTEL) -> Self {
        Self::UCountLeadingZerosINTEL(inst)
    }
}
impl From<OpUCountTrailingZerosINTEL> for CoreInstSet {
    fn from(inst: OpUCountTrailingZerosINTEL) -> Self {
        Self::UCountTrailingZerosINTEL(inst)
    }
}
impl From<OpAbsISubINTEL> for CoreInstSet {
    fn from(inst: OpAbsISubINTEL) -> Self {
        Self::AbsISubINTEL(inst)
    }
}
impl From<OpAbsUSubINTEL> for CoreInstSet {
    fn from(inst: OpAbsUSubINTEL) -> Self {
        Self::AbsUSubINTEL(inst)
    }
}
impl From<OpIAddSatINTEL> for CoreInstSet {
    fn from(inst: OpIAddSatINTEL) -> Self {
        Self::IAddSatINTEL(inst)
    }
}
impl From<OpUAddSatINTEL> for CoreInstSet {
    fn from(inst: OpUAddSatINTEL) -> Self {
        Self::UAddSatINTEL(inst)
    }
}
impl From<OpIAverageINTEL> for CoreInstSet {
    fn from(inst: OpIAverageINTEL) -> Self {
        Self::IAverageINTEL(inst)
    }
}
impl From<OpUAverageINTEL> for CoreInstSet {
    fn from(inst: OpUAverageINTEL) -> Self {
        Self::UAverageINTEL(inst)
    }
}
impl From<OpIAverageRoundedINTEL> for CoreInstSet {
    fn from(inst: OpIAverageRoundedINTEL) -> Self {
        Self::IAverageRoundedINTEL(inst)
    }
}
impl From<OpUAverageRoundedINTEL> for CoreInstSet {
    fn from(inst: OpUAverageRoundedINTEL) -> Self {
        Self::UAverageRoundedINTEL(inst)
    }
}
impl From<OpISubSatINTEL> for CoreInstSet {
    fn from(inst: OpISubSatINTEL) -> Self {
        Self::ISubSatINTEL(inst)
    }
}
impl From<OpUSubSatINTEL> for CoreInstSet {
    fn from(inst: OpUSubSatINTEL) -> Self {
        Self::USubSatINTEL(inst)
    }
}
impl From<OpIMul32x16INTEL> for CoreInstSet {
    fn from(inst: OpIMul32x16INTEL) -> Self {
        Self::IMul32x16INTEL(inst)
    }
}
impl From<OpUMul32x16INTEL> for CoreInstSet {
    fn from(inst: OpUMul32x16INTEL) -> Self {
        Self::UMul32x16INTEL(inst)
    }
}
impl From<OpConstantFunctionPointerINTEL> for CoreInstSet {
    fn from(inst: OpConstantFunctionPointerINTEL) -> Self {
        Self::ConstantFunctionPointerINTEL(inst)
    }
}
impl From<OpFunctionPointerCallINTEL> for CoreInstSet {
    fn from(inst: OpFunctionPointerCallINTEL) -> Self {
        Self::FunctionPointerCallINTEL(inst)
    }
}
impl From<OpAsmTargetINTEL> for CoreInstSet {
    fn from(inst: OpAsmTargetINTEL) -> Self {
        Self::AsmTargetINTEL(inst)
    }
}
impl From<OpAsmINTEL> for CoreInstSet {
    fn from(inst: OpAsmINTEL) -> Self {
        Self::AsmINTEL(inst)
    }
}
impl From<OpAsmCallINTEL> for CoreInstSet {
    fn from(inst: OpAsmCallINTEL) -> Self {
        Self::AsmCallINTEL(inst)
    }
}
impl From<OpAtomicFMinEXT> for CoreInstSet {
    fn from(inst: OpAtomicFMinEXT) -> Self {
        Self::AtomicFMinEXT(inst)
    }
}
impl From<OpAtomicFMaxEXT> for CoreInstSet {
    fn from(inst: OpAtomicFMaxEXT) -> Self {
        Self::AtomicFMaxEXT(inst)
    }
}
impl From<OpAssumeTrueKHR> for CoreInstSet {
    fn from(inst: OpAssumeTrueKHR) -> Self {
        Self::AssumeTrueKHR(inst)
    }
}
impl From<OpExpectKHR> for CoreInstSet {
    fn from(inst: OpExpectKHR) -> Self {
        Self::ExpectKHR(inst)
    }
}
impl From<OpDecorateString> for CoreInstSet {
    fn from(inst: OpDecorateString) -> Self {
        Self::DecorateString(inst)
    }
}
impl From<OpMemberDecorateString> for CoreInstSet {
    fn from(inst: OpMemberDecorateString) -> Self {
        Self::MemberDecorateString(inst)
    }
}
impl From<OpVmeImageINTEL> for CoreInstSet {
    fn from(inst: OpVmeImageINTEL) -> Self {
        Self::VmeImageINTEL(inst)
    }
}
impl From<OpTypeVmeImageINTEL> for CoreInstSet {
    fn from(inst: OpTypeVmeImageINTEL) -> Self {
        Self::TypeVmeImageINTEL(inst)
    }
}
impl From<OpTypeAvcImePayloadINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcImePayloadINTEL) -> Self {
        Self::TypeAvcImePayloadINTEL(inst)
    }
}
impl From<OpTypeAvcRefPayloadINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcRefPayloadINTEL) -> Self {
        Self::TypeAvcRefPayloadINTEL(inst)
    }
}
impl From<OpTypeAvcSicPayloadINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcSicPayloadINTEL) -> Self {
        Self::TypeAvcSicPayloadINTEL(inst)
    }
}
impl From<OpTypeAvcMcePayloadINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcMcePayloadINTEL) -> Self {
        Self::TypeAvcMcePayloadINTEL(inst)
    }
}
impl From<OpTypeAvcMceResultINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcMceResultINTEL) -> Self {
        Self::TypeAvcMceResultINTEL(inst)
    }
}
impl From<OpTypeAvcImeResultINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcImeResultINTEL) -> Self {
        Self::TypeAvcImeResultINTEL(inst)
    }
}
impl From<OpTypeAvcImeResultSingleReferenceStreamoutINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcImeResultSingleReferenceStreamoutINTEL) -> Self {
        Self::TypeAvcImeResultSingleReferenceStreamoutINTEL(inst)
    }
}
impl From<OpTypeAvcImeResultDualReferenceStreamoutINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcImeResultDualReferenceStreamoutINTEL) -> Self {
        Self::TypeAvcImeResultDualReferenceStreamoutINTEL(inst)
    }
}
impl From<OpTypeAvcImeSingleReferenceStreaminINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcImeSingleReferenceStreaminINTEL) -> Self {
        Self::TypeAvcImeSingleReferenceStreaminINTEL(inst)
    }
}
impl From<OpTypeAvcImeDualReferenceStreaminINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcImeDualReferenceStreaminINTEL) -> Self {
        Self::TypeAvcImeDualReferenceStreaminINTEL(inst)
    }
}
impl From<OpTypeAvcRefResultINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcRefResultINTEL) -> Self {
        Self::TypeAvcRefResultINTEL(inst)
    }
}
impl From<OpTypeAvcSicResultINTEL> for CoreInstSet {
    fn from(inst: OpTypeAvcSicResultINTEL) -> Self {
        Self::TypeAvcSicResultINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL) -> Self {
        Self::SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceSetInterShapePenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceSetInterShapePenaltyINTEL) -> Self {
        Self::SubgroupAvcMceSetInterShapePenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceSetInterDirectionPenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceSetInterDirectionPenaltyINTEL) -> Self {
        Self::SubgroupAvcMceSetInterDirectionPenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL) -> Self {
        Self::SubgroupAvcMceSetMotionVectorCostFunctionINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL) -> Self {
        Self::SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceSetAcOnlyHaarINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceSetAcOnlyHaarINTEL) -> Self {
        Self::SubgroupAvcMceSetAcOnlyHaarINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL) -> Self {
        Self::SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL) -> Self {
        Self::SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL) -> Self {
        Self::SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceConvertToImePayloadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceConvertToImePayloadINTEL) -> Self {
        Self::SubgroupAvcMceConvertToImePayloadINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceConvertToImeResultINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceConvertToImeResultINTEL) -> Self {
        Self::SubgroupAvcMceConvertToImeResultINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceConvertToRefPayloadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceConvertToRefPayloadINTEL) -> Self {
        Self::SubgroupAvcMceConvertToRefPayloadINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceConvertToRefResultINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceConvertToRefResultINTEL) -> Self {
        Self::SubgroupAvcMceConvertToRefResultINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceConvertToSicPayloadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceConvertToSicPayloadINTEL) -> Self {
        Self::SubgroupAvcMceConvertToSicPayloadINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceConvertToSicResultINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceConvertToSicResultINTEL) -> Self {
        Self::SubgroupAvcMceConvertToSicResultINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetMotionVectorsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetMotionVectorsINTEL) -> Self {
        Self::SubgroupAvcMceGetMotionVectorsINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetInterDistortionsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetInterDistortionsINTEL) -> Self {
        Self::SubgroupAvcMceGetInterDistortionsINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetBestInterDistortionsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetBestInterDistortionsINTEL) -> Self {
        Self::SubgroupAvcMceGetBestInterDistortionsINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetInterMajorShapeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetInterMajorShapeINTEL) -> Self {
        Self::SubgroupAvcMceGetInterMajorShapeINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetInterMinorShapeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetInterMinorShapeINTEL) -> Self {
        Self::SubgroupAvcMceGetInterMinorShapeINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetInterDirectionsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetInterDirectionsINTEL) -> Self {
        Self::SubgroupAvcMceGetInterDirectionsINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetInterMotionVectorCountINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetInterMotionVectorCountINTEL) -> Self {
        Self::SubgroupAvcMceGetInterMotionVectorCountINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetInterReferenceIdsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetInterReferenceIdsINTEL) -> Self {
        Self::SubgroupAvcMceGetInterReferenceIdsINTEL(inst)
    }
}
impl From<OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL) -> Self {
        Self::SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeInitializeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeInitializeINTEL) -> Self {
        Self::SubgroupAvcImeInitializeINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeSetSingleReferenceINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeSetSingleReferenceINTEL) -> Self {
        Self::SubgroupAvcImeSetSingleReferenceINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeSetDualReferenceINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeSetDualReferenceINTEL) -> Self {
        Self::SubgroupAvcImeSetDualReferenceINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeRefWindowSizeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeRefWindowSizeINTEL) -> Self {
        Self::SubgroupAvcImeRefWindowSizeINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeAdjustRefOffsetINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeAdjustRefOffsetINTEL) -> Self {
        Self::SubgroupAvcImeAdjustRefOffsetINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeConvertToMcePayloadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeConvertToMcePayloadINTEL) -> Self {
        Self::SubgroupAvcImeConvertToMcePayloadINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeSetMaxMotionVectorCountINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeSetMaxMotionVectorCountINTEL) -> Self {
        Self::SubgroupAvcImeSetMaxMotionVectorCountINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL) -> Self {
        Self::SubgroupAvcImeSetUnidirectionalMixDisableINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL) -> Self {
        Self::SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeSetWeightedSadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeSetWeightedSadINTEL) -> Self {
        Self::SubgroupAvcImeSetWeightedSadINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL) -> Self {
        Self::SubgroupAvcImeEvaluateWithSingleReferenceINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeEvaluateWithDualReferenceINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeEvaluateWithDualReferenceINTEL) -> Self {
        Self::SubgroupAvcImeEvaluateWithDualReferenceINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL) -> Self {
        Self::SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL) -> Self {
        Self::SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL) -> Self {
        Self::SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL) -> Self {
        Self::SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL) -> Self {
        Self::SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL) -> Self {
        Self::SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeConvertToMceResultINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeConvertToMceResultINTEL) -> Self {
        Self::SubgroupAvcImeConvertToMceResultINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetSingleReferenceStreaminINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetSingleReferenceStreaminINTEL) -> Self {
        Self::SubgroupAvcImeGetSingleReferenceStreaminINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetDualReferenceStreaminINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetDualReferenceStreaminINTEL) -> Self {
        Self::SubgroupAvcImeGetDualReferenceStreaminINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL) -> Self {
        Self::SubgroupAvcImeStripSingleReferenceStreamoutINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeStripDualReferenceStreamoutINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeStripDualReferenceStreamoutINTEL) -> Self {
        Self::SubgroupAvcImeStripDualReferenceStreamoutINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL) -> Self {
        Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL) -> Self {
        Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL) -> Self {
        Self::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL) -> Self {
        Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL) -> Self {
        Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL) -> Self {
        Self::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetBorderReachedINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetBorderReachedINTEL) -> Self {
        Self::SubgroupAvcImeGetBorderReachedINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL) -> Self {
        Self::SubgroupAvcImeGetTruncatedSearchIndicationINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL) -> Self {
        Self::SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL) -> Self {
        Self::SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(inst)
    }
}
impl From<OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL) -> Self {
        Self::SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(inst)
    }
}
impl From<OpSubgroupAvcFmeInitializeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcFmeInitializeINTEL) -> Self {
        Self::SubgroupAvcFmeInitializeINTEL(inst)
    }
}
impl From<OpSubgroupAvcBmeInitializeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcBmeInitializeINTEL) -> Self {
        Self::SubgroupAvcBmeInitializeINTEL(inst)
    }
}
impl From<OpSubgroupAvcRefConvertToMcePayloadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcRefConvertToMcePayloadINTEL) -> Self {
        Self::SubgroupAvcRefConvertToMcePayloadINTEL(inst)
    }
}
impl From<OpSubgroupAvcRefSetBidirectionalMixDisableINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcRefSetBidirectionalMixDisableINTEL) -> Self {
        Self::SubgroupAvcRefSetBidirectionalMixDisableINTEL(inst)
    }
}
impl From<OpSubgroupAvcRefSetBilinearFilterEnableINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcRefSetBilinearFilterEnableINTEL) -> Self {
        Self::SubgroupAvcRefSetBilinearFilterEnableINTEL(inst)
    }
}
impl From<OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL) -> Self {
        Self::SubgroupAvcRefEvaluateWithSingleReferenceINTEL(inst)
    }
}
impl From<OpSubgroupAvcRefEvaluateWithDualReferenceINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcRefEvaluateWithDualReferenceINTEL) -> Self {
        Self::SubgroupAvcRefEvaluateWithDualReferenceINTEL(inst)
    }
}
impl From<OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL) -> Self {
        Self::SubgroupAvcRefEvaluateWithMultiReferenceINTEL(inst)
    }
}
impl From<OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL) -> Self {
        Self::SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(inst)
    }
}
impl From<OpSubgroupAvcRefConvertToMceResultINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcRefConvertToMceResultINTEL) -> Self {
        Self::SubgroupAvcRefConvertToMceResultINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicInitializeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicInitializeINTEL) -> Self {
        Self::SubgroupAvcSicInitializeINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicConfigureSkcINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicConfigureSkcINTEL) -> Self {
        Self::SubgroupAvcSicConfigureSkcINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicConfigureIpeLumaINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicConfigureIpeLumaINTEL) -> Self {
        Self::SubgroupAvcSicConfigureIpeLumaINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicConfigureIpeLumaChromaINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicConfigureIpeLumaChromaINTEL) -> Self {
        Self::SubgroupAvcSicConfigureIpeLumaChromaINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicGetMotionVectorMaskINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicGetMotionVectorMaskINTEL) -> Self {
        Self::SubgroupAvcSicGetMotionVectorMaskINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicConvertToMcePayloadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicConvertToMcePayloadINTEL) -> Self {
        Self::SubgroupAvcSicConvertToMcePayloadINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL) -> Self {
        Self::SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL) -> Self {
        Self::SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL) -> Self {
        Self::SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicSetBilinearFilterEnableINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicSetBilinearFilterEnableINTEL) -> Self {
        Self::SubgroupAvcSicSetBilinearFilterEnableINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL) -> Self {
        Self::SubgroupAvcSicSetSkcForwardTransformEnableINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL) -> Self {
        Self::SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicEvaluateIpeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicEvaluateIpeINTEL) -> Self {
        Self::SubgroupAvcSicEvaluateIpeINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL) -> Self {
        Self::SubgroupAvcSicEvaluateWithSingleReferenceINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicEvaluateWithDualReferenceINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicEvaluateWithDualReferenceINTEL) -> Self {
        Self::SubgroupAvcSicEvaluateWithDualReferenceINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL) -> Self {
        Self::SubgroupAvcSicEvaluateWithMultiReferenceINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL) -> Self {
        Self::SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicConvertToMceResultINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicConvertToMceResultINTEL) -> Self {
        Self::SubgroupAvcSicConvertToMceResultINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicGetIpeLumaShapeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicGetIpeLumaShapeINTEL) -> Self {
        Self::SubgroupAvcSicGetIpeLumaShapeINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL) -> Self {
        Self::SubgroupAvcSicGetBestIpeLumaDistortionINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL) -> Self {
        Self::SubgroupAvcSicGetBestIpeChromaDistortionINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicGetPackedIpeLumaModesINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicGetPackedIpeLumaModesINTEL) -> Self {
        Self::SubgroupAvcSicGetPackedIpeLumaModesINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicGetIpeChromaModeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicGetIpeChromaModeINTEL) -> Self {
        Self::SubgroupAvcSicGetIpeChromaModeINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL) -> Self {
        Self::SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL) -> Self {
        Self::SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(inst)
    }
}
impl From<OpSubgroupAvcSicGetInterRawSadsINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupAvcSicGetInterRawSadsINTEL) -> Self {
        Self::SubgroupAvcSicGetInterRawSadsINTEL(inst)
    }
}
impl From<OpVariableLengthArrayINTEL> for CoreInstSet {
    fn from(inst: OpVariableLengthArrayINTEL) -> Self {
        Self::VariableLengthArrayINTEL(inst)
    }
}
impl From<OpSaveMemoryINTEL> for CoreInstSet {
    fn from(inst: OpSaveMemoryINTEL) -> Self {
        Self::SaveMemoryINTEL(inst)
    }
}
impl From<OpRestoreMemoryINTEL> for CoreInstSet {
    fn from(inst: OpRestoreMemoryINTEL) -> Self {
        Self::RestoreMemoryINTEL(inst)
    }
}
impl From<OpArbitraryFloatSinCosPiALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatSinCosPiALTERA) -> Self {
        Self::ArbitraryFloatSinCosPiALTERA(inst)
    }
}
impl From<OpArbitraryFloatCastALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatCastALTERA) -> Self {
        Self::ArbitraryFloatCastALTERA(inst)
    }
}
impl From<OpArbitraryFloatCastFromIntALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatCastFromIntALTERA) -> Self {
        Self::ArbitraryFloatCastFromIntALTERA(inst)
    }
}
impl From<OpArbitraryFloatCastToIntALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatCastToIntALTERA) -> Self {
        Self::ArbitraryFloatCastToIntALTERA(inst)
    }
}
impl From<OpArbitraryFloatAddALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatAddALTERA) -> Self {
        Self::ArbitraryFloatAddALTERA(inst)
    }
}
impl From<OpArbitraryFloatSubALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatSubALTERA) -> Self {
        Self::ArbitraryFloatSubALTERA(inst)
    }
}
impl From<OpArbitraryFloatMulALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatMulALTERA) -> Self {
        Self::ArbitraryFloatMulALTERA(inst)
    }
}
impl From<OpArbitraryFloatDivALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatDivALTERA) -> Self {
        Self::ArbitraryFloatDivALTERA(inst)
    }
}
impl From<OpArbitraryFloatGTALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatGTALTERA) -> Self {
        Self::ArbitraryFloatGTALTERA(inst)
    }
}
impl From<OpArbitraryFloatGEALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatGEALTERA) -> Self {
        Self::ArbitraryFloatGEALTERA(inst)
    }
}
impl From<OpArbitraryFloatLTALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatLTALTERA) -> Self {
        Self::ArbitraryFloatLTALTERA(inst)
    }
}
impl From<OpArbitraryFloatLEALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatLEALTERA) -> Self {
        Self::ArbitraryFloatLEALTERA(inst)
    }
}
impl From<OpArbitraryFloatEQALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatEQALTERA) -> Self {
        Self::ArbitraryFloatEQALTERA(inst)
    }
}
impl From<OpArbitraryFloatRecipALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatRecipALTERA) -> Self {
        Self::ArbitraryFloatRecipALTERA(inst)
    }
}
impl From<OpArbitraryFloatRSqrtALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatRSqrtALTERA) -> Self {
        Self::ArbitraryFloatRSqrtALTERA(inst)
    }
}
impl From<OpArbitraryFloatCbrtALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatCbrtALTERA) -> Self {
        Self::ArbitraryFloatCbrtALTERA(inst)
    }
}
impl From<OpArbitraryFloatHypotALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatHypotALTERA) -> Self {
        Self::ArbitraryFloatHypotALTERA(inst)
    }
}
impl From<OpArbitraryFloatSqrtALTERA> for CoreInstSet {
    fn from(inst: OpArbitraryFloatSqrtALTERA) -> Self {
        Self::ArbitraryFloatSqrtALTERA(inst)
    }
}
impl From<OpArbitraryFloatLogINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatLogINTEL) -> Self {
        Self::ArbitraryFloatLogINTEL(inst)
    }
}
impl From<OpArbitraryFloatLog2INTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatLog2INTEL) -> Self {
        Self::ArbitraryFloatLog2INTEL(inst)
    }
}
impl From<OpArbitraryFloatLog10INTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatLog10INTEL) -> Self {
        Self::ArbitraryFloatLog10INTEL(inst)
    }
}
impl From<OpArbitraryFloatLog1pINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatLog1pINTEL) -> Self {
        Self::ArbitraryFloatLog1pINTEL(inst)
    }
}
impl From<OpArbitraryFloatExpINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatExpINTEL) -> Self {
        Self::ArbitraryFloatExpINTEL(inst)
    }
}
impl From<OpArbitraryFloatExp2INTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatExp2INTEL) -> Self {
        Self::ArbitraryFloatExp2INTEL(inst)
    }
}
impl From<OpArbitraryFloatExp10INTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatExp10INTEL) -> Self {
        Self::ArbitraryFloatExp10INTEL(inst)
    }
}
impl From<OpArbitraryFloatExpm1INTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatExpm1INTEL) -> Self {
        Self::ArbitraryFloatExpm1INTEL(inst)
    }
}
impl From<OpArbitraryFloatSinINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatSinINTEL) -> Self {
        Self::ArbitraryFloatSinINTEL(inst)
    }
}
impl From<OpArbitraryFloatCosINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatCosINTEL) -> Self {
        Self::ArbitraryFloatCosINTEL(inst)
    }
}
impl From<OpArbitraryFloatSinCosINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatSinCosINTEL) -> Self {
        Self::ArbitraryFloatSinCosINTEL(inst)
    }
}
impl From<OpArbitraryFloatSinPiINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatSinPiINTEL) -> Self {
        Self::ArbitraryFloatSinPiINTEL(inst)
    }
}
impl From<OpArbitraryFloatCosPiINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatCosPiINTEL) -> Self {
        Self::ArbitraryFloatCosPiINTEL(inst)
    }
}
impl From<OpArbitraryFloatASinINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatASinINTEL) -> Self {
        Self::ArbitraryFloatASinINTEL(inst)
    }
}
impl From<OpArbitraryFloatASinPiINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatASinPiINTEL) -> Self {
        Self::ArbitraryFloatASinPiINTEL(inst)
    }
}
impl From<OpArbitraryFloatACosINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatACosINTEL) -> Self {
        Self::ArbitraryFloatACosINTEL(inst)
    }
}
impl From<OpArbitraryFloatACosPiINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatACosPiINTEL) -> Self {
        Self::ArbitraryFloatACosPiINTEL(inst)
    }
}
impl From<OpArbitraryFloatATanINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatATanINTEL) -> Self {
        Self::ArbitraryFloatATanINTEL(inst)
    }
}
impl From<OpArbitraryFloatATanPiINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatATanPiINTEL) -> Self {
        Self::ArbitraryFloatATanPiINTEL(inst)
    }
}
impl From<OpArbitraryFloatATan2INTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatATan2INTEL) -> Self {
        Self::ArbitraryFloatATan2INTEL(inst)
    }
}
impl From<OpArbitraryFloatPowINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatPowINTEL) -> Self {
        Self::ArbitraryFloatPowINTEL(inst)
    }
}
impl From<OpArbitraryFloatPowRINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatPowRINTEL) -> Self {
        Self::ArbitraryFloatPowRINTEL(inst)
    }
}
impl From<OpArbitraryFloatPowNINTEL> for CoreInstSet {
    fn from(inst: OpArbitraryFloatPowNINTEL) -> Self {
        Self::ArbitraryFloatPowNINTEL(inst)
    }
}
impl From<OpLoopControlINTEL> for CoreInstSet {
    fn from(inst: OpLoopControlINTEL) -> Self {
        Self::LoopControlINTEL(inst)
    }
}
impl From<OpAliasDomainDeclINTEL> for CoreInstSet {
    fn from(inst: OpAliasDomainDeclINTEL) -> Self {
        Self::AliasDomainDeclINTEL(inst)
    }
}
impl From<OpAliasScopeDeclINTEL> for CoreInstSet {
    fn from(inst: OpAliasScopeDeclINTEL) -> Self {
        Self::AliasScopeDeclINTEL(inst)
    }
}
impl From<OpAliasScopeListDeclINTEL> for CoreInstSet {
    fn from(inst: OpAliasScopeListDeclINTEL) -> Self {
        Self::AliasScopeListDeclINTEL(inst)
    }
}
impl From<OpFixedSqrtALTERA> for CoreInstSet {
    fn from(inst: OpFixedSqrtALTERA) -> Self {
        Self::FixedSqrtALTERA(inst)
    }
}
impl From<OpFixedRecipALTERA> for CoreInstSet {
    fn from(inst: OpFixedRecipALTERA) -> Self {
        Self::FixedRecipALTERA(inst)
    }
}
impl From<OpFixedRsqrtALTERA> for CoreInstSet {
    fn from(inst: OpFixedRsqrtALTERA) -> Self {
        Self::FixedRsqrtALTERA(inst)
    }
}
impl From<OpFixedSinALTERA> for CoreInstSet {
    fn from(inst: OpFixedSinALTERA) -> Self {
        Self::FixedSinALTERA(inst)
    }
}
impl From<OpFixedCosALTERA> for CoreInstSet {
    fn from(inst: OpFixedCosALTERA) -> Self {
        Self::FixedCosALTERA(inst)
    }
}
impl From<OpFixedSinCosALTERA> for CoreInstSet {
    fn from(inst: OpFixedSinCosALTERA) -> Self {
        Self::FixedSinCosALTERA(inst)
    }
}
impl From<OpFixedSinPiALTERA> for CoreInstSet {
    fn from(inst: OpFixedSinPiALTERA) -> Self {
        Self::FixedSinPiALTERA(inst)
    }
}
impl From<OpFixedCosPiALTERA> for CoreInstSet {
    fn from(inst: OpFixedCosPiALTERA) -> Self {
        Self::FixedCosPiALTERA(inst)
    }
}
impl From<OpFixedSinCosPiALTERA> for CoreInstSet {
    fn from(inst: OpFixedSinCosPiALTERA) -> Self {
        Self::FixedSinCosPiALTERA(inst)
    }
}
impl From<OpFixedLogALTERA> for CoreInstSet {
    fn from(inst: OpFixedLogALTERA) -> Self {
        Self::FixedLogALTERA(inst)
    }
}
impl From<OpFixedExpALTERA> for CoreInstSet {
    fn from(inst: OpFixedExpALTERA) -> Self {
        Self::FixedExpALTERA(inst)
    }
}
impl From<OpPtrCastToCrossWorkgroupALTERA> for CoreInstSet {
    fn from(inst: OpPtrCastToCrossWorkgroupALTERA) -> Self {
        Self::PtrCastToCrossWorkgroupALTERA(inst)
    }
}
impl From<OpCrossWorkgroupCastToPtrALTERA> for CoreInstSet {
    fn from(inst: OpCrossWorkgroupCastToPtrALTERA) -> Self {
        Self::CrossWorkgroupCastToPtrALTERA(inst)
    }
}
impl From<OpReadPipeBlockingALTERA> for CoreInstSet {
    fn from(inst: OpReadPipeBlockingALTERA) -> Self {
        Self::ReadPipeBlockingALTERA(inst)
    }
}
impl From<OpWritePipeBlockingALTERA> for CoreInstSet {
    fn from(inst: OpWritePipeBlockingALTERA) -> Self {
        Self::WritePipeBlockingALTERA(inst)
    }
}
impl From<OpFPGARegALTERA> for CoreInstSet {
    fn from(inst: OpFPGARegALTERA) -> Self {
        Self::FPGARegALTERA(inst)
    }
}
impl From<OpRayQueryGetRayTMinKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetRayTMinKHR) -> Self {
        Self::RayQueryGetRayTMinKHR(inst)
    }
}
impl From<OpRayQueryGetRayFlagsKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetRayFlagsKHR) -> Self {
        Self::RayQueryGetRayFlagsKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionTKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionTKHR) -> Self {
        Self::RayQueryGetIntersectionTKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionInstanceCustomIndexKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionInstanceCustomIndexKHR) -> Self {
        Self::RayQueryGetIntersectionInstanceCustomIndexKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionInstanceIdKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionInstanceIdKHR) -> Self {
        Self::RayQueryGetIntersectionInstanceIdKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR) -> Self {
        Self::RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionGeometryIndexKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionGeometryIndexKHR) -> Self {
        Self::RayQueryGetIntersectionGeometryIndexKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionPrimitiveIndexKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionPrimitiveIndexKHR) -> Self {
        Self::RayQueryGetIntersectionPrimitiveIndexKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionBarycentricsKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionBarycentricsKHR) -> Self {
        Self::RayQueryGetIntersectionBarycentricsKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionFrontFaceKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionFrontFaceKHR) -> Self {
        Self::RayQueryGetIntersectionFrontFaceKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionCandidateAABBOpaqueKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionCandidateAABBOpaqueKHR) -> Self {
        Self::RayQueryGetIntersectionCandidateAABBOpaqueKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionObjectRayDirectionKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionObjectRayDirectionKHR) -> Self {
        Self::RayQueryGetIntersectionObjectRayDirectionKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionObjectRayOriginKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionObjectRayOriginKHR) -> Self {
        Self::RayQueryGetIntersectionObjectRayOriginKHR(inst)
    }
}
impl From<OpRayQueryGetWorldRayDirectionKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetWorldRayDirectionKHR) -> Self {
        Self::RayQueryGetWorldRayDirectionKHR(inst)
    }
}
impl From<OpRayQueryGetWorldRayOriginKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetWorldRayOriginKHR) -> Self {
        Self::RayQueryGetWorldRayOriginKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionObjectToWorldKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionObjectToWorldKHR) -> Self {
        Self::RayQueryGetIntersectionObjectToWorldKHR(inst)
    }
}
impl From<OpRayQueryGetIntersectionWorldToObjectKHR> for CoreInstSet {
    fn from(inst: OpRayQueryGetIntersectionWorldToObjectKHR) -> Self {
        Self::RayQueryGetIntersectionWorldToObjectKHR(inst)
    }
}
impl From<OpAtomicFAddEXT> for CoreInstSet {
    fn from(inst: OpAtomicFAddEXT) -> Self {
        Self::AtomicFAddEXT(inst)
    }
}
impl From<OpTypeBufferSurfaceINTEL> for CoreInstSet {
    fn from(inst: OpTypeBufferSurfaceINTEL) -> Self {
        Self::TypeBufferSurfaceINTEL(inst)
    }
}
impl From<OpTypeStructContinuedINTEL> for CoreInstSet {
    fn from(inst: OpTypeStructContinuedINTEL) -> Self {
        Self::TypeStructContinuedINTEL(inst)
    }
}
impl From<OpConstantCompositeContinuedINTEL> for CoreInstSet {
    fn from(inst: OpConstantCompositeContinuedINTEL) -> Self {
        Self::ConstantCompositeContinuedINTEL(inst)
    }
}
impl From<OpSpecConstantCompositeContinuedINTEL> for CoreInstSet {
    fn from(inst: OpSpecConstantCompositeContinuedINTEL) -> Self {
        Self::SpecConstantCompositeContinuedINTEL(inst)
    }
}
impl From<OpCompositeConstructContinuedINTEL> for CoreInstSet {
    fn from(inst: OpCompositeConstructContinuedINTEL) -> Self {
        Self::CompositeConstructContinuedINTEL(inst)
    }
}
impl From<OpConvertFToBF16INTEL> for CoreInstSet {
    fn from(inst: OpConvertFToBF16INTEL) -> Self {
        Self::ConvertFToBF16INTEL(inst)
    }
}
impl From<OpConvertBF16ToFINTEL> for CoreInstSet {
    fn from(inst: OpConvertBF16ToFINTEL) -> Self {
        Self::ConvertBF16ToFINTEL(inst)
    }
}
impl From<OpControlBarrierArriveINTEL> for CoreInstSet {
    fn from(inst: OpControlBarrierArriveINTEL) -> Self {
        Self::ControlBarrierArriveINTEL(inst)
    }
}
impl From<OpControlBarrierWaitINTEL> for CoreInstSet {
    fn from(inst: OpControlBarrierWaitINTEL) -> Self {
        Self::ControlBarrierWaitINTEL(inst)
    }
}
impl From<OpArithmeticFenceEXT> for CoreInstSet {
    fn from(inst: OpArithmeticFenceEXT) -> Self {
        Self::ArithmeticFenceEXT(inst)
    }
}
impl From<OpTaskSequenceCreateALTERA> for CoreInstSet {
    fn from(inst: OpTaskSequenceCreateALTERA) -> Self {
        Self::TaskSequenceCreateALTERA(inst)
    }
}
impl From<OpTaskSequenceAsyncALTERA> for CoreInstSet {
    fn from(inst: OpTaskSequenceAsyncALTERA) -> Self {
        Self::TaskSequenceAsyncALTERA(inst)
    }
}
impl From<OpTaskSequenceGetALTERA> for CoreInstSet {
    fn from(inst: OpTaskSequenceGetALTERA) -> Self {
        Self::TaskSequenceGetALTERA(inst)
    }
}
impl From<OpTaskSequenceReleaseALTERA> for CoreInstSet {
    fn from(inst: OpTaskSequenceReleaseALTERA) -> Self {
        Self::TaskSequenceReleaseALTERA(inst)
    }
}
impl From<OpTypeTaskSequenceALTERA> for CoreInstSet {
    fn from(inst: OpTypeTaskSequenceALTERA) -> Self {
        Self::TypeTaskSequenceALTERA(inst)
    }
}
impl From<OpSubgroupBlockPrefetchINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupBlockPrefetchINTEL) -> Self {
        Self::SubgroupBlockPrefetchINTEL(inst)
    }
}
impl From<OpSubgroup2DBlockLoadINTEL> for CoreInstSet {
    fn from(inst: OpSubgroup2DBlockLoadINTEL) -> Self {
        Self::Subgroup2DBlockLoadINTEL(inst)
    }
}
impl From<OpSubgroup2DBlockLoadTransformINTEL> for CoreInstSet {
    fn from(inst: OpSubgroup2DBlockLoadTransformINTEL) -> Self {
        Self::Subgroup2DBlockLoadTransformINTEL(inst)
    }
}
impl From<OpSubgroup2DBlockLoadTransposeINTEL> for CoreInstSet {
    fn from(inst: OpSubgroup2DBlockLoadTransposeINTEL) -> Self {
        Self::Subgroup2DBlockLoadTransposeINTEL(inst)
    }
}
impl From<OpSubgroup2DBlockPrefetchINTEL> for CoreInstSet {
    fn from(inst: OpSubgroup2DBlockPrefetchINTEL) -> Self {
        Self::Subgroup2DBlockPrefetchINTEL(inst)
    }
}
impl From<OpSubgroup2DBlockStoreINTEL> for CoreInstSet {
    fn from(inst: OpSubgroup2DBlockStoreINTEL) -> Self {
        Self::Subgroup2DBlockStoreINTEL(inst)
    }
}
impl From<OpSubgroupMatrixMultiplyAccumulateINTEL> for CoreInstSet {
    fn from(inst: OpSubgroupMatrixMultiplyAccumulateINTEL) -> Self {
        Self::SubgroupMatrixMultiplyAccumulateINTEL(inst)
    }
}
impl From<OpBitwiseFunctionINTEL> for CoreInstSet {
    fn from(inst: OpBitwiseFunctionINTEL) -> Self {
        Self::BitwiseFunctionINTEL(inst)
    }
}
impl From<OpUntypedVariableLengthArrayINTEL> for CoreInstSet {
    fn from(inst: OpUntypedVariableLengthArrayINTEL) -> Self {
        Self::UntypedVariableLengthArrayINTEL(inst)
    }
}
impl From<OpConditionalExtensionINTEL> for CoreInstSet {
    fn from(inst: OpConditionalExtensionINTEL) -> Self {
        Self::ConditionalExtensionINTEL(inst)
    }
}
impl From<OpConditionalEntryPointINTEL> for CoreInstSet {
    fn from(inst: OpConditionalEntryPointINTEL) -> Self {
        Self::ConditionalEntryPointINTEL(inst)
    }
}
impl From<OpConditionalCapabilityINTEL> for CoreInstSet {
    fn from(inst: OpConditionalCapabilityINTEL) -> Self {
        Self::ConditionalCapabilityINTEL(inst)
    }
}
impl From<OpSpecConstantTargetINTEL> for CoreInstSet {
    fn from(inst: OpSpecConstantTargetINTEL) -> Self {
        Self::SpecConstantTargetINTEL(inst)
    }
}
impl From<OpSpecConstantArchitectureINTEL> for CoreInstSet {
    fn from(inst: OpSpecConstantArchitectureINTEL) -> Self {
        Self::SpecConstantArchitectureINTEL(inst)
    }
}
impl From<OpSpecConstantCapabilitiesINTEL> for CoreInstSet {
    fn from(inst: OpSpecConstantCapabilitiesINTEL) -> Self {
        Self::SpecConstantCapabilitiesINTEL(inst)
    }
}
impl From<OpConditionalCopyObjectINTEL> for CoreInstSet {
    fn from(inst: OpConditionalCopyObjectINTEL) -> Self {
        Self::ConditionalCopyObjectINTEL(inst)
    }
}
impl From<OpGroupIMulKHR> for CoreInstSet {
    fn from(inst: OpGroupIMulKHR) -> Self {
        Self::GroupIMulKHR(inst)
    }
}
impl From<OpGroupFMulKHR> for CoreInstSet {
    fn from(inst: OpGroupFMulKHR) -> Self {
        Self::GroupFMulKHR(inst)
    }
}
impl From<OpGroupBitwiseAndKHR> for CoreInstSet {
    fn from(inst: OpGroupBitwiseAndKHR) -> Self {
        Self::GroupBitwiseAndKHR(inst)
    }
}
impl From<OpGroupBitwiseOrKHR> for CoreInstSet {
    fn from(inst: OpGroupBitwiseOrKHR) -> Self {
        Self::GroupBitwiseOrKHR(inst)
    }
}
impl From<OpGroupBitwiseXorKHR> for CoreInstSet {
    fn from(inst: OpGroupBitwiseXorKHR) -> Self {
        Self::GroupBitwiseXorKHR(inst)
    }
}
impl From<OpGroupLogicalAndKHR> for CoreInstSet {
    fn from(inst: OpGroupLogicalAndKHR) -> Self {
        Self::GroupLogicalAndKHR(inst)
    }
}
impl From<OpGroupLogicalOrKHR> for CoreInstSet {
    fn from(inst: OpGroupLogicalOrKHR) -> Self {
        Self::GroupLogicalOrKHR(inst)
    }
}
impl From<OpGroupLogicalXorKHR> for CoreInstSet {
    fn from(inst: OpGroupLogicalXorKHR) -> Self {
        Self::GroupLogicalXorKHR(inst)
    }
}
impl From<OpRoundFToTF32INTEL> for CoreInstSet {
    fn from(inst: OpRoundFToTF32INTEL) -> Self {
        Self::RoundFToTF32INTEL(inst)
    }
}
impl From<OpMaskedGatherINTEL> for CoreInstSet {
    fn from(inst: OpMaskedGatherINTEL) -> Self {
        Self::MaskedGatherINTEL(inst)
    }
}
impl From<OpMaskedScatterINTEL> for CoreInstSet {
    fn from(inst: OpMaskedScatterINTEL) -> Self {
        Self::MaskedScatterINTEL(inst)
    }
}
impl From<OpConvertHandleToImageINTEL> for CoreInstSet {
    fn from(inst: OpConvertHandleToImageINTEL) -> Self {
        Self::ConvertHandleToImageINTEL(inst)
    }
}
impl From<OpConvertHandleToSamplerINTEL> for CoreInstSet {
    fn from(inst: OpConvertHandleToSamplerINTEL) -> Self {
        Self::ConvertHandleToSamplerINTEL(inst)
    }
}
impl From<OpConvertHandleToSampledImageINTEL> for CoreInstSet {
    fn from(inst: OpConvertHandleToSampledImageINTEL) -> Self {
        Self::ConvertHandleToSampledImageINTEL(inst)
    }
}
