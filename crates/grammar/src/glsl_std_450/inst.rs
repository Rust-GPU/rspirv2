use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Round {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RoundEven {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Trunc {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FAbs {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SAbs {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FSign {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SSign {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Floor {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ceil {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Fract {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Radians {
    pub degrees: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Degrees {
    pub radians: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sin {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cos {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tan {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Asin {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Acos {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atan {
    pub y_over_x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sinh {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cosh {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tanh {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Asinh {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Acosh {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atanh {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atan2 {
    pub y: IdRef,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pow {
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Exp {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Log {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Exp2 {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Log2 {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sqrt {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InverseSqrt {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Determinant {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MatrixInverse {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Modf {
    pub x: IdRef,
    pub i: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ModfStruct {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMin {
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UMin {
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SMin {
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMax {
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UMax {
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SMax {
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMix {
    pub x: IdRef,
    pub y: IdRef,
    pub a: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct IMix {
    pub x: IdRef,
    pub y: IdRef,
    pub a: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Step {
    pub edge: IdRef,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SmoothStep {
    pub edge_0: IdRef,
    pub edge_1: IdRef,
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Fma {
    pub a: IdRef,
    pub b: IdRef,
    pub c: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Frexp {
    pub x: IdRef,
    pub exp: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FrexpStruct {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ldexp {
    pub x: IdRef,
    pub exp: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackSnorm4x8 {
    pub v: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackUnorm4x8 {
    pub v: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackSnorm2x16 {
    pub v: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackUnorm2x16 {
    pub v: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackHalf2x16 {
    pub v: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackDouble2x32 {
    pub v: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackSnorm2x16 {
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackUnorm2x16 {
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackHalf2x16 {
    pub v: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackSnorm4x8 {
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackUnorm4x8 {
    pub p: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackDouble2x32 {
    pub v: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Length {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Distance {
    pub p_0: IdRef,
    pub p_1: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cross {
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Normalize {
    pub x: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FaceForward {
    pub n: IdRef,
    pub i: IdRef,
    pub nref: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Reflect {
    pub i: IdRef,
    pub n: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Refract {
    pub i: IdRef,
    pub n: IdRef,
    pub eta: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindILsb {
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindSMsb {
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindUMsb {
    pub value: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtCentroid {
    pub interpolant: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtSample {
    pub interpolant: IdRef,
    pub sample: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtOffset {
    pub interpolant: IdRef,
    pub offset: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NMin {
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NMax {
    pub x: IdRef,
    pub y: IdRef,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
