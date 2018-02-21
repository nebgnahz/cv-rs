#include <opencv2/core.hpp>
#include <opencv2/features2d.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/objdetect.hpp>
#include <opencv2/text/ocr.hpp>
#include <opencv2/video/tracking.hpp>
#include <opencv2/xfeatures2d.hpp>

#include "opencv-wrapper.h"
#include "utils.h"

EXTERN_C_BEGIN

// =============================================================================
//   Core
// =============================================================================

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

// =============================================================================
//  Imgproc
// =============================================================================

// =============================================================================
//  Imgcodecs
// =============================================================================

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================

void cv_imshow(const char* const winname, cv::Mat* mat) {
    if (mat != NULL) {
        cv::imshow(winname, *mat);
    }
}

int cv_wait_key(int delay) {
    return cv::waitKey(delay);
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

EXTERN_C_END
