use super::{Frame, SampleRate, Seconds, SuperFrame};

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

    /// The length of this resource in frames.
    pub fn frames(&self) -> Frame {
        match self {
            AnyPCM::Mono(pcm) => pcm.frames(),
            AnyPCM::Stereo(pcm) => pcm.frames(),
        }
    }

    /// The length of this resource in frames.
    ///
    /// This conversion **IS** lossless if the sample rate of this resource happens to be
    /// equal to one of the common sample rates: `22050, 24000, 44100, 48000, 88200,
    /// 96000, 176400, or 192000`. This conversion is *NOT* lossless otherwise.
    pub fn super_frames(&self) -> SuperFrame {
        match self {
            AnyPCM::Mono(pcm) => pcm.super_frames(),
            AnyPCM::Stereo(pcm) => pcm.super_frames(),
        }
    }

    /// The length of this resource in super-frames.
    ///
    /// Note that this conversion is *NOT* lossless.
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
    len_super_frames: SuperFrame,
}

impl MonoPCM {
    pub fn new_empty(sample_rate: SampleRate) -> Self {
        Self {
            data: Vec::new(),
            sample_rate,
            len_secs: Seconds(0.0),
            len_super_frames: SuperFrame::default(),
        }
    }

    pub fn new(data: Vec<f32>, sample_rate: SampleRate) -> Self {
        let len_secs = Frame(data.len() as u64).to_seconds(sample_rate);
        let len_super_frames = Frame(data.len() as u64).to_super_frames(sample_rate);

        Self {
            data,
            sample_rate,
            len_secs,
            len_super_frames,
        }
    }

    pub fn raw(&self) -> &[f32] {
        &self.data
    }

    pub fn raw_mut(&mut self) -> &mut [f32] {
        &mut self.data
    }

    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) {
        if self.sample_rate != sample_rate {
            self.sample_rate = sample_rate;
            self.len_secs = Frame(self.data.len() as u64).to_seconds(sample_rate);
            self.len_super_frames =
                Frame(self.data.len() as u64).to_super_frames(self.sample_rate);
        }
    }

    pub fn resize(&mut self, new_len: Frame, value: f32) {
        if self.data.len() != new_len.0 as usize {
            self.data.resize(new_len.0 as usize, value);
            self.len_secs = Frame(self.data.len() as u64).to_seconds(self.sample_rate);
            self.len_super_frames =
                Frame(self.data.len() as u64).to_super_frames(self.sample_rate);
        }
    }

    pub unsafe fn set_len(&mut self, new_len: Frame) {
        if self.data.len() != new_len.0 as usize {
            self.data.set_len(new_len.0 as usize);
            self.len_secs = Frame(self.data.len() as u64).to_seconds(self.sample_rate);
            self.len_super_frames =
                Frame(self.data.len() as u64).to_super_frames(self.sample_rate);
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    /// The length of this resource in frames.
    pub fn frames(&self) -> Frame {
        self.data.len().into()
    }

    /// The length of this resource in super-frames.
    ///
    /// This conversion **IS** lossless if the sample rate of this resource happens to be
    /// equal to one of the common sample rates: `22050, 24000, 44100, 48000, 88200,
    /// 96000, 176400, or 192000`. This conversion is *NOT* lossless otherwise.
    pub fn super_frames(&self) -> SuperFrame {
        self.len_super_frames
    }

    /// The length of this resource in super-frames.
    ///
    /// Note that this conversion is *NOT* lossless.
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
    len_super_frames: SuperFrame,
}

impl StereoPCM {
    pub fn new_empty(sample_rate: SampleRate) -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
            sample_rate,
            len_secs: Seconds(0.0),
            len_super_frames: SuperFrame::default(),
        }
    }

    pub fn new(left: Vec<f32>, right: Vec<f32>, sample_rate: SampleRate) -> Self {
        assert_eq!(left.len(), right.len());

        let len_secs = Frame(left.len() as u64).to_seconds(sample_rate);
        let len_super_frames = Frame(left.len() as u64).to_super_frames(sample_rate);

        Self {
            left,
            right,
            sample_rate,
            len_secs,
            len_super_frames,
        }
    }

    pub fn raw_left(&self) -> &[f32] {
        &self.left
    }

    pub fn raw_right(&self) -> &[f32] {
        &self.right
    }

    pub fn raw_left_right(&self) -> (&[f32], &[f32]) {
        (&self.left, &self.right)
    }

    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) {
        if self.sample_rate != sample_rate {
            self.sample_rate = sample_rate;
            self.len_secs = Frame(self.left.len() as u64).to_seconds(sample_rate);
            self.len_super_frames =
                Frame(self.left.len() as u64).to_super_frames(self.sample_rate);
        }
    }

    pub fn resize(&mut self, new_len: Frame, value: f32) {
        if self.left.len() != new_len.0 as usize {
            self.left.resize(new_len.0 as usize, value);
            self.right.resize(new_len.0 as usize, value);
            self.len_secs = Frame(self.left.len() as u64).to_seconds(self.sample_rate);
            self.len_super_frames =
                Frame(self.left.len() as u64).to_super_frames(self.sample_rate);
        }
    }

    pub unsafe fn set_len(&mut self, new_len: Frame) {
        if self.left.len() != new_len.0 as usize {
            self.left.set_len(new_len.0 as usize);
            self.right.set_len(new_len.0 as usize);
            self.len_secs = Frame(self.left.len() as u64).to_seconds(self.sample_rate);
            self.len_super_frames =
                Frame(self.left.len() as u64).to_super_frames(self.sample_rate);
        }
    }

    pub fn clear(&mut self) {
        self.left.clear();
        self.right.clear();
    }

    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    /// The length of this resource in frames.
    pub fn frames(&self) -> Frame {
        self.left.len().into()
    }

    /// The length of this resource in super-frames.
    ///
    /// This conversion **IS** lossless if the sample rate of this resource happens to be
    /// equal to one of the common sample rates: `22050, 24000, 44100, 48000, 88200,
    /// 96000, 176400, or 192000`. This conversion is *NOT* lossless otherwise.
    pub fn super_frames(&self) -> SuperFrame {
        self.len_super_frames
    }

    /// The length of this resource in super-frames.
    ///
    /// Note that this conversion is *NOT* lossless.
    pub fn len_seconds(&self) -> Seconds {
        self.len_secs
    }
}
