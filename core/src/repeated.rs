use crate::scalars::*;
use crate::traits::PackableMarker;

mod packed;
mod unpacked;

impl PackableMarker<Int32> for i32 {}
impl PackableMarker<Int64> for i64 {}
impl PackableMarker<UInt32> for u32 {}
impl PackableMarker<UInt32> for std::net::Ipv4Addr {}
impl PackableMarker<UInt64> for u64 {}
impl PackableMarker<SInt32> for i32 {}
impl PackableMarker<SInt64> for i64 {}
impl PackableMarker<Fixed32> for u32 {}
impl PackableMarker<Fixed64> for u64 {}
impl PackableMarker<SFixed32> for i32 {}
impl PackableMarker<SFixed64> for i64 {}
impl PackableMarker<Float> for f32 {}
impl PackableMarker<Double> for f64 {}
impl PackableMarker<Bool> for bool {}
