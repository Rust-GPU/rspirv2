use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpNop {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUndef {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSourceContinued {
    pub continued_source: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSource {
    pub source_language: SourceLanguage,
    pub version: LiteralInteger,
    pub file: Option<IdRef>,
    pub source: Option<LiteralString>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSourceExtension {
    pub extension: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpName {
    pub target: IdRef,
    pub name: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMemberName {
    pub ty: IdRef,
    pub member: LiteralInteger,
    pub name: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpString {
    pub id_result: IdResult,
    pub string: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLine {
    pub file: IdRef,
    pub line: LiteralInteger,
    pub column: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpExtension {
    pub name: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpExtInstImport {
    pub id_result: IdResult,
    pub name: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpExtInst {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub set: IdRef,
    pub instruction: LiteralExtInstInteger,
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMemoryModel {
    pub addressing_model: AddressingModel,
    pub memory_model: MemoryModel,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpEntryPoint {
    pub execution_model: ExecutionModel,
    pub entry_point: IdRef,
    pub name: LiteralString,
    pub interface: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpExecutionMode {
    pub entry_point: IdRef,
    pub mode: ExecutionMode,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCapability {
    pub capability: Capability,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeVoid {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeBool {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeInt {
    pub id_result: IdResult,
    pub width: LiteralInteger,
    pub signedness: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeFloat {
    pub id_result: IdResult,
    pub width: LiteralInteger,
    pub floating_point_encoding: Option<FPEncoding>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeVector {
    pub id_result: IdResult,
    pub component_type: IdRef,
    pub component_count: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeMatrix {
    pub id_result: IdResult,
    pub column_type: IdRef,
    pub column_count: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeImage {
    pub id_result: IdResult,
    pub sampled_type: IdRef,
    pub dim: Dim,
    pub depth: LiteralInteger,
    pub arrayed: LiteralInteger,
    pub ms: LiteralInteger,
    pub sampled: LiteralInteger,
    pub image_format: ImageFormat,
    pub access_qualifier: Option<AccessQualifier>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeSampler {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeSampledImage {
    pub id_result: IdResult,
    pub image_type: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeArray {
    pub id_result: IdResult,
    pub element_type: IdRef,
    pub length: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeRuntimeArray {
    pub id_result: IdResult,
    pub element_type: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeStruct {
    pub id_result: IdResult,
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeOpaque {
    pub id_result: IdResult,
    pub literal_string: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypePointer {
    pub id_result: IdResult,
    pub storage_class: StorageClass,
    pub ty: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeFunction {
    pub id_result: IdResult,
    pub return_type: IdRef,
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeEvent {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeDeviceEvent {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeReserveId {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeQueue {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypePipe {
    pub id_result: IdResult,
    pub qualifier: AccessQualifier,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeForwardPointer {
    pub pointer_type: IdRef,
    pub storage_class: StorageClass,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantTrue {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantFalse {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstant {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub value: LiteralContextDependentNumber,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantComposite {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub constituents: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantSampler {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampler_addressing_mode: SamplerAddressingMode,
    pub param: LiteralInteger,
    pub sampler_filter_mode: SamplerFilterMode,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantNull {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantTrue {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantFalse {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstant {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub value: LiteralContextDependentNumber,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantComposite {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub constituents: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantOp {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub opcode: LiteralSpecConstantOpInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFunction {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub function_control: FunctionControl,
    pub function_type: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFunctionParameter {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFunctionEnd {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFunctionCall {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub function: IdRef,
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpVariable {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub storage_class: StorageClass,
    pub initializer: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageTexelPointer {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub coordinate: IdRef,
    pub sample: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLoad {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory_access: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpStore {
    pub pointer: IdRef,
    pub object: IdRef,
    pub memory_access: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCopyMemory {
    pub target: IdRef,
    pub source: IdRef,
    pub memory_access_0: Option<MemoryAccess>,
    pub memory_access_1: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCopyMemorySized {
    pub target: IdRef,
    pub source: IdRef,
    pub size: IdRef,
    pub memory_access_0: Option<MemoryAccess>,
    pub memory_access_1: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAccessChain {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub indexes: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpInBoundsAccessChain {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub indexes: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpPtrAccessChain {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub element: IdRef,
    pub indexes: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArrayLength {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub structure: IdRef,
    pub array_member: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGenericPtrMemSemantics {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpInBoundsPtrAccessChain {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub element: IdRef,
    pub indexes: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDecorate {
    pub target: IdRef,
    pub decoration: Decoration,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMemberDecorate {
    pub structure_type: IdRef,
    pub member: LiteralInteger,
    pub decoration: Decoration,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDecorationGroup {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupDecorate {
    pub decoration_group: IdRef,
    pub targets: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupMemberDecorate {
    pub decoration_group: IdRef,
    pub targets: SmallVec<[PairIdRefLiteralInteger; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpVectorExtractDynamic {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector: IdRef,
    pub index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpVectorInsertDynamic {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector: IdRef,
    pub component: IdRef,
    pub index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpVectorShuffle {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector_1: IdRef,
    pub vector_2: IdRef,
    pub components: SmallVec<[LiteralInteger; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCompositeConstruct {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub constituents: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCompositeExtract {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub composite: IdRef,
    pub indexes: SmallVec<[LiteralInteger; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCompositeInsert {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub object: IdRef,
    pub composite: IdRef,
    pub indexes: SmallVec<[LiteralInteger; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCopyObject {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTranspose {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub matrix: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSampledImage {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub sampler: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSampleImplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSampleExplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: ImageOperands,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSampleDrefImplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub id_ref: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSampleDrefExplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub id_ref: IdRef,
    pub image_operands: ImageOperands,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSampleProjImplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSampleProjExplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: ImageOperands,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSampleProjDrefImplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub id_ref: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSampleProjDrefExplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub id_ref: IdRef,
    pub image_operands: ImageOperands,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageFetch {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageGather {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub component: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageDrefGather {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub id_ref: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageRead {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageWrite {
    pub image: IdRef,
    pub coordinate: IdRef,
    pub texel: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImage {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageQueryFormat {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageQueryOrder {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageQuerySizeLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub level_of_detail: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageQuerySize {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageQueryLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageQueryLevels {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageQuerySamples {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertFToU {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub float_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertFToS {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub float_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertSToF {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub signed_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertUToF {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub unsigned_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUConvert {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub unsigned_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSConvert {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub signed_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFConvert {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub float_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpQuantizeToF16 {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertPtrToU {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSatConvertSToU {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub signed_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSatConvertUToS {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub unsigned_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertUToPtr {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub integer_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpPtrCastToGeneric {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGenericCastToPtr {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGenericCastToPtrExplicit {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub storage: StorageClass,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitcast {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSNegate {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFNegate {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIAdd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFAdd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpISub {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFSub {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIMul {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFMul {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUDiv {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSDiv {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFDiv {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUMod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSRem {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSMod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFRem {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFMod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpVectorTimesScalar {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector: IdRef,
    pub scalar: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMatrixTimesScalar {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub matrix: IdRef,
    pub scalar: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpVectorTimesMatrix {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector: IdRef,
    pub matrix: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMatrixTimesVector {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub matrix: IdRef,
    pub vector: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMatrixTimesMatrix {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub left_matrix: IdRef,
    pub right_matrix: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpOuterProduct {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector_1: IdRef,
    pub vector_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDot {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector_1: IdRef,
    pub vector_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIAddCarry {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpISubBorrow {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUMulExtended {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSMulExtended {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAny {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAll {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIsNan {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIsInf {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIsFinite {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIsNormal {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSignBitSet {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLessOrGreater {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpOrdered {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUnordered {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLogicalEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLogicalNotEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLogicalOr {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLogicalAnd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLogicalNot {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSelect {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub condition: IdRef,
    pub object_1: IdRef,
    pub object_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpINotEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUGreaterThan {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSGreaterThan {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUGreaterThanEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSGreaterThanEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpULessThan {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSLessThan {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpULessThanEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSLessThanEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFOrdEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFUnordEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFOrdNotEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFUnordNotEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFOrdLessThan {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFUnordLessThan {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFOrdGreaterThan {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFUnordGreaterThan {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFOrdLessThanEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFUnordLessThanEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFOrdGreaterThanEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFUnordGreaterThanEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpShiftRightLogical {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub shift: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpShiftRightArithmetic {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub shift: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpShiftLeftLogical {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub shift: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitwiseOr {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitwiseXor {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitwiseAnd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpNot {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitFieldInsert {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub insert: IdRef,
    pub offset: IdRef,
    pub count: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitFieldSExtract {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub offset: IdRef,
    pub count: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitFieldUExtract {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub offset: IdRef,
    pub count: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitReverse {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitCount {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDPdx {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDPdy {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFwidth {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDPdxFine {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDPdyFine {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFwidthFine {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDPdxCoarse {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDPdyCoarse {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFwidthCoarse {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpEmitVertex {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpEndPrimitive {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpEmitStreamVertex {
    pub stream: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpEndStreamPrimitive {
    pub stream: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpControlBarrier {
    pub execution: IdScope,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMemoryBarrier {
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicLoad {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicStore {
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicExchange {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicCompareExchange {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub equal: IdMemorySemantics,
    pub unequal: IdMemorySemantics,
    pub value: IdRef,
    pub comparator: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicCompareExchangeWeak {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub equal: IdMemorySemantics,
    pub unequal: IdMemorySemantics,
    pub value: IdRef,
    pub comparator: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicIIncrement {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicIDecrement {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicIAdd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicISub {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicSMin {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicUMin {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicSMax {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicUMax {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicAnd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicOr {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicXor {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpPhi {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pair_id_ref_id_ref: SmallVec<[PairIdRefIdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLoopMerge {
    pub merge_block: IdRef,
    pub continue_target: IdRef,
    pub loop_control: LoopControl,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSelectionMerge {
    pub merge_block: IdRef,
    pub selection_control: SelectionControl,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLabel {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBranch {
    pub target_label: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBranchConditional {
    pub condition: IdRef,
    pub true_label: IdRef,
    pub false_label: IdRef,
    pub branch_weights: SmallVec<[LiteralInteger; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSwitch {
    pub selector: IdRef,
    pub default: IdRef,
    pub target: SmallVec<[PairLiteralIntegerIdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpKill {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReturn {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReturnValue {
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUnreachable {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLifetimeStart {
    pub pointer: IdRef,
    pub size: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLifetimeStop {
    pub pointer: IdRef,
    pub size: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupAsyncCopy {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub destination: IdRef,
    pub source: IdRef,
    pub num_elements: IdRef,
    pub stride: IdRef,
    pub event: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupWaitEvents {
    pub execution: IdScope,
    pub num_events: IdRef,
    pub events_list: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupAll {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupAny {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupBroadcast {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
    pub local_id: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupIAdd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupFAdd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupFMin {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupUMin {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupSMin {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupFMax {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupUMax {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupSMax {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReadPipe {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pipe: IdRef,
    pub pointer: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpWritePipe {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pipe: IdRef,
    pub pointer: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReservedReadPipe {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pipe: IdRef,
    pub reserve_id: IdRef,
    pub index: IdRef,
    pub pointer: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReservedWritePipe {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pipe: IdRef,
    pub reserve_id: IdRef,
    pub index: IdRef,
    pub pointer: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReserveReadPipePackets {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pipe: IdRef,
    pub num_packets: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReserveWritePipePackets {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pipe: IdRef,
    pub num_packets: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCommitReadPipe {
    pub pipe: IdRef,
    pub reserve_id: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCommitWritePipe {
    pub pipe: IdRef,
    pub reserve_id: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIsValidReserveId {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub reserve_id: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGetNumPipePackets {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pipe: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGetMaxPipePackets {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pipe: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupReserveReadPipePackets {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub pipe: IdRef,
    pub num_packets: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupReserveWritePipePackets {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub pipe: IdRef,
    pub num_packets: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupCommitReadPipe {
    pub execution: IdScope,
    pub pipe: IdRef,
    pub reserve_id: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupCommitWritePipe {
    pub execution: IdScope,
    pub pipe: IdRef,
    pub reserve_id: IdRef,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpEnqueueMarker {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub queue: IdRef,
    pub num_events: IdRef,
    pub wait_events: IdRef,
    pub ret_event: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpEnqueueKernel {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub queue: IdRef,
    pub flags: IdRef,
    pub nd_range: IdRef,
    pub num_events: IdRef,
    pub wait_events: IdRef,
    pub ret_event: IdRef,
    pub invoke: IdRef,
    pub param: IdRef,
    pub param_size: IdRef,
    pub param_align: IdRef,
    pub local_size: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGetKernelNDrangeSubGroupCount {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub nd_range: IdRef,
    pub invoke: IdRef,
    pub param: IdRef,
    pub param_size: IdRef,
    pub param_align: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGetKernelNDrangeMaxSubGroupSize {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub nd_range: IdRef,
    pub invoke: IdRef,
    pub param: IdRef,
    pub param_size: IdRef,
    pub param_align: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGetKernelWorkGroupSize {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub invoke: IdRef,
    pub param: IdRef,
    pub param_size: IdRef,
    pub param_align: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGetKernelPreferredWorkGroupSizeMultiple {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub invoke: IdRef,
    pub param: IdRef,
    pub param_size: IdRef,
    pub param_align: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRetainEvent {
    pub event: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReleaseEvent {
    pub event: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCreateUserEvent {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIsValidEvent {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub event: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSetUserEventStatus {
    pub event: IdRef,
    pub status: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCaptureEventProfilingInfo {
    pub event: IdRef,
    pub profiling_info: IdRef,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGetDefaultQueue {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBuildNDRange {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub global_work_size: IdRef,
    pub local_work_size: IdRef,
    pub global_work_offset: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseSampleImplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseSampleExplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: ImageOperands,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseSampleDrefImplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub id_ref: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseSampleDrefExplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub id_ref: IdRef,
    pub image_operands: ImageOperands,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseSampleProjImplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseSampleProjExplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: ImageOperands,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseSampleProjDrefImplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub id_ref: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseSampleProjDrefExplicitLod {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub id_ref: IdRef,
    pub image_operands: ImageOperands,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseFetch {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseGather {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub component: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseDrefGather {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub id_ref: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseTexelsResident {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub resident_code: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpNoLine {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicFlagTestAndSet {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicFlagClear {
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSparseRead {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub coordinate: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSizeOf {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypePipeStorage {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantPipeStorage {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub packet_size: LiteralInteger,
    pub packet_alignment: LiteralInteger,
    pub capacity: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCreatePipeFromPipeStorage {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pipe_storage: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGetKernelLocalSizeForSubgroupCount {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub subgroup_count: IdRef,
    pub invoke: IdRef,
    pub param: IdRef,
    pub param_size: IdRef,
    pub param_align: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGetKernelMaxNumSubgroups {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub invoke: IdRef,
    pub param: IdRef,
    pub param_size: IdRef,
    pub param_align: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeNamedBarrier {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpNamedBarrierInitialize {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub subgroup_count: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMemoryNamedBarrier {
    pub named_barrier: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpModuleProcessed {
    pub process: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpExecutionModeId {
    pub entry_point: IdRef,
    pub mode: ExecutionMode,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDecorateId {
    pub target: IdRef,
    pub decoration: Decoration,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformElect {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformAll {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformAny {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformAllEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformBroadcast {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
    pub invocation_id: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformBroadcastFirst {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformBallot {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformInverseBallot {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformBallotBitExtract {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
    pub index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformBallotBitCount {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformBallotFindLSB {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformBallotFindMSB {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformShuffle {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
    pub invocation_id: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformShuffleXor {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
    pub mask: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformShuffleUp {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
    pub delta: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformShuffleDown {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
    pub delta: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformIAdd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformFAdd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformIMul {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformFMul {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformSMin {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformUMin {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformFMin {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformSMax {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformUMax {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformFMax {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformBitwiseAnd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformBitwiseOr {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformBitwiseXor {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformLogicalAnd {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformLogicalOr {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformLogicalXor {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub value: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformQuadBroadcast {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
    pub index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformQuadSwap {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
    pub direction: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCopyLogical {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpPtrEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpPtrNotEqual {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpPtrDiff {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpColorAttachmentReadEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub attachment: IdRef,
    pub sample: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDepthAttachmentReadEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sample: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpStencilAttachmentReadEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sample: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeTensorARM {
    pub id_result: IdResult,
    pub element_type: IdRef,
    pub rank: Option<IdRef>,
    pub shape: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorReadARM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub tensor: IdRef,
    pub coordinates: IdRef,
    pub tensor_operands: Option<TensorOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorWriteARM {
    pub tensor: IdRef,
    pub coordinates: IdRef,
    pub object: IdRef,
    pub tensor_operands: Option<TensorOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorQuerySizeARM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub tensor: IdRef,
    pub dimension: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGraphConstantARM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub graph_constant_id: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGraphEntryPointARM {
    pub graph: IdRef,
    pub name: LiteralString,
    pub interface: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGraphARM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGraphInputARM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input_index: IdRef,
    pub element_index: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGraphSetOutputARM {
    pub value: IdRef,
    pub output_index: IdRef,
    pub element_index: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGraphEndARM {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeGraphARM {
    pub id_result: IdResult,
    pub num_inputs: LiteralInteger,
    pub in_out_types: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTerminateInvocation {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeUntypedPointerKHR {
    pub id_result: IdResult,
    pub storage_class: StorageClass,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUntypedVariableKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub storage_class: StorageClass,
    pub data_type: Option<IdRef>,
    pub initializer: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUntypedAccessChainKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base_type: IdRef,
    pub base: IdRef,
    pub indexes: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUntypedInBoundsAccessChainKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base_type: IdRef,
    pub base: IdRef,
    pub indexes: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupBallotKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupFirstInvocationKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUntypedPtrAccessChainKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base_type: IdRef,
    pub base: IdRef,
    pub element: IdRef,
    pub indexes: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUntypedInBoundsPtrAccessChainKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base_type: IdRef,
    pub base: IdRef,
    pub element: IdRef,
    pub indexes: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUntypedArrayLengthKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub structure: IdRef,
    pub pointer: IdRef,
    pub array_member: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUntypedPrefetchKHR {
    pub pointer_type: IdRef,
    pub num_bytes: IdRef,
    pub rw: Option<IdRef>,
    pub locality: Option<IdRef>,
    pub cache_type: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFmaKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
    pub operand_3: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAllKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAnyKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAllEqualKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformRotateKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub value: IdRef,
    pub delta: IdRef,
    pub cluster_size: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupReadInvocationKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub value: IdRef,
    pub index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpExtInstWithForwardRefsKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub set: IdRef,
    pub instruction: LiteralExtInstInteger,
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUntypedGroupAsyncCopyKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdRef,
    pub destination: IdRef,
    pub source: IdRef,
    pub element_num_bytes: IdRef,
    pub num_elements: IdRef,
    pub stride: IdRef,
    pub event: IdRef,
    pub destination_memory_operands: Option<MemoryAccess>,
    pub source_memory_operands: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTraceRayKHR {
    pub accel: IdRef,
    pub ray_flags: IdRef,
    pub cull_mask: IdRef,
    pub sbt_offset: IdRef,
    pub sbt_stride: IdRef,
    pub miss_index: IdRef,
    pub ray_origin: IdRef,
    pub ray_tmin: IdRef,
    pub ray_direction: IdRef,
    pub ray_tmax: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpExecuteCallableKHR {
    pub sbt_index: IdRef,
    pub callable_data: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertUToAccelerationStructureKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub accel: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIgnoreIntersectionKHR {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTerminateRayKHR {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSDot {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector_1: IdRef,
    pub vector_2: IdRef,
    pub packed_vector_format: Option<PackedVectorFormat>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUDot {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector_1: IdRef,
    pub vector_2: IdRef,
    pub packed_vector_format: Option<PackedVectorFormat>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSUDot {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector_1: IdRef,
    pub vector_2: IdRef,
    pub packed_vector_format: Option<PackedVectorFormat>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSDotAccSat {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector_1: IdRef,
    pub vector_2: IdRef,
    pub accumulator: IdRef,
    pub packed_vector_format: Option<PackedVectorFormat>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUDotAccSat {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector_1: IdRef,
    pub vector_2: IdRef,
    pub accumulator: IdRef,
    pub packed_vector_format: Option<PackedVectorFormat>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSUDotAccSat {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub vector_1: IdRef,
    pub vector_2: IdRef,
    pub accumulator: IdRef,
    pub packed_vector_format: Option<PackedVectorFormat>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeCooperativeMatrixKHR {
    pub id_result: IdResult,
    pub component_type: IdRef,
    pub scope: IdScope,
    pub rows: IdRef,
    pub columns: IdRef,
    pub usage: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixLoadKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory_layout: IdRef,
    pub stride: Option<IdRef>,
    pub memory_operand: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixStoreKHR {
    pub pointer: IdRef,
    pub object: IdRef,
    pub memory_layout: IdRef,
    pub stride: Option<IdRef>,
    pub memory_operand: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixMulAddKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub b: IdRef,
    pub c: IdRef,
    pub cooperative_matrix_operands: Option<CooperativeMatrixOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixLengthKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ty: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantCompositeReplicateEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantCompositeReplicateEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCompositeConstructReplicateEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeRayQueryKHR {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryInitializeKHR {
    pub ray_query: IdRef,
    pub accel: IdRef,
    pub ray_flags: IdRef,
    pub cull_mask: IdRef,
    pub ray_origin: IdRef,
    pub ray_t_min: IdRef,
    pub ray_direction: IdRef,
    pub ray_t_max: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryTerminateKHR {
    pub ray_query: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGenerateIntersectionKHR {
    pub ray_query: IdRef,
    pub hit_t: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryConfirmIntersectionKHR {
    pub ray_query: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryProceedKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionTypeKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSampleWeightedQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub texture: IdRef,
    pub coordinates: IdRef,
    pub weights: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageBoxFilterQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub texture: IdRef,
    pub coordinates: IdRef,
    pub box_size: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageBlockMatchSSDQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub target: IdRef,
    pub target_coordinates: IdRef,
    pub reference: IdRef,
    pub reference_coordinates: IdRef,
    pub block_size: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageBlockMatchSADQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub target: IdRef,
    pub target_coordinates: IdRef,
    pub reference: IdRef,
    pub reference_coordinates: IdRef,
    pub block_size: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitCastArrayQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub source_array: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageBlockMatchWindowSSDQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub target_sampled_image: IdRef,
    pub target_coordinates: IdRef,
    pub reference_sampled_image: IdRef,
    pub reference_coordinates: IdRef,
    pub block_size: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageBlockMatchWindowSADQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub target_sampled_image: IdRef,
    pub target_coordinates: IdRef,
    pub reference_sampled_image: IdRef,
    pub reference_coordinates: IdRef,
    pub block_size: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageBlockMatchGatherSSDQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub target_sampled_image: IdRef,
    pub target_coordinates: IdRef,
    pub reference_sampled_image: IdRef,
    pub reference_coordinates: IdRef,
    pub block_size: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageBlockMatchGatherSADQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub target_sampled_image: IdRef,
    pub target_coordinates: IdRef,
    pub reference_sampled_image: IdRef,
    pub reference_coordinates: IdRef,
    pub block_size: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCompositeConstructCoopMatQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub source_array: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCompositeExtractCoopMatQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub source_cooperative_matrix: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpExtractSubArrayQCOM {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub source_array: IdRef,
    pub index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupIAddNonUniformAMD {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupFAddNonUniformAMD {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupFMinNonUniformAMD {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupUMinNonUniformAMD {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupSMinNonUniformAMD {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupFMaxNonUniformAMD {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupUMaxNonUniformAMD {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupSMaxNonUniformAMD {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFragmentMaskFetchAMD {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub coordinate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFragmentFetchAMD {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub coordinate: IdRef,
    pub fragment_index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReadClockKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub scope: IdScope,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAllocateNodePayloadsAMDX {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub visibility: IdScope,
    pub payload_count: IdRef,
    pub node_index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpEnqueueNodePayloadsAMDX {
    pub payload_array: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeNodePayloadArrayAMDX {
    pub id_result: IdResult,
    pub payload_type: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFinishWritingNodePayloadAMDX {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpNodePayloadArrayLengthAMDX {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload_array: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIsNodePayloadValidAMDX {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload_type: IdRef,
    pub node_index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantStringAMDX {
    pub id_result: IdResult,
    pub literal_string: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantStringAMDX {
    pub id_result: IdResult,
    pub literal_string: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformQuadAllKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformQuadAnyKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub predicate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeBufferEXT {
    pub id_result: IdResult,
    pub storage_class: StorageClass,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBufferPointerEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub buffer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUntypedImageTexelPointerEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image_type: IdRef,
    pub image: IdRef,
    pub coordinate: IdRef,
    pub sample: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMemberDecorateIdEXT {
    pub structure_type: IdRef,
    pub member: LiteralInteger,
    pub decoration: Decoration,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantSizeOfEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ty: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordHitMotionNV {
    pub hit_object: IdRef,
    pub acceleration_structure: IdRef,
    pub instance_id: IdRef,
    pub primitive_id: IdRef,
    pub geometry_index: IdRef,
    pub hit_kind: IdRef,
    pub sbt_record_offset: IdRef,
    pub sbt_record_stride: IdRef,
    pub origin: IdRef,
    pub t_min: IdRef,
    pub direction: IdRef,
    pub t_max: IdRef,
    pub current_time: IdRef,
    pub hit_object_attributes: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordHitWithIndexMotionNV {
    pub hit_object: IdRef,
    pub acceleration_structure: IdRef,
    pub instance_id: IdRef,
    pub primitive_id: IdRef,
    pub geometry_index: IdRef,
    pub hit_kind: IdRef,
    pub sbt_record_index: IdRef,
    pub origin: IdRef,
    pub t_min: IdRef,
    pub direction: IdRef,
    pub t_max: IdRef,
    pub current_time: IdRef,
    pub hit_object_attributes: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordMissMotionNV {
    pub hit_object: IdRef,
    pub sbt_index: IdRef,
    pub origin: IdRef,
    pub t_min: IdRef,
    pub direction: IdRef,
    pub t_max: IdRef,
    pub current_time: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetWorldToObjectNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetObjectToWorldNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetObjectRayDirectionNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetObjectRayOriginNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectTraceRayMotionNV {
    pub hit_object: IdRef,
    pub acceleration_structure: IdRef,
    pub ray_flags: IdRef,
    pub cullmask: IdRef,
    pub sbt_record_offset: IdRef,
    pub sbt_record_stride: IdRef,
    pub miss_index: IdRef,
    pub origin: IdRef,
    pub t_min: IdRef,
    pub direction: IdRef,
    pub t_max: IdRef,
    pub time: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetShaderRecordBufferHandleNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetShaderBindingTableRecordIndexNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordEmptyNV {
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectTraceRayNV {
    pub hit_object: IdRef,
    pub acceleration_structure: IdRef,
    pub ray_flags: IdRef,
    pub cullmask: IdRef,
    pub sbt_record_offset: IdRef,
    pub sbt_record_stride: IdRef,
    pub miss_index: IdRef,
    pub origin: IdRef,
    pub t_min: IdRef,
    pub direction: IdRef,
    pub t_max: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordHitNV {
    pub hit_object: IdRef,
    pub acceleration_structure: IdRef,
    pub instance_id: IdRef,
    pub primitive_id: IdRef,
    pub geometry_index: IdRef,
    pub hit_kind: IdRef,
    pub sbt_record_offset: IdRef,
    pub sbt_record_stride: IdRef,
    pub origin: IdRef,
    pub t_min: IdRef,
    pub direction: IdRef,
    pub t_max: IdRef,
    pub hit_object_attributes: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordHitWithIndexNV {
    pub hit_object: IdRef,
    pub acceleration_structure: IdRef,
    pub instance_id: IdRef,
    pub primitive_id: IdRef,
    pub geometry_index: IdRef,
    pub hit_kind: IdRef,
    pub sbt_record_index: IdRef,
    pub origin: IdRef,
    pub t_min: IdRef,
    pub direction: IdRef,
    pub t_max: IdRef,
    pub hit_object_attributes: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordMissNV {
    pub hit_object: IdRef,
    pub sbt_index: IdRef,
    pub origin: IdRef,
    pub t_min: IdRef,
    pub direction: IdRef,
    pub t_max: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectExecuteShaderNV {
    pub hit_object: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetCurrentTimeNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetAttributesNV {
    pub hit_object: IdRef,
    pub hit_object_attribute: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetHitKindNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetPrimitiveIndexNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetGeometryIndexNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetInstanceIdNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetInstanceCustomIndexNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetWorldRayDirectionNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetWorldRayOriginNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetRayTMaxNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetRayTMinNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectIsEmptyNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectIsHitNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectIsMissNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReorderThreadWithHitObjectNV {
    pub hit_object: IdRef,
    pub hint: Option<IdRef>,
    pub bits: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReorderThreadWithHintNV {
    pub hint: IdRef,
    pub bits: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeHitObjectNV {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpImageSampleFootprintNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sampled_image: IdRef,
    pub coordinate: IdRef,
    pub granularity: IdRef,
    pub coarse: IdRef,
    pub image_operands: Option<ImageOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeVectorIdEXT {
    pub id_result: IdResult,
    pub component_type: IdRef,
    pub component_count: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeVectorMatrixMulNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub input_interpretation: IdRef,
    pub matrix: IdRef,
    pub matrix_offset: IdRef,
    pub matrix_interpretation: IdRef,
    pub m: IdRef,
    pub k: IdRef,
    pub memory_layout: IdRef,
    pub transpose: IdRef,
    pub matrix_stride: Option<IdRef>,
    pub cooperative_matrix_operands: Option<CooperativeMatrixOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeVectorOuterProductAccumulateNV {
    pub pointer: IdRef,
    pub offset: IdRef,
    pub a: IdRef,
    pub b: IdRef,
    pub memory_layout: IdRef,
    pub matrix_interpretation: IdRef,
    pub matrix_stride: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeVectorReduceSumAccumulateNV {
    pub pointer: IdRef,
    pub offset: IdRef,
    pub v: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeVectorMatrixMulAddNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub input_interpretation: IdRef,
    pub matrix: IdRef,
    pub matrix_offset: IdRef,
    pub matrix_interpretation: IdRef,
    pub bias: IdRef,
    pub bias_offset: IdRef,
    pub bias_interpretation: IdRef,
    pub m: IdRef,
    pub k: IdRef,
    pub memory_layout: IdRef,
    pub transpose: IdRef,
    pub matrix_stride: Option<IdRef>,
    pub cooperative_matrix_operands: Option<CooperativeMatrixOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixConvertNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub matrix: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpEmitMeshTasksEXT {
    pub group_count_x: IdRef,
    pub group_count_y: IdRef,
    pub group_count_z: IdRef,
    pub payload: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSetMeshOutputsEXT {
    pub vertex_count: IdRef,
    pub primitive_count: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupNonUniformPartitionEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpWritePackedPrimitiveIndices4x8NV {
    pub index_offset: IdRef,
    pub packed_indices: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFetchMicroTriangleVertexPositionNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub accel: IdRef,
    pub instance_id: IdRef,
    pub geometry_index: IdRef,
    pub primitive_index: IdRef,
    pub barycentric: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFetchMicroTriangleVertexBarycentricNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub accel: IdRef,
    pub instance_id: IdRef,
    pub geometry_index: IdRef,
    pub primitive_index: IdRef,
    pub barycentric: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeVectorLoadNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub offset: IdRef,
    pub memory_access: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeVectorStoreNV {
    pub pointer: IdRef,
    pub offset: IdRef,
    pub object: IdRef,
    pub memory_access: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordFromQueryEXT {
    pub hit_object: IdRef,
    pub ray_query: IdRef,
    pub sbt_record_index: IdRef,
    pub hit_object_attributes: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordMissEXT {
    pub hit_object: IdRef,
    pub ray_flags: IdRef,
    pub miss_index: IdRef,
    pub ray_origin: IdRef,
    pub ray_tmin: IdRef,
    pub ray_direction: IdRef,
    pub ray_tmax: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordMissMotionEXT {
    pub hit_object: IdRef,
    pub ray_flags: IdRef,
    pub miss_index: IdRef,
    pub ray_origin: IdRef,
    pub ray_tmin: IdRef,
    pub ray_direction: IdRef,
    pub ray_tmax: IdRef,
    pub current_time: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetIntersectionTriangleVertexPositionsEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetRayFlagsEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectSetShaderBindingTableRecordIndexEXT {
    pub hit_object: IdRef,
    pub sbt_record_index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectReorderExecuteShaderEXT {
    pub hit_object: IdRef,
    pub payload: IdRef,
    pub hint: Option<IdRef>,
    pub bits: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectTraceReorderExecuteEXT {
    pub hit_object: IdRef,
    pub acceleration_structure: IdRef,
    pub ray_flags: IdRef,
    pub cull_mask: IdRef,
    pub sbt_offset: IdRef,
    pub sbt_stride: IdRef,
    pub miss_index: IdRef,
    pub ray_origin: IdRef,
    pub ray_tmin: IdRef,
    pub ray_direction: IdRef,
    pub ray_tmax: IdRef,
    pub payload: IdRef,
    pub hint: Option<IdRef>,
    pub bits: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectTraceMotionReorderExecuteEXT {
    pub hit_object: IdRef,
    pub acceleration_structure: IdRef,
    pub ray_flags: IdRef,
    pub cull_mask: IdRef,
    pub sbt_offset: IdRef,
    pub sbt_stride: IdRef,
    pub miss_index: IdRef,
    pub ray_origin: IdRef,
    pub ray_tmin: IdRef,
    pub ray_direction: IdRef,
    pub ray_tmax: IdRef,
    pub current_time: IdRef,
    pub payload: IdRef,
    pub hint: Option<IdRef>,
    pub bits: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeHitObjectEXT {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReorderThreadWithHintEXT {
    pub hint: IdRef,
    pub bits: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReorderThreadWithHitObjectEXT {
    pub hit_object: IdRef,
    pub hint: Option<IdRef>,
    pub bits: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectTraceRayEXT {
    pub hit_object: IdRef,
    pub acceleration_structure: IdRef,
    pub ray_flags: IdRef,
    pub cull_mask: IdRef,
    pub sbt_offset: IdRef,
    pub sbt_stride: IdRef,
    pub miss_index: IdRef,
    pub ray_origin: IdRef,
    pub ray_tmin: IdRef,
    pub ray_direction: IdRef,
    pub ray_tmax: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectTraceRayMotionEXT {
    pub hit_object: IdRef,
    pub acceleration_structure: IdRef,
    pub ray_flags: IdRef,
    pub cull_mask: IdRef,
    pub sbt_offset: IdRef,
    pub sbt_stride: IdRef,
    pub miss_index: IdRef,
    pub ray_origin: IdRef,
    pub ray_tmin: IdRef,
    pub ray_direction: IdRef,
    pub ray_tmax: IdRef,
    pub current_time: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectRecordEmptyEXT {
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectExecuteShaderEXT {
    pub hit_object: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetCurrentTimeEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetAttributesEXT {
    pub hit_object: IdRef,
    pub hit_object_attribute: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetHitKindEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetPrimitiveIndexEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetGeometryIndexEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetInstanceIdEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetInstanceCustomIndexEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetObjectRayOriginEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetObjectRayDirectionEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetWorldRayDirectionEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetWorldRayOriginEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetObjectToWorldEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetWorldToObjectEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetRayTMaxEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReportIntersectionKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit: IdRef,
    pub hit_kind: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIgnoreIntersectionNV {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTerminateRayNV {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTraceNV {
    pub accel: IdRef,
    pub ray_flags: IdRef,
    pub cull_mask: IdRef,
    pub sbt_offset: IdRef,
    pub sbt_stride: IdRef,
    pub miss_index: IdRef,
    pub ray_origin: IdRef,
    pub ray_tmin: IdRef,
    pub ray_direction: IdRef,
    pub ray_tmax: IdRef,
    pub payload_id: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTraceMotionNV {
    pub accel: IdRef,
    pub ray_flags: IdRef,
    pub cull_mask: IdRef,
    pub sbt_offset: IdRef,
    pub sbt_stride: IdRef,
    pub miss_index: IdRef,
    pub ray_origin: IdRef,
    pub ray_tmin: IdRef,
    pub ray_direction: IdRef,
    pub ray_tmax: IdRef,
    pub time: IdRef,
    pub payload_id: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTraceRayMotionNV {
    pub accel: IdRef,
    pub ray_flags: IdRef,
    pub cull_mask: IdRef,
    pub sbt_offset: IdRef,
    pub sbt_stride: IdRef,
    pub miss_index: IdRef,
    pub ray_origin: IdRef,
    pub ray_tmin: IdRef,
    pub ray_direction: IdRef,
    pub ray_tmax: IdRef,
    pub time: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionTriangleVertexPositionsKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAccelerationStructureKHR {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpExecuteCallableNV {
    pub sbt_index: IdRef,
    pub callable_data_id: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionClusterIdNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetClusterIdNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetRayTMinEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetShaderBindingTableRecordIndexEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetShaderRecordBufferHandleEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectIsEmptyEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectIsHitEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectIsMissEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeCooperativeMatrixNV {
    pub id_result: IdResult,
    pub component_type: IdRef,
    pub execution: IdScope,
    pub rows: IdRef,
    pub columns: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixLoadNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub stride: IdRef,
    pub column_major: IdRef,
    pub memory_access: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixStoreNV {
    pub pointer: IdRef,
    pub object: IdRef,
    pub stride: IdRef,
    pub column_major: IdRef,
    pub memory_access: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixMulAddNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub b: IdRef,
    pub c: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixLengthNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ty: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBeginInvocationInterlockEXT {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpEndInvocationInterlockEXT {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixReduceNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub matrix: IdRef,
    pub reduce: CooperativeMatrixReduce,
    pub combine_func: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixLoadTensorNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub object: IdRef,
    pub tensor_layout: IdRef,
    pub memory_operand: MemoryAccess,
    pub tensor_addressing_operands: TensorAddressingOperands,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixStoreTensorNV {
    pub pointer: IdRef,
    pub object: IdRef,
    pub tensor_layout: IdRef,
    pub memory_operand: MemoryAccess,
    pub tensor_addressing_operands: TensorAddressingOperands,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixPerElementOpNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub matrix: IdRef,
    pub func: IdRef,
    pub operands: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeTensorLayoutNV {
    pub id_result: IdResult,
    pub dim: IdRef,
    pub clamp_mode: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeTensorViewNV {
    pub id_result: IdResult,
    pub dim: IdRef,
    pub has_dimensions: IdRef,
    pub p: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCreateTensorLayoutNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorLayoutSetDimensionNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub tensor_layout: IdRef,
    pub dim: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorLayoutSetStrideNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub tensor_layout: IdRef,
    pub stride: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorLayoutSliceNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub tensor_layout: IdRef,
    pub operands: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorLayoutSetClampValueNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub tensor_layout: IdRef,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCreateTensorViewNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorViewSetDimensionNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub tensor_view: IdRef,
    pub dim: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorViewSetStrideNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub tensor_view: IdRef,
    pub stride: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDemoteToHelperInvocation {}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIsHelperInvocationEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorViewSetClipNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub tensor_view: IdRef,
    pub clip_row_offset: IdRef,
    pub clip_row_span: IdRef,
    pub clip_col_offset: IdRef,
    pub clip_col_span: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTensorLayoutSetBlockSizeNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub tensor_layout: IdRef,
    pub block_size: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCooperativeMatrixTransposeNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub matrix: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertUToImageNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertUToSamplerNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertImageToUNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertSamplerToUNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertUToSampledImageNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertSampledImageToUNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSamplerImageAddressingModeNV {
    pub bit_width: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRawAccessChainNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub base: IdRef,
    pub byte_stride: IdRef,
    pub element_index: IdRef,
    pub byte_offset: IdRef,
    pub raw_access_chain_operands: Option<RawAccessChainOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionSpherePositionNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionSphereRadiusNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionLSSPositionsNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionLSSRadiiNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionLSSHitValueNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetSpherePositionNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetSphereRadiusNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetLSSPositionsNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectGetLSSRadiiNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectIsSphereHitNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpHitObjectIsLSSHitNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub hit_object: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryIsSphereHitNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryIsLSSHitNV {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupShuffleINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub data: IdRef,
    pub invocation_id: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupShuffleDownINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub current: IdRef,
    pub next: IdRef,
    pub delta: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupShuffleUpINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub previous: IdRef,
    pub current: IdRef,
    pub delta: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupShuffleXorINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub data: IdRef,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupBlockReadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ptr: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupBlockWriteINTEL {
    pub ptr: IdRef,
    pub data: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupImageBlockReadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub coordinate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupImageBlockWriteINTEL {
    pub image: IdRef,
    pub coordinate: IdRef,
    pub data: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupImageMediaBlockReadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image: IdRef,
    pub coordinate: IdRef,
    pub width: IdRef,
    pub height: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupImageMediaBlockWriteINTEL {
    pub image: IdRef,
    pub coordinate: IdRef,
    pub width: IdRef,
    pub height: IdRef,
    pub data: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUCountLeadingZerosINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUCountTrailingZerosINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAbsISubINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAbsUSubINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIAddSatINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUAddSatINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIAverageINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUAverageINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIAverageRoundedINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUAverageRoundedINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpISubSatINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUSubSatINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpIMul32x16INTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUMul32x16INTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: IdRef,
    pub operand_2: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantFunctionPointerINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub function: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFunctionPointerCallINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand_1: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAsmTargetINTEL {
    pub id_result: IdResult,
    pub asm_target: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAsmINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub asm_type: IdRef,
    pub target: IdRef,
    pub asm_instructions: LiteralString,
    pub constraints: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAsmCallINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub asm: IdRef,
    pub argument: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicFMinEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicFMaxEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAssumeTrueKHR {
    pub condition: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpExpectKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub value: IdRef,
    pub expected_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpDecorateString {
    pub target: IdRef,
    pub decoration: Decoration,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMemberDecorateString {
    pub struct_type: IdRef,
    pub member: LiteralInteger,
    pub decoration: Decoration,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpVmeImageINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image_type: IdRef,
    pub sampler: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeVmeImageINTEL {
    pub id_result: IdResult,
    pub image_type: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcImePayloadINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcRefPayloadINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcSicPayloadINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcMcePayloadINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcMceResultINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcImeResultINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcImeResultSingleReferenceStreamoutINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcImeResultDualReferenceStreamoutINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcImeSingleReferenceStreaminINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcImeDualReferenceStreaminINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcRefResultINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeAvcSicResultINTEL {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub slice_type: IdRef,
    pub qp: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub reference_base_penalty: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub slice_type: IdRef,
    pub qp: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceSetInterShapePenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub packed_shape_penalty: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub slice_type: IdRef,
    pub qp: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceSetInterDirectionPenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub direction_cost: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub slice_type: IdRef,
    pub qp: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub slice_type: IdRef,
    pub qp: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub packed_cost_center_delta: IdRef,
    pub packed_cost_table: IdRef,
    pub cost_precision: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub slice_type: IdRef,
    pub qp: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceSetAcOnlyHaarINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub source_field_polarity: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub reference_field_polarity: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub forward_reference_field_polarity: IdRef,
    pub backward_reference_field_polarity: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceConvertToImePayloadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceConvertToImeResultINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceConvertToRefPayloadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceConvertToRefResultINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceConvertToSicPayloadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceConvertToSicResultINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetMotionVectorsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetInterDistortionsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetBestInterDistortionsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetInterMajorShapeINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetInterMinorShapeINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetInterDirectionsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetInterMotionVectorCountINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetInterReferenceIdsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub packed_reference_ids: IdRef,
    pub packed_reference_parameter_field_polarities: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeInitializeINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_coord: IdRef,
    pub partition_mask: IdRef,
    pub sad_adjustment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeSetSingleReferenceINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ref_offset: IdRef,
    pub search_window_config: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeSetDualReferenceINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub fwd_ref_offset: IdRef,
    pub bwd_ref_offset: IdRef,
    pub search_window_config: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeRefWindowSizeINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub search_window_config: IdRef,
    pub dual_ref: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeAdjustRefOffsetINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ref_offset: IdRef,
    pub src_coord: IdRef,
    pub ref_window_size: IdRef,
    pub image_size: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeConvertToMcePayloadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeSetMaxMotionVectorCountINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub max_motion_vector_count: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub threshold: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeSetWeightedSadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub packed_sad_weights: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub ref_image: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeEvaluateWithDualReferenceINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub fwd_ref_image: IdRef,
    pub bwd_ref_image: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub ref_image: IdRef,
    pub payload: IdRef,
    pub streamin_components: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub fwd_ref_image: IdRef,
    pub bwd_ref_image: IdRef,
    pub payload: IdRef,
    pub streamin_components: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub ref_image: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub fwd_ref_image: IdRef,
    pub bwd_ref_image: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub ref_image: IdRef,
    pub payload: IdRef,
    pub streamin_components: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub fwd_ref_image: IdRef,
    pub bwd_ref_image: IdRef,
    pub payload: IdRef,
    pub streamin_components: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeConvertToMceResultINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetSingleReferenceStreaminINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetDualReferenceStreaminINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeStripDualReferenceStreamoutINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
    pub major_shape: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
    pub major_shape: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
    pub major_shape: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
    pub major_shape: IdRef,
    pub direction: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
    pub major_shape: IdRef,
    pub direction: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
    pub major_shape: IdRef,
    pub direction: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetBorderReachedINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub image_select: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcFmeInitializeINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_coord: IdRef,
    pub motion_vectors: IdRef,
    pub major_shapes: IdRef,
    pub minor_shapes: IdRef,
    pub direction: IdRef,
    pub pixel_resolution: IdRef,
    pub sad_adjustment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcBmeInitializeINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_coord: IdRef,
    pub motion_vectors: IdRef,
    pub major_shapes: IdRef,
    pub minor_shapes: IdRef,
    pub direction: IdRef,
    pub pixel_resolution: IdRef,
    pub bidirectional_weight: IdRef,
    pub sad_adjustment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcRefConvertToMcePayloadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcRefSetBidirectionalMixDisableINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcRefSetBilinearFilterEnableINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub ref_image: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcRefEvaluateWithDualReferenceINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub fwd_ref_image: IdRef,
    pub bwd_ref_image: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub packed_reference_ids: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub packed_reference_ids: IdRef,
    pub packed_reference_field_polarities: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcRefConvertToMceResultINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicInitializeINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_coord: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicConfigureSkcINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub skip_block_partition_type: IdRef,
    pub skip_motion_vector_mask: IdRef,
    pub motion_vectors: IdRef,
    pub bidirectional_weight: IdRef,
    pub sad_adjustment: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicConfigureIpeLumaINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub luma_intra_partition_mask: IdRef,
    pub intra_neighbour_availabilty: IdRef,
    pub left_edge_luma_pixels: IdRef,
    pub upper_left_corner_luma_pixel: IdRef,
    pub upper_edge_luma_pixels: IdRef,
    pub upper_right_edge_luma_pixels: IdRef,
    pub sad_adjustment: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicConfigureIpeLumaChromaINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub luma_intra_partition_mask: IdRef,
    pub intra_neighbour_availabilty: IdRef,
    pub left_edge_luma_pixels: IdRef,
    pub upper_left_corner_luma_pixel: IdRef,
    pub upper_edge_luma_pixels: IdRef,
    pub upper_right_edge_luma_pixels: IdRef,
    pub left_edge_chroma_pixels: IdRef,
    pub upper_left_corner_chroma_pixel: IdRef,
    pub upper_edge_chroma_pixels: IdRef,
    pub sad_adjustment: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicGetMotionVectorMaskINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub skip_block_partition_type: IdRef,
    pub direction: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicConvertToMcePayloadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub packed_shape_penalty: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub luma_mode_penalty: IdRef,
    pub luma_packed_neighbor_modes: IdRef,
    pub luma_packed_non_dc_penalty: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub chroma_mode_base_penalty: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicSetBilinearFilterEnableINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub packed_sad_coefficients: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub block_based_skip_type: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicEvaluateIpeINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub ref_image: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicEvaluateWithDualReferenceINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub fwd_ref_image: IdRef,
    pub bwd_ref_image: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub packed_reference_ids: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub src_image: IdRef,
    pub packed_reference_ids: IdRef,
    pub packed_reference_field_polarities: IdRef,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicConvertToMceResultINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicGetIpeLumaShapeINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicGetPackedIpeLumaModesINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicGetIpeChromaModeINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupAvcSicGetInterRawSadsINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub payload: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpVariableLengthArrayINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub length: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSaveMemoryINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRestoreMemoryINTEL {
    pub ptr: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatSinCosPiALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub m_result: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub rounding_accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatCastALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatCastFromIntALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub mresult: LiteralInteger,
    pub from_sign: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatCastToIntALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub to_sign: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatAddALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
    pub m_result: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatSubALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatMulALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatDivALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatGTALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatGEALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatLTALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatLEALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatEQALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatRecipALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatRSqrtALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatCbrtALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatHypotALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatSqrtALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatLogINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatLog2INTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatLog10INTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatLog1pINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatExpINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatExp2INTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatExp10INTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatExpm1INTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatSinINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatCosINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatSinCosINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatSinPiINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatCosPiINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatASinINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatASinPiINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatACosINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub m_1: LiteralInteger,
    pub mout: LiteralInteger,
    pub enable_subnormals: LiteralInteger,
    pub rounding_mode: LiteralInteger,
    pub rounding_accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatACosPiINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatATanINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatATanPiINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatATan2INTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatPowINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatPowRINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub mb: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArbitraryFloatPowNINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub ma: LiteralInteger,
    pub b: IdRef,
    pub sign_of_b: LiteralInteger,
    pub mresult: LiteralInteger,
    pub subnormal: LiteralInteger,
    pub rounding: LiteralInteger,
    pub accuracy: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpLoopControlINTEL {
    pub loop_control_parameters: SmallVec<[LiteralInteger; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAliasDomainDeclINTEL {
    pub id_result: IdResult,
    pub name: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAliasScopeDeclINTEL {
    pub id_result: IdResult,
    pub alias_domain: IdRef,
    pub name: Option<IdRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAliasScopeListDeclINTEL {
    pub id_result: IdResult,
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedSqrtALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedRecipALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedRsqrtALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedSinALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedCosALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedSinCosALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedSinPiALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedCosPiALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedSinCosPiALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedLogALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFixedExpALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
    pub s: LiteralInteger,
    pub i: LiteralInteger,
    pub r_i: LiteralInteger,
    pub q: LiteralInteger,
    pub o: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpPtrCastToCrossWorkgroupALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCrossWorkgroupCastToPtrALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpReadPipeBlockingALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpWritePipeBlockingALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub packet_size: IdRef,
    pub packet_alignment: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpFPGARegALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub input: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetRayTMinKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetRayFlagsKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionTKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionInstanceCustomIndexKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionInstanceIdKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionGeometryIndexKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionPrimitiveIndexKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionBarycentricsKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionFrontFaceKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionCandidateAABBOpaqueKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionObjectRayDirectionKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionObjectRayOriginKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetWorldRayDirectionKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetWorldRayOriginKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionObjectToWorldKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRayQueryGetIntersectionWorldToObjectKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ray_query: IdRef,
    pub intersection: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpAtomicFAddEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub pointer: IdRef,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeBufferSurfaceINTEL {
    pub id_result: IdResult,
    pub access_qualifier: AccessQualifier,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeStructContinuedINTEL {
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConstantCompositeContinuedINTEL {
    pub constituents: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantCompositeContinuedINTEL {
    pub constituents: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpCompositeConstructContinuedINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub constituents: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertFToBF16INTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub float_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertBF16ToFINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub b_float_16_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpControlBarrierArriveINTEL {
    pub execution: IdScope,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpControlBarrierWaitINTEL {
    pub execution: IdScope,
    pub memory: IdScope,
    pub semantics: IdMemorySemantics,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpArithmeticFenceEXT {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub target: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTaskSequenceCreateALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub function: IdRef,
    pub pipelined: LiteralInteger,
    pub use_stall_enable_clusters: LiteralInteger,
    pub get_capacity: LiteralInteger,
    pub async_capacity: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTaskSequenceAsyncALTERA {
    pub sequence: IdRef,
    pub arguments: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTaskSequenceGetALTERA {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub sequence: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTaskSequenceReleaseALTERA {
    pub sequence: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpTypeTaskSequenceALTERA {
    pub id_result: IdResult,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupBlockPrefetchINTEL {
    pub ptr: IdRef,
    pub num_bytes: IdRef,
    pub memory_access: Option<MemoryAccess>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroup2DBlockLoadINTEL {
    pub element_size: IdRef,
    pub block_width: IdRef,
    pub block_height: IdRef,
    pub block_count: IdRef,
    pub src_base_pointer: IdRef,
    pub memory_width: IdRef,
    pub memory_height: IdRef,
    pub memory_pitch: IdRef,
    pub coordinate: IdRef,
    pub dst_pointer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroup2DBlockLoadTransformINTEL {
    pub element_size: IdRef,
    pub block_width: IdRef,
    pub block_height: IdRef,
    pub block_count: IdRef,
    pub src_base_pointer: IdRef,
    pub memory_width: IdRef,
    pub memory_height: IdRef,
    pub memory_pitch: IdRef,
    pub coordinate: IdRef,
    pub dst_pointer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroup2DBlockLoadTransposeINTEL {
    pub element_size: IdRef,
    pub block_width: IdRef,
    pub block_height: IdRef,
    pub block_count: IdRef,
    pub src_base_pointer: IdRef,
    pub memory_width: IdRef,
    pub memory_height: IdRef,
    pub memory_pitch: IdRef,
    pub coordinate: IdRef,
    pub dst_pointer: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroup2DBlockPrefetchINTEL {
    pub element_size: IdRef,
    pub block_width: IdRef,
    pub block_height: IdRef,
    pub block_count: IdRef,
    pub src_base_pointer: IdRef,
    pub memory_width: IdRef,
    pub memory_height: IdRef,
    pub memory_pitch: IdRef,
    pub coordinate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroup2DBlockStoreINTEL {
    pub element_size: IdRef,
    pub block_width: IdRef,
    pub block_height: IdRef,
    pub block_count: IdRef,
    pub src_pointer: IdRef,
    pub dst_base_pointer: IdRef,
    pub memory_width: IdRef,
    pub memory_height: IdRef,
    pub memory_pitch: IdRef,
    pub coordinate: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSubgroupMatrixMultiplyAccumulateINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub k_dim: IdRef,
    pub matrix_a: IdRef,
    pub matrix_b: IdRef,
    pub matrix_c: IdRef,
    pub matrix_multiply_accumulate_operands: Option<MatrixMultiplyAccumulateOperands>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpBitwiseFunctionINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub a: IdRef,
    pub b: IdRef,
    pub c: IdRef,
    pub lut_index: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpUntypedVariableLengthArrayINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub element_type: IdRef,
    pub length: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConditionalExtensionINTEL {
    pub condition: IdRef,
    pub name: LiteralString,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConditionalEntryPointINTEL {
    pub condition: IdRef,
    pub execution_model: ExecutionModel,
    pub entry_point: IdRef,
    pub name: LiteralString,
    pub interface: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConditionalCapabilityINTEL {
    pub condition: IdRef,
    pub capability: Capability,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantTargetINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub target: LiteralInteger,
    pub features: SmallVec<[LiteralInteger; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantArchitectureINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub category: LiteralInteger,
    pub family: LiteralInteger,
    pub opcode: LiteralInteger,
    pub architecture: LiteralInteger,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantCapabilitiesINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub capabilities: SmallVec<[Capability; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConditionalCopyObjectINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupIMulKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupFMulKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupBitwiseAndKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupBitwiseOrKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupBitwiseXorKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupLogicalAndKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupLogicalOrKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpGroupLogicalXorKHR {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub execution: IdScope,
    pub operation: GroupOperation,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpRoundFToTF32INTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub float_value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMaskedGatherINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub ptr_vector: IdRef,
    pub alignment: LiteralInteger,
    pub mask: IdRef,
    pub fill_empty: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpMaskedScatterINTEL {
    pub input_vector: IdRef,
    pub ptr_vector: IdRef,
    pub alignment: LiteralInteger,
    pub mask: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertHandleToImageINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertHandleToSamplerINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpConvertHandleToSampledImageINTEL {
    pub id_result_type: IdResultType,
    pub id_result: IdResult,
    pub operand: IdRef,
}
