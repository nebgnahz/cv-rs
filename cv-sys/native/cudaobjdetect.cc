#include <opencv2/cudaobjdetect.hpp>

#include "cudaobjdetect.hpp"
#include "utils.hpp"

namespace cvsys {

// =============================================================================
//   Basic
// =============================================================================
cv::cuda::GpuMat* cuda_gpu_mat_default() {
    return new cv::cuda::GpuMat();
}

void cuda_gpu_mat_drop(cv::cuda::GpuMat* gpu_image) {
    delete gpu_image;
    gpu_image = nullptr;
}

void cuda_gpu_mat_upload(cv::cuda::GpuMat* gpu_image, cv::Mat* image) {
    gpu_image->upload(*image);
}

cv::Mat* mat_from_gpu_mat(cv::cuda::GpuMat* gpu_image) {
    return new cv::Mat(*gpu_image);
}

cv::cuda::GpuMat* cuda_gpu_mat_from_mat(cv::Mat* image) {
    return new cv::cuda::GpuMat(*image);
}

// =============================================================================
//   Hog
// =============================================================================
CudaHog* cuda_hog_default() {
    auto hog = cv::cuda::HOG::create();
    return new CudaHog(hog);
}

CudaHog* cuda_hog_new(Size2i win_size, Size2i block_size, Size2i block_stride, Size2i cell_size, int nbins) {
    cv::Size native_win_size(win_size.width, win_size.height);
    cv::Size native_block_size(block_size.width, block_size.height);
    cv::Size native_block_stride(block_stride.width, block_stride.height);
    cv::Size native_cell_size(cell_size.width, cell_size.height);

    auto hog = cv::cuda::HOG::create(native_win_size, native_block_size, native_block_stride, native_cell_size, nbins);
    return new CudaHog(hog);
}

void cuda_hog_drop(CudaHog* hog) {
    delete hog;
}

void cuda_hog_set_detector(CudaHog* hog, SvmDetector* detector) {
    (*hog)->setSVMDetector(*detector);
}

void cuda_hog_detect(CudaHog* hog, cv::cuda::GpuMat* image, CVec<Rect>* found) {
    std::vector<cv::Rect> vec_object;
    (*hog)->detectMultiScale(*image, vec_object);
    to_ffi(vec_object, found);
}

void cuda_hog_detect_with_conf(CudaHog* hog, cv::cuda::GpuMat* image, CVec<Rect>* found, CVec<double>* conf) {
    std::vector<cv::Rect> vec_object;
    std::vector<double> vec_confidences;
    (*hog)->setGroupThreshold(0);
    (*hog)->detectMultiScale(*image, vec_object, &vec_confidences);
    to_ffi(vec_object, found);
    to_ffi(vec_confidences, conf);
}

void cuda_hog_set_gamma_correction(CudaHog* hog, bool gamma) {
    (*hog)->setGammaCorrection(gamma);
}

void cuda_hog_set_group_threshold(CudaHog* hog, int group_threshold) {
    (*hog)->setGroupThreshold(group_threshold);
}

void cuda_hog_set_hit_threshold(CudaHog* hog, double hit_threshold) {
    (*hog)->setHitThreshold(hit_threshold);
}

void cuda_hog_set_l2hys_threshold(CudaHog* hog, double l2hys_threshold) {
    (*hog)->setL2HysThreshold(l2hys_threshold);
}

void cuda_hog_set_num_levels(CudaHog* hog, int num_levels) {
    (*hog)->setNumLevels(num_levels);
}

void cuda_hog_set_scale_factor(CudaHog* hog, double scale_factor) {
    (*hog)->setScaleFactor(scale_factor);
}

void cuda_hog_set_win_sigma(CudaHog* hog, double win_sigma) {
    (*hog)->setWinSigma(win_sigma);
}

void cuda_hog_set_win_stride(CudaHog* hog, Size2i win_stride) {
    cv::Size nat_win_stride(win_stride.width, win_stride.height);
    (*hog)->setWinStride(nat_win_stride);
}

bool cuda_hog_get_gamma_correction(CudaHog* hog) {
    return (*hog)->getGammaCorrection();
}

int cuda_hog_get_group_threshold(CudaHog* hog) {
    return (*hog)->getGroupThreshold();
}

double cuda_hog_get_hit_threshold(CudaHog* hog) {
    return (*hog)->getHitThreshold();
}

double cuda_hog_get_l2hys_threshold(CudaHog* hog) {
    return (*hog)->getL2HysThreshold();
}

int cuda_hog_get_num_levels(CudaHog* hog) {
    return (*hog)->getNumLevels();
}

double cuda_hog_get_scale_factor(CudaHog* hog) {
    return (*hog)->getScaleFactor();
}

double cuda_hog_get_win_sigma(CudaHog* hog) {
    return (*hog)->getWinSigma();
}

Size2i cuda_hog_get_win_stride(CudaHog* hog) {
    cv::Size size = (*hog)->getWinStride();
    Size2i c_size;
    c_size.width = size.width;
    c_size.height = size.height;
    return c_size;
}

// =============================================================================
//   CascadeClassifier
// =============================================================================
void* cuda_cascade_new(const char* const filename) {
    auto cascade = cv::cuda::CascadeClassifier::create(filename);
    return new cv::Ptr<cv::cuda::CascadeClassifier>(cascade);
}

void cuda_cascade_drop(void* v_cascade) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    delete cascade;
    cascade = nullptr;
}

void cuda_cascade_detect(void* v_cascade, cv::cuda::GpuMat* image, CVec<Rect>* objects) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    cv::cuda::GpuMat objbuf;
    std::vector<cv::Rect> vec_object;

    (*cascade)->detectMultiScale(*image, objbuf);
    (*cascade)->convert(objbuf, vec_object);

    to_ffi(vec_object, objects);
}

void cuda_cascade_set_find_largest_object(void* v_cascade, bool value) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    (*cascade)->setFindLargestObject(value);
}

void cuda_cascade_set_max_num_objects(void* v_cascade, int num) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    (*cascade)->setMaxNumObjects(num);
}

void cuda_cascade_set_min_neighbors(void* v_cascade, int min) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    (*cascade)->setMinNeighbors(min);
}

void cuda_cascade_set_max_object_size(void* v_cascade, Size2i max_size) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    cv::Size nat_max_size(max_size.width, max_size.height);
    (*cascade)->setMaxObjectSize(nat_max_size);
}

void cuda_cascade_set_min_object_size(void* v_cascade, Size2i min_size) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    cv::Size nat_min_size(min_size.width, min_size.height);
    (*cascade)->setMinObjectSize(nat_min_size);
}

void cuda_cascade_set_scale_factor(void* v_cascade, double factor) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    (*cascade)->setScaleFactor(factor);
}

Size2i cuda_cascade_get_classifier_size(void* v_cascade) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    cv::Size2i size = (*cascade)->getClassifierSize();
    Size2i c_size = {size.width, size.height};
    return c_size;
}

bool cuda_cascade_get_find_largest_object(void* v_cascade) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    return (*cascade)->getFindLargestObject();
}

int cuda_cascade_get_max_num_objects(void* v_cascade) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    return (*cascade)->getMaxNumObjects();
}

int cuda_cascade_get_min_neighbors(void* v_cascade) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    return (*cascade)->getMinNeighbors();
}

Size2i cuda_cascade_get_max_object_size(void* v_cascade) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    cv::Size2i size = (*cascade)->getMaxObjectSize();
    Size2i c_size = {.width = size.width, .height = size.height};
    return c_size;
}

Size2i cuda_cascade_get_min_object_size(void* v_cascade) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    cv::Size2i size = (*cascade)->getMinObjectSize();
    Size2i c_size = {.width = size.width, .height = size.height};
    return c_size;
}

double cuda_cascade_get_scale_factor(void* v_cascade) {
    cv::Ptr<cv::cuda::CascadeClassifier>* cascade = static_cast<cv::Ptr<cv::cuda::CascadeClassifier>*>(v_cascade);
    return (*cascade)->getScaleFactor();
}

}  // namespace cvsys
