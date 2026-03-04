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
