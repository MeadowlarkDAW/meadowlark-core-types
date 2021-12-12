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

/// Musical time in units of 1 / 28,224,000 beats.
///
/// This number was chosen because it is nicely divisible by a whole slew of factors
/// including `2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 16, 20, 24, 32, 64, 128, 256, 512, and 1920`,
/// as well as common sampling rates such as `22050, 24000, 44100, 48000, 88200, 96000, 176400,
/// and 192000`. This ensures that any recording of note data in this format will always be
/// at-least sample-accurate.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct MusicalTime(pub i64);

impl MusicalTime {
    /// * super_beats - Musical time in units of 1 / 28,224,000 beats.
    #[inline]
    pub fn new(super_beats: i64) -> Self {
        MusicalTime(super_beats)
    }

    /// * beats - Musical time in units of beats (as apposed to
    /// units of "super beats").
    #[inline]
    pub fn from_beats(beats: i64) -> Self {
        MusicalTime(beats * 28_224_000)
    }

    /// * half_beats - Musical time in units of 1 / 2 beats (as apposed to
    /// units of "super beats").
    #[inline]
    pub fn from_half_beats(half_beats: i64) -> Self {
        MusicalTime(half_beats * (28_224_000 / 2))
    }

    /// * quarter_beats - Musical time in units of 1 / 4 beats (as apposed to
    /// units of "super beats").
    #[inline]
    pub fn from_quarter_beats(quarter_beats: i64) -> Self {
        MusicalTime(quarter_beats * (28_224_000 / 4))
    }

    /// * eighth_beats - Musical time in units of 1 / 8 beats (as apposed to
    /// units of "super beats").
    #[inline]
    pub fn from_eighth_beats(eighth_beats: i64) -> Self {
        MusicalTime(eighth_beats * (28_224_000 / 8))
    }

    /// * sixteenth_beats - Musical time in units of 1 / 16 beats (as apposed to
    /// units of "super beats").
    #[inline]
    pub fn from_sixteenth_beats(sixteenth_beats: i64) -> Self {
        MusicalTime(sixteenth_beats * (28_224_000 / 16))
    }

    /// * thirty_second_beats - Musical time in units of 1 / 32 beats (as apposed to
    /// units of "super beats").
    #[inline]
    pub fn from_32nd_beats(thirty_second_beats: i64) -> Self {
        MusicalTime(thirty_second_beats * (28_224_000 / 32))
    }

    /// * sixty_fourth_beats - Musical time in units of 1 / 64 beats (as apposed to
    /// units of "super beats").
    #[inline]
    pub fn from_64th_beats(sixty_fourth_beats: i64) -> Self {
        MusicalTime(sixty_fourth_beats * (28_224_000 / 32))
    }

    /// * third_beats - Musical time in units of 1 / 3 beats (as apposed to
    /// units of "super beats").
    #[inline]
    pub fn from_third_beats(third_beats: i64) -> Self {
        MusicalTime(third_beats * (28_224_000 / 3))
    }

    /// * fifth_beats - Musical time in units of 1 / 5 beats (as apposed to
    /// units of "super beats").
    #[inline]
    pub fn from_fifth_beats(fifth_beats: i64) -> Self {
        MusicalTime(fifth_beats * (28_224_000 / 5))
    }

    /// * seventh_beats - Musical time in units of 1 / 7 beats (as apposed to
    /// units of "super beats").
    #[inline]
    pub fn from_seventh_beats(seventh_beats: i64) -> Self {
        MusicalTime(seventh_beats * (28_224_000 / 7))
    }

    /// Convert the corresponding musical time in units of beats (as an `f64` value).
    ///
    /// This is useful for displaying notes in UI.
    #[inline]
    pub fn as_fractional_beats(&self) -> f64 {
        self.0 as f64 / 28_224_000.0
    }

    /// Convert to the corresponding time in [`Seconds`].
    ///
    /// [`Seconds`]: struct.Seconds.html
    #[inline]
    pub fn to_seconds(&self, bpm: f64) -> Seconds {
        Seconds(self.as_fractional_beats() * 60.0 / bpm)
    }

    /// Convert to the corresponding discrete [`RealSampleTime`]. This will be rounded to the nearest sample.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`RealSampleTime`]: struct.RealSampleTime.html
    #[inline]
    pub fn to_nearest_real_sample_round(
        &self,
        bpm: f64,
        sample_rate: SampleRate,
    ) -> RealSampleTime {
        self.to_seconds(bpm)
            .to_nearest_real_sample_round(sample_rate)
    }

    /// Convert to the corresponding discrete [`RealSampleTime`]. This will be floored to the nearest sample.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`RealSampleTime`]: struct.RealSampleTime.html
    #[inline]
    pub fn to_nearest_real_sample_floor(
        &self,
        bpm: f64,
        sample_rate: SampleRate,
    ) -> RealSampleTime {
        self.to_seconds(bpm)
            .to_nearest_real_sample_floor(sample_rate)
    }

    /// Convert to the corresponding discrete [`RealSampleTime`]. This will be ceil-ed to the nearest sample.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`RealSampleTime`]: struct.RealSampleTime.html
    #[inline]
    pub fn to_nearest_real_sample_ceil(&self, bpm: f64, sample_rate: SampleRate) -> RealSampleTime {
        self.to_seconds(bpm)
            .to_nearest_real_sample_ceil(sample_rate)
    }

    /// Convert to the corresponding discrete [`RealSampleTime`] floored to the nearest sample,
    /// while also returning the fractional sub-sample part.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`RealSampleTime`]: struct.RealSampleTime.html
    #[inline]
    pub fn to_sub_real_sample(&self, bpm: f64, sample_rate: SampleRate) -> (RealSampleTime, f64) {
        self.to_seconds(bpm).to_sub_real_sample(sample_rate)
    }

    /// Convert to the corresponding discrete [`SuperSampleTime`]. This will be rounded to the nearest super-sample.
    ///
    /// [`SuperSampleTime`]: struct.SuperSampleTime.html
    #[inline]
    pub fn to_nearest_super_sample_round(&self, bpm: f64) -> SuperSampleTime {
        self.to_seconds(bpm).to_nearest_super_sample_round()
    }

    /// Convert to the corresponding discrete [`SuperSampleTime`]. This will be floored to the nearest super-sample.
    ///
    /// [`SuperSampleTime`]: struct.SuperSampleTime.html
    #[inline]
    pub fn to_nearest_super_sample_floor(&self, bpm: f64) -> SuperSampleTime {
        self.to_seconds(bpm).to_nearest_super_sample_floor()
    }

    /// Convert to the corresponding discrete [`SuperSampleTime`]. This will be ceil-ed to the nearest super-sample.
    ///
    /// [`SuperSampleTime`]: struct.SuperSampleTime.html
    #[inline]
    pub fn to_nearest_super_sample_ceil(&self, bpm: f64) -> SuperSampleTime {
        self.to_seconds(bpm).to_nearest_super_sample_ceil()
    }

    /// Convert to the corresponding discrete [`SuperSampleTime`] floored to the nearest super-sample,
    /// while also returning the fractional sub-super-sample part.
    ///
    /// [`SuperSampleTime`]: struct.SuperSampleTime.html
    #[inline]
    pub fn to_sub_super_sample(&self, bpm: f64) -> (SuperSampleTime, f64) {
        self.to_seconds(bpm).to_sub_super_sample()
    }
}

impl Default for MusicalTime {
    fn default() -> Self {
        MusicalTime(0)
    }
}

impl From<i8> for MusicalTime {
    fn from(s: i8) -> Self {
        MusicalTime(i64::from(s))
    }
}
impl From<u8> for MusicalTime {
    fn from(s: u8) -> Self {
        MusicalTime(i64::from(s))
    }
}
impl From<i16> for MusicalTime {
    fn from(s: i16) -> Self {
        MusicalTime(i64::from(s))
    }
}
impl From<u16> for MusicalTime {
    fn from(s: u16) -> Self {
        MusicalTime(i64::from(s))
    }
}
impl From<i32> for MusicalTime {
    fn from(s: i32) -> Self {
        MusicalTime(i64::from(s))
    }
}
impl From<u32> for MusicalTime {
    fn from(s: u32) -> Self {
        MusicalTime(i64::from(s))
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

    /// Creates a new time in `Seconds` from [`RealSampleTime`] and a [`SampleRate`].
    ///
    /// [`RealSampleTime`]: struct.RealSampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn from_real_sample(sample_time: RealSampleTime, sample_rate: SampleRate) -> Self {
        sample_time.to_seconds(sample_rate)
    }

    /// Creates a new time in `Seconds` from [`SuperSampleTime`].
    ///
    /// [`SuperSampleTime`]: struct.SuperSampleTime.html
    #[inline]
    pub fn from_super_sample(super_sample_time: SuperSampleTime) -> Self {
        super_sample_time.to_seconds()
    }

    /// Convert to discrete [`RealSampleTime`] with the given [`SampleRate`]. This will
    /// be rounded to the nearest sample.
    ///
    /// [`RealSampleTime`]: struct.RealSampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_nearest_real_sample_round(&self, sample_rate: SampleRate) -> RealSampleTime {
        RealSampleTime((self.0 * sample_rate).round() as i64)
    }

    /// Convert to discrete [`RealSampleTime`] with the given [`SampleRate`]. This will
    /// be floored to the nearest sample.
    ///
    /// [`RealSampleTime`]: struct.RealSampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_nearest_real_sample_floor(&self, sample_rate: SampleRate) -> RealSampleTime {
        RealSampleTime((self.0 * sample_rate).floor() as i64)
    }

    /// Convert to discrete [`RealSampleTime`] with the given [`SampleRate`]. This will
    /// be ceil-ed to the nearest sample.
    ///
    /// [`RealSampleTime`]: struct.RealSampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_nearest_real_sample_ceil(&self, sample_rate: SampleRate) -> RealSampleTime {
        RealSampleTime((self.0 * sample_rate).ceil() as i64)
    }

    /// Convert to discrete [`RealSampleTime`] given the [`SampleRate`] floored to the nearest
    /// sample, while also return the fractional sub-sample part.
    ///
    /// [`RealSampleTime`]: struct.RealSampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_sub_real_sample(&self, sample_rate: SampleRate) -> (RealSampleTime, f64) {
        let smps_f64 = self.0 * sample_rate;
        (RealSampleTime(smps_f64.floor() as i64), smps_f64.fract())
    }

    /// Convert to discrete [`SuperSampleTime`]. This will
    /// be rounded to the nearest super-sample.
    ///
    /// [`SuperSampleTime`]: struct.RealSampleTime.html
    #[inline]
    pub fn to_nearest_super_sample_round(&self) -> SuperSampleTime {
        SuperSampleTime((self.0 * 28_224_000.0).round() as i64)
    }

    /// Convert to discrete [`RealSampleTime`]. This will
    /// be floored to the nearest super-sample.
    ///
    /// [`SuperSampleTime`]: struct.RealSampleTime.html
    #[inline]
    pub fn to_nearest_super_sample_floor(&self) -> SuperSampleTime {
        SuperSampleTime((self.0 * 28_224_000.0).floor() as i64)
    }

    /// Convert to discrete [`RealSampleTime`]. This will
    /// be ceil-ed to the nearest super-sample.
    ///
    /// [`SuperSampleTime`]: struct.RealSampleTime.html
    #[inline]
    pub fn to_nearest_super_sample_ceil(&self) -> SuperSampleTime {
        SuperSampleTime((self.0 * 28_224_000.0).ceil() as i64)
    }

    /// Convert to discrete [`RealSampleTime`] floored to the nearest
    /// super-sample, while also return the fractional sub-super-sample part.
    ///
    /// [`SuperSampleTime`]: struct.RealSampleTime.html
    #[inline]
    pub fn to_sub_super_sample(&self) -> (SuperSampleTime, f64) {
        let super_smps_f64 = self.0 * 28_224_000.0;
        (
            SuperSampleTime(super_smps_f64.floor() as i64),
            super_smps_f64.fract(),
        )
    }

    /// Convert to the corresponding [`MusicalTime`].
    ///
    /// [`MusicalTime`]: ../time/struct.MusicalTime.html
    #[inline]
    pub fn to_musical(&self, bpm: f64) -> MusicalTime {
        MusicalTime((self.0 * bpm * (28_224_000.0 / 60.0)).round() as i64)
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
pub struct RealSampleTime(pub i64);

impl RealSampleTime {
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
        self.to_seconds(sample_rate).to_musical(bpm)
    }

    /// Convert to the corresponding time in [`SuperSampleTime`] from the given [`SampleRate`].
    ///
    /// [`SuperSampleTime`]: struct.SuperSampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_super_sample(&self, sample_rate: SampleRate) -> SuperSampleTime {
        SuperSampleTime::from_real_sample_time(*self, sample_rate)
    }
}

impl Default for RealSampleTime {
    fn default() -> Self {
        RealSampleTime(0)
    }
}

impl From<i8> for RealSampleTime {
    fn from(s: i8) -> Self {
        RealSampleTime(i64::from(s))
    }
}
impl From<u8> for RealSampleTime {
    fn from(s: u8) -> Self {
        RealSampleTime(i64::from(s))
    }
}
impl From<i16> for RealSampleTime {
    fn from(s: i16) -> Self {
        RealSampleTime(i64::from(s))
    }
}
impl From<u16> for RealSampleTime {
    fn from(s: u16) -> Self {
        RealSampleTime(i64::from(s))
    }
}
impl From<i32> for RealSampleTime {
    fn from(s: i32) -> Self {
        RealSampleTime(i64::from(s))
    }
}
impl From<u32> for RealSampleTime {
    fn from(s: u32) -> Self {
        RealSampleTime(i64::from(s))
    }
}
impl From<i64> for RealSampleTime {
    fn from(s: i64) -> Self {
        RealSampleTime(s)
    }
}

impl Add<RealSampleTime> for RealSampleTime {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<RealSampleTime> for RealSampleTime {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign<RealSampleTime> for RealSampleTime {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl SubAssign<RealSampleTime> for RealSampleTime {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

/// Unit of time in discrete units of 1 / 28,224,000 seconds. This number
/// happens to be nicely divisible by all common sample rates, allowing
/// changes to sample rate in a project to be a lossless process.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct SuperSampleTime(pub i64);

impl SuperSampleTime {
    #[inline]
    pub fn new(super_samples: i64) -> Self {
        Self(super_samples)
    }

    #[inline]
    pub fn from_usize(super_samples: usize) -> Self {
        Self(super_samples as i64)
    }

    #[inline]
    pub fn from_u64(super_samples: u64) -> Self {
        Self(super_samples as i64)
    }

    pub fn from_real_sample_time(sample_time: RealSampleTime, sample_rate: SampleRate) -> Self {
        match sample_rate.0 as usize {
            44100 => Self(sample_time.0 * (28_224_000 / 44100)),
            48000 => Self(sample_time.0 * (28_224_000 / 48000)),
            88200 => Self(sample_time.0 * (28_224_000 / 88200)),
            96000 => Self(sample_time.0 * (28_224_000 / 96000)),
            176400 => Self(sample_time.0 * (28_224_000 / 176400)),
            192000 => Self(sample_time.0 * (28_224_000 / 192000)),
            22050 => Self(sample_time.0 * (28_224_000 / 22050)),
            24000 => Self(sample_time.0 * (28_224_000 / 24000)),
            _ => Self((sample_time.0 as f64 * (28_224_000.0 / sample_rate.0)).round() as i64),
        }
    }

    /// Get the super-sample time as a `usize` value.
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

    /// Convert to the corresponding time in [`Seconds`].
    ///
    /// [`Seconds`]: struct.Seconds.html
    #[inline]
    pub fn to_seconds(&self) -> Seconds {
        Seconds(self.0 as f64 / 28_224_000.0)
    }

    /// Convert to the corresponding [`MusicalTime`].
    ///
    /// [`MusicalTime`]: struct.MusicalTime.html
    #[inline]
    pub fn to_musical(&self, bpm: f64) -> MusicalTime {
        self.to_seconds().to_musical(bpm)
    }

    /// Convert to the corresponding [`RealSampleTime`] from the given [`SampleRate`].
    ///
    /// [`RealSampleTime`]: struct.RealSampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_real_sample_time(&self, sample_rate: SampleRate) -> RealSampleTime {
        RealSampleTime((self.0 as f64 * sample_rate.0 / 28_224_000.0).round() as i64)
    }
}

impl Default for SuperSampleTime {
    fn default() -> Self {
        SuperSampleTime(0)
    }
}

impl From<i8> for SuperSampleTime {
    fn from(s: i8) -> Self {
        SuperSampleTime(i64::from(s))
    }
}
impl From<u8> for SuperSampleTime {
    fn from(s: u8) -> Self {
        SuperSampleTime(i64::from(s))
    }
}
impl From<i16> for SuperSampleTime {
    fn from(s: i16) -> Self {
        SuperSampleTime(i64::from(s))
    }
}
impl From<u16> for SuperSampleTime {
    fn from(s: u16) -> Self {
        SuperSampleTime(i64::from(s))
    }
}
impl From<i32> for SuperSampleTime {
    fn from(s: i32) -> Self {
        SuperSampleTime(i64::from(s))
    }
}
impl From<u32> for SuperSampleTime {
    fn from(s: u32) -> Self {
        SuperSampleTime(i64::from(s))
    }
}
impl From<i64> for SuperSampleTime {
    fn from(s: i64) -> Self {
        SuperSampleTime(s)
    }
}

impl Add<SuperSampleTime> for SuperSampleTime {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<SuperSampleTime> for SuperSampleTime {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign<SuperSampleTime> for SuperSampleTime {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl SubAssign<SuperSampleTime> for SuperSampleTime {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

/// Unit of time length (of a single de-interleaved channel) in real samples. This is similar
/// to [`RealSampleTime`] except this will always be positive.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct RealFrames(pub usize);

impl RealFrames {
    #[inline]
    pub fn new(real_frames: usize) -> Self {
        Self(real_frames)
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
        self.to_seconds(sample_rate).to_musical(bpm)
    }

    /// Convert to the corresponding [`RealSampleTime`].
    ///
    /// [`RealSampleTimes`]: struct.RealSampleTime.html
    #[inline]
    pub fn to_real_sample_time(&self) -> RealSampleTime {
        RealSampleTime::new(self.0 as i64)
    }

    /// Convert to the corresponding time length in [`SuperFrames`] from the given [`SampleRate`].
    ///
    /// [`SuperFrames`]: struct.SuperFrames.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_super_frames(&self, sample_rate: SampleRate) -> SuperFrames {
        SuperFrames::from_real_frames(*self, sample_rate)
    }
}

impl Default for RealFrames {
    fn default() -> Self {
        RealFrames(0)
    }
}

impl From<u8> for RealFrames {
    fn from(s: u8) -> Self {
        RealFrames(usize::from(s))
    }
}
impl From<u16> for RealFrames {
    fn from(s: u16) -> Self {
        RealFrames(usize::from(s))
    }
}
impl From<usize> for RealFrames {
    fn from(s: usize) -> Self {
        RealFrames(s)
    }
}

impl Add<RealFrames> for RealFrames {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<RealFrames> for RealFrames {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Add<RealFrames> for RealSampleTime {
    type Output = Self;
    fn add(self, rhs: RealFrames) -> Self::Output {
        Self(self.0 + rhs.0 as i64)
    }
}
impl Sub<RealFrames> for RealSampleTime {
    type Output = Self;
    fn sub(self, rhs: RealFrames) -> Self::Output {
        Self(self.0 - rhs.0 as i64)
    }
}

impl AddAssign<RealFrames> for RealFrames {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl SubAssign<RealFrames> for RealFrames {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl AddAssign<RealFrames> for RealSampleTime {
    fn add_assign(&mut self, other: RealFrames) {
        self.0 += other.0 as i64;
    }
}
impl SubAssign<RealFrames> for RealSampleTime {
    fn sub_assign(&mut self, other: RealFrames) {
        self.0 -= other.0 as i64;
    }
}

/// Unit of time length (of a single de-interleaved channel) in super-samples. This is similar
/// to [`SuperSampleTime`] except this will always be positive.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct SuperFrames(pub usize);

impl SuperFrames {
    #[inline]
    pub fn new(real_frames: usize) -> Self {
        Self(real_frames)
    }

    pub fn from_real_frames(real_frames: RealFrames, sample_rate: SampleRate) -> Self {
        match sample_rate.0 as usize {
            44100 => Self(real_frames.0 * (28_224_000 / 44100)),
            48000 => Self(real_frames.0 * (28_224_000 / 48000)),
            88200 => Self(real_frames.0 * (28_224_000 / 88200)),
            96000 => Self(real_frames.0 * (28_224_000 / 96000)),
            176400 => Self(real_frames.0 * (28_224_000 / 176400)),
            192000 => Self(real_frames.0 * (28_224_000 / 192000)),
            22050 => Self(real_frames.0 * (28_224_000 / 22050)),
            24000 => Self(real_frames.0 * (28_224_000 / 24000)),
            _ => Self((real_frames.0 as f64 * (28_224_000.0 / sample_rate.0)).round() as usize),
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
        self.to_seconds(sample_rate).to_musical(bpm)
    }

    /// Convert to the corresponding [`SuperSampleTime`].
    ///
    /// [`SuperSampleTimes`]: struct.SuperSampleTime.html
    #[inline]
    pub fn to_super_sample_time(&self) -> SuperSampleTime {
        SuperSampleTime::new(self.0 as i64)
    }

    /// Convert to the corresponding time length in [`RealFrames`] from the given [`SampleRate`].
    ///
    /// [`RealFrames`]: struct.RealFrames.html
    /// [`SampleRate`]: struct.SampleRate.html
    #[inline]
    pub fn to_real_frames(&self, sample_rate: SampleRate) -> RealFrames {
        RealFrames((self.0 as f64 * sample_rate.0 / 28_224_000.0).round() as usize)
    }
}

impl Default for SuperFrames {
    fn default() -> Self {
        SuperFrames(0)
    }
}

impl From<u8> for SuperFrames {
    fn from(s: u8) -> Self {
        SuperFrames(usize::from(s))
    }
}
impl From<u16> for SuperFrames {
    fn from(s: u16) -> Self {
        SuperFrames(usize::from(s))
    }
}
impl From<usize> for SuperFrames {
    fn from(s: usize) -> Self {
        SuperFrames(s)
    }
}

impl Add<SuperFrames> for SuperFrames {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<SuperFrames> for SuperFrames {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Add<SuperFrames> for SuperSampleTime {
    type Output = Self;
    fn add(self, rhs: SuperFrames) -> Self::Output {
        Self(self.0 + rhs.0 as i64)
    }
}
impl Sub<SuperFrames> for SuperSampleTime {
    type Output = Self;
    fn sub(self, rhs: SuperFrames) -> Self::Output {
        Self(self.0 - rhs.0 as i64)
    }
}

impl AddAssign<SuperFrames> for SuperFrames {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl SubAssign<SuperFrames> for SuperFrames {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl AddAssign<SuperFrames> for SuperSampleTime {
    fn add_assign(&mut self, other: SuperFrames) {
        self.0 += other.0 as i64;
    }
}
impl SubAssign<SuperFrames> for SuperSampleTime {
    fn sub_assign(&mut self, other: SuperFrames) {
        self.0 -= other.0 as i64;
    }
}
