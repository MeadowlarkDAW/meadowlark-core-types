use super::{SampleRate, SampleTime, Seconds};

pub static U24_TO_F32_RATIO: f32 = 2.0 / 0x00FFFFFF as f32;
pub static I16_TO_F32_RATIO: f32 = 1.0 / std::i16::MAX as f32;
pub static U8_TO_F32_RATIO: f32 = 2.0 / std::u8::MAX as f32;

// TODO: Allow storing PCM in non-f32 format

#[non_exhaustive]
#[derive(Debug)]
pub enum AnyPCM {
    Mono(MonoPCM),
    Stereo(StereoPCM),
}

impl AnyPCM {
    pub fn sample_rate(&self) -> SampleRate {
        match self {
            AnyPCM::Mono(pcm) => pcm.sample_rate(),
            AnyPCM::Stereo(pcm) => pcm.sample_rate(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            AnyPCM::Mono(pcm) => pcm.len(),
            AnyPCM::Stereo(pcm) => pcm.len(),
        }
    }

    pub fn len_seconds(&self) -> Seconds {
        match self {
            AnyPCM::Mono(pcm) => pcm.len_seconds(),
            AnyPCM::Stereo(pcm) => pcm.len_seconds(),
        }
    }
}

#[derive(Debug)]
pub struct MonoPCM {
    data: Vec<f32>,
    sample_rate: SampleRate,
    len_secs: Seconds,
}

impl MonoPCM {
    pub fn new(data: Vec<f32>, sample_rate: SampleRate) -> Self {
        let len_secs = SampleTime(data.len() as i64).to_seconds(sample_rate);

        Self {
            data,
            sample_rate,
            len_secs,
        }
    }

    #[inline]
    pub fn raw(&self) -> &[f32] {
        &self.data
    }

    #[inline]
    pub fn raw_mut(&mut self) -> &mut [f32] {
        &mut self.data
    }

    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) {
        if self.sample_rate != sample_rate {
            self.sample_rate = sample_rate;
            self.len_secs = SampleTime(self.data.len() as i64).to_seconds(sample_rate);
        }
    }

    pub fn resize(&mut self, new_len: usize, value: f32) {
        if self.data.len() != new_len {
            self.data.resize(new_len, value);
            self.len_secs = SampleTime(self.data.len() as i64).to_seconds(self.sample_rate);
        }
    }

    pub unsafe fn set_len(&mut self, new_len: usize) {
        if self.data.len() != new_len {
            self.data.set_len(new_len);
            self.len_secs = SampleTime(self.data.len() as i64).to_seconds(self.sample_rate);
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    #[inline]
    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn len_seconds(&self) -> Seconds {
        self.len_secs
    }
}

#[derive(Debug)]
pub struct StereoPCM {
    left: Vec<f32>,
    right: Vec<f32>,

    sample_rate: SampleRate,
    len_secs: Seconds,
}

impl StereoPCM {
    pub fn new(left: Vec<f32>, right: Vec<f32>, sample_rate: SampleRate) -> Self {
        assert_eq!(left.len(), right.len());

        let len_secs = SampleTime(left.len() as i64).to_seconds(sample_rate);

        Self {
            left,
            right,
            sample_rate,
            len_secs,
        }
    }

    #[inline]
    pub fn raw_left(&self) -> &[f32] {
        &self.left
    }

    #[inline]
    pub fn raw_right(&self) -> &[f32] {
        &self.right
    }

    #[inline]
    pub fn raw_left_right(&self) -> (&[f32], &[f32]) {
        (&self.left, &self.right)
    }

    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) {
        if self.sample_rate != sample_rate {
            self.sample_rate = sample_rate;
            self.len_secs = SampleTime(self.left.len() as i64).to_seconds(sample_rate);
        }
    }

    pub fn resize(&mut self, new_len: usize, value: f32) {
        if self.left.len() != new_len {
            self.left.resize(new_len, value);
            self.right.resize(new_len, value);
            self.len_secs = SampleTime(self.left.len() as i64).to_seconds(self.sample_rate);
        }
    }

    pub unsafe fn set_len(&mut self, new_len: usize) {
        if self.left.len() != new_len {
            self.left.set_len(new_len);
            self.right.set_len(new_len);
            self.len_secs = SampleTime(self.left.len() as i64).to_seconds(self.sample_rate);
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.left.clear();
        self.right.clear();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.left.len()
    }

    #[inline]
    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    #[inline]
    pub fn len_seconds(&self) -> Seconds {
        self.len_secs
    }
}