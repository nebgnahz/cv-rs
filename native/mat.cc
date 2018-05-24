#include "mat.h"

extern "C" {

void* cv_mat_from_file_storage(const char* path, const char* section) {
    auto result = new cv::Mat();
    cv::FileStorage fs(path, cv::FileStorage::READ);
    fs[section] >> *result;
    fs.release();
    return result;
}

void* cv_mat_new() {
    cv::Mat* image = new cv::Mat();
    return (image);
}

void* cv_mat_new_with_size(int rows, int cols, int type) {
    return (new cv::Mat(rows, cols, type));
}

void* cv_mat_zeros(int rows, int cols, int type) {
    cv::Mat* mat = new cv::Mat();
    *mat = cv::Mat::zeros(rows, cols, type);
    return (mat);
}

void* cv_mat_from_buffer(int rows, int cols, int type, const uint8_t* buf) {
    return new cv::Mat(rows, cols, type, const_cast<void*>(reinterpret_cast<const void*>(buf)));
}

void* cv_mat_eye(int rows, int cols, int type) {
    auto result = new cv::Mat();
    *result = cv::Mat::eye(rows, cols, type);
    return result;
}

bool cv_mat_is_valid(cv::Mat* mat) {
    return mat->data != NULL;
}

void* cv_mat_roi(cv::Mat* mat, Rect crect) {
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::Mat* dst = new cv::Mat(*mat, rect);
    return (dst);
}

void cv_mat_flip(cv::Mat* image, int code) {
    cv::flip(*image, *image, code);
}

int cv_mat_cols(const cv::Mat* const mat) {
    return mat->cols;
}

int cv_mat_rows(const cv::Mat* const mat) {
    return mat->rows;
}

int cv_mat_depth(const cv::Mat* const mat) {
    return mat->depth();
}

int cv_mat_channels(const cv::Mat* const mat) {
    return mat->channels();
}

int cv_mat_type(const cv::Mat* const mat) {
    return mat->type();
}

const uint8_t* cv_mat_data(const cv::Mat* const mat) {
    return mat->data;
}

size_t cv_mat_total(const cv::Mat* const mat) {
    return mat->total();
}

size_t cv_mat_elem_size(const cv::Mat* const mat) {
    return mat->elemSize();
}

size_t cv_mat_elem_size1(const cv::Mat* const mat) {
    return mat->elemSize1();
}

size_t cv_mat_step1(const cv::Mat* const mat, int i) {
    return mat->step1(i);
}

void cv_mat_drop(cv::Mat* mat) {
    delete mat;
    mat = nullptr;
}

void cv_mat_in_range(cv::Mat* mat, Scalar lowerb, Scalar upperb, cv::Mat* dst) {
    cv::Scalar lb(lowerb.v0, lowerb.v1, lowerb.v2);
    cv::Scalar ub(upperb.v0, upperb.v1, upperb.v2);
    cv::inRange(*mat, lb, ub, *dst);
}

void cv_mat_min_max_loc(
    const cv::Mat* const mat, double* min, double* max, Point2i* minLoc, Point2i* maxLoc, const cv::Mat* const mask) {
    if (minLoc == NULL && maxLoc == NULL) {
        cv::minMaxLoc(*mat, min, max, NULL, NULL, *mask);
    } else if (minLoc == NULL && maxLoc != NULL) {
        cv::Point maxPoint = cv::Point();
        cv::minMaxLoc(*mat, min, max, NULL, &maxPoint, *mask);
        maxLoc->x = maxPoint.x;
        maxLoc->y = maxPoint.y;
    } else if (minLoc != NULL && maxLoc == NULL) {
        cv::Point minPoint = cv::Point();
        cv::minMaxLoc(*mat, min, max, &minPoint, NULL, *mask);
        minLoc->x = minPoint.x;
        minLoc->y = minPoint.y;
    } else {
        cv::Point minPoint = cv::Point();
        cv::Point maxPoint = cv::Point();
        cv::minMaxLoc(*mat, min, max, &minPoint, &maxPoint, *mask);
        minLoc->x = minPoint.x;
        minLoc->y = minPoint.y;
        maxLoc->x = maxPoint.x;
        maxLoc->y = maxPoint.y;
    }
}

void cv_mat_mix_channels(cv::Mat* src, size_t nsrcs, cv::Mat* dst, size_t ndsts, const int* from_to, size_t npairs) {
    cv::mixChannels(src, nsrcs, dst, ndsts, from_to, npairs);
}

void cv_mat_normalize(cv::Mat* src, cv::Mat* dst, double alpha, double beta, int norm_type) {
    cv::normalize(*src, *dst, alpha, beta, norm_type);
}

void cv_mat_bitwise_and(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst) {
    cv::bitwise_and(*src1, *src2, *dst);
}

void cv_mat_bitwise_not(const cv::Mat* const src, cv::Mat* const dst) {
    cv::bitwise_not(*src, *dst);
}

void cv_mat_bitwise_or(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst) {
    cv::bitwise_or(*src1, *src2, *dst);
}

void cv_mat_bitwise_xor(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst) {
    cv::bitwise_xor(*src1, *src2, *dst);
}

int cv_mat_count_non_zero(const cv::Mat* const src) {
    return cv::countNonZero(*src);
}

void cv_mat_copy_make_border(
    const cv::Mat* const src, cv::Mat* const d, int t, int b, int l, int r, int type, Scalar color) {
    cv::Scalar c(color.v0, color.v1, color.v2, color.v3);
    copyMakeBorder(*src, *d, t, b, l, r, type, c);
}
}
