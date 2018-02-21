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

// =============================================================================
//   VideoCapture
// =============================================================================

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

// =============================================================================
//   MSER
// =============================================================================

EXTERN_C_END
