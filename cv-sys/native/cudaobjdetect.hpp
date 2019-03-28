#ifndef OPENCV_CUDA_H_
#define OPENCV_CUDA_H_

#include "common.hpp"
#include "objdetect.hpp"
#include <opencv2/cudaobjdetect.hpp>
#include <stddef.h>

namespace cvsys {

// =============================================================================
//   Basic
// =============================================================================
cv::cuda::GpuMat* cuda_gpu_mat_default();
void cuda_gpu_mat_drop(cv::cuda::GpuMat* gpu_image);
void cuda_gpu_mat_upload(cv::cuda::GpuMat*, cv::Mat*);
cv::Mat* mat_from_gpu_mat(cv::cuda::GpuMat*);
cv::cuda::GpuMat* cuda_gpu_mat_from_mat(cv::Mat*);

// =============================================================================
//   Hog
// =============================================================================
void* cuda_hog_default();
void* cuda_hog_new(Size2i win_size, Size2i block_size, Size2i block_stride, Size2i cell_size, int nbins);
void cuda_hog_drop(void*);
void cuda_hog_set_detector(void*, SvmDetector* detector);
void cuda_hog_detect(void*, cv::cuda::GpuMat* image, CVec<Rect>*);
void cuda_hog_detect_with_conf(void*, cv::cuda::GpuMat* image, CVec<Rect>*, CVec<double>*);

void cuda_hog_set_gamma_correction(void*, bool gamma);
void cuda_hog_set_group_threshold(void*, int group_threshold);
void cuda_hog_set_hit_threshold(void*, double hit_threshold);
void cuda_hog_set_l2hys_threshold(void*, double l2hys_threshold);
void cuda_hog_set_num_levels(void*, int num_levels);
void cuda_hog_set_scale_factor(void*, double scale_factor);
void cuda_hog_set_win_sigma(void*, double win_sigma);
void cuda_hog_set_win_stride(void*, Size2i win_stride);

bool cuda_hog_get_gamma_correction(void*);
int cuda_hog_get_group_threshold(void*);
double cuda_hog_get_hit_threshold(void*);
double cuda_hog_get_l2hys_threshold(void*);
int cuda_hog_get_num_levels(void*);
double cuda_hog_get_scale_factor(void*);
double cuda_hog_get_win_sigma(void*);
Size2i cuda_hog_get_win_stride(void*);

// =============================================================================
//   CascadeClassifier
// =============================================================================
void* cuda_cascade_new(const char* const filename);
void cuda_cascade_drop(void*);
void cuda_cascade_detect(void*, cv::cuda::GpuMat* image, CVec<Rect>*);

void cuda_cascade_set_find_largest_object(void*, bool);
void cuda_cascade_set_max_num_objects(void*, int);
void cuda_cascade_set_min_neighbors(void*, int);
void cuda_cascade_set_max_object_size(void*, Size2i);
void cuda_cascade_set_min_object_size(void*, Size2i);
void cuda_cascade_set_scale_factor(void*, double);

Size2i cuda_cascade_get_classifier_size(void*);
bool cuda_cascade_get_find_largest_object(void*);
int cuda_cascade_get_max_num_objects(void*);
int cuda_cascade_get_min_neighbors(void*);
Size2i cuda_cascade_get_max_object_size(void*);
Size2i cuda_cascade_get_min_object_size(void*);
double cuda_cascade_get_scale_factor(void*);

}  // namespace cvsys

#endif  // OPENCV_GPU_H_
