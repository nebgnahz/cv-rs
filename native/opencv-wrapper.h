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
void* cv_mat_from_file_storage(const char* path, const char* section);
void* cv_mat_new();
void* cv_mat_new_with_size(int rows, int cols, int type);
void* cv_mat_zeros(int rows, int cols, int type);
void* cv_mat_from_buffer(int rows, int cols, int type, const uint8_t* buf);
void* cv_mat_eye(int rows, int cols, int type);

bool cv_mat_valid(cv::Mat* mat);

// The caller owns the returned cv::Mat
void* cv_mat_roi(cv::Mat* mat, Rect crect);
void cv_mat_flip(cv::Mat* image, int code);

// The caller owns the returned data cv::Mat

int cv_mat_rows(const cv::Mat* const mat);
int cv_mat_cols(const cv::Mat* const mat);
int cv_mat_depth(const cv::Mat* const mat);
int cv_mat_channels(const cv::Mat* const mat);
int cv_mat_type(const cv::Mat* const mat);
const uint8_t* cv_mat_data(const cv::Mat* const mat);
size_t cv_mat_total(const cv::Mat* const mat);
size_t cv_mat_elem_size(const cv::Mat* const mat);
size_t cv_mat_elem_size1(const cv::Mat* const mat);
size_t cv_mat_step1(const cv::Mat* const mat, int i);

// Free a Mat object
void cv_mat_drop(cv::Mat* mat);

void cv_vec_drop(CVec<void>* vec, unsigned int depth);
void c_drop(void* value);

// =============================================================================
//  core array
// =============================================================================
void cv_mat_in_range(cv::Mat* mat, Scalar lowerb, Scalar upperb, cv::Mat* dst);
void cv_mat_min_max_loc(
    const cv::Mat* const mat, double* min, double* max, Point2i* minLoc, Point2i* maxLoc, const cv::Mat* const cmask);
void cv_mat_mix_channels(cv::Mat* mat, size_t nsrcs, cv::Mat* dst, size_t ndsts, const int* from_to, size_t npairs);
void cv_mat_normalize(cv::Mat* csrc, cv::Mat* cdst, double alpha, double beta, int norm_type);
void cv_mat_bitwise_and(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst);
void cv_mat_bitwise_not(const cv::Mat* const src, cv::Mat* const dst);
void cv_mat_bitwise_or(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst);
void cv_mat_bitwise_xor(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst);
int cv_mat_count_non_zero(const cv::Mat* const src);

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
//   VideoIO
// =============================================================================
void* cv_videocapture_new(int index);
void* cv_videocapture_from_file(const char* const filename);
bool cv_videocapture_is_opened(const cv::VideoCapture* const cap);
bool cv_videocapture_read(cv::VideoCapture* cap, cv::Mat* mat);
void cv_videocapture_drop(cv::VideoCapture* cap);
bool cv_videocapture_set(cv::VideoCapture* cap, int property, double value);
double cv_videocapture_get(cv::VideoCapture* cap, int property);

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
void* cv_cascade_classifier_new();
void* cv_cascade_classifier_from_path(const char* const path);
bool cv_cascade_classifier_load(cv::CascadeClassifier* cc, const char* const path);
void cv_cascade_classifier_drop(cv::CascadeClassifier* cc);

// vec_of_rect is dynamically allocated, the caller should take ownership of it.
void cv_cascade_classifier_detect(cv::CascadeClassifier* cascade,
                                  cv::Mat* mat,
                                  CVec<Rect>* vec_of_rect,
                                  double scale_factor,
                                  int min_neighbors,
                                  int flags,
                                  Size2i min_size,
                                  Size2i max_size);

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
void* cv_term_criteria_new(int type, int count, double epsilon);
void cv_term_criteria_drop(cv::TermCriteria* criteria);
RotatedRect cv_camshift(cv::Mat* back_project_image, Rect window, cv::TermCriteria* criteria);

// =============================================================================
//   Text
// =============================================================================

EXTERN_C_END

#endif  // OPENCV_WRAPPER_H_
