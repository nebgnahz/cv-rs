#include "mat.hpp"

cv::Mat* cvsys_mat_from_file_storage(const char* path, const char* section) {
    auto result = new cv::Mat();
    cv::FileStorage fs(path, cv::FileStorage::READ);
    fs[section] >> *result;
    fs.release();
    return result;
}

cv::Mat* cvsys_mat_new() {
    cv::Mat* image = new cv::Mat();
    return (image);
}

cv::Mat* cvsys_mat_new_with_size(int rows, int cols, int type) {
    return (new cv::Mat(rows, cols, type));
}

cv::Mat* cvsys_mat_zeros(int rows, int cols, int type) {
    cv::Mat* mat = new cv::Mat();
    *mat = cv::Mat::zeros(rows, cols, type);
    return (mat);
}

cv::Mat* cvsys_mat_from_buffer(int rows, int cols, int type, const uint8_t* buf) {
    return new cv::Mat(rows, cols, type, const_cast<void*>(reinterpret_cast<const void*>(buf)));
}

cv::Mat* cvsys_mat_eye(int rows, int cols, int type) {
    auto result = new cv::Mat();
    *result = cv::Mat::eye(rows, cols, type);
    return result;
}

bool cvsys_mat_valid(cv::Mat* mat) {
    return mat->data != NULL;
}

cv::Mat* cvsys_mat_roi(cv::Mat* mat, Rect crect) {
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::Mat* dst = new cv::Mat(*mat, rect);
    return (dst);
}

void cvsys_mat_flip(cv::Mat* image, int code) {
    cv::flip(*image, *image, code);
}

int cvsys_mat_cols(const cv::Mat* const mat) {
    return mat->cols;
}

int cvsys_mat_rows(const cv::Mat* const mat) {
    return mat->rows;
}

int cvsys_mat_depth(const cv::Mat* const mat) {
    return mat->depth();
}

int cvsys_mat_channels(const cv::Mat* const mat) {
    return mat->channels();
}

int cvsys_mat_type(const cv::Mat* const mat) {
    return mat->type();
}

const uint8_t* cvsys_mat_data(const cv::Mat* const mat) {
    return mat->data;
}

size_t cvsys_mat_total(const cv::Mat* const mat) {
    return mat->total();
}

size_t cvsys_mat_elem_size(const cv::Mat* const mat) {
    return mat->elemSize();
}

size_t cvsys_mat_elem_size1(const cv::Mat* const mat) {
    return mat->elemSize1();
}

size_t cvsys_mat_step1(const cv::Mat* const mat, int i) {
    return mat->step1(i);
}

void cvsys_mat_drop(cv::Mat* mat) {
    delete mat;
    mat = nullptr;
}

void cvsys_mat_in_range(cv::Mat* mat, Scalar lowerb, Scalar upperb, cv::Mat* dst) {
    cv::Scalar lb(lowerb.v0, lowerb.v1, lowerb.v2);
    cv::Scalar ub(upperb.v0, upperb.v1, upperb.v2);
    cv::inRange(*mat, lb, ub, *dst);
}

void cvsys_mat_min_max_loc(
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

void cvsys_mat_mix_channels(cv::Mat* src, size_t nsrcs, cv::Mat* dst, size_t ndsts, const int* from_to, size_t npairs) {
    cv::mixChannels(src, nsrcs, dst, ndsts, from_to, npairs);
}

void cvsys_mat_normalize(cv::Mat* src, cv::Mat* dst, double alpha, double beta, int norm_type) {
    cv::normalize(*src, *dst, alpha, beta, norm_type);
}

void cvsys_mat_bitwise_and(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst) {
    cv::bitwise_and(*src1, *src2, *dst);
}

void cvsys_mat_bitwise_not(const cv::Mat* const src, cv::Mat* const dst) {
    cv::bitwise_not(*src, *dst);
}

void cvsys_mat_bitwise_or(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst) {
    cv::bitwise_or(*src1, *src2, *dst);
}

void cvsys_mat_bitwise_xor(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst) {
    cv::bitwise_xor(*src1, *src2, *dst);
}

int cvsys_mat_count_non_zero(const cv::Mat* const src) {
    return cv::countNonZero(*src);
}

void cvsys_mat_copy_make_border(
    const cv::Mat* const src, cv::Mat* const d, int t, int b, int l, int r, int type, Scalar color) {
    cv::Scalar c(color.v0, color.v1, color.v2, color.v3);
    copyMakeBorder(*src, *d, t, b, l, r, type, c);
}
