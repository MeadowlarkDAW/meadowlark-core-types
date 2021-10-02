use std::mem::MaybeUninit;
use std::ops::Range;

use num_traits::Num;

/// An audio buffer with a single channel.
///
/// This has a constant number of frames (`N`), so this can be allocated on
/// the stack.
#[derive(Debug)]
pub struct MonoBlockBuffer<T: Num + Copy + Clone, const N: usize> {
    pub buf: [T; N],
}

impl<T: Num + Copy + Clone, const N: usize> MonoBlockBuffer<T, N> {
    /// Create a new buffer.
    ///
    /// This is a constant size (`N`), so this can be allocated on the stack.
    ///
    /// All samples will be cleared to 0.
    pub fn new() -> Self {
        Self {
            buf: [T::zero(); N],
        }
    }

    /// Create a new buffer without initializing.
    ///
    /// This is a constant size (`N`), so this can be allocated on the stack.
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
    /// This is a constant size (`N`), so this can be allocated on the stack.
    ///
    /// ## Undefined behavior
    /// The portion of data not in the given range will be unitialized, so undefined behavior
    /// may occur if you try to read any of that data without writing to it first.
    pub unsafe fn new_uninit_after_frames(frames: usize) -> Self {
        let frames = frames.min(N);
        let mut buf: [T; N] = MaybeUninit::uninit().assume_init();

        let buf_part = &mut buf[0..frames];
        buf_part.fill(T::zero());

        Self { buf }
    }

    /// Create a new buffer that only initializes the given range of data to 0.
    ///
    /// This is a constant size (`N`), so this can be allocated on the stack.
    ///
    /// ## Undefined behavior
    /// The portion of data not in the given range will be unitialized, so undefined behavior
    /// may occur if you try to read any of that data without writing to it first.
    ///
    /// ## Panics
    /// This will panic if the given range lies outside the valid range `[0, N)`.
    pub unsafe fn new_partially_uninit(init_range: Range<usize>) -> Self {
        let mut buf: [T; N] = MaybeUninit::uninit().assume_init();

        let buf_part = &mut buf[init_range];
        buf_part.fill(T::zero());

        Self { buf }
    }

    /// Clear all samples in the buffer to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.buf.fill(T::zero());
    }

    /// Clear a number of frames in the buffer to 0.
    #[inline]
    pub fn clear_frames(&mut self, frames: usize) {
        let frames = frames.min(N);
        let buf_part = &mut self.buf[0..frames];
        buf_part.fill(T::zero());
    }

    /// Clear a range in the buffer to 0.
    ///
    /// ## Panics
    /// This will panic if the given range lies outside the valid range `[0, N)`.
    #[inline]
    pub fn clear_range(&mut self, range: Range<usize>) {
        let buf_part = &mut self.buf[range];
        buf_part.fill(T::zero());
    }

    /// Copy all frames from `src` to this buffer.
    #[inline]
    pub fn copy_from(&mut self, src: &MonoBlockBuffer<T, N>) {
        self.buf.copy_from_slice(&src.buf);
    }

    /// Copy the given number of `frames` from `src` to this buffer.
    #[inline]
    pub fn copy_frames_from(&mut self, src: &MonoBlockBuffer<T, N>, frames: usize) {
        let frames = frames.min(N);
        self.buf[0..frames].copy_from_slice(&src.buf[0..frames]);
    }

    /// Add all frames from `src` to this buffer.
    #[inline]
    pub fn sum_from(&mut self, src: &MonoBlockBuffer<T, N>) {
        for i in 0..N {
            self.buf[i] = self.buf[i] + src.buf[i];
        }
    }

    /// Add the given number of frames from `src` to this buffer.
    #[inline]
    pub fn sum_frames_from(&mut self, src: &MonoBlockBuffer<T, N>, frames: usize) {
        let frames = frames.min(N);
        for i in 0..frames {
            self.buf[i] = self.buf[i] + src.buf[i];
        }
    }

    /// Multiplay all frames from `src` to this buffer.
    #[inline]
    pub fn multiply_from(&mut self, src: &MonoBlockBuffer<T, N>) {
        for i in 0..N {
            self.buf[i] = self.buf[i] * src.buf[i];
        }
    }

    /// Multiply the given number of frames from `src` to this buffer.
    #[inline]
    pub fn multiply_frames_from(&mut self, src: &MonoBlockBuffer<T, N>, frames: usize) {
        let frames = frames.min(N);
        for i in 0..frames {
            self.buf[i] = self.buf[i] * src.buf[i];
        }
    }
}

impl<T, I, const N: usize> std::ops::Index<I> for MonoBlockBuffer<T, N>
where
    I: std::slice::SliceIndex<[T]>,
    T: Num + Copy + Clone,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, idx: I) -> &I::Output {
        &self.buf[idx]
    }
}

impl<T, I, const N: usize> std::ops::IndexMut<I> for MonoBlockBuffer<T, N>
where
    I: std::slice::SliceIndex<[T]>,
    T: Num + Copy + Clone,
{
    #[inline]
    fn index_mut(&mut self, idx: I) -> &mut I::Output {
        &mut self.buf[idx]
    }
}

/// An audio buffer with a single channel.
///
/// This has a constant number of frames (`N`), so this can be allocated on
/// the stack.
#[derive(Debug)]
pub struct StereoBlockBuffer<T: Num + Copy + Clone, const N: usize> {
    pub left: [T; N],
    pub right: [T; N],
}

impl<T: Num + Copy + Clone, const N: usize> StereoBlockBuffer<T, N> {
    /// Create a new buffer.
    ///
    /// This is a constant size (`N`), so this can be allocated on the stack.
    ///
    /// All samples will be cleared to 0.
    pub fn new() -> Self {
        Self {
            left: [T::zero(); N],
            right: [T::zero(); N],
        }
    }

    /// Create a new buffer without initializing.
    ///
    /// This is a constant size (`N`), so this can be allocated on the stack.
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
    /// This is a constant size (`N`), so this can be allocated on the stack.
    ///
    /// ## Undefined behavior
    /// The portion of data not in the given range will be unitialized, so undefined behavior
    /// may occur if you try to read any of that data without writing to it first.
    pub unsafe fn new_uninit_after_frames(frames: usize) -> Self {
        let frames = frames.min(N);
        let mut buf_left: [T; N] = MaybeUninit::uninit().assume_init();
        let mut buf_right: [T; N] = MaybeUninit::uninit().assume_init();

        let buf_left_part = &mut buf_left[0..frames];
        let buf_right_part = &mut buf_right[0..frames];
        buf_left_part.fill(T::zero());
        buf_right_part.fill(T::zero());

        Self {
            left: buf_left,
            right: buf_right,
        }
    }

    /// Create a new buffer that only initializes the given range of data to 0.
    ///
    /// This is a constant size (`N`), so this can be allocated on the stack.
    ///
    /// ## Undefined behavior
    /// The portion of data not in the given range will be unitialized, so undefined behavior
    /// may occur if you try to read any of that data without writing to it first.
    ///
    /// ## Panics
    /// This will panic if the given range lies outside the valid range `[0, N)`.
    pub unsafe fn new_partially_uninit(init_range: Range<usize>) -> Self {
        let mut buf_left: [T; N] = MaybeUninit::uninit().assume_init();
        let mut buf_right: [T; N] = MaybeUninit::uninit().assume_init();

        let buf_left_part = &mut buf_left[init_range.clone()];
        let buf_right_part = &mut buf_right[init_range];
        buf_left_part.fill(T::zero());
        buf_right_part.fill(T::zero());

        Self {
            left: buf_left,
            right: buf_right,
        }
    }

    /// Clear all samples in the buffer to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.left.fill(T::zero());
        self.right.fill(T::zero());
    }

    /// Clear a number of frames in the buffer to 0.
    #[inline]
    pub fn clear_frames(&mut self, frames: usize) {
        let frames = frames.min(N);
        let buf_left_part = &mut self.left[0..frames];
        let buf_right_part = &mut self.right[0..frames];
        buf_left_part.fill(T::zero());
        buf_right_part.fill(T::zero());
    }

    /// Clear a range in the buffer to 0.
    ///
    /// ## Panics
    /// This will panic if the given range lies outside the valid range `[0, N)`.
    #[inline]
    pub fn clear_range(&mut self, range: Range<usize>) {
        let buf_left_part = &mut self.left[range.clone()];
        let buf_right_part = &mut self.right[range];
        buf_left_part.fill(T::zero());
        buf_right_part.fill(T::zero());
    }

    /// Copy all frames from `src` to this buffer.
    #[inline]
    pub fn copy_from(&mut self, src: &StereoBlockBuffer<T, N>) {
        self.left.copy_from_slice(&src.left);
        self.right.copy_from_slice(&src.right);
    }

    /// Copy the given number of `frames` from `src` to this buffer.
    #[inline]
    pub fn copy_frames_from(&mut self, src: &StereoBlockBuffer<T, N>, frames: usize) {
        let frames = frames.min(N);
        self.left[0..frames].copy_from_slice(&src.left[0..frames]);
        self.right[0..frames].copy_from_slice(&src.right[0..frames]);
    }

    /// Add all frames from `src` to this buffer.
    #[inline]
    pub fn sum_from(&mut self, src: &StereoBlockBuffer<T, N>) {
        for i in 0..N {
            self.left[i] = self.left[i] + src.left[i];
            self.right[i] = self.right[i] + src.right[i];
        }
    }

    /// Add the given number of frames from `src` to this buffer.
    #[inline]
    pub fn sum_frames_from(&mut self, src: &StereoBlockBuffer<T, N>, frames: usize) {
        let frames = frames.min(N);
        for i in 0..frames {
            self.left[i] = self.left[i] + src.left[i];
            self.right[i] = self.right[i] + src.right[i];
        }
    }

    /// Multiplay all frames from `src` to this buffer.
    #[inline]
    pub fn multiply_from(&mut self, src: &StereoBlockBuffer<T, N>) {
        for i in 0..N {
            self.left[i] = self.left[i] * src.left[i];
            self.right[i] = self.right[i] * src.right[i];
        }
    }

    /// Multiply the given number of frames from `src` to this buffer.
    #[inline]
    pub fn multiply_frames_from(&mut self, src: &StereoBlockBuffer<T, N>, frames: usize) {
        let frames = frames.min(N);
        for i in 0..frames {
            self.left[i] = self.left[i] * src.left[i];
            self.right[i] = self.right[i] * src.right[i];
        }
    }

    #[inline]
    pub fn left_right_mut(&mut self) -> (&mut [T; N], &mut [T; N]) {
        (&mut self.left, &mut self.right)
    }
}
