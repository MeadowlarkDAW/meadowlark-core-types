// Some modified code from baseplug:
//
// https://github.com/wrl/baseplug/blob/trunk/src/smooth.rs
// https://github.com/wrl/baseplug/blob/trunk/LICENSE-APACHE
// https://github.com/wrl/baseplug/blob/trunk/LICENSE-MIT
//
//  Thanks wrl! :)

use std::fmt;
use std::ops;
use std::slice;

use super::{ProcFrames, SampleRate, Seconds};

const SETTLE: f32 = 0.00001f32;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SmoothStatus {
    Inactive,
    Active,
    Deactivating,
}

impl SmoothStatus {
    fn is_active(&self) -> bool {
        self != &SmoothStatus::Inactive
    }
}

pub struct SmoothOutputF32<'a, const MAX_BLOCKSIZE: usize> {
    pub values: &'a [f32; MAX_BLOCKSIZE],
    pub status: SmoothStatus,
}

impl<'a, const MAX_BLOCKSIZE: usize> SmoothOutputF32<'a, MAX_BLOCKSIZE> {
    pub fn is_smoothing(&self) -> bool {
        self.status.is_active()
    }
}

impl<'a, I, const MAX_BLOCKSIZE: usize> ops::Index<I> for SmoothOutputF32<'a, MAX_BLOCKSIZE>
where
    I: slice::SliceIndex<[f32]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, idx: I) -> &I::Output {
        &self.values[idx]
    }
}

pub struct SmoothF32<const MAX_BLOCKSIZE: usize> {
    output: [f32; MAX_BLOCKSIZE],
    input: f32,

    status: SmoothStatus,

    a: f32,
    b: f32,
    last_output: f32,
}

impl<const MAX_BLOCKSIZE: usize> SmoothF32<MAX_BLOCKSIZE> {
    pub fn new(input: f32) -> Self {
        Self {
            status: SmoothStatus::Inactive,
            input,
            output: [input; MAX_BLOCKSIZE],

            a: 1.0,
            b: 0.0,
            last_output: input,
        }
    }

    pub fn reset(&mut self, val: f32) {
        *self = Self {
            a: self.a,
            b: self.b,
            ..Self::new(val)
        };
    }

    pub fn set(&mut self, val: f32) {
        self.input = val;
        self.status = SmoothStatus::Active;
    }

    pub fn dest(&self) -> f32 {
        self.input
    }

    pub fn output(&self) -> SmoothOutputF32<MAX_BLOCKSIZE> {
        SmoothOutputF32 {
            values: &self.output,
            status: self.status,
        }
    }

    pub fn current_value(&self) -> (f32, SmoothStatus) {
        (self.last_output, self.status)
    }

    pub fn update_status_with_epsilon(&mut self, epsilon: f32) -> SmoothStatus {
        let status = self.status;

        match status {
            SmoothStatus::Active => {
                if (self.input - self.output[0]).abs() < epsilon {
                    self.reset(self.input);
                    self.status = SmoothStatus::Deactivating;
                }
            }

            SmoothStatus::Deactivating => self.status = SmoothStatus::Inactive,

            _ => (),
        };

        self.status
    }

    pub fn process(&mut self, proc_frames: ProcFrames<MAX_BLOCKSIZE>) {
        if self.status != SmoothStatus::Active {
            return;
        }

        let frames = proc_frames.compiler_hint_frames();
        let input = self.input * self.a;

        self.output[0] = input + (self.last_output * self.b);

        for i in 1..frames {
            self.output[i] = input + (self.output[i - 1] * self.b);
        }

        self.last_output = self.output[frames - 1];
    }

    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }
}

impl<const MAX_BLOCKSIZE: usize> SmoothF32<MAX_BLOCKSIZE> {
    pub fn set_speed(&mut self, sample_rate: SampleRate, seconds: Seconds) {
        self.b = (-1.0f32 / (seconds.0 as f32 * sample_rate.0 as f32)).exp();
        self.a = 1.0f32 - self.b;
    }

    pub fn update_status(&mut self) -> SmoothStatus {
        self.update_status_with_epsilon(SETTLE)
    }
}

impl<const MAX_BLOCKSIZE: usize> From<f32> for SmoothF32<MAX_BLOCKSIZE> {
    fn from(val: f32) -> Self {
        Self::new(val)
    }
}

impl<const MAX_BLOCKSIZE: usize> fmt::Debug for SmoothF32<MAX_BLOCKSIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(concat!("SmoothF32"))
            .field("output[0]", &self.output[0])
            .field("input", &self.input)
            .field("status", &self.status)
            .field("last_output", &self.last_output)
            .finish()
    }
}

// ------  F64  -------------------------------------------------------------------------

pub struct SmoothOutputF64<'a, const MAX_BLOCKSIZE: usize> {
    pub values: &'a [f64; MAX_BLOCKSIZE],
    pub status: SmoothStatus,
}

impl<'a, const MAX_BLOCKSIZE: usize> SmoothOutputF64<'a, MAX_BLOCKSIZE> {
    pub fn is_smoothing(&self) -> bool {
        self.status.is_active()
    }
}

impl<'a, I, const MAX_BLOCKSIZE: usize> ops::Index<I> for SmoothOutputF64<'a, MAX_BLOCKSIZE>
where
    I: slice::SliceIndex<[f64]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, idx: I) -> &I::Output {
        &self.values[idx]
    }
}

pub struct SmoothF64<const MAX_BLOCKSIZE: usize> {
    output: [f64; MAX_BLOCKSIZE],
    input: f64,

    status: SmoothStatus,

    a: f64,
    b: f64,
    last_output: f64,
}

impl<const MAX_BLOCKSIZE: usize> SmoothF64<MAX_BLOCKSIZE> {
    pub fn new(input: f64) -> Self {
        Self {
            status: SmoothStatus::Inactive,
            input,
            output: [input; MAX_BLOCKSIZE],

            a: 1.0,
            b: 0.0,
            last_output: input,
        }
    }

    pub fn reset(&mut self, val: f64) {
        *self = Self {
            a: self.a,
            b: self.b,
            ..Self::new(val)
        };
    }

    pub fn set(&mut self, val: f64) {
        self.input = val;
        self.status = SmoothStatus::Active;
    }

    pub fn dest(&self) -> f64 {
        self.input
    }

    pub fn output(&self) -> SmoothOutputF64<MAX_BLOCKSIZE> {
        SmoothOutputF64 {
            values: &self.output,
            status: self.status,
        }
    }

    pub fn current_value(&self) -> (f64, SmoothStatus) {
        (self.last_output, self.status)
    }

    pub fn update_status_with_epsilon(&mut self, epsilon: f64) -> SmoothStatus {
        let status = self.status;

        match status {
            SmoothStatus::Active => {
                if (self.input - self.output[0]).abs() < epsilon {
                    self.reset(self.input);
                    self.status = SmoothStatus::Deactivating;
                }
            }

            SmoothStatus::Deactivating => self.status = SmoothStatus::Inactive,

            _ => (),
        };

        self.status
    }

    pub fn process(&mut self, proc_frames: ProcFrames<MAX_BLOCKSIZE>) {
        if self.status != SmoothStatus::Active {
            return;
        }

        let frames = proc_frames.compiler_hint_frames();
        let input = self.input * self.a;

        self.output[0] = input + (self.last_output * self.b);

        for i in 1..frames {
            self.output[i] = input + (self.output[i - 1] * self.b);
        }

        self.last_output = self.output[frames - 1];
    }

    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }
}

impl<const MAX_BLOCKSIZE: usize> SmoothF64<MAX_BLOCKSIZE> {
    pub fn set_speed(&mut self, sample_rate: SampleRate, seconds: Seconds) {
        self.b = (-1.0f64 / (seconds.0 as f64 * sample_rate.0 as f64)).exp();
        self.a = 1.0f64 - self.b;
    }

    pub fn update_status(&mut self) -> SmoothStatus {
        self.update_status_with_epsilon(SETTLE as f64)
    }
}

impl<const MAX_BLOCKSIZE: usize> From<f64> for SmoothF64<MAX_BLOCKSIZE> {
    fn from(val: f64) -> Self {
        Self::new(val)
    }
}

impl<const MAX_BLOCKSIZE: usize> fmt::Debug for SmoothF64<MAX_BLOCKSIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(concat!("SmoothF64"))
            .field("output[0]", &self.output[0])
            .field("input", &self.input)
            .field("status", &self.status)
            .field("last_output", &self.last_output)
            .finish()
    }
}
