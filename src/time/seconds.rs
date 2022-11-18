use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::{MusicalTime, SampleRate, SampleTime, SuperSampleTime, SUPER_SAMPLE_TICKS_PER_SECOND};

/// Unit of time in "Seconds"
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Seconds(pub f64);

impl Seconds {
    pub fn new(seconds: f64) -> Self {
        Seconds(seconds)
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
        sample.to_seconds(sample_rate)
    }

    /// Creates a new time in `Seconds` from [`SuperSampleTime`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SuperSampleTime`]: struct.SuperSampleTime.html
    pub fn from_super_sample(super_sample: SuperSampleTime) -> Self {
        super_sample.to_seconds()
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

    /// Convert to discrete [`SuperSampleTime`]. This will
    /// be rounded to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperSampleTime`'s value will be 0.
    ///
    /// [`SuperSampleTime`]: struct.SampleTime.html
    pub fn to_nearest_super_sample_round(&self) -> SuperSampleTime {
        if self.0 > 0.0 {
            SuperSampleTime((self.0 * f64::from(SUPER_SAMPLE_TICKS_PER_SECOND)).round() as u64)
        } else {
            SuperSampleTime(0)
        }
    }

    /// Convert to discrete [`SuperSampleTime`]. This will
    /// be floored to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperSampleTime`'s values will be 0.
    ///
    /// [`SuperSampleTime`]: struct.SampleTime.html
    pub fn to_nearest_super_sample_floor(&self) -> SuperSampleTime {
        if self.0 > 0.0 {
            SuperSampleTime((self.0 * f64::from(SUPER_SAMPLE_TICKS_PER_SECOND)).floor() as u64)
        } else {
            SuperSampleTime(0)
        }
    }

    /// Convert to discrete [`SuperSampleTime`]. This will
    /// be ceil-ed to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperSampleTime`'s values will be 0.
    ///
    /// [`SuperSampleTime`]: struct.SampleTime.html
    pub fn to_nearest_super_sample_ceil(&self) -> SuperSampleTime {
        if self.0 > 0.0 {
            SuperSampleTime((self.0 * f64::from(SUPER_SAMPLE_TICKS_PER_SECOND)).ceil() as u64)
        } else {
            SuperSampleTime(0)
        }
    }

    /// Convert to discrete [`SampleTime`] floored to the nearest
    /// super-frame, while also return the fractional sub-super-frame part.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperSampleTime`'s values and the
    /// fractional value will both be 0.
    ///
    /// [`SuperSampleTime`]: struct.SampleTime.html
    pub fn to_sub_super_sample(&self) -> (SuperSampleTime, f64) {
        if self.0 > 0.0 {
            let samples_f64 = self.0 * f64::from(SUPER_SAMPLE_TICKS_PER_SECOND);
            (
                SuperSampleTime(samples_f64.floor() as u64),
                samples_f64.fract(),
            )
        } else {
            (SuperSampleTime(0), 0.0)
        }
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
