#include <opencv2/core.hpp>
#include <opencv2/features2d.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/objdetect.hpp>
#include <opencv2/video/tracking.hpp>
#include <opencv2/xfeatures2d.hpp>

#include "opencv-wrapper.h"
#include "utils.h"

EXTERN_C_BEGIN

// =============================================================================
//   Core
// =============================================================================
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
    return (new cv::Mat(rows, cols, type, const_cast<void*>(reinterpret_cast<const void*>(buf))));
}

bool cv_mat_is_valid(cv::Mat* mat) {
    return mat->data != NULL;
}

void* cv_mat_roi(cv::Mat* mat, Rect crect) {
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::Mat* dst = new cv::Mat(*mat, rect);
    return (dst);
}

void cv_mat_logic_and(cv::Mat* image, const cv::Mat* const mask) {
    (*image) &= (*mask);
}

void cv_mat_flip(cv::Mat* image, int code) {
    cv::flip(*image, *image, code);
}

void* cv_imread(const char* const filename, int flags) {
    cv::Mat* image = new cv::Mat();
    *image = cv::imread(filename, flags);
    return (image);
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

void cv_vec_drop(CVec<void>* vec, unsigned int depth) {
    if (vec->array != nullptr) {
        if (depth > 1) {
            auto nestedVec = (CVec<void>*) vec->array;
            for (size_t i = 0; i < vec->size; ++i) {
                cv_vec_drop(&nestedVec[i], depth - 1);
            }
        }
        free(vec->array);
        vec->array = nullptr;
        vec->size = 0;
    }
}

void c_drop(void* value) {
    free(value);
    value = nullptr;
}

// =============================================================================
//  core array
// =============================================================================
void cv_in_range(cv::Mat* mat, Scalar lowerb, Scalar upperb, cv::Mat* dst) {
    cv::Scalar lb(lowerb.v0, lowerb.v1, lowerb.v2);
    cv::Scalar ub(upperb.v0, upperb.v1, upperb.v2);
    cv::inRange(*mat, lb, ub, *dst);
}

void cv_min_max_loc(
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

void cv_mix_channels(cv::Mat* src, size_t nsrcs, cv::Mat* dst, size_t ndsts, const int* from_to, size_t npairs) {
    cv::mixChannels(src, nsrcs, dst, ndsts, from_to, npairs);
}

void cv_normalize(cv::Mat* src, cv::Mat* dst, double alpha, double beta, int norm_type) {
    cv::normalize(*src, *dst, alpha, beta, norm_type);
}

void cv_bitwise_and(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst) {
    cv::bitwise_and(*src1, *src2, *dst);
}

void cv_bitwise_not(const cv::Mat* const src, cv::Mat* const dst) {
    cv::bitwise_not(*src, *dst);
}

void cv_bitwise_or(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst) {
    cv::bitwise_or(*src1, *src2, *dst);
}

void cv_bitwise_xor(const cv::Mat* const src1, const cv::Mat* const src2, cv::Mat* dst) {
    cv::bitwise_xor(*src1, *src2, *dst);
}

int cv_count_non_zero(const cv::Mat* const src) {
    return cv::countNonZero(*src);
}

// =============================================================================
//  Imgproc
// =============================================================================
void cv_line(cv::Mat* mat, Point2i pt1, Point2i pt2, Scalar color, int thickness, int linetype, int shift) {
    cv::Point point1(pt1.x, pt1.y);
    cv::Point point2(pt2.x, pt2.y);
    cv::Scalar colour(color.v0, color.v1, color.v2, color.v3);
    cv::line(*mat, point1, point2, colour, thickness, linetype, shift);
}

void cv_rectangle(cv::Mat* mat, Rect crect, Scalar color, int thickness, int linetype) {
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::Scalar colour(color.v0, color.v1, color.v2, color.v3);
    cv::rectangle(*mat, rect, colour, thickness, linetype);
}

void cv_ellipse(cv::Mat* mat,
                Point2i center,
                Size2i axes,
                double angle,
                double start_angle,
                double end_angle,
                Scalar color,
                int thickness,
                int linetype,
                int shift) {
    cv::Point cv_center(center.x, center.y);
    cv::Size cv_axes(axes.width, axes.height);
    cv::Scalar cv_color(color.v0, color.v1, color.v2, color.v3);

    cv::ellipse(*mat, cv_center, cv_axes, angle, start_angle, end_angle, cv_color, thickness, linetype, shift);
}

void cv_cvt_color(cv::Mat* mat, cv::Mat* out, int code) {
    cv::cvtColor(*mat, *out, code);
}

void cv_pyr_down(cv::Mat* mat, cv::Mat* out) {
    cv::pyrDown(*mat, *out);
}

void cv_resize(cv::Mat* from, cv::Mat* to, Size2i dsize, double fx, double fy, int interpolation) {
    cv::Size cv_dsize(dsize.width, dsize.height);
    cv::resize(*from, *to, cv_dsize, fx, fy, interpolation);
}

void cv_calc_hist(const cv::Mat* images,
                  int nimages,
                  const int* channels,
                  cv::Mat* mask,
                  cv::Mat* hist,
                  int dims,
                  const int* hist_size,
                  const float** ranges) {
    cv::calcHist(images, nimages, channels, *mask, *hist, dims, hist_size, ranges);
}

void cv_calc_back_project(const cv::Mat* images,
                          int nimages,
                          const int* channels,
                          cv::Mat* hist,
                          cv::Mat* back_project,
                          const float** ranges) {
    cv::calcBackProject(images, nimages, channels, *hist, *back_project, ranges);
}

// =============================================================================
//  Imgcodecs
// =============================================================================
void* cv_imdecode(const uint8_t* const buffer, size_t len, int flag) {
    cv::Mat* dst = new cv::Mat();
    std::vector<uchar> input(buffer, buffer + len);
    cv::imdecode(cv::Mat(input), flag, dst);
    return (dst);
}

// The caller is responsible for the allocated buffer
ImencodeResult
cv_imencode(const char* const ext, const cv::Mat* const image, const int* const flag_ptr, size_t flag_size) {
    std::vector<uchar> buf;
    std::vector<int> params(flag_ptr, flag_ptr + flag_size);
    bool r = cv::imencode(ext, *image, buf, params);

    int size = buf.size();
    uint8_t* buffer = new uint8_t[size];
    std::copy(buf.begin(), buf.begin() + size, buffer);

    ImencodeResult result;
    result.status = r;
    result.size = size;
    result.buf = buffer;
    return result;
}

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================
void cv_named_window(const char* const winname, int flags) {
    cv::namedWindow(winname, flags);
}

void cv_destroy_window(const char* const winname) {
    cv::destroyWindow(winname);
}

void cv_imshow(const char* const winname, cv::Mat* mat) {
    if (mat != NULL) {
        cv::imshow(winname, *mat);
    }
}

int cv_wait_key(int delay) {
    return cv::waitKey(delay);
}

void cv_set_mouse_callback(const char* const winname, MouseCallback on_mouse, void* userdata) {
    cv::setMouseCallback(winname, on_mouse, userdata);
}

// =============================================================================
//   VideoCapture
// =============================================================================
void* cv_videocapture_new(int index) {
    return new cv::VideoCapture(index);
}

void* cv_videocapture_from_file(const char* const filename) {
    return new cv::VideoCapture(filename);
}

bool cv_videocapture_is_opened(const cv::VideoCapture* const cap) {
    return cap->isOpened();
}

bool cv_videocapture_read(cv::VideoCapture* cap, cv::Mat* mat) {
    return cap->read(*mat);
}

void cv_videocapture_drop(cv::VideoCapture* cap) {
    delete cap;
    cap = nullptr;
}

bool cv_videocapture_set(cv::VideoCapture* cap, int property, double value) {
    return cap->set(property, value);
}

double cv_videocapture_get(cv::VideoCapture* cap, int property) {
    return cap->get(property);
}

// =============================================================================
//   VideoWriter
// =============================================================================
/// http://www.fourcc.org/codecs.php
int cv_fourcc(char c1, char c2, char c3, char c4) {
    return (((c1) &255) + (((c2) &255) << 8) + (((c3) &255) << 16) + (((c4) &255) << 24));
}

void* cv_videowriter_default() {
    return new cv::VideoWriter();
}

void* cv_videowriter_new(const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color) {
    cv::Size cv_frame_size(frame_size.width, frame_size.height);
    cv::VideoWriter* writer = new cv::VideoWriter(path, fourcc, fps, cv_frame_size, is_color);
    return writer;
}

void cv_videowriter_drop(cv::VideoWriter* writer) {
    delete writer;
    writer = nullptr;
}

bool cv_videowriter_open(
    cv::VideoWriter* writer, const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color) {
    cv::Size cv_frame_size(frame_size.width, frame_size.height);
    return writer->open(path, fourcc, fps, cv_frame_size, is_color);
}

bool cv_videowriter_is_opened(cv::VideoWriter* writer) {
    return writer->isOpened();
}

void cv_videowriter_write(cv::VideoWriter* writer, cv::Mat* mat) {
    (*writer) << (*mat);
}

bool cv_videowriter_set(cv::VideoWriter* writer, int property, double value) {
    return writer->set(property, value);
}

double cv_videowriter_get(cv::VideoWriter* writer, int property) {
    return writer->get(property);
}

// =============================================================================
//   CascadeClassifier
// =============================================================================
void* cv_cascade_classifier_new() {
    return new cv::CascadeClassifier();
}

bool cv_cascade_classifier_load(cv::CascadeClassifier* cascade, const char* const p) {
    return cascade->load(p);
}

void* cv_cascade_classifier_from_path(const char* const p) {
    return new cv::CascadeClassifier(p);
}

void cv_cascade_classifier_drop(cv::CascadeClassifier* cascade) {
    delete cascade;
    cascade = nullptr;
}

void cv_cascade_classifier_detect(cv::CascadeClassifier* cascade,
                                  cv::Mat* image,
                                  CVec<Rect>* vec_of_rect,
                                  double scale_factor,
                                  int min_neighbors,
                                  int flags,
                                  Size2i min_size,
                                  Size2i max_size) {
    std::vector<cv::Rect> objects;

    cv::Size cv_min_size(min_size.width, min_size.height);
    cv::Size cv_max_size(max_size.width, max_size.height);
    cascade->detectMultiScale(*image, objects, scale_factor, min_neighbors, flags, cv_min_size, cv_max_size);
    // Move objects to vec_of_rect
    size_t num = objects.size();
    vec_of_rect->array = (Rect*) malloc(num * sizeof(Rect));
    vec_of_rect->size = num;
    for (size_t i = 0; i < num; i++) {
        vec_of_rect->array[i].x = objects[i].x;
        vec_of_rect->array[i].y = objects[i].y;
        vec_of_rect->array[i].width = objects[i].width;
        vec_of_rect->array[i].height = objects[i].height;
    }
}

void* cv_hog_default_people_detector() {
    return new std::vector<float>(cv::HOGDescriptor::getDefaultPeopleDetector());
}

void* cv_hog_daimler_people_detector() {
    return new std::vector<float>(cv::HOGDescriptor::getDaimlerPeopleDetector());
}

void cv_hog_detector_drop(std::vector<float>* detector) {
    delete detector;
    detector = nullptr;
}

void* cv_hog_new() {
    return new cv::HOGDescriptor();
}

void cv_hog_drop(cv::HOGDescriptor* hog) {
    delete hog;
    hog = nullptr;
}

void cv_hog_set_svm_detector(cv::HOGDescriptor* hog, std::vector<float>* detector) {
    hog->setSVMDetector(*detector);
}

void cv_hog_detect(cv::HOGDescriptor* hog,
                   cv::Mat* image,
                   CVec<Rect>* vec_rect,
                   CVec<double>* vec_weight,
                   Size2i win_stride,
                   Size2i padding,
                   double scale,
                   double final_threshold,
                   bool use_means_shift) {
    // convert all types

    std::vector<cv::Rect> objects;
    std::vector<double> weights;
    cv::Size cv_win_stride(win_stride.width, win_stride.height);
    cv::Size cv_padding(padding.width, padding.height);

    // Call the function
    hog->detectMultiScale(
        *image, objects, weights, 0.1, cv_win_stride, cv_padding, scale, final_threshold, use_means_shift);

    // Prepare the results
    cv_to_ffi(objects, vec_rect);
    cv_to_ffi(weights, vec_weight);
}

// =============================================================================
//  Object Tracking
// =============================================================================
void* cv_term_criteria_new(int type, int count, double epsilon) {
    return new cv::TermCriteria(type, count, epsilon);
}

void cv_term_criteria_drop(cv::TermCriteria* criteria) {
    delete criteria;
    criteria = nullptr;
}

RotatedRect cv_camshift(cv::Mat* bp_image, Rect crect, cv::TermCriteria* criteria) {
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::RotatedRect rr = cv::CamShift(*bp_image, rect, *criteria);
    RotatedRect c_rr;
    c_rr.center.x = rr.center.x;
    c_rr.center.y = rr.center.y;
    c_rr.size.width = rr.size.width;
    c_rr.size.height = rr.size.height;
    c_rr.angle = rr.angle;
    return c_rr;
}

// =============================================================================
//   MSER
// =============================================================================

void* cv_mser_new(int delta,
                  int min_area,
                  int max_area,
                  double max_variation,
                  double min_diversity,
                  int max_evolution,
                  double area_threshold,
                  double min_margin,
                  int edge_blur_size) {
    cv::Ptr<cv::MSER> result = cv::MSER::create(delta,
                                                min_area,
                                                max_area,
                                                max_variation,
                                                min_diversity,
                                                max_evolution,
                                                area_threshold,
                                                min_margin,
                                                edge_blur_size);
    return new cv::Ptr<cv::MSER>(result);
}

void cv_mser_drop(cv::Ptr<cv::MSER>* detector) {
    delete detector;
    detector = nullptr;
}

void cv_mser_detect_regions(cv::Ptr<cv::MSER>* detector,
                            cv::Mat* image,
                            CVec<CVec<Point2i>>* msers,
                            CVec<Rect>* bboxes) {
    std::vector<std::vector<cv::Point>> msers_vector;
    std::vector<cv::Rect> bboxes_vector;

    detector->get()->detectRegions(*image, msers_vector, bboxes_vector);

    cv_to_ffi(msers_vector, msers);
    cv_to_ffi(bboxes_vector, bboxes);
}

void cv_mser_detect_and_compute(cv::Ptr<cv::MSER>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    cv_to_ffi(keypoints_vector, keypoints);
}

void* cv_surf_new(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright) {
    auto result = cv::xfeatures2d::SURF::create(hessianThreshold, nOctaves, nOctaveLayers, extended, upright);
    return new cv::Ptr<cv::xfeatures2d::SURF>(result);
}
void cv_surf_drop(cv::Ptr<cv::xfeatures2d::SURF>* detector) {
    delete detector;
    detector = nullptr;
}

void cv_surf_detect_and_compute(cv::Ptr<cv::xfeatures2d::SURF>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    cv_to_ffi(keypoints_vector, keypoints);
}

void* cv_sift_new(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma) {
    auto result = cv::xfeatures2d::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma);
    return new cv::Ptr<cv::xfeatures2d::SIFT>(result);
}
void cv_sift_drop(cv::Ptr<cv::xfeatures2d::SIFT>* detector) {
    delete detector;
    detector = nullptr;
}

void cv_sift_detect_and_compute(cv::Ptr<cv::xfeatures2d::SIFT>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    cv_to_ffi(keypoints_vector, keypoints);
}

void cv_compare_hist(cv::Mat* first_image, cv::Mat* second_image, int method, Result<double>* result) {
    *result = Result<double>::FromFunction(
        [first_image, second_image, method]() { return cv::compareHist(*first_image, *second_image, method); });
}

EXTERN_C_END
