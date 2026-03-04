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
