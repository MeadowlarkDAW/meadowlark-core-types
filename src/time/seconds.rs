use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::{MusicalTime, SampleRate, SampleTime, SuperclockTime};

/// Unit of time in "Seconds"
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SecondsF64(pub f64);

impl SecondsF64 {
    pub fn new(seconds: f64) -> Self {
        SecondsF64(seconds)
    }

    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }

    /// Creates a new time in `Seconds` from [`SampleTime`] and a [`SampleRate`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn from_sample(sample: SampleTime, sample_rate: SampleRate) -> Self {
        sample.to_seconds_f64(sample_rate)
    }

    /// Creates a new time in `Seconds` from [`SuperclockTime`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SuperclockTime`]: struct.SuperclockTime.html
    pub fn from_superclock_time(superclock_time: SuperclockTime) -> Self {
        superclock_time.to_seconds_f64()
    }

    /// Convert to discrete [`SampleTime`] with the given [`SampleRate`]. This will
    /// be rounded to the nearest sample.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then `SampleTime(0)` will be returned instead.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_sample_round(&self, sample_rate: SampleRate) -> SampleTime {
        if self.0 > 0.0 {
            SampleTime((self.0 * sample_rate).round() as u64)
        } else {
            SampleTime(0)
        }
    }

    /// Convert to discrete [`SampleTime`] with the given [`SampleRate`]. This will
    /// be floored to the nearest sample.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then `SampleTime(0)` will be returned instead.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_sample_floor(&self, sample_rate: SampleRate) -> SampleTime {
        if self.0 > 0.0 {
            SampleTime((self.0 * sample_rate).floor() as u64)
        } else {
            SampleTime(0)
        }
    }

    /// Convert to discrete [`SampleTime`] with the given [`SampleRate`]. This will
    /// be ceil-ed to the nearest sample.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then `SampleTime(0)` will be returned instead.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_sample_ceil(&self, sample_rate: SampleRate) -> SampleTime {
        if self.0 > 0.0 {
            SampleTime((self.0 * sample_rate).ceil() as u64)
        } else {
            SampleTime(0)
        }
    }

    /// Convert to discrete [`SampleTime`] given the [`SampleRate`] floored to the nearest
    /// sample, while also return the fractional sub-sample part.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then `(SampleTime(0), 0.0)` will be returned instead.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_sub_sample(&self, sample_rate: SampleRate) -> (SampleTime, f64) {
        if self.0 > 0.0 {
            let samples_f64 = self.0 * sample_rate;
            (SampleTime(samples_f64.floor() as u64), samples_f64.fract())
        } else {
            (SampleTime(0), 0.0)
        }
    }

    /// Convert to discrete [`SuperclockTime`]. This will
    /// be rounded to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperclockTime`'s value will be 0.
    ///
    /// [`SuperclockTime`]: struct.SampleTime.html
    pub fn to_nearest_super_sample_round(&self) -> SuperclockTime {
        SuperclockTime::from_seconds_f64(*self)
    }

    /// Convert to discrete [`SuperclockTime`]. This will
    /// be floored to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperclockTime`'s values will be 0.
    ///
    /// [`SuperclockTime`]: struct.SampleTime.html
    pub fn to_nearest_super_sample_floor(&self) -> SuperclockTime {
        SuperclockTime::from_seconds_f64_floor(*self)
    }

    /// Convert to discrete [`SuperclockTime`]. This will
    /// be ceil-ed to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperclockTime`'s values will be 0.
    ///
    /// [`SuperclockTime`]: struct.SampleTime.html
    pub fn to_nearest_super_sample_ceil(&self) -> SuperclockTime {
        SuperclockTime::from_seconds_f64_ceil(*self)
    }

    /// Convert to discrete [`SampleTime`] floored to the nearest
    /// super-frame, while also return the fractional sub-super-frame part.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperclockTime`'s values and the
    /// fractional value will both be 0.
    ///
    /// [`SuperclockTime`]: struct.SampleTime.html
    pub fn to_sub_super_sample(&self) -> (SuperclockTime, f64) {
        SuperclockTime::from_seconds_f64_with_sub_tick(*self)
    }

    /// Convert to the corresponding [`MusicalTime`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`MusicalTime`]: ../time/struct.MusicalTime.html
    pub fn to_musical(&self, bpm: f64) -> MusicalTime {
        MusicalTime::from_beats_f64(self.0 * (bpm / 60.0))
    }
}

impl Default for SecondsF64 {
    fn default() -> Self {
        SecondsF64(0.0)
    }
}

impl From<i8> for SecondsF64 {
    fn from(s: i8) -> Self {
        SecondsF64(f64::from(s))
    }
}
impl From<u8> for SecondsF64 {
    fn from(s: u8) -> Self {
        SecondsF64(f64::from(s))
    }
}
impl From<i16> for SecondsF64 {
    fn from(s: i16) -> Self {
        SecondsF64(f64::from(s))
    }
}
impl From<u16> for SecondsF64 {
    fn from(s: u16) -> Self {
        SecondsF64(f64::from(s))
    }
}
impl From<i32> for SecondsF64 {
    fn from(s: i32) -> Self {
        SecondsF64(f64::from(s))
    }
}
impl From<u32> for SecondsF64 {
    fn from(s: u32) -> Self {
        SecondsF64(f64::from(s))
    }
}
impl From<f32> for SecondsF64 {
    fn from(s: f32) -> Self {
        SecondsF64(f64::from(s))
    }
}

impl Add<SecondsF64> for SecondsF64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<SecondsF64> for SecondsF64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul<SecondsF64> for SecondsF64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl Div<SecondsF64> for SecondsF64 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl AddAssign<SecondsF64> for SecondsF64 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl SubAssign<SecondsF64> for SecondsF64 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}
impl MulAssign<SecondsF64> for SecondsF64 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}
impl DivAssign<SecondsF64> for SecondsF64 {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
    }
}
