#ifndef OPENCV_WRAPPER_H_
#define OPENCV_WRAPPER_H_

#include <functional>
#include <opencv2/core.hpp>
#include <opencv2/features2d.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/objdetect.hpp>
#include <opencv2/text/ocr.hpp>
#include <opencv2/video/tracking.hpp>
#include <opencv2/xfeatures2d.hpp>
#include <stddef.h>
#include <stdint.h>

#include "common.h"

#define EXTERN_C_BEGIN extern "C" {
#define EXTERN_C_END }

EXTERN_C_BEGIN

// The caller owns the returned data cv::Mat

void cv_vec_drop(CVec<void>* vec, unsigned int depth);
void c_drop(void* value);

// =============================================================================
//  core array
// =============================================================================

// =============================================================================
//  Imgproc
// =============================================================================

// =============================================================================
//  Imgcodecs
// =============================================================================

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================

void cv_imshow(const char* const winname, cv::Mat* mat);
int cv_wait_key(int delay_in_millis);

// =============================================================================
//
// =============================================================================

void* cv_videowriter_default();
void* cv_videowriter_new(const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color);
void cv_videowriter_drop(cv::VideoWriter* writer);
bool cv_videowriter_open(
    cv::VideoWriter* writer, const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color);
bool cv_videowriter_is_opened(cv::VideoWriter* writer);
void cv_videowriter_write(cv::VideoWriter* writer, cv::Mat* mat);
bool cv_videowriter_set(cv::VideoWriter* writer, int property, double value);
double cv_videowriter_get(cv::VideoWriter* writer, int property);

// =============================================================================
//   CascadeClassifier
// =============================================================================

void* cv_hog_default_people_detector();
void* cv_hog_daimler_people_detector();
void cv_hog_detector_drop(std::vector<float>*);

void* cv_hog_new();
void cv_hog_drop(cv::HOGDescriptor*);
void cv_hog_set_svm_detector(cv::HOGDescriptor*, std::vector<float>*);
void cv_hog_detect(cv::HOGDescriptor*,
                   cv::Mat*,
                   CVec<Rect>* vec_detected,
                   CVec<double>* vec_weight,
                   Size2i win_stride,
                   Size2i padding,
                   double scale,
                   double final_threshold,
                   bool use_means_shift);

// =============================================================================
//   VideoTrack
// =============================================================================

// =============================================================================
//   Text
// =============================================================================

EXTERN_C_END

#endif  // OPENCV_WRAPPER_H_
