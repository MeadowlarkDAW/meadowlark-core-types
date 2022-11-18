use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use super::{MusicalTime, SampleRate, SampleTime, Seconds};

/// (`282,240,000`) This number was chosen because it is nicely divisible by all the common sample
/// rates: `22050, 24000, 44100, 48000, 88200, 96000, 176400, and 192000`. This ensures that no
/// information is lost when switching between sample rates.
pub static SUPER_SAMPLE_TICKS_PER_SECOND: u32 = 282_240_000;

/// Unit of time length in super-samples (of a single de-interleaved channel).
///
/// A "super-sample" is a unit of time that is exactly 1 / 282,240,000 of a second. This number
/// happens to be nicely divisible by all common sampling rates. This ensures that no information
/// is lost when switching between sample rates.
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct SuperSampleTime(pub u64);

impl SuperSampleTime {
    /// * `super_samples` - The number of super-samples
    ///
    /// A "super-sample" is a unit of time that is exactly 1 / 282,240,000 of a second. This
    /// number happens to be nicely divisible by all common sampling rates. This ensures that
    /// no information is lost when switching between sample rates.
    pub fn new(super_sample: u64) -> Self {
        Self(super_sample)
    }

    /// Get the time in [`SuperSampleTime`] from the time in [`Seconds`]
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperSampleTime`'s values will be 0.
    ///
    /// [`SuperSampleTime`]: struct.SuperSampleTime.html
    /// [`Seconds`]: struct.Seconds.html
    pub fn from_seconds(seconds: Seconds) -> Self {
        seconds.to_nearest_super_sample_round()
    }

    /// Get the time in [`SuperSampleTime`] from the time in [`SampleTime`].
    ///
    /// This conversion **IS** lossless if the sample rate happens to be equal to one of the
    /// common sample rates: `22050, 24000, 44100, 48000, 88200, 96000, 176400, or 192000`.
    /// This conversion is *NOT* lossless otherwise.
    ///
    /// [`SuperSampleTime`]: struct.SuperSampleTime.html
    /// [`SampleTime`]: struct.SampleTime.html
    pub fn from_sample(sample: SampleTime, sample_rate: SampleRate) -> Self {
        match sample_rate.0 as usize {
            44100 => Self(sample.0 * (u64::from(SUPER_SAMPLE_TICKS_PER_SECOND) / 44100)),
            48000 => Self(sample.0 * (u64::from(SUPER_SAMPLE_TICKS_PER_SECOND) / 48000)),
            88200 => Self(sample.0 * (u64::from(SUPER_SAMPLE_TICKS_PER_SECOND) / 88200)),
            96000 => Self(sample.0 * (u64::from(SUPER_SAMPLE_TICKS_PER_SECOND) / 96000)),
            176400 => Self(sample.0 * (u64::from(SUPER_SAMPLE_TICKS_PER_SECOND) / 176400)),
            192000 => Self(sample.0 * (u64::from(SUPER_SAMPLE_TICKS_PER_SECOND) / 192000)),
            22050 => Self(sample.0 * (u64::from(SUPER_SAMPLE_TICKS_PER_SECOND) / 22050)),
            24000 => Self(sample.0 * (u64::from(SUPER_SAMPLE_TICKS_PER_SECOND) / 24000)),
            _ => Self(
                (sample.0 as f64
                    * ((u64::from(SUPER_SAMPLE_TICKS_PER_SECOND) as f64) / sample_rate.0))
                    .round() as u64,
            ),
        }
    }

    /// Convert to the corresponding time in [`Seconds`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`Seconds`]: struct.Seconds.html
    pub fn to_seconds(&self) -> Seconds {
        Seconds(self.0 as f64 / f64::from(SUPER_SAMPLE_TICKS_PER_SECOND))
    }

    /// Convert to the corresponding [`MusicalTime`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`MusicalTime`]: struct.MusicalTime.html
    pub fn to_musical(&self, bpm: f64) -> MusicalTime {
        self.to_seconds().to_musical(bpm)
    }

    /// Convert to the corresponding time length in [`SampleTime`] from the given [`SampleRate`],
    /// rounded to the nearest frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_sample_round(&self, sample_rate: SampleRate) -> SampleTime {
        self.to_seconds().to_nearest_sample_round(sample_rate)
    }

    /// Convert to the corresponding time length in [`SampleTime`] from the given [`SampleRate`],
    /// floored to the nearest frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_sample_floor(&self, sample_rate: SampleRate) -> SampleTime {
        self.to_seconds().to_nearest_sample_floor(sample_rate)
    }

    /// Convert to the corresponding time length in [`SampleTime`] from the given [`SampleRate`],
    /// ceil-ed to the nearest frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SampleTime`]: struct.SampleTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_sample_ceil(&self, sample_rate: SampleRate) -> SampleTime {
        self.to_seconds().to_nearest_sample_ceil(sample_rate)
    }
}

impl Default for SuperSampleTime {
    fn default() -> Self {
        SuperSampleTime(0)
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
impl Mul<u64> for SuperSampleTime {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl AddAssign<SuperSampleTime> for SuperSampleTime {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}
impl SubAssign<SuperSampleTime> for SuperSampleTime {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}
impl MulAssign<u64> for SuperSampleTime {
    fn mul_assign(&mut self, other: u64) {
        *self = *self * other
    }
}
