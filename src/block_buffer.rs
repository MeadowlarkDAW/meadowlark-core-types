use std::mem::MaybeUninit;
use std::ops::Range;

use super::RealFrames;

/// An audio buffer with a single channel.
///
/// This has a constant number of frames (`MAX_BLOCKSIZE`), so this can be allocated on
/// the stack.
#[derive(Debug)]
pub struct MonoBlockBuffer<T: Default + Copy + Clone, const MAX_BLOCKSIZE: usize> {
    pub buf: [T; MAX_BLOCKSIZE],
}

impl<T: Default + Copy + Clone, const MAX_BLOCKSIZE: usize> MonoBlockBuffer<T, MAX_BLOCKSIZE> {
    /// Create a new buffer.
    ///
    /// This is a constant size (`MAX_BLOCKSIZE`), so this can be allocated on the stack.
    ///
    /// All samples will be cleared to 0.
    pub fn new() -> Self {
        Self {
            buf: [T::default(); MAX_BLOCKSIZE],
        }
    }

    /// Create a new buffer without initializing.
    ///
    /// This is a constant size (`MAX_BLOCKSIZE`), so this can be allocated on the stack.
    ///
    /// ## Undefined behavior
    /// This data will be unitialized, so undefined behavior may occur if you try to read
    /// any data without writing to it first.
    pub unsafe fn new_uninit() -> Self {
        Self {
            buf: MaybeUninit::uninit().assume_init(),
        }
    }

    /// Create a new buffer that only initializes the given number of frames to 0. Any samples
    /// after `frames` will be uninitialized.
    ///
    /// This is a constant size (`MAX_BLOCKSIZE`), so this can be allocated on the stack.
    ///
    /// ## Undefined behavior
    /// The portion of data not in the given range will be unitialized, so undefined behavior
    /// may occur if you try to read any of that data without writing to it first.
    pub unsafe fn new_uninit_after_frames(frames: RealFrames) -> Self {
        let frames = frames.0.min(MAX_BLOCKSIZE);
        let mut buf: [T; MAX_BLOCKSIZE] = MaybeUninit::uninit().assume_init();

        let buf_part = &mut buf[0..frames];
        buf_part.fill(T::default());

        Self { buf }
    }

    /// Create a new buffer that only initializes the given range of data to 0.
    ///
    /// This is a constant size (`MAX_BLOCKSIZE`), so this can be allocated on the stack.
    ///
    /// ## Undefined behavior
    /// The portion of data not in the given range will be unitialized, so undefined behavior
    /// may occur if you try to read any of that data without writing to it first.
    ///
    /// ## Panics
    /// This will panic if the given range lies outside the valid range `[0, N)`.
    pub unsafe fn new_partially_uninit(init_range: Range<usize>) -> Self {
        let mut buf: [T; MAX_BLOCKSIZE] = MaybeUninit::uninit().assume_init();

        let buf_part = &mut buf[init_range];
        buf_part.fill(T::default());

        Self { buf }
    }

    /// Clear all samples in the buffer to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.buf.fill(T::default());
    }

    /// Clear a number of frames in the buffer to 0.
    #[inline]
    pub fn clear_frames(&mut self, frames: RealFrames) {
        let frames = frames.0.min(MAX_BLOCKSIZE);
        let buf_part = &mut self.buf[0..frames];
        buf_part.fill(T::default());
    }

    /// Clear a range in the buffer to 0.
    ///
    /// ## Panics
    /// This will panic if the given range lies outside the valid range `[0, N)`.
    #[inline]
    pub fn clear_range(&mut self, range: Range<usize>) {
        let buf_part = &mut self.buf[range];
        buf_part.fill(T::default());
    }

    /// Copy all frames from `src` to this buffer.
    #[inline]
    pub fn copy_from(&mut self, src: &MonoBlockBuffer<T, MAX_BLOCKSIZE>) {
        self.buf.copy_from_slice(&src.buf);
    }

    /// Copy the given number of `frames` from `src` to this buffer.
    #[inline]
    pub fn copy_frames_from(
        &mut self,
        src: &MonoBlockBuffer<T, MAX_BLOCKSIZE>,
        frames: RealFrames,
    ) {
        let frames = frames.0.min(MAX_BLOCKSIZE);
        self.buf[0..frames].copy_from_slice(&src.buf[0..frames]);
    }
}

impl<T, I, const MAX_BLOCKSIZE: usize> std::ops::Index<I> for MonoBlockBuffer<T, MAX_BLOCKSIZE>
where
    I: std::slice::SliceIndex<[T]>,
    T: Default + Copy + Clone,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, idx: I) -> &I::Output {
        &self.buf[idx]
    }
}

impl<T, I, const MAX_BLOCKSIZE: usize> std::ops::IndexMut<I> for MonoBlockBuffer<T, MAX_BLOCKSIZE>
where
    I: std::slice::SliceIndex<[T]>,
    T: Default + Copy + Clone,
{
    #[inline]
    fn index_mut(&mut self, idx: I) -> &mut I::Output {
        &mut self.buf[idx]
    }
}

/// An audio buffer with two channels.
///
/// This has a constant number of frames (`MAX_BLOCKSIZE`), so this can be allocated on
/// the stack.
#[derive(Debug)]
pub struct StereoBlockBuffer<T: Default + Copy + Clone, const MAX_BLOCKSIZE: usize> {
    pub left: [T; MAX_BLOCKSIZE],
    pub right: [T; MAX_BLOCKSIZE],
}

impl<T: Default + Copy + Clone, const MAX_BLOCKSIZE: usize> StereoBlockBuffer<T, MAX_BLOCKSIZE> {
    /// Create a new buffer.
    ///
    /// This is a constant size (`MAX_BLOCKSIZE`), so this can be allocated on the stack.
    ///
    /// All samples will be cleared to 0.
    pub fn new() -> Self {
        Self {
            left: [T::default(); MAX_BLOCKSIZE],
            right: [T::default(); MAX_BLOCKSIZE],
        }
    }

    /// Create a new buffer without initializing.
    ///
    /// This is a constant size (`MAX_BLOCKSIZE`), so this can be allocated on the stack.
    ///
    /// ## Undefined behavior
    /// This data will be unitialized, so undefined behavior may occur if you try to read
    /// any data without writing to it first.
    pub unsafe fn new_uninit() -> Self {
        Self {
            left: MaybeUninit::uninit().assume_init(),
            right: MaybeUninit::uninit().assume_init(),
        }
    }

    /// Create a new buffer that only initializes the given number of frames to 0. Any samples
    /// after `frames` will be uninitialized.
    ///
    /// This is a constant size (`MAX_BLOCKSIZE`), so this can be allocated on the stack.
    ///
    /// ## Undefined behavior
    /// The portion of data not in the given range will be unitialized, so undefined behavior
    /// may occur if you try to read any of that data without writing to it first.
    pub unsafe fn new_uninit_after_frames(frames: RealFrames) -> Self {
        let frames = frames.0.min(MAX_BLOCKSIZE);
        let mut buf_left: [T; MAX_BLOCKSIZE] = MaybeUninit::uninit().assume_init();
        let mut buf_right: [T; MAX_BLOCKSIZE] = MaybeUninit::uninit().assume_init();

        let buf_left_part = &mut buf_left[0..frames];
        let buf_right_part = &mut buf_right[0..frames];
        buf_left_part.fill(T::default());
        buf_right_part.fill(T::default());

        Self {
            left: buf_left,
            right: buf_right,
        }
    }

    /// Create a new buffer that only initializes the given range of data to 0.
    ///
    /// This is a constant size (`MAX_BLOCKSIZE`), so this can be allocated on the stack.
    ///
    /// ## Undefined behavior
    /// The portion of data not in the given range will be unitialized, so undefined behavior
    /// may occur if you try to read any of that data without writing to it first.
    ///
    /// ## Panics
    /// This will panic if the given range lies outside the valid range `[0, N)`.
    pub unsafe fn new_partially_uninit(init_range: Range<usize>) -> Self {
        let mut buf_left: [T; MAX_BLOCKSIZE] = MaybeUninit::uninit().assume_init();
        let mut buf_right: [T; MAX_BLOCKSIZE] = MaybeUninit::uninit().assume_init();

        let buf_left_part = &mut buf_left[init_range.clone()];
        let buf_right_part = &mut buf_right[init_range];
        buf_left_part.fill(T::default());
        buf_right_part.fill(T::default());

        Self {
            left: buf_left,
            right: buf_right,
        }
    }

    /// Clear all samples in the buffer to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.left.fill(T::default());
        self.right.fill(T::default());
    }

    /// Clear a number of frames in the buffer to 0.
    #[inline]
    pub fn clear_frames(&mut self, frames: RealFrames) {
        let frames = frames.0.min(MAX_BLOCKSIZE);
        let buf_left_part = &mut self.left[0..frames];
        let buf_right_part = &mut self.right[0..frames];
        buf_left_part.fill(T::default());
        buf_right_part.fill(T::default());
    }

    /// Clear a range in the buffer to 0.
    ///
    /// ## Panics
    /// This will panic if the given range lies outside the valid range `[0, N)`.
    #[inline]
    pub fn clear_range(&mut self, range: Range<usize>) {
        let buf_left_part = &mut self.left[range.clone()];
        let buf_right_part = &mut self.right[range];
        buf_left_part.fill(T::default());
        buf_right_part.fill(T::default());
    }

    /// Copy all frames from `src` to this buffer.
    #[inline]
    pub fn copy_from(&mut self, src: &StereoBlockBuffer<T, MAX_BLOCKSIZE>) {
        self.left.copy_from_slice(&src.left);
        self.right.copy_from_slice(&src.right);
    }

    /// Copy the given number of `frames` from `src` to this buffer.
    #[inline]
    pub fn copy_frames_from(
        &mut self,
        src: &StereoBlockBuffer<T, MAX_BLOCKSIZE>,
        frames: RealFrames,
    ) {
        let frames = frames.0.min(MAX_BLOCKSIZE);
        self.left[0..frames].copy_from_slice(&src.left[0..frames]);
        self.right[0..frames].copy_from_slice(&src.right[0..frames]);
    }

    /// Return a mutable reference to the left and right channels (in that order).
    #[inline]
    pub fn left_right_mut(&mut self) -> (&mut [T; MAX_BLOCKSIZE], &mut [T; MAX_BLOCKSIZE]) {
        (&mut self.left, &mut self.right)
    }
}
