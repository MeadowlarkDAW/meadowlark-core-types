use std::sync::atomic::{AtomicU64, Ordering};

use crate::{MusicalTime, SuperFrames};

/// Simple atomic `MusicalTime` variable with relaxed ordering.
pub struct AtomicMusicalTime {
    atomic: AtomicU64,
}

#[inline]
fn u32x2_to_u64(v1: u32, v2: u32) -> u64 {
    let v1_bytes: [u8; 4] = v1.to_ne_bytes();
    let v2_bytes: [u8; 4] = v2.to_ne_bytes();
    let bytes: [u8; 8] = [
        v1_bytes[0],
        v1_bytes[1],
        v1_bytes[2],
        v1_bytes[3],
        v2_bytes[0],
        v2_bytes[1],
        v2_bytes[2],
        v2_bytes[3],
    ];

    u64::from_ne_bytes(bytes)
}

#[inline]
fn u64_to_u32x2(v: u64) -> (u32, u32) {
    let bytes: [u8; 8] = v.to_ne_bytes();
    let v1_bytes: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
    let v2_bytes: [u8; 4] = [bytes[4], bytes[5], bytes[6], bytes[7]];

    (u32::from_ne_bytes(v1_bytes), u32::from_ne_bytes(v2_bytes))
}

impl AtomicMusicalTime {
    /// New atomic musical time with initial value `value`.
    pub fn new(musical_time: MusicalTime) -> AtomicMusicalTime {
        AtomicMusicalTime {
            atomic: AtomicU64::new(u32x2_to_u64(
                musical_time.beats(),
                musical_time.super_beats(),
            )),
        }
    }

    /// Get the current value of the atomic musical time.
    pub fn get(&self) -> MusicalTime {
        let (beats, super_beats) = u64_to_u32x2(self.atomic.load(Ordering::Relaxed));
        MusicalTime::new(beats, super_beats)
    }

    /// Set the value of the atomic musical time to `musical_time`.
    pub fn set(&self, musical_time: MusicalTime) {
        self.atomic.store(
            u32x2_to_u64(musical_time.beats(), musical_time.super_beats()),
            Ordering::Relaxed,
        )
    }
}

impl Default for AtomicMusicalTime {
    fn default() -> Self {
        AtomicMusicalTime::new(MusicalTime::default())
    }
}

impl std::fmt::Debug for AtomicMusicalTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.get(), f)
    }
}

impl From<MusicalTime> for AtomicMusicalTime {
    fn from(value: MusicalTime) -> Self {
        AtomicMusicalTime::new(value)
    }
}

impl From<AtomicMusicalTime> for MusicalTime {
    fn from(value: AtomicMusicalTime) -> Self {
        value.get()
    }
}

/// Simple atomic `SuperFrames` variable with relaxed ordering.
pub struct AtomicSuperFrames {
    atomic: AtomicU64,
}

impl AtomicSuperFrames {
    /// New atomic musical time with initial value `value`.
    pub fn new(super_frames: SuperFrames) -> AtomicSuperFrames {
        AtomicSuperFrames {
            atomic: AtomicU64::new(u32x2_to_u64(
                super_frames.seconds_u32(),
                super_frames.super_frames(),
            )),
        }
    }

    /// Get the current value of the atomic super-frames.
    pub fn get(&self) -> SuperFrames {
        let (seconds, super_frames) = u64_to_u32x2(self.atomic.load(Ordering::Relaxed));
        SuperFrames::new(seconds, super_frames)
    }

    /// Set the value of the atomic super-frames to `super_frames`.
    pub fn set(&self, super_frames: SuperFrames) {
        self.atomic.store(
            u32x2_to_u64(super_frames.seconds_u32(), super_frames.super_frames()),
            Ordering::Relaxed,
        )
    }
}

impl Default for AtomicSuperFrames {
    fn default() -> Self {
        AtomicSuperFrames::new(SuperFrames::default())
    }
}

impl std::fmt::Debug for AtomicSuperFrames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.get(), f)
    }
}

impl From<SuperFrames> for AtomicSuperFrames {
    fn from(value: SuperFrames) -> Self {
        AtomicSuperFrames::new(value)
    }
}

impl From<AtomicSuperFrames> for SuperFrames {
    fn from(value: AtomicSuperFrames) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_musical_time() {
        let musical_time_1 = MusicalTime::new(4578749, 12390);
        let musical_time_2 = MusicalTime::new(5720495, 45781);

        let atomic_musical_time = AtomicMusicalTime::new(musical_time_1);

        assert_eq!(atomic_musical_time.get(), musical_time_1);

        atomic_musical_time.set(musical_time_2);

        std::thread::sleep(std::time::Duration::from_millis(1));

        assert_eq!(atomic_musical_time.get(), musical_time_2);
    }

    #[test]
    fn test_atomic_super_frames() {
        let super_frames_1 = SuperFrames::new(4578749, 12390);
        let super_frames_2 = SuperFrames::new(5720495, 45781);

        let atomic_super_frames = AtomicSuperFrames::new(super_frames_1);

        assert_eq!(atomic_super_frames.get(), super_frames_1);

        atomic_super_frames.set(super_frames_2);

        std::thread::sleep(std::time::Duration::from_millis(1));

        assert_eq!(atomic_super_frames.get(), super_frames_2);
    }
}
