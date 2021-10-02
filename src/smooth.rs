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

use num_traits::Float;

use super::{SampleRate, Seconds};

const SETTLE: f32 = 0.0001f32;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SmoothStatus {
    Inactive,
    Active,
    Deactivating,
}

impl SmoothStatus {
    #[inline]
    fn is_active(&self) -> bool {
        self != &SmoothStatus::Inactive
    }
}

pub struct SmoothOutput<'a, T, const MAX_BLOCKSIZE: usize> {
    pub values: &'a [T; MAX_BLOCKSIZE],
    pub status: SmoothStatus,
}

impl<'a, T, const MAX_BLOCKSIZE: usize> SmoothOutput<'a, T, MAX_BLOCKSIZE> {
    #[inline]
    pub fn is_smoothing(&self) -> bool {
        self.status.is_active()
    }
}

impl<'a, I, T, const MAX_BLOCKSIZE: usize> ops::Index<I> for SmoothOutput<'a, T, MAX_BLOCKSIZE>
where
    I: slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, idx: I) -> &I::Output {
        &self.values[idx]
    }
}

pub struct Smooth<T: Float, const MAX_BLOCKSIZE: usize> {
    output: [T; MAX_BLOCKSIZE],
    input: T,

    status: SmoothStatus,

    a: T,
    b: T,
    last_output: T,
}

impl<T, const MAX_BLOCKSIZE: usize> Smooth<T, MAX_BLOCKSIZE>
where
    T: Float + fmt::Display,
{
    pub fn new(input: T) -> Self {
        Self {
            status: SmoothStatus::Inactive,
            input,
            output: [input; MAX_BLOCKSIZE],

            a: T::one(),
            b: T::zero(),
            last_output: input,
        }
    }

    pub fn reset(&mut self, val: T) {
        *self = Self {
            a: self.a,
            b: self.b,
            ..Self::new(val)
        };
    }

    pub fn set(&mut self, val: T) {
        self.input = val;
        self.status = SmoothStatus::Active;
    }

    #[inline]
    pub fn dest(&self) -> T {
        self.input
    }

    #[inline]
    pub fn output(&self) -> SmoothOutput<T, MAX_BLOCKSIZE> {
        SmoothOutput {
            values: &self.output,
            status: self.status,
        }
    }

    #[inline]
    pub fn current_value(&self) -> (T, SmoothStatus) {
        (self.last_output, self.status)
    }

    pub fn update_status_with_epsilon(&mut self, epsilon: T) -> SmoothStatus {
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

    pub fn process(&mut self, nframes: usize) {
        if self.status != SmoothStatus::Active {
            return;
        }

        let nframes = nframes.min(MAX_BLOCKSIZE);
        let input = self.input * self.a;

        self.output[0] = input + (self.last_output * self.b);

        for i in 1..nframes {
            self.output[i] = input + (self.output[i - 1] * self.b);
        }

        self.last_output = self.output[nframes - 1];
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }
}

impl<const MAX_BLOCKSIZE: usize> Smooth<f32, MAX_BLOCKSIZE> {
    pub fn set_speed(&mut self, sample_rate: SampleRate, seconds: Seconds) {
        self.b = (-1.0f32 / (seconds.0 as f32 * sample_rate.0 as f32)).exp();
        self.a = 1.0f32 - self.b;
    }

    #[inline]
    pub fn update_status(&mut self) -> SmoothStatus {
        self.update_status_with_epsilon(SETTLE)
    }
}

impl<T, const MAX_BLOCKSIZE: usize> From<T> for Smooth<T, MAX_BLOCKSIZE>
where
    T: Float + fmt::Display,
{
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

impl<T, const MAX_BLOCKSIZE: usize> fmt::Debug for Smooth<T, MAX_BLOCKSIZE>
where
    T: Float + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(concat!("Smooth<", stringify!(T), ">"))
            .field("output[0]", &self.output[0])
            .field("input", &self.input)
            .field("status", &self.status)
            .field("last_output", &self.last_output)
            .finish()
    }
}
