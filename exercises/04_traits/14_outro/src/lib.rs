// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

// wrapping_add 溢出

// sturating_add 饱和，超过最大值也是最大值

use std::ops::Add;

/*
Debug是用来打印错误信息的，
PartialEq使用来assert_eq比较中需要调用的切片
Clone, Copy 是结构体相加需要调用的切片，所有权不被转移
*/ 
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16(u16);

// u8引用转换为SaturatingU16
impl From<&u8> for SaturatingU16{
    fn from(value: &u8) -> Self {
        SaturatingU16(*value as u16)
    }
}

// u8转换为SaturatingU16
impl From<u8> for SaturatingU16{
    fn from(value: u8) -> Self {
        SaturatingU16(value as u16)
    }
}

// u16转换为SaturatingU16
impl From<u16> for SaturatingU16{
    fn from(value: u16) -> Self {
        SaturatingU16(value)
    }
}

// u16引用转换为SaturatingU16
impl From<&u16> for SaturatingU16{
    fn from(value: &u16) -> Self {
        SaturatingU16(*value)
    }
}

// SaturatingU16 结构体相加
impl Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs.0))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs))
    }
}

// SaturatingU16 + SaturatingU16引用类型
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs.0))
    }
}

// SaturatingU16 和 u16比较大小
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        if self.0 == *other {
            true
        } else {
            false
        }
    }
}