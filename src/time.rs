//! Structs for accurate timekeeping in musical audio applications.

use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// (`56,448,000`) This number was chosen because it is nicely divisible by a whole slew of factors
/// including `2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 15, 16, 18, 20, 24, 32, 64, 128, 256, 512, 1024,
/// and 1920`, as well as common sampling rates such as `22050, 24000, 44100, 48000, 88200, 96000,
/// 176400, and 192000`. This ensures that any recording of note or sample data in this format
/// will always be at-least sample-accurate.
pub static SUPER_UNITS: u32 = 56_448_000;

/// Sampling rate in samples per second.
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

/// Musical time in units of beats + super-beats.
///
/// Note this value is always positive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MusicalTime {
    /// The time in musical beats.
    beats: u32,

    /// The number of super-beats (after the time in `self.beats`). A "super-beat" is a unit of time
    /// equal to 1 / 508,032,000 of a beat. This will auto-wrap so this will always be within the
    /// range `[0, 508,032,000)`.
    ///
    /// This number was chosen because it is nicely divisible by a whole slew of factors
    /// including `2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 15, 16, 18, 20, 24, 32, 64, 128, 256, 512,
    /// and 1920`, as well as common sampling rates such as `22050, 24000, 44100, 48000, 88200, 96000,
    /// 176400, and 192000`. This ensures that any recording of note data in this format will always be
    /// at-least sample-accurate.
    super_beats: u32,
}

impl MusicalTime {
    /// * `beats` - The time in musical beats.
    /// * `super_beats` - The number of super-beats (after the time in `self.beats`) (Note this value
    /// will be constrained to the range `[0, 508,032,000)`).
    ///
    /// A "super-beat" is a unit of time equal to 1 / 508,032,000 of a beat. This number was chosen
    /// because it is nicely divisible by a whole slew of factors including `2, 3, 4, 5, 6, 7, 8, 9,
    /// 10, 12, 14, 15, 16, 18, 20, 24, 32, 64, 128, 256, 512, and 1920`, as well as common sampling
    /// rates such as `22050, 24000, 44100, 48000, 88200, 96000, 176400, and 192000`. This ensures that
    /// any recording of note data in this format will always be at-least sample-accurate.
    pub fn new(beats: u32, super_beats: u32) -> Self {
        Self {
            beats,
            super_beats: super_beats.min(SUPER_UNITS - 1),
        }
    }

    /// The time in musical beats.
    pub fn beats(&self) -> u32 {
        self.beats
    }

    /// The number of super-beats (after the time in `self.beats()`).
    ///
    /// A "super-beat" is a unit of time equal to 1 / 508,032,000 of a beat. This number was chosen
    /// because it is nicely divisible by a whole slew of factors including `2, 3, 4, 5, 6, 7, 8, 9,
    /// 10, 12, 14, 15, 16, 18, 20, 24, 32, 64, 128, 256, 512, and 1920`, as well as common sampling
    /// rates such as `22050, 24000, 44100, 48000, 88200, 96000, 176400, and 192000`. This ensures that
    /// any recording of note data in this format will always be at-least sample-accurate.
    ///
    /// This value will always be in the range `[0, 508,032,000)`.
    pub fn super_beats(&self) -> u32 {
        self.super_beats
    }

    /// * `beats` - The time in musical beats.
    pub fn from_beats(beats: u32) -> Self {
        Self {
            beats,
            super_beats: 0,
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `half_beats` - The number of half-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 1]`.
    pub fn from_half_beats(beats: u32, half_beats: u32) -> Self {
        static N: u32 = 2;

        Self {
            beats,
            super_beats: half_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `quarter_beats` - The number of quarter-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 3]`.
    pub fn from_quarter_beats(beats: u32, quarter_beats: u32) -> Self {
        static N: u32 = 4;

        Self {
            beats,
            super_beats: quarter_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `eighth_beats` - The number of eighth-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 7]`.
    pub fn from_eighth_beats(beats: u32, eighth_beats: u32) -> Self {
        static N: u32 = 8;

        Self {
            beats,
            super_beats: eighth_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `sixteenth_beats` - The number of sixteenth-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 15]`.
    pub fn from_sixteenth_beats(beats: u32, sixteenth_beats: u32) -> Self {
        static N: u32 = 16;

        Self {
            beats,
            super_beats: sixteenth_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `_32nd_beats` - The number of 32nd-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 31]`.
    pub fn from_32nd_beats(beats: u32, _32nd_beats: u32) -> Self {
        static N: u32 = 32;

        Self {
            beats,
            super_beats: _32nd_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `_64th_beats` - The number of 64th-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 63]`.
    pub fn from_64th_beats(beats: u32, _64th_beats: u32) -> Self {
        static N: u32 = 64;

        Self {
            beats,
            super_beats: _64th_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `_128th_beats` - The number of 128th-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 127]`.
    pub fn from_128th_beats(beats: u32, _128th_beats: u32) -> Self {
        static N: u32 = 128;

        Self {
            beats,
            super_beats: _128th_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `third_beats` - The number of third-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 2]`.
    pub fn from_third_beats(beats: u32, third_beats: u32) -> Self {
        static N: u32 = 3;

        Self {
            beats,
            super_beats: third_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `fifth_beats` - The number of fifth-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 4]`.
    pub fn from_fifth_beats(beats: u32, fifth_beats: u32) -> Self {
        static N: u32 = 5;

        Self {
            beats,
            super_beats: fifth_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `sixth_beats` - The number of sixth-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 5]`.
    pub fn from_sixth_beats(beats: u32, fifth_beats: u32) -> Self {
        static N: u32 = 6;

        Self {
            beats,
            super_beats: fifth_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `seventh_beats` - The number of seventh-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 6]`.
    pub fn from_seventh_beats(beats: u32, seventh_beats: u32) -> Self {
        static N: u32 = 7;

        Self {
            beats,
            super_beats: seventh_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `ninth_beats` - The number of ninth-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 8]`.
    pub fn from_ninth_beats(beats: u32, ninth_beats: u32) -> Self {
        static N: u32 = 9;

        Self {
            beats,
            super_beats: ninth_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `tenth_beats` - The number of tenth-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 9]`.
    pub fn from_tenth_beats(beats: u32, tenth_beats: u32) -> Self {
        static N: u32 = 10;

        Self {
            beats,
            super_beats: tenth_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `twelth_beats` - The number of twelth-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 11]`.
    pub fn from_twelth_beats(beats: u32, twelth_beats: u32) -> Self {
        static N: u32 = 12;

        Self {
            beats,
            super_beats: twelth_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// * `beats` - The time in musical beats.
    /// * `_24th_beats` - The number of 24th-beats (after the time `beats`). This will be
    /// constrained to the range `[0, 23]`.
    pub fn from_24th_beats(beats: u32, _24th_beats: u32) -> Self {
        static N: u32 = 24;

        Self {
            beats,
            super_beats: _24th_beats.min(N - 1) * (SUPER_UNITS / N),
        }
    }

    /// Get the corresponding musical time from the number of beats (as an `f64`).
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If `beats` is negative, then a MusicalTime of 0 will be returned instead.
    pub fn from_beats_f64(beats: f64) -> Self {
        if beats > 0.0 {
            Self {
                beats: beats.floor() as u32,
                super_beats: (beats.fract() * f64::from(SUPER_UNITS)).round() as u32,
            }
        } else {
            Self {
                beats: 0,
                super_beats: 0,
            }
        }
    }

    /// Convert the corresponding musical time in units of beats (as an `f64` value).
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// This is useful for displaying notes in UI.
    pub fn as_beats_f64(&self) -> f64 {
        f64::from(self.beats) + (f64::from(self.super_beats) / f64::from(SUPER_UNITS))
    }

    pub fn snap_to_nearest_beat(&self) -> MusicalTime {
        let beats = if self.super_beats < (SUPER_UNITS / 2) {
            self.beats
        } else {
            self.beats + 1
        };

        MusicalTime {
            beats,
            super_beats: 0,
        }
    }

    /// Snap to the nearest multiple of whole beats.
    pub fn snap_to_nearest_whole_beats(&self, beats: u32) -> MusicalTime {
        let nearest_beat = self.snap_to_nearest_beat();
        let mut new_beats = (nearest_beat.beats % beats) * beats;
        if nearest_beat.beats - new_beats >= beats / 2 {
            new_beats += beats / 2;
        }

        MusicalTime {
            beats: new_beats,
            super_beats: 0,
        }
    }

    pub fn snap_to_nearest_fractional_beat<const DIVISOR: u32>(&self) -> MusicalTime {
        let mut beats = self.beats;
        let mut super_beats =
            (self.super_beats % (SUPER_UNITS / DIVISOR)) * (SUPER_UNITS / DIVISOR);
        if self.super_beats - super_beats >= (SUPER_UNITS / DIVISOR) / 2 {
            super_beats += SUPER_UNITS / DIVISOR;
        }
        if super_beats >= SUPER_UNITS {
            beats += 1;
            super_beats = 0;
        }

        MusicalTime { beats, super_beats }
    }

    pub fn snap_to_nearest_half_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<2>()
    }

    pub fn snap_to_nearest_quarter_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<4>()
    }

    pub fn snap_to_nearest_eigth_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<8>()
    }

    pub fn snap_to_nearest_sixteenth_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<16>()
    }

    pub fn snap_to_nearest_32nd_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<32>()
    }

    pub fn snap_to_nearest_64th_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<64>()
    }

    pub fn snap_to_nearest_128th_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<128>()
    }

    pub fn snap_to_nearest_third_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<3>()
    }

    pub fn snap_to_nearest_fifth_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<5>()
    }

    pub fn snap_to_nearest_sixth_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<6>()
    }

    pub fn snap_to_nearest_seventh_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<7>()
    }

    pub fn snap_to_nearest_ninth_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<9>()
    }

    pub fn snap_to_nearest_tenth_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<10>()
    }

    pub fn snap_to_nearest_twelth_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<12>()
    }

    pub fn snap_to_nearest_24th_beat(&self) -> MusicalTime {
        self.snap_to_nearest_fractional_beat::<24>()
    }

    /// The number of fractional-beats *after* `self.beats()` (floored to
    /// the nearest fractional-beat).
    ///
    /// This will always be in the range `[0, DIVISOR - 1]`.
    pub fn num_fractional_beats<const DIVISOR: u32>(&self) -> u32 {
        self.super_beats % (SUPER_UNITS / DIVISOR)
    }

    /// The number of half-beats *after* `self.beats()` (floored to
    /// the nearest half-beat).
    ///
    /// This will always be in the range `[0, 1]`.
    pub fn num_half_beats(&self) -> u32 {
        self.num_fractional_beats::<2>()
    }

    /// The number of quarter-beats *after* `self.beats()` (floored to
    /// the nearest quarter-beat).
    ///
    /// This will always be in the range `[0, 3]`.
    pub fn num_quarter_beats(&self) -> u32 {
        self.num_fractional_beats::<4>()
    }

    /// The number of eigth-beats *after* `self.beats()` (floored to
    /// the nearest eigth-beat).
    ///
    /// This will always be in the range `[0, 7]`.
    pub fn num_eigth_beats(&self) -> u32 {
        self.num_fractional_beats::<8>()
    }

    /// The number of sixteenth-beats *after* `self.beats()` (floored to
    /// the nearest sixteenth-beat).
    ///
    /// This will always be in the range `[0, 15]`.
    pub fn num_sixteenth_beats(&self) -> u32 {
        self.num_fractional_beats::<16>()
    }

    /// The number of 32nd-beats *after* `self.beats()` (floored to
    /// the nearest 32nd-beat).
    ///
    /// This will always be in the range `[0, 31]`.
    pub fn num_32nd_beats(&self) -> u32 {
        self.num_fractional_beats::<32>()
    }

    /// The number of 128th-beats *after* `self.beats()` (floored to
    /// the nearest 64th-beat).
    ///
    /// This will always be in the range `[0, 63]`.
    pub fn num_64th_beats(&self) -> u32 {
        self.num_fractional_beats::<64>()
    }

    /// The number of 64th-beats *after* `self.beats()` (floored to
    /// the nearest 128th-beat).
    ///
    /// This will always be in the range `[0, 127]`.
    pub fn num_128th_beats(&self) -> u32 {
        self.num_fractional_beats::<128>()
    }

    /// The number of third-beats *after* `self.beats()` (floored to
    /// the nearest third-beat).
    ///
    /// This will always be in the range `[0, 2]`.
    pub fn num_third_beats(&self) -> u32 {
        self.num_fractional_beats::<3>()
    }

    /// The number of fifth-beats *after* `self.beats()` (floored to
    /// the nearest fifth-beat).
    ///
    /// This will always be in the range `[0, 4]`.
    pub fn num_fifth_beats(&self) -> u32 {
        self.num_fractional_beats::<5>()
    }

    /// The number of sixth-beats *after* `self.beats()` (floored to
    /// the nearest sixth-beat).
    ///
    /// This will always be in the range `[0, 5]`.
    pub fn num_sixth_beats(&self) -> u32 {
        self.num_fractional_beats::<6>()
    }

    /// The number of seventh-beats *after* `self.beats()` (floored to
    /// the nearest seventh-beat).
    ///
    /// This will always be in the range `[0, 6]`.
    pub fn num_seventh_beats(&self) -> u32 {
        self.num_fractional_beats::<7>()
    }

    /// The number of ninth-beats *after* `self.beats()` (floored to
    /// the nearest ninth-beat).
    ///
    /// This will always be in the range `[0, 8]`.
    pub fn num_ninth_beats(&self) -> u32 {
        self.num_fractional_beats::<9>()
    }

    /// The number of tenth-beats *after* `self.beats()` (floored to
    /// the nearest tenth-beat).
    ///
    /// This will always be in the range `[0, 9]`.
    pub fn num_tenth_beats(&self) -> u32 {
        self.num_fractional_beats::<10>()
    }

    /// The number of twelth-beats *after* `self.beats()` (floored to
    /// the nearest twelth-beat).
    ///
    /// This will always be in the range `[0, 11]`.
    pub fn num_twelth_beats(&self) -> u32 {
        self.num_fractional_beats::<12>()
    }

    /// The number of 24th-beats *after* `self.beats()` (floored to
    /// the nearest 24th-beat).
    ///
    /// This will always be in the range `[0, 23]`.
    pub fn num_24th_beats(&self) -> u32 {
        self.num_fractional_beats::<24>()
    }

    /// Convert to the corresponding time in [`Seconds`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`Seconds`]: struct.Seconds.html
    pub fn to_seconds(&self, bpm: f64) -> Seconds {
        Seconds(self.as_beats_f64() * 60.0 / bpm)
    }

    /// Convert to the corresponding discrete [`Frames`]. This will be rounded to the nearest sample.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`Frames`]: struct.Frames.html
    pub fn to_nearest_frame_round(&self, bpm: f64, sample_rate: SampleRate) -> Frames {
        self.to_seconds(bpm).to_nearest_frame_round(sample_rate)
    }

    /// Convert to the corresponding discrete [`Frames`]. This will be floored to the nearest sample.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`Frames`]: struct.Frames.html
    pub fn to_nearest_frame_floor(&self, bpm: f64, sample_rate: SampleRate) -> Frames {
        self.to_seconds(bpm).to_nearest_frame_floor(sample_rate)
    }

    /// Convert to the corresponding discrete [`Frames`]. This will be ceil-ed to the nearest sample.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`Frames`]: struct.Frames.html
    pub fn to_nearest_frame_ceil(&self, bpm: f64, sample_rate: SampleRate) -> Frames {
        self.to_seconds(bpm).to_nearest_frame_ceil(sample_rate)
    }

    /// Convert to the corresponding discrete [`Frames`] floored to the nearest sample,
    /// while also returning the fractional sub-sample part.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`Frames`]: struct.Frames.html
    pub fn to_sub_frames(&self, bpm: f64, sample_rate: SampleRate) -> (Frames, f64) {
        self.to_seconds(bpm).to_sub_frames(sample_rate)
    }

    /// Convert to the corresponding discrete [`SuperFrames`]. This will be rounded to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SuperFrames`]: struct.SuperFrames.html
    pub fn to_nearest_super_frame_round(&self, bpm: f64) -> SuperFrames {
        self.to_seconds(bpm).to_nearest_super_frame_round()
    }

    /// Convert to the corresponding discrete [`SuperFrames`]. This will be floored to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SuperFrames`]: struct.SuperFrames.html
    pub fn to_nearest_super_frame_floor(&self, bpm: f64) -> SuperFrames {
        self.to_seconds(bpm).to_nearest_super_frame_floor()
    }

    /// Convert to the corresponding discrete [`SuperFrames`]. This will be ceil-ed to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SuperFrames`]: struct.SuperFrames.html
    pub fn to_nearest_super_frame_ceil(&self, bpm: f64) -> SuperFrames {
        self.to_seconds(bpm).to_nearest_super_frame_ceil()
    }

    /// Convert to the corresponding discrete [`SuperFrames`] floored to the nearest super-frame,
    /// while also returning the fractional sub-super-frame part.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SuperFrames`]: struct.SuperFrames.html
    pub fn to_sub_super_frames(&self, bpm: f64) -> (SuperFrames, f64) {
        self.to_seconds(bpm).to_sub_super_frames()
    }

    /// Try subtracting `rhs` from self. This will return `None` if the resulting value
    /// is negative due to `rhs` being larger than self (overflow).
    pub fn checked_sub(self, rhs: MusicalTime) -> Option<MusicalTime> {
        if self >= rhs {
            let mut beats = self.beats - rhs.beats;
            let super_beats = if self.super_beats < rhs.super_beats {
                beats -= 1;
                SUPER_UNITS - (rhs.super_beats - self.super_beats)
            } else {
                self.super_beats - rhs.super_beats
            };

            Some(Self { beats, super_beats })
        } else {
            None
        }
    }
}

impl Default for MusicalTime {
    fn default() -> Self {
        MusicalTime {
            beats: 0,
            super_beats: 0,
        }
    }
}

impl PartialOrd for MusicalTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.beats.partial_cmp(&other.beats) {
            Some(std::cmp::Ordering::Equal) => self.super_beats.partial_cmp(&other.super_beats),
            res => res,
        }
    }
}

impl Add<MusicalTime> for MusicalTime {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut beats = self.beats + rhs.beats;
        let mut super_beats = self.super_beats + rhs.super_beats;
        if super_beats >= SUPER_UNITS {
            super_beats -= SUPER_UNITS;
            beats += 1;
        }

        Self { beats, super_beats }
    }
}
impl Mul<u32> for MusicalTime {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        let mut beats = self.beats * rhs;
        let mut super_beats_u64 = u64::from(self.super_beats) * u64::from(rhs);
        if super_beats_u64 >= u64::from(SUPER_UNITS) {
            let additional_beats = super_beats_u64 % u64::from(SUPER_UNITS);
            beats += additional_beats as u32;
            super_beats_u64 -= additional_beats * u64::from(SUPER_UNITS);
        }

        Self {
            beats,
            super_beats: super_beats_u64 as u32,
        }
    }
}

impl AddAssign<MusicalTime> for MusicalTime {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}
impl MulAssign<u32> for MusicalTime {
    fn mul_assign(&mut self, other: u32) {
        *self = *self * other
    }
}

/// Unit of time in "Seconds"
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Seconds(pub f64);

impl Seconds {
    pub fn new(seconds: f64) -> Self {
        Seconds(seconds)
    }

    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }

    /// Creates a new time in `Seconds` from [`Frames`] and a [`SampleRate`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`Frames`]: struct.Frames.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn from_frames(sample_time: Frames, sample_rate: SampleRate) -> Self {
        sample_time.to_seconds(sample_rate)
    }

    /// Creates a new time in `Seconds` from [`SuperFrames`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`SuperFrames`]: struct.SuperFrames.html
    pub fn from_super_frames(super_frame_time: SuperFrames) -> Self {
        super_frame_time.to_seconds()
    }

    /// Convert to discrete [`Frames`] with the given [`SampleRate`]. This will
    /// be rounded to the nearest sample.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then `Frames(0)` will be returned instead.
    ///
    /// [`Frames`]: struct.Frames.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_frame_round(&self, sample_rate: SampleRate) -> Frames {
        if self.0 > 0.0 {
            Frames((self.0 * sample_rate).round() as u64)
        } else {
            Frames(0)
        }
    }

    /// Convert to discrete [`Frames`] with the given [`SampleRate`]. This will
    /// be floored to the nearest sample.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then `Frames(0)` will be returned instead.
    ///
    /// [`Frames`]: struct.Frames.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_frame_floor(&self, sample_rate: SampleRate) -> Frames {
        if self.0 > 0.0 {
            Frames((self.0 * sample_rate).floor() as u64)
        } else {
            Frames(0)
        }
    }

    /// Convert to discrete [`Frames`] with the given [`SampleRate`]. This will
    /// be ceil-ed to the nearest sample.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then `Frames(0)` will be returned instead.
    ///
    /// [`Frames`]: struct.Frames.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_frame_ceil(&self, sample_rate: SampleRate) -> Frames {
        if self.0 > 0.0 {
            Frames((self.0 * sample_rate).ceil() as u64)
        } else {
            Frames(0)
        }
    }

    /// Convert to discrete [`Frames`] given the [`SampleRate`] floored to the nearest
    /// sample, while also return the fractional sub-sample part.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then `(Frames(0), 0.0)` will be returned instead.
    ///
    /// [`Frames`]: struct.Frames.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_sub_frames(&self, sample_rate: SampleRate) -> (Frames, f64) {
        if self.0 > 0.0 {
            let frames_f64 = self.0 * sample_rate;
            (Frames(frames_f64.floor() as u64), frames_f64.fract())
        } else {
            (Frames(0), 0.0)
        }
    }

    /// Convert to discrete [`SuperFrames`]. This will
    /// be rounded to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperFrames`'s value will be 0.
    ///
    /// [`SuperFrames`]: struct.Frames.html
    pub fn to_nearest_super_frame_round(&self) -> SuperFrames {
        if self.0 > 0.0 {
            SuperFrames((self.0 * f64::from(SUPER_UNITS)).round() as u64)
        } else {
            SuperFrames(0)
        }
    }

    /// Convert to discrete [`SuperFrames`]. This will
    /// be floored to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperFrames`'s values will be 0.
    ///
    /// [`SuperFrames`]: struct.Frames.html
    pub fn to_nearest_super_frame_floor(&self) -> SuperFrames {
        if self.0 > 0.0 {
            SuperFrames((self.0 * f64::from(SUPER_UNITS)).floor() as u64)
        } else {
            SuperFrames(0)
        }
    }

    /// Convert to discrete [`SuperFrames`]. This will
    /// be ceil-ed to the nearest super-frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperFrames`'s values will be 0.
    ///
    /// [`SuperFrames`]: struct.Frames.html
    pub fn to_nearest_super_frame_ceil(&self) -> SuperFrames {
        if self.0 > 0.0 {
            SuperFrames((self.0 * f64::from(SUPER_UNITS)).ceil() as u64)
        } else {
            SuperFrames(0)
        }
    }

    /// Convert to discrete [`Frames`] floored to the nearest
    /// super-frame, while also return the fractional sub-super-frame part.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperFrames`'s values and the
    /// fractional value will both be 0.
    ///
    /// [`SuperFrames`]: struct.Frames.html
    pub fn to_sub_super_frames(&self) -> (SuperFrames, f64) {
        if self.0 > 0.0 {
            let frames_f64 = self.0 * f64::from(SUPER_UNITS);
            (SuperFrames(frames_f64.floor() as u64), frames_f64.fract())
        } else {
            (SuperFrames(0), 0.0)
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

/// Unit of time length (of a single de-interleaved channel) in samples.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct Frames(pub u64);

impl Frames {
    pub fn new(frames: u64) -> Self {
        Self(frames)
    }

    /// Convert to the corresponding time in [`Seconds`] with the given [`SampleRate`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`Seconds`]: struct.Seconds.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_seconds(&self, sample_rate: SampleRate) -> Seconds {
        Seconds(self.0 as f64 / sample_rate)
    }

    /// Convert to the corresponding [`MusicalTime`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// Note that this must be re-calculated after recieving a new [`SampleRate`].
    ///
    /// [`MusicalTime`]: struct.MusicalTime.html
    pub fn to_musical(&self, bpm: f64, sample_rate: SampleRate) -> MusicalTime {
        self.to_seconds(sample_rate).to_musical(bpm)
    }

    /// Convert to the corresponding time length in [`SuperFrames`] from the given [`SampleRate`].
    ///
    /// This conversion **IS** lossless if the sample rate happens to be equal to one of the common
    /// sample rates: `22050, 24000, 44100, 48000, 88200, 96000, 176400, or 192000`. This
    /// conversion is *NOT* lossless otherwise.
    ///
    /// [`SuperFrames`]: struct.SuperFrames.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_super_frames(&self, sample_rate: SampleRate) -> SuperFrames {
        SuperFrames::from_frames(*self, sample_rate)
    }
}

impl Default for Frames {
    fn default() -> Self {
        Frames(0)
    }
}

impl From<u8> for Frames {
    fn from(s: u8) -> Self {
        Frames(u64::from(s))
    }
}
impl From<u16> for Frames {
    fn from(s: u16) -> Self {
        Frames(u64::from(s))
    }
}
impl From<u32> for Frames {
    fn from(s: u32) -> Self {
        Frames(u64::from(s))
    }
}
impl From<u64> for Frames {
    fn from(s: u64) -> Self {
        Frames(s)
    }
}
impl From<usize> for Frames {
    fn from(s: usize) -> Self {
        Frames(s as u64)
    }
}

impl Add<Frames> for Frames {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<Frames> for Frames {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul<u64> for Frames {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl AddAssign<Frames> for Frames {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
impl SubAssign<Frames> for Frames {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}
impl MulAssign<u64> for Frames {
    fn mul_assign(&mut self, other: u64) {
        *self = *self * other
    }
}

/// Unit of time length (of a single de-interleaved channel) in super-frames.
///
/// A "super-frame" is a unit of time that is exactly 1 / 508,032,000 of a second.
/// This number happens to be nicely divisible by all common sampling rates, allowing
/// changes to sample rate in a project to be a lossless process.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct SuperFrames(pub u64);

impl SuperFrames {
    /// * `super_frames` - The number of super-frames
    ///
    /// A "super-frame" is a unit of time equal to 1 / 508,032,000 of a second. This number was chosen
    /// because it is nicely divisible by a whole slew of factors including `2, 3, 4, 5, 6, 7, 8, 9,
    /// 10, 12, 14, 15, 16, 18, 20, 24, 32, 64, 128, 256, 512, and 1920`, as well as common sampling
    /// rates such as `22050, 24000, 44100, 48000, 88200, 96000, 176400, and 192000`. This ensures that
    /// any recording of frame data in this format will always be at-least sample-accurate.
    pub fn new(super_frames: u64) -> Self {
        Self(super_frames)
    }

    /// Get the time in [`SuperFrames`] from the time in [`Seconds`]
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// If the seconds value is negative, then the `SuperFrames`'s values will be 0.
    ///
    /// [`SuperFrames`]: struct.SuperFrames.html
    /// [`Seconds`]: struct.Seconds.html
    pub fn from_seconds(seconds: Seconds) -> Self {
        seconds.to_nearest_super_frame_round()
    }

    /// Get the time in [`SuperFrames`] from the time in [`Frames`].
    ///
    /// This conversion **IS** lossless if the sample rate happens to be equal to one of the common
    /// sample rates: `22050, 24000, 44100, 48000, 88200, 96000, 176400, or 192000`. This
    /// conversion is *NOT* lossless otherwise.
    ///
    /// [`SuperFrames`]: struct.SuperFrames.html
    /// [`Frames`]: struct.Frames.html
    pub fn from_frames(frames: Frames, sample_rate: SampleRate) -> Self {
        match sample_rate.0 as usize {
            44100 => Self(frames.0 * (u64::from(SUPER_UNITS) / 44100)),
            48000 => Self(frames.0 * (u64::from(SUPER_UNITS) / 48000)),
            88200 => Self(frames.0 * (u64::from(SUPER_UNITS) / 88200)),
            96000 => Self(frames.0 * (u64::from(SUPER_UNITS) / 96000)),
            176400 => Self(frames.0 * (u64::from(SUPER_UNITS) / 176400)),
            192000 => Self(frames.0 * (u64::from(SUPER_UNITS) / 192000)),
            22050 => Self(frames.0 * (u64::from(SUPER_UNITS) / 22050)),
            24000 => Self(frames.0 * (u64::from(SUPER_UNITS) / 24000)),
            _ => Self((frames.0 as f64 * (f64::from(SUPER_UNITS) / sample_rate.0)).round() as u64),
        }
    }

    /// Convert to the corresponding time in [`Seconds`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`Seconds`]: struct.Seconds.html
    pub fn to_seconds(&self) -> Seconds {
        Seconds(self.0 as f64 / f64::from(SUPER_UNITS))
    }

    /// Convert to the corresponding [`MusicalTime`].
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`MusicalTime`]: struct.MusicalTime.html
    pub fn to_musical(&self, bpm: f64) -> MusicalTime {
        self.to_seconds().to_musical(bpm)
    }

    /// Convert to the corresponding time length in [`Frames`] from the given [`SampleRate`],
    /// rounded to the nearest frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`Frames`]: struct.Frames.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_frame_round(&self, sample_rate: SampleRate) -> Frames {
        self.to_seconds().to_nearest_frame_round(sample_rate)
    }

    /// Convert to the corresponding time length in [`Frames`] from the given [`SampleRate`],
    /// floored to the nearest frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`Frames`]: struct.Frames.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_frame_floor(&self, sample_rate: SampleRate) -> Frames {
        self.to_seconds().to_nearest_frame_floor(sample_rate)
    }

    /// Convert to the corresponding time length in [`Frames`] from the given [`SampleRate`],
    /// ceil-ed to the nearest frame.
    ///
    /// Note that this conversion is *NOT* lossless.
    ///
    /// [`Frames`]: struct.Frames.html
    /// [`SampleRate`]: struct.SampleRate.html
    pub fn to_nearest_frame_ceil(&self, sample_rate: SampleRate) -> Frames {
        self.to_seconds().to_nearest_frame_ceil(sample_rate)
    }
}

impl Default for SuperFrames {
    fn default() -> Self {
        SuperFrames(0)
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
impl Mul<u64> for SuperFrames {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl AddAssign<SuperFrames> for SuperFrames {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}
impl SubAssign<SuperFrames> for SuperFrames {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}
impl MulAssign<u64> for SuperFrames {
    fn mul_assign(&mut self, other: u64) {
        *self = *self * other
    }
}
