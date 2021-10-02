//! Structs for accurate timekeeping in musical audio applications.

use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Sampling rate in samples per second.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SampleRate(pub f64);

impl SampleRate {
    #[inline]
    pub fn new(sample_rate: f64) -> Self {
        SampleRate(sample_rate)
    }

    /// Returns the reciprocal of the sample rate (`1.0 / sample_rate`)
    #[inline]
    pub fn recip(&self) -> f64 {
        self.0.recip()
    }

    #[inline]
    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }

    #[inline]
    pub fn as_u16(&self) -> u16 {
        self.0.round() as u16
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.0.round() as u32
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        self.0.round() as usize
    }
}

impl Default for SampleRate {
    fn default() -> Self {
        SampleRate(44_100.0)
    }
}

impl From<u16> for SampleRate {
    fn from(sr: u16) -> Self {
        SampleRate(f64::from(sr))
    }
}
impl From<u32> for SampleRate {
    fn from(sr: u32) -> Self {
        SampleRate(f64::from(sr))
    }
}
impl From<f32> for SampleRate {
    fn from(sr: f32) -> Self {
        SampleRate(f64::from(sr))
    }
}
impl From<f64> for SampleRate {
    fn from(sr: f64) -> Self {
        SampleRate(sr)
    }
}

impl Mul<SampleRate> for f32 {
    type Output = Self;
    fn mul(self, rhs: SampleRate) -> Self::Output {
        self * rhs.0 as f32
    }
}
impl Mul<SampleRate> for f64 {
    type Output = Self;
    fn mul(self, rhs: SampleRate) -> Self::Output {
        self * rhs.0
    }
}
impl Div<SampleRate> for f32 {
    type Output = Self;
    fn div(self, rhs: SampleRate) -> Self::Output {
        self / rhs.0 as f32
    }
}
impl Div<SampleRate> for f64 {
    type Output = Self;
    fn div(self, rhs: SampleRate) -> Self::Output {
        self / rhs.0
    }
}

/// Musical time in units of "beats"
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MusicalTime(pub f64);

impl MusicalTime {
    #[inline]
    pub fn new(beats: f64) -> Self {
        MusicalTime(beats)
    }

    #[inline]
    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }

    /// Convert to the corresponding time in [`Seconds`].
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`Seconds`]: struct.Seconds.html
    #[inline]
    pub fn to_seconds(&self, bpm: f64) -> Seconds {
        Seconds(self.0 * 60.0 / bpm)
    }

    /// Convert to the corresponding discrete [`SampleTime`]. This will be rounded to the nearest sample.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    #[inline]
    pub fn to_nearest_sample_round(&self, bpm: f64, sample_rate: SampleRate) -> SampleTime {
        self.to_seconds(bpm).to_nearest_sample_round(sample_rate)
    }

    /// Convert to the corresponding discrete [`SampleTime`]. This will be floored to the nearest sample.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    #[inline]
    pub fn to_nearest_sample_floor(&self, bpm: f64, sample_rate: SampleRate) -> SampleTime {
        self.to_seconds(bpm).to_nearest_sample_floor(sample_rate)
    }

    /// Convert to the corresponding discrete [`SampleTime`]. This will be ceil-ed to the nearest sample.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    #[inline]
    pub fn to_nearest_sample_ceil(&self, bpm: f64, sample_rate: SampleRate) -> SampleTime {
        self.to_seconds(bpm).to_nearest_sample_ceil(sample_rate)
    }

    /// Convert to the corresponding discrete [`SampleTime`] floored to the nearest sample,
    /// while also returning the fractional sub-sample part.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    #[inline]
    pub fn to_sub_sample(&self, bpm: f64, sample_rate: SampleRate) -> (SampleTime, f64) {
        self.to_seconds(bpm).to_sub_sample(sample_rate)
    }
}

impl Default for MusicalTime {
    fn default() -> Self {
        MusicalTime(0.0)
    }
}

impl From<i8> for MusicalTime {
    fn from(s: i8) -> Self {
        MusicalTime(f64::from(s))
    }
}
impl From<u8> for MusicalTime {
    fn from(s: u8) -> Self {
        MusicalTime(f64::from(s))
    }
}
impl From<i16> for MusicalTime {
    fn from(s: i16) -> Self {
        MusicalTime(f64::from(s))
    }
}
impl From<u16> for MusicalTime {
    fn from(s: u16) -> Self {
        MusicalTime(f64::from(s))
    }
}
impl From<i32> for MusicalTime {
    fn from(s: i32) -> Self {
        MusicalTime(f64::from(s))
    }
}
impl From<u32> for MusicalTime {
    fn from(s: u32) -> Self {
        MusicalTime(f64::from(s))
    }
}
impl From<f32> for MusicalTime {
    fn from(s: f32) -> Self {
        MusicalTime(f64::from(s))
    }
}

impl Add<MusicalTime> for MusicalTime {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<MusicalTime> for MusicalTime {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul<MusicalTime> for MusicalTime {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl Div<MusicalTime> for MusicalTime {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl AddAssign<MusicalTime> for MusicalTime {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl SubAssign<MusicalTime> for MusicalTime {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}
impl MulAssign<MusicalTime> for MusicalTime {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}
impl DivAssign<MusicalTime> for MusicalTime {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
    }
}

/// Unit of time in "Seconds"
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Seconds(pub f64);

impl Seconds {
    #[inline]
    pub fn new(seconds: f64) -> Self {
        Seconds(seconds)
    }

    #[inline]
    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }

    /// Creates a new time in `Seconds` from [`SampleTime`] and a [`SampleRate`].
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn from_sample(sample_time: SampleTime, sample_rate: SampleRate) -> Self {
        sample_time.to_seconds(sample_rate)
    }

    /// Convert to discrete [`SampleTime`] with the given [`SampleRate`]. This will
    /// be rounded to the nearest sample.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_nearest_sample_round(&self, sample_rate: SampleRate) -> SampleTime {
        SampleTime((self.0 * sample_rate).round() as i64)
    }

    /// Convert to discrete [`SampleTime`] with the given [`SampleRate`]. This will
    /// be floored to the nearest sample.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_nearest_sample_floor(&self, sample_rate: SampleRate) -> SampleTime {
        SampleTime((self.0 * sample_rate).floor() as i64)
    }

    /// Convert to discrete [`SampleTime`] with the given [`SampleRate`]. This will
    /// be ceil-ed to the nearest sample.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_nearest_sample_ceil(&self, sample_rate: SampleRate) -> SampleTime {
        SampleTime((self.0 * sample_rate).ceil() as i64)
    }

    /// Convert to discrete [`SampleTime`] given the [`SampleRate`] floored to the nearest
    /// sample, while also return the fractional sub-sample part.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_sub_sample(&self, sample_rate: SampleRate) -> (SampleTime, f64) {
        let smps_f64 = self.0 * sample_rate;
        (SampleTime(smps_f64.floor() as i64), smps_f64.fract())
    }

    /// Convert to the corresponding [`MusicalTime`].
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`MusicalTime`]: ../time/struct.MusicalTime.html
    #[inline]
    pub fn to_musical(&self, bpm: f64) -> MusicalTime {
        MusicalTime(self.0 * bpm / 60.0)
    }
}

impl Default for Seconds {
    fn default() -> Self {
        Seconds(0.0)
    }
}

impl From<i8> for Seconds {
    fn from(s: i8) -> Self {
        Seconds(f64::from(s))
    }
}
impl From<u8> for Seconds {
    fn from(s: u8) -> Self {
        Seconds(f64::from(s))
    }
}
impl From<i16> for Seconds {
    fn from(s: i16) -> Self {
        Seconds(f64::from(s))
    }
}
impl From<u16> for Seconds {
    fn from(s: u16) -> Self {
        Seconds(f64::from(s))
    }
}
impl From<i32> for Seconds {
    fn from(s: i32) -> Self {
        Seconds(f64::from(s))
    }
}
impl From<u32> for Seconds {
    fn from(s: u32) -> Self {
        Seconds(f64::from(s))
    }
}
impl From<f32> for Seconds {
    fn from(s: f32) -> Self {
        Seconds(f64::from(s))
    }
}

impl Add<Seconds> for Seconds {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<Seconds> for Seconds {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul<Seconds> for Seconds {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl Div<Seconds> for Seconds {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl AddAssign<Seconds> for Seconds {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl SubAssign<Seconds> for Seconds {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}
impl MulAssign<Seconds> for Seconds {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}
impl DivAssign<Seconds> for Seconds {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
    }
}

/// Unit of time in discrete samples.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct SampleTime(pub i64);

impl SampleTime {
    #[inline]
    pub fn new(samples: i64) -> Self {
        Self(samples)
    }

    #[inline]
    pub fn from_usize(samples: usize) -> Self {
        Self(samples as i64)
    }

    #[inline]
    pub fn from_u64(samples: u64) -> Self {
        Self(samples as i64)
    }

    /// Get the sample time as a `usize` value.
    ///
    /// This will return `None` when this sample time is negative.
    #[inline]
    pub fn as_usize(&self) -> Option<usize> {
        if self.0 >= 0 {
            Some(self.0 as usize)
        } else {
            None
        }
    }

    /// Convert to the corresponding time in [`Seconds`] with the given [`SampleRate`].
    ///
    /// [`Seconds`]: struct.Seconds.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_seconds(&self, sample_rate: SampleRate) -> Seconds {
        Seconds(self.0 as f64 / sample_rate)
    }

    /// Convert to the corresponding [`MusicalTime`].
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`MusicalTime`]: struct.MusicalTime.html
    #[inline]
    pub fn to_musical(&self, bpm: f64, sample_rate: SampleRate) -> MusicalTime {
        MusicalTime(self.to_seconds(sample_rate).0 * bpm / 60.0)
    }
}

impl Default for SampleTime {
    fn default() -> Self {
        SampleTime(0)
    }
}

impl From<i8> for SampleTime {
    fn from(s: i8) -> Self {
        SampleTime(i64::from(s))
    }
}
impl From<u8> for SampleTime {
    fn from(s: u8) -> Self {
        SampleTime(i64::from(s))
    }
}
impl From<i16> for SampleTime {
    fn from(s: i16) -> Self {
        SampleTime(i64::from(s))
    }
}
impl From<u16> for SampleTime {
    fn from(s: u16) -> Self {
        SampleTime(i64::from(s))
    }
}
impl From<i32> for SampleTime {
    fn from(s: i32) -> Self {
        SampleTime(i64::from(s))
    }
}
impl From<u32> for SampleTime {
    fn from(s: u32) -> Self {
        SampleTime(i64::from(s))
    }
}
impl From<i64> for SampleTime {
    fn from(s: i64) -> Self {
        SampleTime(s)
    }
}

impl Add<SampleTime> for SampleTime {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<SampleTime> for SampleTime {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign<SampleTime> for SampleTime {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl SubAssign<SampleTime> for SampleTime {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}
