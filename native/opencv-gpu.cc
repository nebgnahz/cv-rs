#include "opencv-gpu.h"
#include "opencv-wrapper.h"
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

void cv_gpu_mat_upload(GpuMat* gpu_mat, CvMat* cpu_mat) {
    cv::cuda::GpuMat* gpu_image = reinterpret_cast<cv::cuda::GpuMat*>(gpu_mat);
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cpu_mat);
    gpu_image->upload(*image);
}

CvMat* cv_mat_from_gpu_mat(GpuMat* gpu_mat) {
    cv::cuda::GpuMat* gpu_image = reinterpret_cast<cv::cuda::GpuMat*>(gpu_mat);
    return reinterpret_cast<CvMat*>(new cv::Mat(*gpu_image));
}

GpuMat* cv_gpu_mat_from_mat(CvMat* cmat) {
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cmat);
    return reinterpret_cast<GpuMat*>(new cv::cuda::GpuMat(*image));
}

// =============================================================================
//   Hog
// =============================================================================
using CV_GPU_HOG = cv::Ptr<cv::cuda::HOG>;

GpuHog* cv_gpu_hog_default() {
    auto hog = cv::cuda::HOG::create();
    return reinterpret_cast<GpuHog*>(new CV_GPU_HOG(cv::cuda::HOG::create()));
}

GpuHog* cv_gpu_hog_new(CSize2i win_size, CSize2i block_size,
                       CSize2i block_stride, CSize2i cell_size, int32_t nbins) {
    cv::Size cv_win_size(win_size.width, win_size.height);
    cv::Size cv_block_size(block_size.width, block_size.height);
    cv::Size cv_block_stride(block_stride.width, block_stride.height);
    cv::Size cv_cell_size(cell_size.width, cell_size.height);

    return reinterpret_cast<GpuHog*>(new CV_GPU_HOG(cv::cuda::HOG::create(
        cv_win_size, cv_block_size, cv_block_stride, cv_cell_size, nbins)));
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

void cv_gpu_hog_set_gamma_correction(GpuHog* hog, bool gamma) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    (*cv_hog)->setGammaCorrection(gamma);
}

void cv_gpu_hog_set_group_threshold(GpuHog* hog, int32_t group_threshold) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    (*cv_hog)->setGroupThreshold(group_threshold);
}

void cv_gpu_hog_set_hit_threshold(GpuHog* hog, double hit_threshold) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    (*cv_hog)->setHitThreshold(hit_threshold);
}

void cv_gpu_hog_set_l2hys_threshold(GpuHog* hog, double l2hys_threshold) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    (*cv_hog)->setL2HysThreshold(l2hys_threshold);
}

void cv_gpu_hog_set_num_levels(GpuHog* hog, size_t num_levels) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    (*cv_hog)->setNumLevels(num_levels);
}

void cv_gpu_hog_set_scale_factor(GpuHog* hog, double scale_factor) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    (*cv_hog)->setScaleFactor(scale_factor);
}

void cv_gpu_hog_set_win_sigma(GpuHog* hog, double win_sigma) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    (*cv_hog)->setWinSigma(win_sigma);
}

void cv_gpu_hog_set_win_stride(GpuHog* hog, CSize2i win_stride) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    cv::Size cv_win_stride(win_stride.width, win_stride.height);
    (*cv_hog)->setWinStride(cv_win_stride);
}

bool cv_gpu_hog_get_gamma_correction(GpuHog* hog) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    return (*cv_hog)->getGammaCorrection();
}

int32_t cv_gpu_hog_get_group_threshold(GpuHog* hog) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    return (*cv_hog)->getGroupThreshold();
}

double cv_gpu_hog_get_hit_threshold(GpuHog* hog) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    return (*cv_hog)->getHitThreshold();
}

double cv_gpu_hog_get_l2hys_threshold(GpuHog* hog) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    return (*cv_hog)->getL2HysThreshold();
}

size_t cv_gpu_hog_get_num_levels(GpuHog* hog) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    return (*cv_hog)->getNumLevels();
}

double cv_gpu_hog_get_scale_factor(GpuHog* hog) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    return (*cv_hog)->getScaleFactor();
}

double cv_gpu_hog_get_win_sigma(GpuHog* hog) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    return (*cv_hog)->getWinSigma();
}

CSize2i cv_gpu_hog_get_win_stride(GpuHog* hog) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    cv::Size size = (*cv_hog)->getWinStride();
    CSize2i c_size;
    c_size.width = size.width;
    c_size.height = size.height;
    return c_size;
}

EXTERN_C_END
