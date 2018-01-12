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

void cv_gpu_mat_upload(GpuMat* gpu_mat, CvMatrix* cpu_mat) {
    cv::cuda::GpuMat* gpu_image = reinterpret_cast<cv::cuda::GpuMat*>(gpu_mat);
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cpu_mat);
    gpu_image->upload(*image);
}

CvMatrix* cv_mat_from_gpu_mat(GpuMat* gpu_mat) {
    cv::cuda::GpuMat* gpu_image = reinterpret_cast<cv::cuda::GpuMat*>(gpu_mat);
    return reinterpret_cast<CvMatrix*>(new cv::Mat(*gpu_image));
}

GpuMat *cv_gpu_mat_from_mat(CvMatrix *cmat) {
  cv::Mat *image = reinterpret_cast<cv::Mat *>(cmat);
  return reinterpret_cast<GpuMat *>(new cv::cuda::GpuMat(*image));
}

// =============================================================================
//   Hog
// =============================================================================
using CV_GPU_HOG = cv::Ptr<cv::cuda::HOG>;

GpuHog* cv_gpu_hog_default() {
    auto hog = cv::cuda::HOG::create();
    return reinterpret_cast<GpuHog*>(new CV_GPU_HOG(hog));
}

GpuHog* cv_gpu_hog_new(Size2i win_size, Size2i block_size,
                       Size2i block_stride, Size2i cell_size, int32_t nbins) {
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

void cv_gpu_hog_detect_with_conf(GpuHog* hog, GpuMat* image, VecRect* found, VecDouble* conf) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    cv::cuda::GpuMat* cv_image = reinterpret_cast<cv::cuda::GpuMat*>(image);
    std::vector<cv::Rect> vec_object;
    std::vector<double> vec_confidences;
    (*cv_hog)->setGroupThreshold(0);
    (*cv_hog)->detectMultiScale(*cv_image, vec_object, &vec_confidences);
    vec_rect_cxx_to_c(vec_object, found);
    vec_double_cxx_to_c(vec_confidences, conf);
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

void cv_gpu_hog_set_win_stride(GpuHog* hog, Size2i win_stride) {
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

Size2i cv_gpu_hog_get_win_stride(GpuHog* hog) {
    CV_GPU_HOG* cv_hog = reinterpret_cast<CV_GPU_HOG*>(hog);
    cv::Size size = (*cv_hog)->getWinStride();
    Size2i c_size;
    c_size.width = size.width;
    c_size.height = size.height;
    return c_size;
}


// =============================================================================
//   CascadeClassifier
// =============================================================================
using GpuCascadePtr = cv::Ptr<cv::cuda::CascadeClassifier>;

GpuCascade* cv_gpu_cascade_new(const char* const filename) {
    auto cascade = cv::cuda::CascadeClassifier::create(filename);
    return reinterpret_cast<GpuCascade*>(new GpuCascadePtr(cascade));
}

void cv_gpu_cascade_drop(GpuCascade* cascade) {
    GpuCascadePtr* cascade_ptr = reinterpret_cast<GpuCascadePtr*>(cascade);
    delete cascade_ptr;
    cascade_ptr = nullptr;
}

void cv_gpu_cascade_detect(GpuCascade* cascade, GpuMat* image, VecRect* objects) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    cv::cuda::GpuMat* cv_image = reinterpret_cast<cv::cuda::GpuMat*>(image);
    cv::cuda::GpuMat objbuf;
    std::vector<cv::Rect> vec_object;

    (*cv_cascade)->detectMultiScale(*cv_image, objbuf);
    (*cv_cascade)->convert(objbuf, vec_object);

    vec_rect_cxx_to_c(vec_object, objects);
}

void cv_gpu_cascade_set_find_largest_object(GpuCascade* cascade, bool value) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    (*cv_cascade)->setFindLargestObject(value);
}

void cv_gpu_cascade_set_max_num_objects(GpuCascade* cascade, int32_t num) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    (*cv_cascade)->setMaxNumObjects(num);
}

void cv_gpu_cascade_set_min_neighbors(GpuCascade* cascade, int32_t min) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    (*cv_cascade)->setMinNeighbors(min);
}

void cv_gpu_cascade_set_max_object_size(GpuCascade* cascade, Size2i max_size) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    cv::Size cv_max_size(max_size.width, max_size.height);
    (*cv_cascade)->setMaxObjectSize(cv_max_size);
}

void cv_gpu_cascade_set_min_object_size(GpuCascade* cascade, Size2i min_size) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    cv::Size cv_min_size(min_size.width, min_size.height);
    (*cv_cascade)->setMinObjectSize(cv_min_size);
}

void cv_gpu_cascade_set_scale_factor(GpuCascade* cascade, double factor) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    (*cv_cascade)->setScaleFactor(factor);
}

Size2i cv_gpu_cascade_get_classifier_size(GpuCascade* cascade) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    cv::Size2i size = (*cv_cascade)->getClassifierSize();
    Size2i c_size = {.width = size.width, .height = size.height };
    return c_size;
}

bool cv_gpu_cascade_get_find_largest_object(GpuCascade* cascade) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    return (*cv_cascade)->getFindLargestObject();
}

int32_t cv_gpu_cascade_get_max_num_objects(GpuCascade* cascade) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    return (*cv_cascade)->getMaxNumObjects();
}

int32_t cv_gpu_cascade_get_min_neighbors(GpuCascade* cascade) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    return (*cv_cascade)->getMinNeighbors();
}

Size2i cv_gpu_cascade_get_max_object_size(GpuCascade* cascade) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    cv::Size2i size = (*cv_cascade)->getMaxObjectSize();
    Size2i c_size = {.width = size.width, .height = size.height};
    return c_size;
}

Size2i cv_gpu_cascade_get_min_object_size(GpuCascade* cascade) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    cv::Size2i size = (*cv_cascade)->getMinObjectSize();
    Size2i c_size = {.width = size.width, .height = size.height};
    return c_size;
}

double cv_gpu_cascade_get_scale_factor(GpuCascade* cascade) {
    GpuCascadePtr* cv_cascade = reinterpret_cast<GpuCascadePtr*>(cascade);
    return (*cv_cascade)->getScaleFactor();
}

EXTERN_C_END
