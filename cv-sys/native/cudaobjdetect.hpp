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

struct CudaHog : cv::Ptr<cv::cuda::HOG> {
    CudaHog(cv::Ptr<cv::cuda::HOG> p) : cv::Ptr<cv::cuda::HOG>(p) {
    }
};

CudaHog* cuda_hog_default();
CudaHog* cuda_hog_new(Size2i win_size, Size2i block_size, Size2i block_stride, Size2i cell_size, int nbins);
void cuda_hog_drop(CudaHog* hog);
void cuda_hog_set_detector(CudaHog*, SvmDetector* detector);
void cuda_hog_detect(CudaHog*, cv::cuda::GpuMat* image, CVec<Rect>*);
void cuda_hog_detect_with_conf(CudaHog*, cv::cuda::GpuMat* image, CVec<Rect>*, CVec<double>*);

void cuda_hog_set_gamma_correction(CudaHog*, bool gamma);
void cuda_hog_set_group_threshold(CudaHog*, int group_threshold);
void cuda_hog_set_hit_threshold(CudaHog*, double hit_threshold);
void cuda_hog_set_l2hys_threshold(CudaHog*, double l2hys_threshold);
void cuda_hog_set_num_levels(CudaHog*, int num_levels);
void cuda_hog_set_scale_factor(CudaHog*, double scale_factor);
void cuda_hog_set_win_sigma(CudaHog*, double win_sigma);
void cuda_hog_set_win_stride(CudaHog*, Size2i win_stride);

bool cuda_hog_get_gamma_correction(CudaHog*);
int cuda_hog_get_group_threshold(CudaHog*);
double cuda_hog_get_hit_threshold(CudaHog*);
double cuda_hog_get_l2hys_threshold(CudaHog*);
int cuda_hog_get_num_levels(CudaHog*);
double cuda_hog_get_scale_factor(CudaHog*);
double cuda_hog_get_win_sigma(CudaHog*);
Size2i cuda_hog_get_win_stride(CudaHog*);

// =============================================================================
//   CascadeClassifier
// =============================================================================

struct CudaCascadeClassifier : cv::Ptr<cv::cuda::CascadeClassifier> {
    CudaCascadeClassifier(cv::Ptr<cv::cuda::CascadeClassifier> p) : cv::Ptr<cv::cuda::CascadeClassifier>(p) {
    }
};

CudaCascadeClassifier* cuda_cascade_new(const char* const filename);
void cuda_cascade_drop(CudaCascadeClassifier*);
void cuda_cascade_detect(CudaCascadeClassifier*, cv::cuda::GpuMat* image, CVec<Rect>*);

void cuda_cascade_set_find_largest_object(CudaCascadeClassifier*, bool);
void cuda_cascade_set_max_num_objects(CudaCascadeClassifier*, int);
void cuda_cascade_set_min_neighbors(CudaCascadeClassifier*, int);
void cuda_cascade_set_max_object_size(CudaCascadeClassifier*, Size2i);
void cuda_cascade_set_min_object_size(CudaCascadeClassifier*, Size2i);
void cuda_cascade_set_scale_factor(CudaCascadeClassifier*, double);

Size2i cuda_cascade_get_classifier_size(CudaCascadeClassifier*);
bool cuda_cascade_get_find_largest_object(CudaCascadeClassifier*);
int cuda_cascade_get_max_num_objects(CudaCascadeClassifier*);
int cuda_cascade_get_min_neighbors(CudaCascadeClassifier*);
Size2i cuda_cascade_get_max_object_size(CudaCascadeClassifier*);
Size2i cuda_cascade_get_min_object_size(CudaCascadeClassifier*);
double cuda_cascade_get_scale_factor(CudaCascadeClassifier*);

}  // namespace cvsys

#endif  // OPENCV_GPU_H_
