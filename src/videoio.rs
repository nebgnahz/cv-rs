//! Media I/O, see [OpenCV
//! videoio](http://docs.opencv.org/3.1.0/dd/de7/group__videoio.html)

use core::{CMat, Mat, Size2i};
use std::os::raw::{c_char, c_double, c_int};

// =============================================================================
//   VideoCapture
// =============================================================================
enum CvVideoCapture {}

unsafe impl Send for CvVideoCapture {}

/// Video capturing from video files, image sequences or cameras.
#[derive(Debug)]
pub struct VideoCapture {
    inner: *mut CvVideoCapture,
}

unsafe impl Send for VideoCapture {}

extern "C" {
    fn cv_videocapture_new(index: c_int) -> *mut CvVideoCapture;
    fn cv_videocapture_from_file(path: *const c_char) -> *mut CvVideoCapture;
    fn cv_videocapture_is_opened(ccap: *const CvVideoCapture) -> bool;
    fn cv_videocapture_read(v: *mut CvVideoCapture, m: *mut CMat) -> bool;
    fn cv_videocapture_drop(cap: *mut CvVideoCapture);
    fn cv_videocapture_set(cap: *mut CvVideoCapture, property: c_int, value: c_double) -> bool;
    fn cv_videocapture_get(cap: *mut CvVideoCapture, property: c_int) -> c_double;
}

#[allow(missing_docs)]
/// Video capture's property identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CapProp {
    /// Current position of the video file in milliseconds or video capture
    /// timestamp.
    PosMsec = 0,

    /// 0-based index of the frame to be decoded/captured next.
    PosFrames = 1,

    /// Relative position of the video file: 0 - start of the film, 1 - end of
    /// the film.
    PosAviRatio = 2,

    /// Width of the frames in the video stream.
    FrameWidth = 3,

    /// Height of the frames in the video stream.
    FrameHeight = 4,

    /// Frame rate.
    Fps = 5,

    /// 4-character code of codec.
    Fourcc = 6,

    /// Number of frames in the video file.
    FrameCount = 7,

    /// Format of the Mat objects returned by retrieve() .
    Format = 8,

    /// Backend-specific value indicating the current capture mode.
    Mode = 9,

    /// Brightness of the image (only for cameras).
    Brightness = 10,

    /// Contrast of the image (only for cameras).
    Contrast = 11,

    /// Saturation of the image (only for cameras).
    Saturation = 12,

    /// Hue of the image (only for cameras).
    Hue = 13,

    /// Gain of the image (only for cameras).
    Gain = 14,

    /// Exposure (only for cameras).
    Exposure = 15,

    /// Boolean flags indicating whether images should be converted to RGB.
    ConvertRgb = 16,

    /// Currently not supported
    WhiteBalanceBlueU = 17,

    /// Rectification flag for stereo cameras (note: only supported by DC1394 v
    /// 2.x backend currently)
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
        unsafe { cv_videocapture_set(self.inner, property as c_int, value) }
    }

    /// Gets a property in the `VideoCapture`.
    pub fn get(&self, property: CapProp) -> Option<f64> {
        let ret = unsafe { cv_videocapture_get(self.inner, property as c_int) };
        if ret != 0.0 {
            Some(ret)
        } else {
            None
        }
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

/// Opaque VideoWriter type.
enum CvVideoWriter {}

/// `VideoWriter` provides easy access to write videos to files.
/// - On Linux FFMPEG is used to write videos;
/// - On Windows FFMPEG or VFW is used;
/// - On MacOSX QTKit is used.
#[derive(Debug)]
pub struct VideoWriter {
    inner: *mut CvVideoWriter,
}

extern "C" {
    fn cv_videowriter_default() -> *mut CvVideoWriter;
    fn cv_videowriter_new(
        path: *const c_char,
        fourcc: c_int,
        fps: c_double,
        frame_size: Size2i,
        is_color: bool,
    ) -> *mut CvVideoWriter;
    fn cv_videowriter_drop(w: *mut CvVideoWriter);

    fn cv_videowriter_open(
        w: *mut CvVideoWriter,
        path: *const c_char,
        fourcc: c_int,
        fps: c_double,
        frame_size: Size2i,
        is_color: bool,
    ) -> bool;
    fn cv_videowriter_is_opened(w: *mut CvVideoWriter) -> bool;
    fn cv_videowriter_write(w: *mut CvVideoWriter, m: *mut CMat);
    fn cv_videowriter_set(w: *mut CvVideoWriter, property: c_int, value: c_double) -> bool;
    fn cv_videowriter_get(w: *mut CvVideoWriter, property: c_int) -> c_double;
}

impl VideoWriter {
    /// `VideoWriter` constructor.
    /// * path – Name of the output video file.
    /// * fourcc – 4-character code of codec used to compress the frames. For
    ///   example, VideoWriter::fourcc('P','I','M','1') is a MPEG-1 codec,
    ///   VideoWriter::fourcc('M','J','P','G') is a motion-jpeg codec etc. List
    ///   of codes can be obtained at Video Codecs by FOURCC page.
    /// * fps – Framerate of the created video stream.
    /// * frame_size – Size of the video frames.
    /// * is_color – If it is not zero, the encoder will expect and encode color
    ///   frames, otherwise it will work with grayscale frames (the flag is
    ///   currently supported on Windows only).
    pub fn new(path: &str, fourcc: i32, fps: f64, frame_size: Size2i, is_color: bool) -> VideoWriter {
        let s = ::std::ffi::CString::new(path).unwrap();
        let writer = unsafe { cv_videowriter_new((&s).as_ptr(), fourcc, fps, frame_size, is_color) };
        VideoWriter { inner: writer }
    }

    /// `VideoWriter` constructor.
    /// * path – Name of the output video file.
    /// * fourcc – 4-character code of codec used to compress the frames. For
    ///   example, VideoWriter::fourcc('P','I','M','1') is a MPEG-1 codec,
    ///   VideoWriter::fourcc('M','J','P','G') is a motion-jpeg codec etc. List
    ///   of codes can be obtained at Video Codecs by FOURCC page.
    /// * fps – Framerate of the created video stream.
    /// * frame_size – Size of the video frames.
    /// * is_color – If it is not zero, the encoder will expect and encode color
    ///   frames, otherwise it will work with grayscale frames (the flag is
    ///   currently supported on Windows only).
    pub fn open(&self, path: &str, fourcc: i32, fps: f64, frame_size: Size2i, is_color: bool) -> bool {
        let s = ::std::ffi::CString::new(path).unwrap();
        unsafe { cv_videowriter_open(self.inner, (&s).as_ptr(), fourcc, fps, frame_size, is_color) }
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
        unsafe { cv_videowriter_set(self.inner, property as c_int, value) }
    }

    /// Gets a property in the `VideoWriter`.
    pub fn get(&self, property: VideoWriterProperty) -> Option<f64> {
        let ret = unsafe { cv_videowriter_get(self.inner, property as c_int) };
        if ret != 0.0 {
            Some(ret)
        } else {
            None
        }
    }
}

impl Default for VideoWriter {
    fn default() -> VideoWriter {
        VideoWriter {
            inner: unsafe { cv_videowriter_default() },
        }
    }
}

impl Drop for VideoWriter {
    fn drop(&mut self) {
        unsafe {
            cv_videowriter_drop(self.inner);
        }
    }
}

/// `VideoWriter`'s property identifier.
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

/// Converts from [four character code](https://www.fourcc.org/) to `i32` for
/// OpenCV.
pub fn fourcc(c1: char, c2: char, c3: char, c4: char) -> i32 {
    unsafe { cv_fourcc(c1 as c_char, c2 as c_char, c3 as c_char, c4 as c_char) }
}

/// Converts from OpenCV's int to [four character
/// code](https://www.fourcc.org/).
pub fn codec_name(fourcc: i32) -> Option<String> {
    let ex = fourcc as u32;
    let vec = vec![
        (ex & 0xFFu32) as u8,
        ((ex & 0xFF00u32) >> 8) as u8,
        ((ex & 0xFF0000u32) >> 16) as u8,
        ((ex & 0xFF000000u32) >> 24) as u8,
    ];
    String::from_utf8(vec).ok()
}
