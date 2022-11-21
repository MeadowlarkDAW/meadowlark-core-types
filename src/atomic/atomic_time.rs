use std::sync::atomic::{AtomicU64, Ordering};

use crate::time::{MusicalTime, SuperclockTime};

/// Simple atomic `MusicalTime` variable.
pub struct AtomicMusicalTime {
    atomic: AtomicU64,
}
/// Simple atomic `SuperclockTime` variable.
pub struct AtomicSuperclockTime {
    atomic: AtomicU64,
}

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
            atomic: AtomicU64::new(u32x2_to_u64(musical_time.beats(), musical_time.ticks())),
        }
    }

    /// Get the current value of the atomic musical time.
    pub fn get(&self, order: Ordering) -> MusicalTime {
        let (beats, super_beats) = u64_to_u32x2(self.atomic.load(order));
        MusicalTime::new(beats, super_beats)
    }

    /// Set the value of the atomic musical time to `musical_time`.
    pub fn set(&self, musical_time: MusicalTime, order: Ordering) {
        self.atomic.store(
            u32x2_to_u64(musical_time.beats(), musical_time.ticks()),
            order,
        )
    }

    /// Set the value of the atomic musical time to `musical_time`, while also
    /// returning the previous value that was stored.
    pub fn swap(&self, musical_time: MusicalTime, order: Ordering) -> MusicalTime {
        let val = self.atomic.swap(
            u32x2_to_u64(musical_time.beats(), musical_time.ticks()),
            order,
        );

        let (beats, super_beats) = u64_to_u32x2(val);
        MusicalTime::new(beats, super_beats)
    }
}

impl Default for AtomicMusicalTime {
    fn default() -> Self {
        AtomicMusicalTime::new(MusicalTime::default())
    }
}

impl AtomicSuperclockTime {
    /// New atomic musical time with initial value `value`.
    pub fn new(superclock_time: SuperclockTime) -> AtomicSuperclockTime {
        AtomicSuperclockTime {
            atomic: AtomicU64::new(u32x2_to_u64(
                superclock_time.seconds(),
                superclock_time.ticks(),
            )),
        }
    }

    /// Get the current value of the atomic musical time.
    pub fn get(&self, order: Ordering) -> SuperclockTime {
        let (seconds, super_beats) = u64_to_u32x2(self.atomic.load(order));
        SuperclockTime::new(seconds, super_beats)
    }

    /// Set the value of the atomic musical time to `musical_time`.
    pub fn set(&self, superclock_time: SuperclockTime, order: Ordering) {
        self.atomic.store(
            u32x2_to_u64(superclock_time.seconds(), superclock_time.ticks()),
            order,
        )
    }

    /// Set the value of the atomic musical time to `musical_time`, while also
    /// returning the previous value that was stored.
    pub fn swap(&self, superclock_time: SuperclockTime, order: Ordering) -> SuperclockTime {
        let val = self.atomic.swap(
            u32x2_to_u64(superclock_time.seconds(), superclock_time.ticks()),
            order,
        );

        let (seconds, super_beats) = u64_to_u32x2(val);
        SuperclockTime::new(seconds, super_beats)
    }
}

impl Default for AtomicSuperclockTime {
    fn default() -> Self {
        AtomicSuperclockTime::new(SuperclockTime::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_time() {
        let musical_time_1 = MusicalTime::new(4578749, 12390);
        let musical_time_2 = MusicalTime::new(5720495, 45781);

        let atomic_musical_time = AtomicMusicalTime::new(musical_time_1);
        assert_eq!(atomic_musical_time.get(Ordering::SeqCst), musical_time_1);

        atomic_musical_time.set(musical_time_2, Ordering::SeqCst);
        assert_eq!(atomic_musical_time.get(Ordering::SeqCst), musical_time_2);

        let old_val = atomic_musical_time.swap(musical_time_1, Ordering::SeqCst);
        assert_eq!(old_val, musical_time_2);
        assert_eq!(atomic_musical_time.get(Ordering::SeqCst), musical_time_1);

        let superclock_time_1 = SuperclockTime::new(4578749, 12390);
        let superclock_time_2 = SuperclockTime::new(5720495, 45781);

        let atomic_superclock_time = AtomicSuperclockTime::new(superclock_time_1);
        assert_eq!(
            atomic_superclock_time.get(Ordering::SeqCst),
            superclock_time_1
        );

        atomic_superclock_time.set(superclock_time_2, Ordering::SeqCst);
        assert_eq!(
            atomic_superclock_time.get(Ordering::SeqCst),
            superclock_time_2
        );

        let old_val = atomic_superclock_time.swap(superclock_time_1, Ordering::SeqCst);
        assert_eq!(old_val, superclock_time_2);
        assert_eq!(
            atomic_superclock_time.get(Ordering::SeqCst),
            superclock_time_1
        );
    }
}
