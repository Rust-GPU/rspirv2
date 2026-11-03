use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum CoreOpSpecOpInstSet {
    AccessChain(OpAccessChain),
    InBoundsAccessChain(OpInBoundsAccessChain),
    PtrAccessChain(OpPtrAccessChain),
    InBoundsPtrAccessChain(OpInBoundsPtrAccessChain),
    VectorShuffle(OpVectorShuffle),
    CompositeExtract(OpCompositeExtract),
    CompositeInsert(OpCompositeInsert),
    ConvertFToU(OpConvertFToU),
    ConvertFToS(OpConvertFToS),
    ConvertSToF(OpConvertSToF),
    ConvertUToF(OpConvertUToF),
    UConvert(OpUConvert),
    SConvert(OpSConvert),
    FConvert(OpFConvert),
    QuantizeToF16(OpQuantizeToF16),
    ConvertPtrToU(OpConvertPtrToU),
    ConvertUToPtr(OpConvertUToPtr),
    PtrCastToGeneric(OpPtrCastToGeneric),
    GenericCastToPtr(OpGenericCastToPtr),
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
    ShiftRightLogical(OpShiftRightLogical),
    ShiftRightArithmetic(OpShiftRightArithmetic),
    ShiftLeftLogical(OpShiftLeftLogical),
    BitwiseOr(OpBitwiseOr),
    BitwiseXor(OpBitwiseXor),
    BitwiseAnd(OpBitwiseAnd),
    Not(OpNot),
}
impl InstEncoding for CoreOpSpecOpInstSet {
    type IdResult = Option<IdResult>;
    type IdResultType = Option<IdResultType>;
    fn id_result(&self) -> Self::IdResult {
        profiling::function_scope!();
        match self {
            Self::AccessChain(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::InBoundsAccessChain(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::PtrAccessChain(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::InBoundsPtrAccessChain(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::VectorShuffle(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::CompositeExtract(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::CompositeInsert(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ConvertFToU(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ConvertFToS(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ConvertSToF(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ConvertUToF(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::UConvert(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::SConvert(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::FConvert(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::QuantizeToF16(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ConvertPtrToU(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ConvertUToPtr(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::PtrCastToGeneric(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::GenericCastToPtr(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::Bitcast(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::SNegate(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::FNegate(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::IAdd(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::FAdd(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ISub(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::FSub(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::IMul(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::FMul(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::UDiv(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::SDiv(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::FDiv(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::UMod(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::SRem(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::SMod(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::FRem(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::FMod(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::LogicalEqual(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::LogicalNotEqual(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::LogicalOr(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::LogicalAnd(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::LogicalNot(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::Select(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::IEqual(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::INotEqual(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::UGreaterThan(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::SGreaterThan(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::UGreaterThanEqual(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::SGreaterThanEqual(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ULessThan(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::SLessThan(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ULessThanEqual(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::SLessThanEqual(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ShiftRightLogical(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ShiftRightArithmetic(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::ShiftLeftLogical(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::BitwiseOr(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::BitwiseXor(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::BitwiseAnd(inst) => InstEncoding::id_result(inst).to_optional(),
            Self::Not(inst) => InstEncoding::id_result(inst).to_optional(),
        }
    }
    fn id_result_type(&self) -> Self::IdResultType {
        profiling::function_scope!();
        match self {
            Self::AccessChain(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::InBoundsAccessChain(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::PtrAccessChain(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::InBoundsPtrAccessChain(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::VectorShuffle(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::CompositeExtract(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::CompositeInsert(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ConvertFToU(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ConvertFToS(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ConvertSToF(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ConvertUToF(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::UConvert(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::SConvert(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::FConvert(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::QuantizeToF16(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ConvertPtrToU(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ConvertUToPtr(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::PtrCastToGeneric(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::GenericCastToPtr(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::Bitcast(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::SNegate(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::FNegate(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::IAdd(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::FAdd(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ISub(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::FSub(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::IMul(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::FMul(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::UDiv(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::SDiv(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::FDiv(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::UMod(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::SRem(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::SMod(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::FRem(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::FMod(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::LogicalEqual(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::LogicalNotEqual(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::LogicalOr(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::LogicalAnd(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::LogicalNot(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::Select(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::IEqual(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::INotEqual(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::UGreaterThan(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::SGreaterThan(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::UGreaterThanEqual(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::SGreaterThanEqual(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ULessThan(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::SLessThan(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ULessThanEqual(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::SLessThanEqual(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ShiftRightLogical(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ShiftRightArithmetic(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::ShiftLeftLogical(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::BitwiseOr(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::BitwiseXor(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::BitwiseAnd(inst) => InstEncoding::id_result_type(inst).to_optional(),
            Self::Not(inst) => InstEncoding::id_result_type(inst).to_optional(),
        }
    }
    fn name() -> &'static str {
        stringify!(CoreOpSpecOpInstSet)
    }
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        match self {
            Self::AccessChain(inst) => InstEncoding::encode(inst, writer),
            Self::InBoundsAccessChain(inst) => InstEncoding::encode(inst, writer),
            Self::PtrAccessChain(inst) => InstEncoding::encode(inst, writer),
            Self::InBoundsPtrAccessChain(inst) => InstEncoding::encode(inst, writer),
            Self::VectorShuffle(inst) => InstEncoding::encode(inst, writer),
            Self::CompositeExtract(inst) => InstEncoding::encode(inst, writer),
            Self::CompositeInsert(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertFToU(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertFToS(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertSToF(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertUToF(inst) => InstEncoding::encode(inst, writer),
            Self::UConvert(inst) => InstEncoding::encode(inst, writer),
            Self::SConvert(inst) => InstEncoding::encode(inst, writer),
            Self::FConvert(inst) => InstEncoding::encode(inst, writer),
            Self::QuantizeToF16(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertPtrToU(inst) => InstEncoding::encode(inst, writer),
            Self::ConvertUToPtr(inst) => InstEncoding::encode(inst, writer),
            Self::PtrCastToGeneric(inst) => InstEncoding::encode(inst, writer),
            Self::GenericCastToPtr(inst) => InstEncoding::encode(inst, writer),
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
            Self::ShiftRightLogical(inst) => InstEncoding::encode(inst, writer),
            Self::ShiftRightArithmetic(inst) => InstEncoding::encode(inst, writer),
            Self::ShiftLeftLogical(inst) => InstEncoding::encode(inst, writer),
            Self::BitwiseOr(inst) => InstEncoding::encode(inst, writer),
            Self::BitwiseXor(inst) => InstEncoding::encode(inst, writer),
            Self::BitwiseAnd(inst) => InstEncoding::encode(inst, writer),
            Self::Not(inst) => InstEncoding::encode(inst, writer),
        }
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let opcode = reader.opcode();
        Ok(match opcode {
            65u16 => Self::AccessChain(<OpAccessChain as InstEncoding>::decode(reader)?),
            66u16 => {
                Self::InBoundsAccessChain(<OpInBoundsAccessChain as InstEncoding>::decode(reader)?)
            }
            67u16 => Self::PtrAccessChain(<OpPtrAccessChain as InstEncoding>::decode(reader)?),
            70u16 => Self::InBoundsPtrAccessChain(
                <OpInBoundsPtrAccessChain as InstEncoding>::decode(reader)?,
            ),
            79u16 => Self::VectorShuffle(<OpVectorShuffle as InstEncoding>::decode(reader)?),
            81u16 => Self::CompositeExtract(<OpCompositeExtract as InstEncoding>::decode(reader)?),
            82u16 => Self::CompositeInsert(<OpCompositeInsert as InstEncoding>::decode(reader)?),
            109u16 => Self::ConvertFToU(<OpConvertFToU as InstEncoding>::decode(reader)?),
            110u16 => Self::ConvertFToS(<OpConvertFToS as InstEncoding>::decode(reader)?),
            111u16 => Self::ConvertSToF(<OpConvertSToF as InstEncoding>::decode(reader)?),
            112u16 => Self::ConvertUToF(<OpConvertUToF as InstEncoding>::decode(reader)?),
            113u16 => Self::UConvert(<OpUConvert as InstEncoding>::decode(reader)?),
            114u16 => Self::SConvert(<OpSConvert as InstEncoding>::decode(reader)?),
            115u16 => Self::FConvert(<OpFConvert as InstEncoding>::decode(reader)?),
            116u16 => Self::QuantizeToF16(<OpQuantizeToF16 as InstEncoding>::decode(reader)?),
            117u16 => Self::ConvertPtrToU(<OpConvertPtrToU as InstEncoding>::decode(reader)?),
            120u16 => Self::ConvertUToPtr(<OpConvertUToPtr as InstEncoding>::decode(reader)?),
            121u16 => Self::PtrCastToGeneric(<OpPtrCastToGeneric as InstEncoding>::decode(reader)?),
            122u16 => Self::GenericCastToPtr(<OpGenericCastToPtr as InstEncoding>::decode(reader)?),
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
            164u16 => Self::LogicalEqual(<OpLogicalEqual as InstEncoding>::decode(reader)?),
            165u16 => Self::LogicalNotEqual(<OpLogicalNotEqual as InstEncoding>::decode(reader)?),
            166u16 => Self::LogicalOr(<OpLogicalOr as InstEncoding>::decode(reader)?),
            167u16 => Self::LogicalAnd(<OpLogicalAnd as InstEncoding>::decode(reader)?),
            168u16 => Self::LogicalNot(<OpLogicalNot as InstEncoding>::decode(reader)?),
            169u16 => Self::Select(<OpSelect as InstEncoding>::decode(reader)?),
            170u16 => Self::IEqual(<OpIEqual as InstEncoding>::decode(reader)?),
            171u16 => Self::INotEqual(<OpINotEqual as InstEncoding>::decode(reader)?),
            172u16 => Self::UGreaterThan(<OpUGreaterThan as InstEncoding>::decode(reader)?),
            173u16 => Self::SGreaterThan(<OpSGreaterThan as InstEncoding>::decode(reader)?),
            174u16 => {
                Self::UGreaterThanEqual(<OpUGreaterThanEqual as InstEncoding>::decode(reader)?)
            }
            175u16 => {
                Self::SGreaterThanEqual(<OpSGreaterThanEqual as InstEncoding>::decode(reader)?)
            }
            176u16 => Self::ULessThan(<OpULessThan as InstEncoding>::decode(reader)?),
            177u16 => Self::SLessThan(<OpSLessThan as InstEncoding>::decode(reader)?),
            178u16 => Self::ULessThanEqual(<OpULessThanEqual as InstEncoding>::decode(reader)?),
            179u16 => Self::SLessThanEqual(<OpSLessThanEqual as InstEncoding>::decode(reader)?),
            194u16 => {
                Self::ShiftRightLogical(<OpShiftRightLogical as InstEncoding>::decode(reader)?)
            }
            195u16 => Self::ShiftRightArithmetic(<OpShiftRightArithmetic as InstEncoding>::decode(
                reader,
            )?),
            196u16 => Self::ShiftLeftLogical(<OpShiftLeftLogical as InstEncoding>::decode(reader)?),
            197u16 => Self::BitwiseOr(<OpBitwiseOr as InstEncoding>::decode(reader)?),
            198u16 => Self::BitwiseXor(<OpBitwiseXor as InstEncoding>::decode(reader)?),
            199u16 => Self::BitwiseAnd(<OpBitwiseAnd as InstEncoding>::decode(reader)?),
            200u16 => Self::Not(<OpNot as InstEncoding>::decode(reader)?),
            _ => {
                return Err(DecodeErrorKind::UnknownOpCode { opcode }.into());
            }
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        match self {
            Self::AccessChain(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::InBoundsAccessChain(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PtrAccessChain(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::InBoundsPtrAccessChain(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::VectorShuffle(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CompositeExtract(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::CompositeInsert(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertFToU(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertFToS(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertSToF(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertUToF(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UConvert(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SConvert(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FConvert(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::QuantizeToF16(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertPtrToU(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ConvertUToPtr(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PtrCastToGeneric(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::GenericCastToPtr(inst) => InstEncoding::dis_fmt(inst, f, ctx),
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
            Self::ShiftRightLogical(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ShiftRightArithmetic(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ShiftLeftLogical(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitwiseOr(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitwiseXor(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::BitwiseAnd(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Not(inst) => InstEncoding::dis_fmt(inst, f, ctx),
        }
    }
}
impl From<OpAccessChain> for CoreOpSpecOpInstSet {
    fn from(inst: OpAccessChain) -> Self {
        Self::AccessChain(inst)
    }
}
impl From<OpInBoundsAccessChain> for CoreOpSpecOpInstSet {
    fn from(inst: OpInBoundsAccessChain) -> Self {
        Self::InBoundsAccessChain(inst)
    }
}
impl From<OpPtrAccessChain> for CoreOpSpecOpInstSet {
    fn from(inst: OpPtrAccessChain) -> Self {
        Self::PtrAccessChain(inst)
    }
}
impl From<OpInBoundsPtrAccessChain> for CoreOpSpecOpInstSet {
    fn from(inst: OpInBoundsPtrAccessChain) -> Self {
        Self::InBoundsPtrAccessChain(inst)
    }
}
impl From<OpVectorShuffle> for CoreOpSpecOpInstSet {
    fn from(inst: OpVectorShuffle) -> Self {
        Self::VectorShuffle(inst)
    }
}
impl From<OpCompositeExtract> for CoreOpSpecOpInstSet {
    fn from(inst: OpCompositeExtract) -> Self {
        Self::CompositeExtract(inst)
    }
}
impl From<OpCompositeInsert> for CoreOpSpecOpInstSet {
    fn from(inst: OpCompositeInsert) -> Self {
        Self::CompositeInsert(inst)
    }
}
impl From<OpConvertFToU> for CoreOpSpecOpInstSet {
    fn from(inst: OpConvertFToU) -> Self {
        Self::ConvertFToU(inst)
    }
}
impl From<OpConvertFToS> for CoreOpSpecOpInstSet {
    fn from(inst: OpConvertFToS) -> Self {
        Self::ConvertFToS(inst)
    }
}
impl From<OpConvertSToF> for CoreOpSpecOpInstSet {
    fn from(inst: OpConvertSToF) -> Self {
        Self::ConvertSToF(inst)
    }
}
impl From<OpConvertUToF> for CoreOpSpecOpInstSet {
    fn from(inst: OpConvertUToF) -> Self {
        Self::ConvertUToF(inst)
    }
}
impl From<OpUConvert> for CoreOpSpecOpInstSet {
    fn from(inst: OpUConvert) -> Self {
        Self::UConvert(inst)
    }
}
impl From<OpSConvert> for CoreOpSpecOpInstSet {
    fn from(inst: OpSConvert) -> Self {
        Self::SConvert(inst)
    }
}
impl From<OpFConvert> for CoreOpSpecOpInstSet {
    fn from(inst: OpFConvert) -> Self {
        Self::FConvert(inst)
    }
}
impl From<OpQuantizeToF16> for CoreOpSpecOpInstSet {
    fn from(inst: OpQuantizeToF16) -> Self {
        Self::QuantizeToF16(inst)
    }
}
impl From<OpConvertPtrToU> for CoreOpSpecOpInstSet {
    fn from(inst: OpConvertPtrToU) -> Self {
        Self::ConvertPtrToU(inst)
    }
}
impl From<OpConvertUToPtr> for CoreOpSpecOpInstSet {
    fn from(inst: OpConvertUToPtr) -> Self {
        Self::ConvertUToPtr(inst)
    }
}
impl From<OpPtrCastToGeneric> for CoreOpSpecOpInstSet {
    fn from(inst: OpPtrCastToGeneric) -> Self {
        Self::PtrCastToGeneric(inst)
    }
}
impl From<OpGenericCastToPtr> for CoreOpSpecOpInstSet {
    fn from(inst: OpGenericCastToPtr) -> Self {
        Self::GenericCastToPtr(inst)
    }
}
impl From<OpBitcast> for CoreOpSpecOpInstSet {
    fn from(inst: OpBitcast) -> Self {
        Self::Bitcast(inst)
    }
}
impl From<OpSNegate> for CoreOpSpecOpInstSet {
    fn from(inst: OpSNegate) -> Self {
        Self::SNegate(inst)
    }
}
impl From<OpFNegate> for CoreOpSpecOpInstSet {
    fn from(inst: OpFNegate) -> Self {
        Self::FNegate(inst)
    }
}
impl From<OpIAdd> for CoreOpSpecOpInstSet {
    fn from(inst: OpIAdd) -> Self {
        Self::IAdd(inst)
    }
}
impl From<OpFAdd> for CoreOpSpecOpInstSet {
    fn from(inst: OpFAdd) -> Self {
        Self::FAdd(inst)
    }
}
impl From<OpISub> for CoreOpSpecOpInstSet {
    fn from(inst: OpISub) -> Self {
        Self::ISub(inst)
    }
}
impl From<OpFSub> for CoreOpSpecOpInstSet {
    fn from(inst: OpFSub) -> Self {
        Self::FSub(inst)
    }
}
impl From<OpIMul> for CoreOpSpecOpInstSet {
    fn from(inst: OpIMul) -> Self {
        Self::IMul(inst)
    }
}
impl From<OpFMul> for CoreOpSpecOpInstSet {
    fn from(inst: OpFMul) -> Self {
        Self::FMul(inst)
    }
}
impl From<OpUDiv> for CoreOpSpecOpInstSet {
    fn from(inst: OpUDiv) -> Self {
        Self::UDiv(inst)
    }
}
impl From<OpSDiv> for CoreOpSpecOpInstSet {
    fn from(inst: OpSDiv) -> Self {
        Self::SDiv(inst)
    }
}
impl From<OpFDiv> for CoreOpSpecOpInstSet {
    fn from(inst: OpFDiv) -> Self {
        Self::FDiv(inst)
    }
}
impl From<OpUMod> for CoreOpSpecOpInstSet {
    fn from(inst: OpUMod) -> Self {
        Self::UMod(inst)
    }
}
impl From<OpSRem> for CoreOpSpecOpInstSet {
    fn from(inst: OpSRem) -> Self {
        Self::SRem(inst)
    }
}
impl From<OpSMod> for CoreOpSpecOpInstSet {
    fn from(inst: OpSMod) -> Self {
        Self::SMod(inst)
    }
}
impl From<OpFRem> for CoreOpSpecOpInstSet {
    fn from(inst: OpFRem) -> Self {
        Self::FRem(inst)
    }
}
impl From<OpFMod> for CoreOpSpecOpInstSet {
    fn from(inst: OpFMod) -> Self {
        Self::FMod(inst)
    }
}
impl From<OpLogicalEqual> for CoreOpSpecOpInstSet {
    fn from(inst: OpLogicalEqual) -> Self {
        Self::LogicalEqual(inst)
    }
}
impl From<OpLogicalNotEqual> for CoreOpSpecOpInstSet {
    fn from(inst: OpLogicalNotEqual) -> Self {
        Self::LogicalNotEqual(inst)
    }
}
impl From<OpLogicalOr> for CoreOpSpecOpInstSet {
    fn from(inst: OpLogicalOr) -> Self {
        Self::LogicalOr(inst)
    }
}
impl From<OpLogicalAnd> for CoreOpSpecOpInstSet {
    fn from(inst: OpLogicalAnd) -> Self {
        Self::LogicalAnd(inst)
    }
}
impl From<OpLogicalNot> for CoreOpSpecOpInstSet {
    fn from(inst: OpLogicalNot) -> Self {
        Self::LogicalNot(inst)
    }
}
impl From<OpSelect> for CoreOpSpecOpInstSet {
    fn from(inst: OpSelect) -> Self {
        Self::Select(inst)
    }
}
impl From<OpIEqual> for CoreOpSpecOpInstSet {
    fn from(inst: OpIEqual) -> Self {
        Self::IEqual(inst)
    }
}
impl From<OpINotEqual> for CoreOpSpecOpInstSet {
    fn from(inst: OpINotEqual) -> Self {
        Self::INotEqual(inst)
    }
}
impl From<OpUGreaterThan> for CoreOpSpecOpInstSet {
    fn from(inst: OpUGreaterThan) -> Self {
        Self::UGreaterThan(inst)
    }
}
impl From<OpSGreaterThan> for CoreOpSpecOpInstSet {
    fn from(inst: OpSGreaterThan) -> Self {
        Self::SGreaterThan(inst)
    }
}
impl From<OpUGreaterThanEqual> for CoreOpSpecOpInstSet {
    fn from(inst: OpUGreaterThanEqual) -> Self {
        Self::UGreaterThanEqual(inst)
    }
}
impl From<OpSGreaterThanEqual> for CoreOpSpecOpInstSet {
    fn from(inst: OpSGreaterThanEqual) -> Self {
        Self::SGreaterThanEqual(inst)
    }
}
impl From<OpULessThan> for CoreOpSpecOpInstSet {
    fn from(inst: OpULessThan) -> Self {
        Self::ULessThan(inst)
    }
}
impl From<OpSLessThan> for CoreOpSpecOpInstSet {
    fn from(inst: OpSLessThan) -> Self {
        Self::SLessThan(inst)
    }
}
impl From<OpULessThanEqual> for CoreOpSpecOpInstSet {
    fn from(inst: OpULessThanEqual) -> Self {
        Self::ULessThanEqual(inst)
    }
}
impl From<OpSLessThanEqual> for CoreOpSpecOpInstSet {
    fn from(inst: OpSLessThanEqual) -> Self {
        Self::SLessThanEqual(inst)
    }
}
impl From<OpShiftRightLogical> for CoreOpSpecOpInstSet {
    fn from(inst: OpShiftRightLogical) -> Self {
        Self::ShiftRightLogical(inst)
    }
}
impl From<OpShiftRightArithmetic> for CoreOpSpecOpInstSet {
    fn from(inst: OpShiftRightArithmetic) -> Self {
        Self::ShiftRightArithmetic(inst)
    }
}
impl From<OpShiftLeftLogical> for CoreOpSpecOpInstSet {
    fn from(inst: OpShiftLeftLogical) -> Self {
        Self::ShiftLeftLogical(inst)
    }
}
impl From<OpBitwiseOr> for CoreOpSpecOpInstSet {
    fn from(inst: OpBitwiseOr) -> Self {
        Self::BitwiseOr(inst)
    }
}
impl From<OpBitwiseXor> for CoreOpSpecOpInstSet {
    fn from(inst: OpBitwiseXor) -> Self {
        Self::BitwiseXor(inst)
    }
}
impl From<OpBitwiseAnd> for CoreOpSpecOpInstSet {
    fn from(inst: OpBitwiseAnd) -> Self {
        Self::BitwiseAnd(inst)
    }
}
impl From<OpNot> for CoreOpSpecOpInstSet {
    fn from(inst: OpNot) -> Self {
        Self::Not(inst)
    }
}
impl OpSpecOpInst for CoreOpSpecOpInstSet {
    fn op_spec_op_encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        match self {
            Self::AccessChain(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::InBoundsAccessChain(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::PtrAccessChain(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::InBoundsPtrAccessChain(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::VectorShuffle(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::CompositeExtract(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::CompositeInsert(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ConvertFToU(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ConvertFToS(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ConvertSToF(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ConvertUToF(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::UConvert(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::SConvert(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::FConvert(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::QuantizeToF16(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ConvertPtrToU(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ConvertUToPtr(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::PtrCastToGeneric(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::GenericCastToPtr(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::Bitcast(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::SNegate(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::FNegate(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::IAdd(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::FAdd(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ISub(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::FSub(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::IMul(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::FMul(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::UDiv(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::SDiv(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::FDiv(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::UMod(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::SRem(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::SMod(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::FRem(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::FMod(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::LogicalEqual(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::LogicalNotEqual(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::LogicalOr(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::LogicalAnd(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::LogicalNot(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::Select(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::IEqual(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::INotEqual(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::UGreaterThan(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::SGreaterThan(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::UGreaterThanEqual(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::SGreaterThanEqual(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ULessThan(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::SLessThan(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ULessThanEqual(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::SLessThanEqual(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ShiftRightLogical(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ShiftRightArithmetic(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::ShiftLeftLogical(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::BitwiseOr(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::BitwiseXor(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::BitwiseAnd(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
            Self::Not(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),
        }
    }
    fn op_spec_op_decode(reader: &[Word]) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let opcode = reader[2].to_op().0;
        Ok(match opcode {
            65u16 => Self::AccessChain(<OpAccessChain as OpSpecOpInst>::op_spec_op_decode(reader)?),
            66u16 => Self::InBoundsAccessChain(
                <OpInBoundsAccessChain as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            67u16 => Self::PtrAccessChain(<OpPtrAccessChain as OpSpecOpInst>::op_spec_op_decode(
                reader,
            )?),
            70u16 => Self::InBoundsPtrAccessChain(
                <OpInBoundsPtrAccessChain as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            79u16 => Self::VectorShuffle(<OpVectorShuffle as OpSpecOpInst>::op_spec_op_decode(
                reader,
            )?),
            81u16 => Self::CompositeExtract(
                <OpCompositeExtract as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            82u16 => Self::CompositeInsert(<OpCompositeInsert as OpSpecOpInst>::op_spec_op_decode(
                reader,
            )?),
            109u16 => {
                Self::ConvertFToU(<OpConvertFToU as OpSpecOpInst>::op_spec_op_decode(reader)?)
            }
            110u16 => {
                Self::ConvertFToS(<OpConvertFToS as OpSpecOpInst>::op_spec_op_decode(reader)?)
            }
            111u16 => {
                Self::ConvertSToF(<OpConvertSToF as OpSpecOpInst>::op_spec_op_decode(reader)?)
            }
            112u16 => {
                Self::ConvertUToF(<OpConvertUToF as OpSpecOpInst>::op_spec_op_decode(reader)?)
            }
            113u16 => Self::UConvert(<OpUConvert as OpSpecOpInst>::op_spec_op_decode(reader)?),
            114u16 => Self::SConvert(<OpSConvert as OpSpecOpInst>::op_spec_op_decode(reader)?),
            115u16 => Self::FConvert(<OpFConvert as OpSpecOpInst>::op_spec_op_decode(reader)?),
            116u16 => Self::QuantizeToF16(<OpQuantizeToF16 as OpSpecOpInst>::op_spec_op_decode(
                reader,
            )?),
            117u16 => Self::ConvertPtrToU(<OpConvertPtrToU as OpSpecOpInst>::op_spec_op_decode(
                reader,
            )?),
            120u16 => Self::ConvertUToPtr(<OpConvertUToPtr as OpSpecOpInst>::op_spec_op_decode(
                reader,
            )?),
            121u16 => Self::PtrCastToGeneric(
                <OpPtrCastToGeneric as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            122u16 => Self::GenericCastToPtr(
                <OpGenericCastToPtr as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            124u16 => Self::Bitcast(<OpBitcast as OpSpecOpInst>::op_spec_op_decode(reader)?),
            126u16 => Self::SNegate(<OpSNegate as OpSpecOpInst>::op_spec_op_decode(reader)?),
            127u16 => Self::FNegate(<OpFNegate as OpSpecOpInst>::op_spec_op_decode(reader)?),
            128u16 => Self::IAdd(<OpIAdd as OpSpecOpInst>::op_spec_op_decode(reader)?),
            129u16 => Self::FAdd(<OpFAdd as OpSpecOpInst>::op_spec_op_decode(reader)?),
            130u16 => Self::ISub(<OpISub as OpSpecOpInst>::op_spec_op_decode(reader)?),
            131u16 => Self::FSub(<OpFSub as OpSpecOpInst>::op_spec_op_decode(reader)?),
            132u16 => Self::IMul(<OpIMul as OpSpecOpInst>::op_spec_op_decode(reader)?),
            133u16 => Self::FMul(<OpFMul as OpSpecOpInst>::op_spec_op_decode(reader)?),
            134u16 => Self::UDiv(<OpUDiv as OpSpecOpInst>::op_spec_op_decode(reader)?),
            135u16 => Self::SDiv(<OpSDiv as OpSpecOpInst>::op_spec_op_decode(reader)?),
            136u16 => Self::FDiv(<OpFDiv as OpSpecOpInst>::op_spec_op_decode(reader)?),
            137u16 => Self::UMod(<OpUMod as OpSpecOpInst>::op_spec_op_decode(reader)?),
            138u16 => Self::SRem(<OpSRem as OpSpecOpInst>::op_spec_op_decode(reader)?),
            139u16 => Self::SMod(<OpSMod as OpSpecOpInst>::op_spec_op_decode(reader)?),
            140u16 => Self::FRem(<OpFRem as OpSpecOpInst>::op_spec_op_decode(reader)?),
            141u16 => Self::FMod(<OpFMod as OpSpecOpInst>::op_spec_op_decode(reader)?),
            164u16 => {
                Self::LogicalEqual(<OpLogicalEqual as OpSpecOpInst>::op_spec_op_decode(reader)?)
            }
            165u16 => Self::LogicalNotEqual(
                <OpLogicalNotEqual as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            166u16 => Self::LogicalOr(<OpLogicalOr as OpSpecOpInst>::op_spec_op_decode(reader)?),
            167u16 => Self::LogicalAnd(<OpLogicalAnd as OpSpecOpInst>::op_spec_op_decode(reader)?),
            168u16 => Self::LogicalNot(<OpLogicalNot as OpSpecOpInst>::op_spec_op_decode(reader)?),
            169u16 => Self::Select(<OpSelect as OpSpecOpInst>::op_spec_op_decode(reader)?),
            170u16 => Self::IEqual(<OpIEqual as OpSpecOpInst>::op_spec_op_decode(reader)?),
            171u16 => Self::INotEqual(<OpINotEqual as OpSpecOpInst>::op_spec_op_decode(reader)?),
            172u16 => {
                Self::UGreaterThan(<OpUGreaterThan as OpSpecOpInst>::op_spec_op_decode(reader)?)
            }
            173u16 => {
                Self::SGreaterThan(<OpSGreaterThan as OpSpecOpInst>::op_spec_op_decode(reader)?)
            }
            174u16 => Self::UGreaterThanEqual(
                <OpUGreaterThanEqual as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            175u16 => Self::SGreaterThanEqual(
                <OpSGreaterThanEqual as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            176u16 => Self::ULessThan(<OpULessThan as OpSpecOpInst>::op_spec_op_decode(reader)?),
            177u16 => Self::SLessThan(<OpSLessThan as OpSpecOpInst>::op_spec_op_decode(reader)?),
            178u16 => Self::ULessThanEqual(<OpULessThanEqual as OpSpecOpInst>::op_spec_op_decode(
                reader,
            )?),
            179u16 => Self::SLessThanEqual(<OpSLessThanEqual as OpSpecOpInst>::op_spec_op_decode(
                reader,
            )?),
            194u16 => Self::ShiftRightLogical(
                <OpShiftRightLogical as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            195u16 => Self::ShiftRightArithmetic(
                <OpShiftRightArithmetic as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            196u16 => Self::ShiftLeftLogical(
                <OpShiftLeftLogical as OpSpecOpInst>::op_spec_op_decode(reader)?,
            ),
            197u16 => Self::BitwiseOr(<OpBitwiseOr as OpSpecOpInst>::op_spec_op_decode(reader)?),
            198u16 => Self::BitwiseXor(<OpBitwiseXor as OpSpecOpInst>::op_spec_op_decode(reader)?),
            199u16 => Self::BitwiseAnd(<OpBitwiseAnd as OpSpecOpInst>::op_spec_op_decode(reader)?),
            200u16 => Self::Not(<OpNot as OpSpecOpInst>::op_spec_op_decode(reader)?),
            _ => {
                return Err(DecodeErrorKind::UnknownOpCode { opcode }.into());
            }
        })
    }
}
