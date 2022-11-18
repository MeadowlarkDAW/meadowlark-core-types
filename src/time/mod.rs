//! Structs for accurate timekeeping in musical audio applications.

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

mod musical_time;
mod sample_rate;
mod sample_time;
mod seconds;
mod super_sample_time;
//mod video_timecode;

pub use musical_time::{MusicalTime, SUPER_BEAT_TICKS_PER_BEAT};
pub use sample_rate::SampleRate;
pub use sample_time::SampleTime;
pub use seconds::Seconds;
pub use super_sample_time::{SuperSampleTime, SUPER_SAMPLE_TICKS_PER_SECOND};
//pub use video_timecode::{VideoFpsFormat, VideoTimecode};

/// A reliable timestamp for events on the timeline.
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Timestamp {
    Musical(MusicalTime),
    Sample(SuperSampleTime),
    // TODO: Flesh this out once I have a better idea how this should work.
    // Video(VideoTimecode),
}
