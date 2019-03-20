#include "imgproc.hpp"

namespace cvsys {

void nat_line(cv::Mat* mat, Point2i pt1, Point2i pt2, Scalar color, int thickness, int linetype, int shift) {
    cv::Point point1(pt1.x, pt1.y);
    cv::Point point2(pt2.x, pt2.y);
    cv::Scalar colour(color.v0, color.v1, color.v2, color.v3);
    cv::line(*mat, point1, point2, colour, thickness, linetype, shift);
}

void nat_rectangle(cv::Mat* mat, Rect crect, Scalar color, int thickness, int linetype) {
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::Scalar colour(color.v0, color.v1, color.v2, color.v3);
    cv::rectangle(*mat, rect, colour, thickness, linetype);
}

void nat_ellipse(cv::Mat* mat,
                 Point2i center,
                 Size2i axes,
                 double angle,
                 double start_angle,
                 double end_angle,
                 Scalar color,
                 int thickness,
                 int linetype,
                 int shift) {
    cv::Point native_center(center.x, center.y);
    cv::Size native_axes(axes.width, axes.height);
    cv::Scalar native_color(color.v0, color.v1, color.v2, color.v3);

    cv::ellipse(
        *mat, native_center, native_axes, angle, start_angle, end_angle, native_color, thickness, linetype, shift);
}

void cvt_color(cv::Mat* mat, cv::Mat* out, int code) {
    cv::cvtColor(*mat, *out, code);
}

void pyr_down(cv::Mat* mat, cv::Mat* out) {
    cv::pyrDown(*mat, *out);
}

void nat_threshold(cv::Mat* mat, cv::Mat* out, double thresh, double maxval, int ttype) {
    cv::threshold(*mat, *out, thresh, maxval, ttype);
}

void nat_erode(
    cv::Mat* mat, cv::Mat* out, cv::Mat* kernel, Point2i anchor, int iterations, int borderType, Scalar borderValue) {
    cv::Point pta(anchor.x, anchor.y);
    cv::Scalar bv(borderValue.v0, borderValue.v1, borderValue.v2, borderValue.v3);
    cv::erode(*mat, *out, *kernel, pta, iterations, borderType, bv);
}

void nat_dilate(
    cv::Mat* mat, cv::Mat* out, cv::Mat* kernel, Point2i anchor, int iterations, int borderType, Scalar borderValue) {
    cv::Point pta(anchor.x, anchor.y);
    cv::Scalar bv(borderValue.v0, borderValue.v1, borderValue.v2, borderValue.v3);
    cv::dilate(*mat, *out, *kernel, pta, iterations, borderType, bv);
}

void gaussian_blur(cv::Mat* mat, cv::Mat* out, Size2i ksize, double sigma_x, double sigma_y, int bordertype) {
    cv::Size native_ksize(ksize.width, ksize.height);
    cv::GaussianBlur(*mat, *out, native_ksize, sigma_x, sigma_y, bordertype);
}

void nat_resize(cv::Mat* from, cv::Mat* to, Size2i dsize, double fx, double fy, int interpolation) {
    cv::Size native_dsize(dsize.width, dsize.height);
    cv::resize(*from, *to, native_dsize, fx, fy, interpolation);
}

void calc_hist(const cv::Mat* images,
               int nimages,
               const int* channels,
               cv::Mat* mask,
               cv::Mat* hist,
               int dims,
               const int* hist_size,
               const float* const* ranges) {
    cv::calcHist(images, nimages, channels, *mask, *hist, dims, hist_size, const_cast<const float**>(ranges));
}

void calc_back_project(const cv::Mat* images,
                       int nimages,
                       const int* channels,
                       cv::Mat* hist,
                       cv::Mat* back_project,
                       const float* const* ranges) {
    cv::calcBackProject(images, nimages, channels, *hist, *back_project, const_cast<const float**>(ranges));
}

void compare_hist(cv::Mat* first_image, cv::Mat* second_image, int method, Result<double>* result) {
    if (first_image->size != second_image->size) {
        *result = Result<double>{0.0, CString("compare_hist requires matrices that are the same size")};
        return;
    }
    *result = Result<double>::FromFunction(
        [first_image, second_image, method]() { return cv::compareHist(*first_image, *second_image, method); });
}

void sobel(
    cv::Mat* src, cv::Mat* dst, int ddepth, int dx, int dy, int k_size, double scale, double delta, int borderType) {
    cv::Sobel(*src, *dst, ddepth, dx, dy, k_size, scale, delta, borderType);
}

void scharr(cv::Mat* src, cv::Mat* dst, int ddepth, int dx, int dy, double scale, double delta, int borderType) {
    cv::Scharr(*src, *dst, ddepth, dx, dy, scale, delta, borderType);
}

EmptyResult
canny(cv::Mat* image, cv::Mat* edges, double threshold1, double threshold2, int aperture_size, bool l2_gradient) {
    return EmptyResult::FromFunction([image, edges, threshold1, threshold2, aperture_size, l2_gradient]() {
        cv::Canny(*image, *edges, threshold1, threshold2, aperture_size, l2_gradient);
    });
}

}  // namespace cvsys
