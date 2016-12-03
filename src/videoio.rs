//! Mainly `VideoCapture` and `VideoWriter`

use core::{Mat, CMat, Size2i};
use libc::{c_int, c_char, c_double};

// =============================================================================
//   VideoCapture
// =============================================================================
enum CvVideoCapture {}

/// Video capturing from video files, image sequences or cameras.
pub struct VideoCapture {
    inner: *mut CvVideoCapture,
}

extern "C" {
    fn cv_videocapture_new(index: c_int) -> *mut CvVideoCapture;
    fn cv_videocapture_from_file(path: *const c_char) -> *mut CvVideoCapture;
    fn cv_videocapture_is_opened(ccap: *const CvVideoCapture) -> bool;
    fn cv_videocapture_read(v: *mut CvVideoCapture, m: *mut CMat) -> bool;
    fn cv_videocapture_drop(cap: *mut CvVideoCapture);
    fn cv_videocapture_set(cap: *mut CvVideoCapture, property: c_int, value: c_double) -> bool;
    fn cv_videocapture_get(cap: *mut CvVideoCapture, property: c_int) -> c_double;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CapProp {
    PosMsec = 0,
    PosFrames = 1,
    PosAviRatio = 2,
    FrameWidth = 3,
    FrameHeight = 4,
    Fps = 5,
    Fourcc = 6,
    FrameCount = 7,
    Format = 8,
    Mode = 9,
    Brightness = 10,
    Contrast = 11,
    Saturation = 12,
    Hue = 13,
    Gain = 14,
    Exposure = 15,
    ConvertRgb = 16,
    WhiteBalanceBlueU = 17,
    Rectification = 18,
    Monochrome = 19,
    Sharpness = 20,
    AutoExposure = 21,
    Gamma = 22,
    Temperature = 23,
    Trigger = 24,
    TriggerDelay = 25,
    WhiteBalanceRedV = 26,
    Zoom = 27,
    Focus = 28,
    Guid = 29,
    IsoSpeed = 30,
    Backlight = 32,
    Pan = 33,
    Tilt = 34,
    Roll = 35,
    Iris = 36,
    Settings = 37,
    Buffersize = 38,
    Autofocus = 39,
}

impl VideoCapture {
    /// Creates a capture device with specified camera id. If there is a single
    /// camera connected, just pass 0.
    pub fn new(index: i32) -> Self {
        let cap = unsafe { cv_videocapture_new(index) };
        VideoCapture { inner: cap }
    }

    /// Creates a capture device with the path of a video file (eg. video.avi).
    /// This also supports image sequence, eg. img_%02d.jpg, which will read
    /// samples like img_00.jpg, img_01.jpg, img_02.jpg, ...).
    pub fn from_path(path: &str) -> Self {
        let s = ::std::ffi::CString::new(path).unwrap();
        let cap = unsafe { cv_videocapture_from_file((&s).as_ptr()) };
        VideoCapture { inner: cap }
    }

    /// Returns true if video capturing has been initialized already.
    pub fn is_open(&self) -> bool {
        unsafe { cv_videocapture_is_opened(self.inner) }
    }

    /// Grabs, decodes and returns the next video frame. `read` combines
    /// `VideoCapture::grab` and `VideoCapture::retrieve` in one call. This is
    /// the most convenient method for reading video files or capturing data
    /// from decode and return the just grabbed frame.
    ///
    /// If no frames has been grabbed (camera has been disconnected, or there
    /// are no more frames in video file), the methods return `None`.
    pub fn read(&self) -> Option<Mat> {
        let inner = CMat::new();
        let status = unsafe { cv_videocapture_read(self.inner, inner) };
        if status {
            Some(Mat::from_raw(inner))
        } else {
            None
        }
    }

    /// Sets a property in the `VideoCapture`.
    pub fn set(&self, property: CapProp, value: f64) -> bool {
        unsafe { cv_videocapture_set(self.inner, property as c_int, value as c_double) }
    }

    /// Gets a property in the `VideoCapture`.
    pub fn get(&self, property: CapProp) -> Option<f64> {
        let ret = unsafe { cv_videocapture_get(self.inner, property as c_int) };
        if ret != 0.0 { Some(ret as f64) } else { None }
    }
}

impl Drop for VideoCapture {
    fn drop(&mut self) {
        unsafe {
            cv_videocapture_drop(self.inner);
        }
    }
}

// =============================================================================
//   VideoWriter
// =============================================================================

/// VideoWriter
enum CvVideoWriter {}
pub struct VideoWriter {
    inner: *mut CvVideoWriter,
}

extern "C" {
    fn cv_videowriter_default() -> *mut CvVideoWriter;
    fn cv_videowriter_new(path: *const c_char,
                          fourcc: c_int,
                          fps: c_double,
                          frame_size: Size2i,
                          is_color: bool)
                          -> *mut CvVideoWriter;
    fn cv_videowriter_drop(w: *mut CvVideoWriter);

    fn cv_videowriter_open(w: *mut CvVideoWriter,
                           path: *const c_char,
                           fourcc: c_int,
                           fps: c_double,
                           frame_size: Size2i,
                           is_color: bool)
                           -> bool;
    fn cv_videowriter_is_opened(w: *mut CvVideoWriter) -> bool;
    fn cv_videowriter_write(w: *mut CvVideoWriter, m: *mut CMat);
    fn cv_videowriter_set(w: *mut CvVideoWriter, property: c_int, value: c_double) -> bool;
    fn cv_videowriter_get(w: *mut CvVideoWriter, property: c_int) -> c_double;
}

impl VideoWriter {
    pub fn new(path: &str, fourcc: i32, fps: f64, frame_size: Size2i, is_color: bool) -> VideoWriter {
        let s = ::std::ffi::CString::new(path).unwrap();
        let writer = unsafe {
            cv_videowriter_new((&s).as_ptr(),
                               fourcc as c_int,
                               fps as c_double,
                               frame_size,
                               is_color)
        };
        VideoWriter { inner: writer }
    }

    pub fn open(&self, path: &str, fourcc: i32, fps: f64, frame_size: Size2i, is_color: bool) -> bool {
        let s = ::std::ffi::CString::new(path).unwrap();
        unsafe {
            cv_videowriter_open(self.inner,
                                (&s).as_ptr(),
                                fourcc as c_int,
                                fps as c_double,
                                frame_size,
                                is_color)
        }
    }

    /// Writes the specified image to video file. It must have the same size as
    /// has been specified when opening the video writer.
    pub fn write(&self, mat: &Mat) {
        unsafe { cv_videowriter_write(self.inner, mat.inner) }
    }

    /// Returns true if video writer has been initialized already.
    pub fn is_open(&self) -> bool {
        unsafe { cv_videowriter_is_opened(self.inner) }
    }

    /// Sets a property in the `VideoWriter`.
    /// Note: `VideoWriterProperty::FrameBytes` is read-only.
    pub fn set(&self, property: VideoWriterProperty, value: f64) -> bool {
        unsafe { cv_videowriter_set(self.inner, property as c_int, value as c_double) }
    }

    /// Gets a property in the `VideoWriter`.
    pub fn get(&self, property: VideoWriterProperty) -> Option<f64> {
        let ret = unsafe { cv_videowriter_get(self.inner, property as c_int) };
        if ret != 0.0 { Some(ret as f64) } else { None }
    }
}

impl Default for VideoWriter {
    fn default() -> VideoWriter {
        VideoWriter { inner: unsafe { cv_videowriter_default() } }
    }
}

impl Drop for VideoWriter {
    fn drop(&mut self) {
        unsafe {
            cv_videowriter_drop(self.inner);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VideoWriterProperty {
    /// Current quality of the encoded videostream.
    Quality = 1,

    /// (Read-only) Size of just encoded video frame; note that the encoding
    /// order may be different from representation order.
    FrameBytes = 2,

    /// Number of stripes for parallel encoding
    NStripes = 3,
}

// =============================================================================
//   Utility functions
// =============================================================================
extern "C" {
    fn cv_fourcc(c1: c_char, c2: c_char, c3: c_char, c4: c_char) -> c_int;
}

pub fn fourcc(c1: char, c2: char, c3: char, c4: char) -> i32 {
    unsafe { cv_fourcc(c1 as c_char, c2 as c_char, c3 as c_char, c4 as c_char) }
}
