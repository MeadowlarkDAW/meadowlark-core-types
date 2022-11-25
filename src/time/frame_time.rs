use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use super::{MusicalTime, SampleRate, SecondsF64, SuperclockTime};

/// Unit of time length in frames (samples in a single audio channel).
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct FrameTime(pub u64);

impl FrameTime {
    pub fn new(frame: u64) -> Self {
        Self(frame)
    }

    /// Convert to the corresponding time in [`SecondsF64`] with the given [`SampleRate`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SecondsF64`]: struct.SecondsF64.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_seconds_f64(&self, sample_rate: SampleRate) -> SecondsF64 {
        SecondsF64(self.0 as f64 / sample_rate)
    }

    /// Convert to the corresponding [`MusicalTime`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`MusicalTime`]: struct.MusicalTime.html
    pub fn to_musical(&self, bpm: f64, sample_rate: SampleRate) -> MusicalTime {
        self.to_seconds_f64(sample_rate).to_musical(bpm)
    }

    /// Convert to the corresponding time length in [`SuperclockTime`] from the given [`SampleRate`].
    ///
    /// This conversion **IS** lossless if the sample rate happens to be equal to one of the common
    /// sample rates: `22050, 24000, 44100, 48000, 88200, 96000, 176400, or 192000`. This
    /// conversion is *NOT* lossless otherwise.
    ///
    /// [`SuperclockTime`]: struct.SuperclockTime.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_super_frame(&self, sample_rate: SampleRate) -> SuperclockTime {
        SuperclockTime::from_frame(*self, sample_rate)
    }
}

impl Default for FrameTime {
    fn default() -> Self {
        FrameTime(0)
    }
}

impl From<u8> for FrameTime {
    fn from(s: u8) -> Self {
        FrameTime(u64::from(s))
    }
}
impl From<u16> for FrameTime {
    fn from(s: u16) -> Self {
        FrameTime(u64::from(s))
    }
}
impl From<u32> for FrameTime {
    fn from(s: u32) -> Self {
        FrameTime(u64::from(s))
    }
}
impl From<u64> for FrameTime {
    fn from(s: u64) -> Self {
        FrameTime(s)
    }
}
impl From<usize> for FrameTime {
    fn from(s: usize) -> Self {
        FrameTime(s as u64)
    }
}

impl Add<FrameTime> for FrameTime {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<FrameTime> for FrameTime {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul<u64> for FrameTime {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl AddAssign<FrameTime> for FrameTime {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl SubAssign<FrameTime> for FrameTime {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}
impl MulAssign<u64> for FrameTime {
    fn mul_assign(&mut self, other: u64) {
        *self = *self * other
    }
}
