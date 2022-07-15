use crate::time::{Frames, SampleRate};

pub mod convert;

pub struct PcmResource {
    pub pcm_type: PcmResourceType,
    pub sample_rate: SampleRate,
    pub channels: usize,
    pub len_frames: Frames,
}

/// The format of the raw PCM samples
///
/// Note that there is no option for U32/I32. This is because we want to use
/// float for everything anyway. We only store the other types to save memory.
pub enum PcmResourceType {
    U8(Vec<Vec<u8>>),
    U16(Vec<Vec<u16>>),
    /// The endianness of the samples must be the native endianness of the
    /// target platform.
    U24(Vec<Vec<[u8; 3]>>),
    S8(Vec<Vec<i8>>),
    S16(Vec<Vec<i16>>),
    /// The endianness of the samples must be the native endianness of the
    /// target platform.
    S24(Vec<Vec<[u8; 3]>>),
    F32(Vec<Vec<f32>>),
    F64(Vec<Vec<f64>>),
}

impl PcmResource {
    /// Fill the buffer with samples from the given `channel`, starting from the
    /// given `frame`. Portions that are out-of-bounds will be filled with zeros.
    /// 
    /// The will return an error if the given channel does not exist.
    pub fn fill_channel_f32(
        &self,
        channel: usize,
        frame: isize,
        buf: &mut [f32],
    ) -> Result<(), ()> {
        if channel >= self.channels {
            buf.fill(0.0);
            return Err(());
        }

        let len_frames = self.len_frames.0 as usize;
        let buf_len = buf.len();

        let (buf_start, pcm_start, len) =
            if frame >= len_frames as isize || frame + buf_len as isize <= 0 {
                // out of range, fill buffer with zeros
                buf.fill(0.0);
                return Ok(());
            } else if frame < 0 {
                let skip_frames = (0 - frame) as usize;

                // clear the out-of-range part
                buf[0..skip_frames].fill(0.0);

                let new_buf_len = buf_len - skip_frames;

                if new_buf_len <= len_frames {
                    (skip_frames, 0, new_buf_len)
                } else {
                    let copy_frames = len_frames - new_buf_len;

                    // clear the out-of-range part
                    buf[skip_frames + copy_frames..buf_len].fill(0.0);

                    (skip_frames, 0, copy_frames)
                }
            } else if frame as usize + buf_len <= len_frames {
                (0, frame as usize, buf_len)
            } else {
                let copy_frames = len_frames - frame as usize;

                // clear the out-of-range part
                buf[copy_frames..buf_len].fill(0.0);

                (0, frame as usize, copy_frames)
            };
        
        debug_assert!(buf_start + len <= buf_len);

        match &self.pcm_type {
            PcmResourceType::U8(pcm) => {
                debug_assert!(pcm_start + len <= pcm[channel].len());

                let buf_part = &mut buf[buf_start..buf_start + len];
                let pcm_part = &pcm[channel][pcm_start..pcm_start + len];

                for (b, s) in buf_part.iter_mut().zip(pcm_part.iter()) {
                    *b = convert::pcm_u8_to_f32(*s);
                }
            }
            PcmResourceType::U16(pcm) => {
                debug_assert!(pcm_start + len <= pcm[channel].len());

                let buf_part = &mut buf[buf_start..buf_start + len];
                let pcm_part = &pcm[channel][pcm_start..pcm_start + len];

                for (b, p) in buf_part.iter_mut().zip(pcm_part.iter()) {
                    *b = convert::pcm_u16_to_f32(*p);
                }
            }
            PcmResourceType::U24(pcm) => {
                debug_assert!(pcm_start + len <= pcm[channel].len());

                let buf_part = &mut buf[buf_start..buf_start + len];
                let pcm_part = &pcm[channel][pcm_start..pcm_start + len];

                for (b, p) in buf_part.iter_mut().zip(pcm_part.iter()) {
                    *b = convert::pcm_u24_to_f32_ne(*p);
                }
            }
            PcmResourceType::S8(pcm) => {
                debug_assert!(pcm_start + len <= pcm[channel].len());

                let buf_part = &mut buf[buf_start..buf_start + len];
                let pcm_part = &pcm[channel][pcm_start..pcm_start + len];

                for (b, p) in buf_part.iter_mut().zip(pcm_part.iter()) {
                    *b = convert::pcm_s8_to_f32(*p);
                }
            }
            PcmResourceType::S16(pcm) => {
                debug_assert!(pcm_start + len <= pcm[channel].len());

                let buf_part = &mut buf[buf_start..buf_start + len];
                let pcm_part = &pcm[channel][pcm_start..pcm_start + len];

                for (b, p) in buf_part.iter_mut().zip(pcm_part.iter()) {
                    *b = convert::pcm_s16_to_f32(*p);
                }
            }
            PcmResourceType::S24(pcm) => {
                debug_assert!(pcm_start + len <= pcm[channel].len());

                let buf_part = &mut buf[buf_start..buf_start + len];
                let pcm_part = &pcm[channel][pcm_start..pcm_start + len];

                for (b, p) in buf_part.iter_mut().zip(pcm_part.iter()) {
                    *b = convert::pcm_s24_to_f32_ne(*p);
                }
            }
            PcmResourceType::F32(pcm) => {
                debug_assert!(pcm_start + len <= pcm[channel].len());

                let buf_part = &mut buf[buf_start..buf_start + len];
                let pcm_part = &pcm[channel][pcm_start..pcm_start + len];

                buf_part.copy_from_slice(pcm_part);
            }
            PcmResourceType::F64(pcm) => {
                debug_assert!(pcm_start + len <= pcm[channel].len());

                let buf_part = &mut buf[buf_start..buf_start + len];
                let pcm_part = &pcm[channel][pcm_start..pcm_start + len];

                for (b, p) in buf_part.iter_mut().zip(pcm_part.iter()) {
                    *b = *p as f32;
                }
            }
        }

        Ok(())
    }

    /// Fill the stereo buffer with samples, starting from the given `frame`.
    /// Portions that are out-of-bounds will be filled with zeros.
    /// 
    /// If this resource has only one channel, then both channels will be
    /// filled with the same data.
    pub fn fill_stereo_f32(&self, frame: isize, buf_l: &mut [f32], buf_r: &mut [f32]) {
        debug_assert_eq!(buf_l.len(), buf_r.len());

        if self.channels == 1 {
            self.fill_channel_f32(0, frame, buf_l).unwrap();
            buf_r.copy_from_slice(buf_l);
            return;
        }

        let len_frames = self.len_frames.0 as usize;
        let buf_len = buf_l.len();

        let (buf_start, pcm_start, len) =
            if frame >= len_frames as isize || frame + buf_len as isize <= 0 {
                // out of range, fill buffer with zeros
                buf_l.fill(0.0);
                buf_r.fill(0.0);
                return;
            } else if frame < 0 {
                let skip_frames = (0 - frame) as usize;

                // clear the out-of-range part
                buf_l[0..skip_frames].fill(0.0);
                buf_r[0..skip_frames].fill(0.0);

                let new_buf_len = buf_len - skip_frames;

                if new_buf_len <= len_frames {
                    (skip_frames, 0, new_buf_len)
                } else {
                    let copy_frames = len_frames - new_buf_len;

                    // clear the out-of-range part
                    buf_l[skip_frames + copy_frames..buf_len].fill(0.0);
                    buf_r[skip_frames + copy_frames..buf_len].fill(0.0);

                    (skip_frames, 0, copy_frames)
                }
            } else if frame as usize + buf_len <= len_frames {
                (0, frame as usize, buf_len)
            } else {
                let copy_frames = len_frames - frame as usize;

                // clear the out-of-range part
                buf_l[copy_frames..buf_len].fill(0.0);
                buf_r[copy_frames..buf_len].fill(0.0);

                (0, frame as usize, copy_frames)
            };
        
        debug_assert!(buf_start + len <= buf_len);

        match &self.pcm_type {
            PcmResourceType::U8(pcm) => {
                debug_assert!(pcm_start + len <= pcm[0].len());
                debug_assert!(pcm_start + len <= pcm[1].len());

                let buf_l_part = &mut buf_l[buf_start..buf_start + len];
                let buf_r_part = &mut buf_r[buf_start..buf_start + len];
                let pcm_l_part = &pcm[0][pcm_start..pcm_start + len];
                let pcm_r_part = &pcm[1][pcm_start..pcm_start + len];

                for i in 0..buf_l_part.len() {
                    buf_l_part[i] = convert::pcm_u8_to_f32(pcm_l_part[i]);
                    buf_r_part[i] = convert::pcm_u8_to_f32(pcm_r_part[i]);
                }
            }
            PcmResourceType::U16(pcm) => {
                debug_assert!(pcm_start + len <= pcm[0].len());
                debug_assert!(pcm_start + len <= pcm[1].len());

                let buf_l_part = &mut buf_l[buf_start..buf_start + len];
                let buf_r_part = &mut buf_r[buf_start..buf_start + len];
                let pcm_l_part = &pcm[0][pcm_start..pcm_start + len];
                let pcm_r_part = &pcm[1][pcm_start..pcm_start + len];

                for i in 0..buf_l_part.len() {
                    buf_l_part[i] = convert::pcm_u16_to_f32(pcm_l_part[i]);
                    buf_r_part[i] = convert::pcm_u16_to_f32(pcm_r_part[i]);
                }
            }
            PcmResourceType::U24(pcm) => {
                debug_assert!(pcm_start + len <= pcm[0].len());
                debug_assert!(pcm_start + len <= pcm[1].len());

                let buf_l_part = &mut buf_l[buf_start..buf_start + len];
                let buf_r_part = &mut buf_r[buf_start..buf_start + len];
                let pcm_l_part = &pcm[0][pcm_start..pcm_start + len];
                let pcm_r_part = &pcm[1][pcm_start..pcm_start + len];

                for i in 0..buf_l_part.len() {
                    buf_l_part[i] = convert::pcm_u24_to_f32_ne(pcm_l_part[i]);
                    buf_r_part[i] = convert::pcm_u24_to_f32_ne(pcm_r_part[i]);
                }
            }
            PcmResourceType::S8(pcm) => {
                debug_assert!(pcm_start + len <= pcm[0].len());
                debug_assert!(pcm_start + len <= pcm[1].len());

                let buf_l_part = &mut buf_l[buf_start..buf_start + len];
                let buf_r_part = &mut buf_r[buf_start..buf_start + len];
                let pcm_l_part = &pcm[0][pcm_start..pcm_start + len];
                let pcm_r_part = &pcm[1][pcm_start..pcm_start + len];

                for i in 0..buf_l_part.len() {
                    buf_l_part[i] = convert::pcm_s8_to_f32(pcm_l_part[i]);
                    buf_r_part[i] = convert::pcm_s8_to_f32(pcm_r_part[i]);
                }
            }
            PcmResourceType::S16(pcm) => {
                debug_assert!(pcm_start + len <= pcm[0].len());
                debug_assert!(pcm_start + len <= pcm[1].len());

                let buf_l_part = &mut buf_l[buf_start..buf_start + len];
                let buf_r_part = &mut buf_r[buf_start..buf_start + len];
                let pcm_l_part = &pcm[0][pcm_start..pcm_start + len];
                let pcm_r_part = &pcm[1][pcm_start..pcm_start + len];

                for i in 0..buf_l_part.len() {
                    buf_l_part[i] = convert::pcm_s16_to_f32(pcm_l_part[i]);
                    buf_r_part[i] = convert::pcm_s16_to_f32(pcm_r_part[i]);
                }
            }
            PcmResourceType::S24(pcm) => {
                debug_assert!(pcm_start + len <= pcm[0].len());
                debug_assert!(pcm_start + len <= pcm[1].len());

                let buf_l_part = &mut buf_l[buf_start..buf_start + len];
                let buf_r_part = &mut buf_r[buf_start..buf_start + len];
                let pcm_l_part = &pcm[0][pcm_start..pcm_start + len];
                let pcm_r_part = &pcm[1][pcm_start..pcm_start + len];

                for i in 0..buf_l_part.len() {
                    buf_l_part[i] = convert::pcm_s24_to_f32_ne(pcm_l_part[i]);
                    buf_r_part[i] = convert::pcm_s24_to_f32_ne(pcm_r_part[i]);
                }
            }
            PcmResourceType::F32(pcm) => {
                debug_assert!(pcm_start + len <= pcm[0].len());
                debug_assert!(pcm_start + len <= pcm[1].len());

                let buf_l_part = &mut buf_l[buf_start..buf_start + len];
                let buf_r_part = &mut buf_r[buf_start..buf_start + len];
                let pcm_l_part = &pcm[0][pcm_start..pcm_start + len];
                let pcm_r_part = &pcm[1][pcm_start..pcm_start + len];

                buf_l_part.copy_from_slice(pcm_l_part);
                buf_r_part.copy_from_slice(pcm_r_part);
            }
            PcmResourceType::F64(pcm) => {
                debug_assert!(pcm_start + len <= pcm[0].len());
                debug_assert!(pcm_start + len <= pcm[1].len());

                let buf_l_part = &mut buf_l[buf_start..buf_start + len];
                let buf_r_part = &mut buf_r[buf_start..buf_start + len];
                let pcm_l_part = &pcm[0][pcm_start..pcm_start + len];
                let pcm_r_part = &pcm[1][pcm_start..pcm_start + len];

                for i in 0..buf_l_part.len() {
                    buf_l_part[i] = pcm_l_part[i] as f32;
                    buf_r_part[i] = pcm_r_part[i] as f32;
                }
            }
        }
    }
}
