use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GlslInstSet {
    Round(Round),
    RoundEven(RoundEven),
    Trunc(Trunc),
    FAbs(FAbs),
    SAbs(SAbs),
    FSign(FSign),
    SSign(SSign),
    Floor(Floor),
    Ceil(Ceil),
    Fract(Fract),
    Radians(Radians),
    Degrees(Degrees),
    Sin(Sin),
    Cos(Cos),
    Tan(Tan),
    Asin(Asin),
    Acos(Acos),
    Atan(Atan),
    Sinh(Sinh),
    Cosh(Cosh),
    Tanh(Tanh),
    Asinh(Asinh),
    Acosh(Acosh),
    Atanh(Atanh),
    Atan2(Atan2),
    Pow(Pow),
    Exp(Exp),
    Log(Log),
    Exp2(Exp2),
    Log2(Log2),
    Sqrt(Sqrt),
    InverseSqrt(InverseSqrt),
    Determinant(Determinant),
    MatrixInverse(MatrixInverse),
    Modf(Modf),
    ModfStruct(ModfStruct),
    FMin(FMin),
    UMin(UMin),
    SMin(SMin),
    FMax(FMax),
    UMax(UMax),
    SMax(SMax),
    FClamp(FClamp),
    UClamp(UClamp),
    SClamp(SClamp),
    FMix(FMix),
    IMix(IMix),
    Step(Step),
    SmoothStep(SmoothStep),
    Fma(Fma),
    Frexp(Frexp),
    FrexpStruct(FrexpStruct),
    Ldexp(Ldexp),
    PackSnorm4x8(PackSnorm4x8),
    PackUnorm4x8(PackUnorm4x8),
    PackSnorm2x16(PackSnorm2x16),
    PackUnorm2x16(PackUnorm2x16),
    PackHalf2x16(PackHalf2x16),
    PackDouble2x32(PackDouble2x32),
    UnpackSnorm2x16(UnpackSnorm2x16),
    UnpackUnorm2x16(UnpackUnorm2x16),
    UnpackHalf2x16(UnpackHalf2x16),
    UnpackSnorm4x8(UnpackSnorm4x8),
    UnpackUnorm4x8(UnpackUnorm4x8),
    UnpackDouble2x32(UnpackDouble2x32),
    Length(Length),
    Distance(Distance),
    Cross(Cross),
    Normalize(Normalize),
    FaceForward(FaceForward),
    Reflect(Reflect),
    Refract(Refract),
    FindILsb(FindILsb),
    FindSMsb(FindSMsb),
    FindUMsb(FindUMsb),
    InterpolateAtCentroid(InterpolateAtCentroid),
    InterpolateAtSample(InterpolateAtSample),
    InterpolateAtOffset(InterpolateAtOffset),
    NMin(NMin),
    NMax(NMax),
    NClamp(NClamp),
}
impl InstEncoding for GlslInstSet {
    fn name() -> &'static str {
        stringify!(GlslInstSet)
    }
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        match self {
            Self::Round(inst) => InstEncoding::encode(inst, writer),
            Self::RoundEven(inst) => InstEncoding::encode(inst, writer),
            Self::Trunc(inst) => InstEncoding::encode(inst, writer),
            Self::FAbs(inst) => InstEncoding::encode(inst, writer),
            Self::SAbs(inst) => InstEncoding::encode(inst, writer),
            Self::FSign(inst) => InstEncoding::encode(inst, writer),
            Self::SSign(inst) => InstEncoding::encode(inst, writer),
            Self::Floor(inst) => InstEncoding::encode(inst, writer),
            Self::Ceil(inst) => InstEncoding::encode(inst, writer),
            Self::Fract(inst) => InstEncoding::encode(inst, writer),
            Self::Radians(inst) => InstEncoding::encode(inst, writer),
            Self::Degrees(inst) => InstEncoding::encode(inst, writer),
            Self::Sin(inst) => InstEncoding::encode(inst, writer),
            Self::Cos(inst) => InstEncoding::encode(inst, writer),
            Self::Tan(inst) => InstEncoding::encode(inst, writer),
            Self::Asin(inst) => InstEncoding::encode(inst, writer),
            Self::Acos(inst) => InstEncoding::encode(inst, writer),
            Self::Atan(inst) => InstEncoding::encode(inst, writer),
            Self::Sinh(inst) => InstEncoding::encode(inst, writer),
            Self::Cosh(inst) => InstEncoding::encode(inst, writer),
            Self::Tanh(inst) => InstEncoding::encode(inst, writer),
            Self::Asinh(inst) => InstEncoding::encode(inst, writer),
            Self::Acosh(inst) => InstEncoding::encode(inst, writer),
            Self::Atanh(inst) => InstEncoding::encode(inst, writer),
            Self::Atan2(inst) => InstEncoding::encode(inst, writer),
            Self::Pow(inst) => InstEncoding::encode(inst, writer),
            Self::Exp(inst) => InstEncoding::encode(inst, writer),
            Self::Log(inst) => InstEncoding::encode(inst, writer),
            Self::Exp2(inst) => InstEncoding::encode(inst, writer),
            Self::Log2(inst) => InstEncoding::encode(inst, writer),
            Self::Sqrt(inst) => InstEncoding::encode(inst, writer),
            Self::InverseSqrt(inst) => InstEncoding::encode(inst, writer),
            Self::Determinant(inst) => InstEncoding::encode(inst, writer),
            Self::MatrixInverse(inst) => InstEncoding::encode(inst, writer),
            Self::Modf(inst) => InstEncoding::encode(inst, writer),
            Self::ModfStruct(inst) => InstEncoding::encode(inst, writer),
            Self::FMin(inst) => InstEncoding::encode(inst, writer),
            Self::UMin(inst) => InstEncoding::encode(inst, writer),
            Self::SMin(inst) => InstEncoding::encode(inst, writer),
            Self::FMax(inst) => InstEncoding::encode(inst, writer),
            Self::UMax(inst) => InstEncoding::encode(inst, writer),
            Self::SMax(inst) => InstEncoding::encode(inst, writer),
            Self::FClamp(inst) => InstEncoding::encode(inst, writer),
            Self::UClamp(inst) => InstEncoding::encode(inst, writer),
            Self::SClamp(inst) => InstEncoding::encode(inst, writer),
            Self::FMix(inst) => InstEncoding::encode(inst, writer),
            Self::IMix(inst) => InstEncoding::encode(inst, writer),
            Self::Step(inst) => InstEncoding::encode(inst, writer),
            Self::SmoothStep(inst) => InstEncoding::encode(inst, writer),
            Self::Fma(inst) => InstEncoding::encode(inst, writer),
            Self::Frexp(inst) => InstEncoding::encode(inst, writer),
            Self::FrexpStruct(inst) => InstEncoding::encode(inst, writer),
            Self::Ldexp(inst) => InstEncoding::encode(inst, writer),
            Self::PackSnorm4x8(inst) => InstEncoding::encode(inst, writer),
            Self::PackUnorm4x8(inst) => InstEncoding::encode(inst, writer),
            Self::PackSnorm2x16(inst) => InstEncoding::encode(inst, writer),
            Self::PackUnorm2x16(inst) => InstEncoding::encode(inst, writer),
            Self::PackHalf2x16(inst) => InstEncoding::encode(inst, writer),
            Self::PackDouble2x32(inst) => InstEncoding::encode(inst, writer),
            Self::UnpackSnorm2x16(inst) => InstEncoding::encode(inst, writer),
            Self::UnpackUnorm2x16(inst) => InstEncoding::encode(inst, writer),
            Self::UnpackHalf2x16(inst) => InstEncoding::encode(inst, writer),
            Self::UnpackSnorm4x8(inst) => InstEncoding::encode(inst, writer),
            Self::UnpackUnorm4x8(inst) => InstEncoding::encode(inst, writer),
            Self::UnpackDouble2x32(inst) => InstEncoding::encode(inst, writer),
            Self::Length(inst) => InstEncoding::encode(inst, writer),
            Self::Distance(inst) => InstEncoding::encode(inst, writer),
            Self::Cross(inst) => InstEncoding::encode(inst, writer),
            Self::Normalize(inst) => InstEncoding::encode(inst, writer),
            Self::FaceForward(inst) => InstEncoding::encode(inst, writer),
            Self::Reflect(inst) => InstEncoding::encode(inst, writer),
            Self::Refract(inst) => InstEncoding::encode(inst, writer),
            Self::FindILsb(inst) => InstEncoding::encode(inst, writer),
            Self::FindSMsb(inst) => InstEncoding::encode(inst, writer),
            Self::FindUMsb(inst) => InstEncoding::encode(inst, writer),
            Self::InterpolateAtCentroid(inst) => InstEncoding::encode(inst, writer),
            Self::InterpolateAtSample(inst) => InstEncoding::encode(inst, writer),
            Self::InterpolateAtOffset(inst) => InstEncoding::encode(inst, writer),
            Self::NMin(inst) => InstEncoding::encode(inst, writer),
            Self::NMax(inst) => InstEncoding::encode(inst, writer),
            Self::NClamp(inst) => InstEncoding::encode(inst, writer),
        }
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        let opcode = reader.opcode();
        Ok(match opcode {
            1u16 => Self::Round(<Round as InstEncoding>::decode(reader)?),
            2u16 => Self::RoundEven(<RoundEven as InstEncoding>::decode(reader)?),
            3u16 => Self::Trunc(<Trunc as InstEncoding>::decode(reader)?),
            4u16 => Self::FAbs(<FAbs as InstEncoding>::decode(reader)?),
            5u16 => Self::SAbs(<SAbs as InstEncoding>::decode(reader)?),
            6u16 => Self::FSign(<FSign as InstEncoding>::decode(reader)?),
            7u16 => Self::SSign(<SSign as InstEncoding>::decode(reader)?),
            8u16 => Self::Floor(<Floor as InstEncoding>::decode(reader)?),
            9u16 => Self::Ceil(<Ceil as InstEncoding>::decode(reader)?),
            10u16 => Self::Fract(<Fract as InstEncoding>::decode(reader)?),
            11u16 => Self::Radians(<Radians as InstEncoding>::decode(reader)?),
            12u16 => Self::Degrees(<Degrees as InstEncoding>::decode(reader)?),
            13u16 => Self::Sin(<Sin as InstEncoding>::decode(reader)?),
            14u16 => Self::Cos(<Cos as InstEncoding>::decode(reader)?),
            15u16 => Self::Tan(<Tan as InstEncoding>::decode(reader)?),
            16u16 => Self::Asin(<Asin as InstEncoding>::decode(reader)?),
            17u16 => Self::Acos(<Acos as InstEncoding>::decode(reader)?),
            18u16 => Self::Atan(<Atan as InstEncoding>::decode(reader)?),
            19u16 => Self::Sinh(<Sinh as InstEncoding>::decode(reader)?),
            20u16 => Self::Cosh(<Cosh as InstEncoding>::decode(reader)?),
            21u16 => Self::Tanh(<Tanh as InstEncoding>::decode(reader)?),
            22u16 => Self::Asinh(<Asinh as InstEncoding>::decode(reader)?),
            23u16 => Self::Acosh(<Acosh as InstEncoding>::decode(reader)?),
            24u16 => Self::Atanh(<Atanh as InstEncoding>::decode(reader)?),
            25u16 => Self::Atan2(<Atan2 as InstEncoding>::decode(reader)?),
            26u16 => Self::Pow(<Pow as InstEncoding>::decode(reader)?),
            27u16 => Self::Exp(<Exp as InstEncoding>::decode(reader)?),
            28u16 => Self::Log(<Log as InstEncoding>::decode(reader)?),
            29u16 => Self::Exp2(<Exp2 as InstEncoding>::decode(reader)?),
            30u16 => Self::Log2(<Log2 as InstEncoding>::decode(reader)?),
            31u16 => Self::Sqrt(<Sqrt as InstEncoding>::decode(reader)?),
            32u16 => Self::InverseSqrt(<InverseSqrt as InstEncoding>::decode(reader)?),
            33u16 => Self::Determinant(<Determinant as InstEncoding>::decode(reader)?),
            34u16 => Self::MatrixInverse(<MatrixInverse as InstEncoding>::decode(reader)?),
            35u16 => Self::Modf(<Modf as InstEncoding>::decode(reader)?),
            36u16 => Self::ModfStruct(<ModfStruct as InstEncoding>::decode(reader)?),
            37u16 => Self::FMin(<FMin as InstEncoding>::decode(reader)?),
            38u16 => Self::UMin(<UMin as InstEncoding>::decode(reader)?),
            39u16 => Self::SMin(<SMin as InstEncoding>::decode(reader)?),
            40u16 => Self::FMax(<FMax as InstEncoding>::decode(reader)?),
            41u16 => Self::UMax(<UMax as InstEncoding>::decode(reader)?),
            42u16 => Self::SMax(<SMax as InstEncoding>::decode(reader)?),
            43u16 => Self::FClamp(<FClamp as InstEncoding>::decode(reader)?),
            44u16 => Self::UClamp(<UClamp as InstEncoding>::decode(reader)?),
            45u16 => Self::SClamp(<SClamp as InstEncoding>::decode(reader)?),
            46u16 => Self::FMix(<FMix as InstEncoding>::decode(reader)?),
            47u16 => Self::IMix(<IMix as InstEncoding>::decode(reader)?),
            48u16 => Self::Step(<Step as InstEncoding>::decode(reader)?),
            49u16 => Self::SmoothStep(<SmoothStep as InstEncoding>::decode(reader)?),
            50u16 => Self::Fma(<Fma as InstEncoding>::decode(reader)?),
            51u16 => Self::Frexp(<Frexp as InstEncoding>::decode(reader)?),
            52u16 => Self::FrexpStruct(<FrexpStruct as InstEncoding>::decode(reader)?),
            53u16 => Self::Ldexp(<Ldexp as InstEncoding>::decode(reader)?),
            54u16 => Self::PackSnorm4x8(<PackSnorm4x8 as InstEncoding>::decode(reader)?),
            55u16 => Self::PackUnorm4x8(<PackUnorm4x8 as InstEncoding>::decode(reader)?),
            56u16 => Self::PackSnorm2x16(<PackSnorm2x16 as InstEncoding>::decode(reader)?),
            57u16 => Self::PackUnorm2x16(<PackUnorm2x16 as InstEncoding>::decode(reader)?),
            58u16 => Self::PackHalf2x16(<PackHalf2x16 as InstEncoding>::decode(reader)?),
            59u16 => Self::PackDouble2x32(<PackDouble2x32 as InstEncoding>::decode(reader)?),
            60u16 => Self::UnpackSnorm2x16(<UnpackSnorm2x16 as InstEncoding>::decode(reader)?),
            61u16 => Self::UnpackUnorm2x16(<UnpackUnorm2x16 as InstEncoding>::decode(reader)?),
            62u16 => Self::UnpackHalf2x16(<UnpackHalf2x16 as InstEncoding>::decode(reader)?),
            63u16 => Self::UnpackSnorm4x8(<UnpackSnorm4x8 as InstEncoding>::decode(reader)?),
            64u16 => Self::UnpackUnorm4x8(<UnpackUnorm4x8 as InstEncoding>::decode(reader)?),
            65u16 => Self::UnpackDouble2x32(<UnpackDouble2x32 as InstEncoding>::decode(reader)?),
            66u16 => Self::Length(<Length as InstEncoding>::decode(reader)?),
            67u16 => Self::Distance(<Distance as InstEncoding>::decode(reader)?),
            68u16 => Self::Cross(<Cross as InstEncoding>::decode(reader)?),
            69u16 => Self::Normalize(<Normalize as InstEncoding>::decode(reader)?),
            70u16 => Self::FaceForward(<FaceForward as InstEncoding>::decode(reader)?),
            71u16 => Self::Reflect(<Reflect as InstEncoding>::decode(reader)?),
            72u16 => Self::Refract(<Refract as InstEncoding>::decode(reader)?),
            73u16 => Self::FindILsb(<FindILsb as InstEncoding>::decode(reader)?),
            74u16 => Self::FindSMsb(<FindSMsb as InstEncoding>::decode(reader)?),
            75u16 => Self::FindUMsb(<FindUMsb as InstEncoding>::decode(reader)?),
            76u16 => Self::InterpolateAtCentroid(<InterpolateAtCentroid as InstEncoding>::decode(
                reader,
            )?),
            77u16 => {
                Self::InterpolateAtSample(<InterpolateAtSample as InstEncoding>::decode(reader)?)
            }
            78u16 => {
                Self::InterpolateAtOffset(<InterpolateAtOffset as InstEncoding>::decode(reader)?)
            }
            79u16 => Self::NMin(<NMin as InstEncoding>::decode(reader)?),
            80u16 => Self::NMax(<NMax as InstEncoding>::decode(reader)?),
            81u16 => Self::NClamp(<NClamp as InstEncoding>::decode(reader)?),
            _ => {
                return Err(DecodeErrorKind::UnknownOpCode { opcode }.into());
            }
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        match self {
            Self::Round(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::RoundEven(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Trunc(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FAbs(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SAbs(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FSign(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SSign(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Floor(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Ceil(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Fract(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Radians(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Degrees(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Sin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Cos(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Tan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Asin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Acos(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Atan(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Sinh(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Cosh(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Tanh(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Asinh(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Acosh(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Atanh(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Atan2(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Pow(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Exp(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Log(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Exp2(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Log2(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Sqrt(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::InverseSqrt(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Determinant(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::MatrixInverse(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Modf(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::ModfStruct(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FClamp(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UClamp(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SClamp(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FMix(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::IMix(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Step(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::SmoothStep(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Fma(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Frexp(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FrexpStruct(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Ldexp(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PackSnorm4x8(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PackUnorm4x8(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PackSnorm2x16(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PackUnorm2x16(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PackHalf2x16(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::PackDouble2x32(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UnpackSnorm2x16(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UnpackUnorm2x16(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UnpackHalf2x16(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UnpackSnorm4x8(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UnpackUnorm4x8(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::UnpackDouble2x32(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Length(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Distance(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Cross(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Normalize(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FaceForward(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Reflect(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::Refract(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FindILsb(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FindSMsb(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::FindUMsb(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::InterpolateAtCentroid(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::InterpolateAtSample(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::InterpolateAtOffset(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::NMin(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::NMax(inst) => InstEncoding::dis_fmt(inst, f, ctx),
            Self::NClamp(inst) => InstEncoding::dis_fmt(inst, f, ctx),
        }
    }
}
impl From<Round> for GlslInstSet {
    fn from(inst: Round) -> Self {
        Self::Round(inst)
    }
}
impl From<RoundEven> for GlslInstSet {
    fn from(inst: RoundEven) -> Self {
        Self::RoundEven(inst)
    }
}
impl From<Trunc> for GlslInstSet {
    fn from(inst: Trunc) -> Self {
        Self::Trunc(inst)
    }
}
impl From<FAbs> for GlslInstSet {
    fn from(inst: FAbs) -> Self {
        Self::FAbs(inst)
    }
}
impl From<SAbs> for GlslInstSet {
    fn from(inst: SAbs) -> Self {
        Self::SAbs(inst)
    }
}
impl From<FSign> for GlslInstSet {
    fn from(inst: FSign) -> Self {
        Self::FSign(inst)
    }
}
impl From<SSign> for GlslInstSet {
    fn from(inst: SSign) -> Self {
        Self::SSign(inst)
    }
}
impl From<Floor> for GlslInstSet {
    fn from(inst: Floor) -> Self {
        Self::Floor(inst)
    }
}
impl From<Ceil> for GlslInstSet {
    fn from(inst: Ceil) -> Self {
        Self::Ceil(inst)
    }
}
impl From<Fract> for GlslInstSet {
    fn from(inst: Fract) -> Self {
        Self::Fract(inst)
    }
}
impl From<Radians> for GlslInstSet {
    fn from(inst: Radians) -> Self {
        Self::Radians(inst)
    }
}
impl From<Degrees> for GlslInstSet {
    fn from(inst: Degrees) -> Self {
        Self::Degrees(inst)
    }
}
impl From<Sin> for GlslInstSet {
    fn from(inst: Sin) -> Self {
        Self::Sin(inst)
    }
}
impl From<Cos> for GlslInstSet {
    fn from(inst: Cos) -> Self {
        Self::Cos(inst)
    }
}
impl From<Tan> for GlslInstSet {
    fn from(inst: Tan) -> Self {
        Self::Tan(inst)
    }
}
impl From<Asin> for GlslInstSet {
    fn from(inst: Asin) -> Self {
        Self::Asin(inst)
    }
}
impl From<Acos> for GlslInstSet {
    fn from(inst: Acos) -> Self {
        Self::Acos(inst)
    }
}
impl From<Atan> for GlslInstSet {
    fn from(inst: Atan) -> Self {
        Self::Atan(inst)
    }
}
impl From<Sinh> for GlslInstSet {
    fn from(inst: Sinh) -> Self {
        Self::Sinh(inst)
    }
}
impl From<Cosh> for GlslInstSet {
    fn from(inst: Cosh) -> Self {
        Self::Cosh(inst)
    }
}
impl From<Tanh> for GlslInstSet {
    fn from(inst: Tanh) -> Self {
        Self::Tanh(inst)
    }
}
impl From<Asinh> for GlslInstSet {
    fn from(inst: Asinh) -> Self {
        Self::Asinh(inst)
    }
}
impl From<Acosh> for GlslInstSet {
    fn from(inst: Acosh) -> Self {
        Self::Acosh(inst)
    }
}
impl From<Atanh> for GlslInstSet {
    fn from(inst: Atanh) -> Self {
        Self::Atanh(inst)
    }
}
impl From<Atan2> for GlslInstSet {
    fn from(inst: Atan2) -> Self {
        Self::Atan2(inst)
    }
}
impl From<Pow> for GlslInstSet {
    fn from(inst: Pow) -> Self {
        Self::Pow(inst)
    }
}
impl From<Exp> for GlslInstSet {
    fn from(inst: Exp) -> Self {
        Self::Exp(inst)
    }
}
impl From<Log> for GlslInstSet {
    fn from(inst: Log) -> Self {
        Self::Log(inst)
    }
}
impl From<Exp2> for GlslInstSet {
    fn from(inst: Exp2) -> Self {
        Self::Exp2(inst)
    }
}
impl From<Log2> for GlslInstSet {
    fn from(inst: Log2) -> Self {
        Self::Log2(inst)
    }
}
impl From<Sqrt> for GlslInstSet {
    fn from(inst: Sqrt) -> Self {
        Self::Sqrt(inst)
    }
}
impl From<InverseSqrt> for GlslInstSet {
    fn from(inst: InverseSqrt) -> Self {
        Self::InverseSqrt(inst)
    }
}
impl From<Determinant> for GlslInstSet {
    fn from(inst: Determinant) -> Self {
        Self::Determinant(inst)
    }
}
impl From<MatrixInverse> for GlslInstSet {
    fn from(inst: MatrixInverse) -> Self {
        Self::MatrixInverse(inst)
    }
}
impl From<Modf> for GlslInstSet {
    fn from(inst: Modf) -> Self {
        Self::Modf(inst)
    }
}
impl From<ModfStruct> for GlslInstSet {
    fn from(inst: ModfStruct) -> Self {
        Self::ModfStruct(inst)
    }
}
impl From<FMin> for GlslInstSet {
    fn from(inst: FMin) -> Self {
        Self::FMin(inst)
    }
}
impl From<UMin> for GlslInstSet {
    fn from(inst: UMin) -> Self {
        Self::UMin(inst)
    }
}
impl From<SMin> for GlslInstSet {
    fn from(inst: SMin) -> Self {
        Self::SMin(inst)
    }
}
impl From<FMax> for GlslInstSet {
    fn from(inst: FMax) -> Self {
        Self::FMax(inst)
    }
}
impl From<UMax> for GlslInstSet {
    fn from(inst: UMax) -> Self {
        Self::UMax(inst)
    }
}
impl From<SMax> for GlslInstSet {
    fn from(inst: SMax) -> Self {
        Self::SMax(inst)
    }
}
impl From<FClamp> for GlslInstSet {
    fn from(inst: FClamp) -> Self {
        Self::FClamp(inst)
    }
}
impl From<UClamp> for GlslInstSet {
    fn from(inst: UClamp) -> Self {
        Self::UClamp(inst)
    }
}
impl From<SClamp> for GlslInstSet {
    fn from(inst: SClamp) -> Self {
        Self::SClamp(inst)
    }
}
impl From<FMix> for GlslInstSet {
    fn from(inst: FMix) -> Self {
        Self::FMix(inst)
    }
}
impl From<IMix> for GlslInstSet {
    fn from(inst: IMix) -> Self {
        Self::IMix(inst)
    }
}
impl From<Step> for GlslInstSet {
    fn from(inst: Step) -> Self {
        Self::Step(inst)
    }
}
impl From<SmoothStep> for GlslInstSet {
    fn from(inst: SmoothStep) -> Self {
        Self::SmoothStep(inst)
    }
}
impl From<Fma> for GlslInstSet {
    fn from(inst: Fma) -> Self {
        Self::Fma(inst)
    }
}
impl From<Frexp> for GlslInstSet {
    fn from(inst: Frexp) -> Self {
        Self::Frexp(inst)
    }
}
impl From<FrexpStruct> for GlslInstSet {
    fn from(inst: FrexpStruct) -> Self {
        Self::FrexpStruct(inst)
    }
}
impl From<Ldexp> for GlslInstSet {
    fn from(inst: Ldexp) -> Self {
        Self::Ldexp(inst)
    }
}
impl From<PackSnorm4x8> for GlslInstSet {
    fn from(inst: PackSnorm4x8) -> Self {
        Self::PackSnorm4x8(inst)
    }
}
impl From<PackUnorm4x8> for GlslInstSet {
    fn from(inst: PackUnorm4x8) -> Self {
        Self::PackUnorm4x8(inst)
    }
}
impl From<PackSnorm2x16> for GlslInstSet {
    fn from(inst: PackSnorm2x16) -> Self {
        Self::PackSnorm2x16(inst)
    }
}
impl From<PackUnorm2x16> for GlslInstSet {
    fn from(inst: PackUnorm2x16) -> Self {
        Self::PackUnorm2x16(inst)
    }
}
impl From<PackHalf2x16> for GlslInstSet {
    fn from(inst: PackHalf2x16) -> Self {
        Self::PackHalf2x16(inst)
    }
}
impl From<PackDouble2x32> for GlslInstSet {
    fn from(inst: PackDouble2x32) -> Self {
        Self::PackDouble2x32(inst)
    }
}
impl From<UnpackSnorm2x16> for GlslInstSet {
    fn from(inst: UnpackSnorm2x16) -> Self {
        Self::UnpackSnorm2x16(inst)
    }
}
impl From<UnpackUnorm2x16> for GlslInstSet {
    fn from(inst: UnpackUnorm2x16) -> Self {
        Self::UnpackUnorm2x16(inst)
    }
}
impl From<UnpackHalf2x16> for GlslInstSet {
    fn from(inst: UnpackHalf2x16) -> Self {
        Self::UnpackHalf2x16(inst)
    }
}
impl From<UnpackSnorm4x8> for GlslInstSet {
    fn from(inst: UnpackSnorm4x8) -> Self {
        Self::UnpackSnorm4x8(inst)
    }
}
impl From<UnpackUnorm4x8> for GlslInstSet {
    fn from(inst: UnpackUnorm4x8) -> Self {
        Self::UnpackUnorm4x8(inst)
    }
}
impl From<UnpackDouble2x32> for GlslInstSet {
    fn from(inst: UnpackDouble2x32) -> Self {
        Self::UnpackDouble2x32(inst)
    }
}
impl From<Length> for GlslInstSet {
    fn from(inst: Length) -> Self {
        Self::Length(inst)
    }
}
impl From<Distance> for GlslInstSet {
    fn from(inst: Distance) -> Self {
        Self::Distance(inst)
    }
}
impl From<Cross> for GlslInstSet {
    fn from(inst: Cross) -> Self {
        Self::Cross(inst)
    }
}
impl From<Normalize> for GlslInstSet {
    fn from(inst: Normalize) -> Self {
        Self::Normalize(inst)
    }
}
impl From<FaceForward> for GlslInstSet {
    fn from(inst: FaceForward) -> Self {
        Self::FaceForward(inst)
    }
}
impl From<Reflect> for GlslInstSet {
    fn from(inst: Reflect) -> Self {
        Self::Reflect(inst)
    }
}
impl From<Refract> for GlslInstSet {
    fn from(inst: Refract) -> Self {
        Self::Refract(inst)
    }
}
impl From<FindILsb> for GlslInstSet {
    fn from(inst: FindILsb) -> Self {
        Self::FindILsb(inst)
    }
}
impl From<FindSMsb> for GlslInstSet {
    fn from(inst: FindSMsb) -> Self {
        Self::FindSMsb(inst)
    }
}
impl From<FindUMsb> for GlslInstSet {
    fn from(inst: FindUMsb) -> Self {
        Self::FindUMsb(inst)
    }
}
impl From<InterpolateAtCentroid> for GlslInstSet {
    fn from(inst: InterpolateAtCentroid) -> Self {
        Self::InterpolateAtCentroid(inst)
    }
}
impl From<InterpolateAtSample> for GlslInstSet {
    fn from(inst: InterpolateAtSample) -> Self {
        Self::InterpolateAtSample(inst)
    }
}
impl From<InterpolateAtOffset> for GlslInstSet {
    fn from(inst: InterpolateAtOffset) -> Self {
        Self::InterpolateAtOffset(inst)
    }
}
impl From<NMin> for GlslInstSet {
    fn from(inst: NMin) -> Self {
        Self::NMin(inst)
    }
}
impl From<NMax> for GlslInstSet {
    fn from(inst: NMax) -> Self {
        Self::NMax(inst)
    }
}
impl From<NClamp> for GlslInstSet {
    fn from(inst: NClamp) -> Self {
        Self::NClamp(inst)
    }
}
