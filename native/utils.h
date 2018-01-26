#ifndef UTILS_H_
#define UTILS_H_

#include <vector>
#include <opencv2/core.hpp>
#include "opencv-wrapper.h"

// =============================================================================
//   Utils
// =============================================================================

void vec_rect_cxx_to_c(const std::vector<cv::Rect>& cxx_vec_rect, VecRect* vr);
void vec_double_cxx_to_c(const std::vector<double>& cxx_vec, VecDouble* v);
void vec_point_cxx_to_c(const std::vector<cv::Point>& cxx_vec_point, VecPoint* vp);
void vec_points_cxx_to_c(const std::vector<std::vector<cv::Point>> &cxx_vec_points, VecPoints* vps);
#endif  // UTILS_H_
