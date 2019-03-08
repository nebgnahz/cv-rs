#ifndef CV_RS_IMCODECS_H
#define CV_RS_IMCODECS_H

#include "common.hpp"
#include "utils.hpp"
#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>
#include <stddef.h>
#include <stdint.h>

extern "C" {

cv::Mat* cv_nat_imread(const char* const filename, int flags);
cv::Mat* cv_nat_imdecode(const uint8_t* const buffer, size_t len, int flag);
void cv_nat_imencode(const char* const ext,
                     const cv::Mat* const image,
                     const int* const flag_ptr,
                     size_t flag_size,
                     COption<CVec<uint8_t>>* result);
}

#endif  // CV_RS_IMCODECS_H
