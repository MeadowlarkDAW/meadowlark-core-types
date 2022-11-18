use std::ops::{Div, Mul};

/// Sampling rate in samples per second.
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SampleRate(pub f64);

impl SampleRate {
    pub fn new(sample_rate: f64) -> Self {
        assert!(sample_rate > 0.0);

        SampleRate(sample_rate)
    }

    /// Returns the reciprocal of the sample rate (`1.0 / sample_rate`).
    ///
    /// Note this is *NOT* cached, so this will always use a division operation.
    pub fn recip(&self) -> f64 {
        self.0.recip()
    }

    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }

    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }

    pub fn as_u16(&self) -> u16 {
        self.0.round() as u16
    }

    pub fn as_u32(&self) -> u32 {
        self.0.round() as u32
    }

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
