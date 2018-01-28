#ifndef OPENCV_GPU_H_
#define OPENCV_GPU_H_
#define EXTERN_C_BEGIN extern "C" {
#define EXTERN_C_END }

#include <stddef.h>
#include <stdint.h>
#include <opencv2/cudaobjdetect.hpp>
#include "common.h"

EXTERN_C_BEGIN

// =============================================================================
//   Basic
// =============================================================================
void* cv_gpu_mat_default();
void cv_gpu_mat_drop(cv::cuda::GpuMat*);
void cv_gpu_mat_upload(cv::cuda::GpuMat*, cv::Mat*);
void* cv_mat_from_gpu_mat(cv::cuda::GpuMat*);
void* cv_gpu_mat_from_mat(cv::Mat*);

// =============================================================================
//   Hog
// =============================================================================
void* cv_gpu_hog_default();
void* cv_gpu_hog_new(Size2i win_size, Size2i block_size,
                     Size2i block_stride, Size2i cell_size, int32_t nbins);
void cv_gpu_hog_drop(cv::Ptr<cv::cuda::HOG>*);
void cv_gpu_hog_set_detector(cv::Ptr<cv::cuda::HOG>*, std::vector<float>*);
void cv_gpu_hog_detect(cv::Ptr<cv::cuda::HOG>*, cv::cuda::GpuMat*, CVec<Rect>*);
void cv_gpu_hog_detect_with_conf(cv::Ptr<cv::cuda::HOG>*, cv::cuda::GpuMat*, CVec<Rect>*, CVec<double>*);

void cv_gpu_hog_set_gamma_correction(cv::Ptr<cv::cuda::HOG>*, bool gamma);
void cv_gpu_hog_set_group_threshold(cv::Ptr<cv::cuda::HOG>*, int32_t group_threshold);
void cv_gpu_hog_set_hit_threshold(cv::Ptr<cv::cuda::HOG>*, double hit_threshold);
void cv_gpu_hog_set_l2hys_threshold(cv::Ptr<cv::cuda::HOG>*, double l2hys_threshold);
void cv_gpu_hog_set_num_levels(cv::Ptr<cv::cuda::HOG>*, size_t num_levels);
void cv_gpu_hog_set_scale_factor(cv::Ptr<cv::cuda::HOG>*, double scale_factor);
void cv_gpu_hog_set_win_sigma(cv::Ptr<cv::cuda::HOG>*, double win_sigma);
void cv_gpu_hog_set_win_stride(cv::Ptr<cv::cuda::HOG>*, Size2i win_stride);

bool cv_gpu_hog_get_gamma_correction(cv::Ptr<cv::cuda::HOG>*);
int32_t cv_gpu_hog_get_group_threshold(cv::Ptr<cv::cuda::HOG>*);
double cv_gpu_hog_get_hit_threshold(cv::Ptr<cv::cuda::HOG>*);
double cv_gpu_hog_get_l2hys_threshold(cv::Ptr<cv::cuda::HOG>*);
size_t cv_gpu_hog_get_num_levels(cv::Ptr<cv::cuda::HOG>*);
double cv_gpu_hog_get_scale_factor(cv::Ptr<cv::cuda::HOG>*);
double cv_gpu_hog_get_win_sigma(cv::Ptr<cv::cuda::HOG>*);
Size2i cv_gpu_hog_get_win_stride(cv::Ptr<cv::cuda::HOG> *);

// =============================================================================
//   CascadeClassifier
// =============================================================================
void* cv_gpu_cascade_new(const char* const filename);
void cv_gpu_cascade_drop(cv::Ptr<cv::cuda::CascadeClassifier>*);
void cv_gpu_cascade_detect(cv::Ptr<cv::cuda::CascadeClassifier>*, cv::cuda::GpuMat*, CVec<Rect>*);

void cv_gpu_cascade_set_find_largest_object(cv::Ptr<cv::cuda::CascadeClassifier>*, bool);
void cv_gpu_cascade_set_max_num_objects(cv::Ptr<cv::cuda::CascadeClassifier>*, int32_t);
void cv_gpu_cascade_set_min_neighbors(cv::Ptr<cv::cuda::CascadeClassifier>*, int32_t);
void cv_gpu_cascade_set_max_object_size(cv::Ptr<cv::cuda::CascadeClassifier>*, Size2i);
void cv_gpu_cascade_set_min_object_size(cv::Ptr<cv::cuda::CascadeClassifier>*, Size2i);
void cv_gpu_cascade_set_scale_factor(cv::Ptr<cv::cuda::CascadeClassifier>*, double);

Size2i cv_gpu_cascade_get_classifier_size(cv::Ptr<cv::cuda::CascadeClassifier>*);
bool cv_gpu_cascade_get_find_largest_object(cv::Ptr<cv::cuda::CascadeClassifier>*);
int32_t cv_gpu_cascade_get_max_num_objects(cv::Ptr<cv::cuda::CascadeClassifier>*);
int32_t cv_gpu_cascade_get_min_neighbors(cv::Ptr<cv::cuda::CascadeClassifier>*);
Size2i cv_gpu_cascade_get_max_object_size(cv::Ptr<cv::cuda::CascadeClassifier>*);
Size2i cv_gpu_cascade_get_min_object_size(cv::Ptr<cv::cuda::CascadeClassifier>*);
double cv_gpu_cascade_get_scale_factor(cv::Ptr<cv::cuda::CascadeClassifier>*);

EXTERN_C_END

#endif  // OPENCV_GPU_H_
