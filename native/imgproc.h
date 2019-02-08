#ifndef CV_RS_IMGPROC_H
#define CV_RS_IMGPROC_H

#include "common.h"
#include <opencv2/core.hpp>
#include <opencv2/imgproc.hpp>

extern "C" {

void cv_line(cv::Mat* mat, Point2i pt1, Point2i pt2, Scalar color, int thickness, int linetype, int shift);
void cv_rectangle(cv::Mat* mat, Rect crect, Scalar color, int thickness, int linetype);
void cv_ellipse(cv::Mat* mat,
                Point2i center,
                Size2i axes,
                double angle,
                double start_angle,
                double end_angle,
                Scalar color,
                int thickness,
                int linetype,
                int shift);

void cv_cvt_color(cv::Mat* mat, cv::Mat* output, int code);
void cv_pyr_down(cv::Mat* mat, cv::Mat* output);
void cv_threshold(cv::Mat* mat, cv::Mat* out, double thresh, double maxval, int ttype);
void cv_erode(
    cv::Mat* mat, cv::Mat* out, cv::Mat* kernel, Point2i anchor, int iterations, int borderType, Scalar borderValue);
void cv_dilate(
    cv::Mat* mat, cv::Mat* out, cv::Mat* kernel, Point2i anchor, int iterations, int borderType, Scalar borderValue);
void cv_gaussian_blur(cv::Mat* mat, cv::Mat* out, Size2i ksize, double sigmaX, double sigmaY, int bordertype);
void cv_resize(cv::Mat* from, cv::Mat* to, Size2i dsize, double fx, double fy, int interpolation);
void cv_calc_hist(const cv::Mat* const cimages,
                  int nimages,
                  const int* channels,
                  cv::Mat* mask,
                  cv::Mat* hist,
                  int dims,
                  const int* hist_size,
                  const float** ranges);
void cv_calc_back_project(const cv::Mat* images,
                          int nimages,
                          const int* channels,
                          cv::Mat* hist,
                          cv::Mat* back_project,
                          const float** ranges);
void cv_compare_hist(cv::Mat* first_image, cv::Mat* second_image, int method, Result<double>* result);
EmptyResult
cv_canny(cv::Mat* image, cv::Mat* edges, double threshold1, double threshold2, int aperture_size, bool l2_gradient);
}

#endif  // CV_RS_IMGPROC_H
