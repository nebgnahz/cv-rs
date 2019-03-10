#ifndef CV_RS_MAT_H
#define CV_RS_MAT_H

#include "../common.hpp"
#include <opencv2/core.hpp>
#include <stddef.h>
#include <stdint.h>

namespace cvsys {

cv::Mat* mat_from_file_storage(const char* path, const char* section);
cv::Mat* mat_new();
cv::Mat* mat_new_with_size(int rows, int cols, int type);
cv::Mat* mat_zeros(int rows, int cols, int type);
cv::Mat* mat_from_buffer(int rows, int cols, int type, const uint8_t* buf);
cv::Mat* mat_eye(int rows, int cols, int type);
bool mat_valid(cv::Mat* mat);
cv::Mat* mat_roi(cv::Mat* mat, Rect crect);
void mat_flip(cv::Mat* image, int code);
int mat_rows(const cv::Mat* const mat);
int mat_cols(const cv::Mat* const mat);
int mat_depth(const cv::Mat* const mat);
int mat_channels(const cv::Mat* const mat);
int mat_type(const cv::Mat* const mat);
const uint8_t* mat_data(const cv::Mat* const mat);
size_t mat_total(const cv::Mat* const mat);
size_t mat_elem_size(const cv::Mat* const mat);
size_t mat_elem_size1(const cv::Mat* const mat);
size_t mat_step1(const cv::Mat* const mat, int i);
void mat_drop(cv::Mat* mat);
void mat_in_range(cv::Mat* mat, Scalar lowerb, Scalar upperb, cv::Mat* dst);
void mat_min_max_loc(
    const cv::Mat* const mat, double* min, double* max, Point2i* minLoc, Point2i* maxLoc, const cv::Mat* const cmask);
void mat_mix_channels(cv::Mat* mat, size_t nsrcs, cv::Mat* dst, size_t ndsts, const int* from_to, size_t npairs);
void mat_normalize(cv::Mat* csrc, cv::Mat* cdst, double alpha, double beta, int norm_type);
void mat_bitwise_and(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst);
void mat_bitwise_not(const cv::Mat* const src, cv::Mat* const dst);
void mat_bitwise_or(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst);
void mat_bitwise_xor(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst);
int mat_count_non_zero(const cv::Mat* const src);
void mat_copy_make_border(const cv::Mat* const src1,
                          cv::Mat* const dst,
                          int top,
                          int bottom,
                          int left,
                          int right,
                          int borderType,
                          Scalar value);

}  // namespace cvsys

#endif  // CV_RS_MAT_H
