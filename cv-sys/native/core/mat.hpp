#ifndef CV_RS_MAT_H
#define CV_RS_MAT_H

#include "../common.hpp"
#include <opencv2/core.hpp>
#include <stddef.h>
#include <stdint.h>

cv::Mat* cvsys_mat_from_file_storage(const char* path, const char* section);
cv::Mat* cvsys_mat_new();
cv::Mat* cvsys_mat_new_with_size(int rows, int cols, int type);
cv::Mat* cvsys_mat_zeros(int rows, int cols, int type);
cv::Mat* cvsys_mat_from_buffer(int rows, int cols, int type, const uint8_t* buf);
cv::Mat* cvsys_mat_eye(int rows, int cols, int type);
bool cvsys_mat_valid(cv::Mat* mat);
cv::Mat* cvsys_mat_roi(cv::Mat* mat, Rect crect);
void cvsys_mat_flip(cv::Mat* image, int code);
int cvsys_mat_rows(const cv::Mat* const mat);
int cvsys_mat_cols(const cv::Mat* const mat);
int cvsys_mat_depth(const cv::Mat* const mat);
int cvsys_mat_channels(const cv::Mat* const mat);
int cvsys_mat_type(const cv::Mat* const mat);
const uint8_t* cvsys_mat_data(const cv::Mat* const mat);
size_t cvsys_mat_total(const cv::Mat* const mat);
size_t cvsys_mat_elem_size(const cv::Mat* const mat);
size_t cvsys_mat_elem_size1(const cv::Mat* const mat);
size_t cvsys_mat_step1(const cv::Mat* const mat, int i);
void cvsys_mat_drop(cv::Mat* mat);
void cvsys_mat_in_range(cv::Mat* mat, Scalar lowerb, Scalar upperb, cv::Mat* dst);
void cvsys_mat_min_max_loc(
    const cv::Mat* const mat, double* min, double* max, Point2i* minLoc, Point2i* maxLoc, const cv::Mat* const cmask);
void cvsys_mat_mix_channels(cv::Mat* mat, size_t nsrcs, cv::Mat* dst, size_t ndsts, const int* from_to, size_t npairs);
void cvsys_mat_normalize(cv::Mat* csrc, cv::Mat* cdst, double alpha, double beta, int norm_type);
void cvsys_mat_bitwise_and(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst);
void cvsys_mat_bitwise_not(const cv::Mat* const src, cv::Mat* const dst);
void cvsys_mat_bitwise_or(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst);
void cvsys_mat_bitwise_xor(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst);
int cvsys_mat_count_non_zero(const cv::Mat* const src);
void cvsys_mat_copy_make_border(const cv::Mat* const src1,
                                cv::Mat* const dst,
                                int top,
                                int bottom,
                                int left,
                                int right,
                                int borderType,
                                Scalar value);

#endif  // CV_RS_MAT_H
