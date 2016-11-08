#include "opencv-wrapper.h"
#include "opencv-gpu.h"
#include "utils.h"
#include <opencv2/cudaobjdetect.hpp>

EXTERN_C_BEGIN

// =============================================================================
//   Basic
// =============================================================================
typedef struct _GpuMat GpuMat;
GpuMat* cv_gpu_mat_default() {
    return reinterpret_cast<GpuMat*>(new cv::cuda::GpuMat());
}

void cv_gpu_mat_drop(GpuMat* gpu_mat) {
    cv::cuda::GpuMat* gpu_image = reinterpret_cast<cv::cuda::GpuMat*>(gpu_mat);
    delete gpu_image;
    gpu_mat = nullptr;
}

void cv_gpu_mat_upload(GpuMat* gpu_mat, CMat* cpu_mat) {
    cv::cuda::GpuMat* gpu_image = reinterpret_cast<cv::cuda::GpuMat*>(gpu_mat);
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cpu_mat);
    gpu_image->upload(*image);
}

CMat* cv_mat_from_gpu_mat(GpuMat* gpu_mat) {
    cv::cuda::GpuMat* gpu_image = reinterpret_cast<cv::cuda::GpuMat*>(gpu_mat);
    return reinterpret_cast<CMat*>(new cv::Mat(*gpu_image));
}

GpuMat* cv_gpu_mat_from_mat(CMat* cmat) {
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cmat);
    return reinterpret_cast<GpuMat*>(new cv::cuda::GpuMat(*image));
}

// =============================================================================
//   Hog
// =============================================================================
using CV_GPU_HOG = cv::Ptr<cv::cuda::HOG>;

GpuHog* cv_gpu_hog_default() {
    return reinterpret_cast<GpuHog*>(new CV_GPU_HOG(cv::cuda::HOG::create()));
}

void cv_gpu_hog_drop(GpuHog* hog) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    delete cv_hog;
    hog = nullptr;
}

void cv_gpu_hog_set_detector(GpuHog* hog, SvmDetector* detector) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    std::vector<float>* cv_detector =
        reinterpret_cast<std::vector<float>*>(detector);
    (*cv_hog)->setSVMDetector(*cv_detector);
}

void cv_gpu_hog_detect(GpuHog* hog, GpuMat* image, VecRect* found) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    cv::cuda::GpuMat* cv_image = reinterpret_cast<cv::cuda::GpuMat*>(image);
    std::vector<cv::Rect> vec_object;
    (*cv_hog)->detectMultiScale(*cv_image, vec_object);
    vec_rect_cxx_to_c(vec_object, found);
}

EXTERN_C_END
