#include "opencv-wrapper.h"
#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/objdetect.hpp>

EXTERN_C_BEGIN

// =============================================================================
//   Core
// =============================================================================
CMat* opencv_mat_new() {
    cv::Mat* image = new cv::Mat();
    return static_cast<CMat*>(image);
}

bool opencv_mat_is_valid(CMat* cmat) {
    cv::Mat* mat = static_cast<cv::Mat*>(cmat);
    return mat->data != NULL;
}

CMat* opencv_imread(const char* const filename, int flags) {
    cv::Mat* image = new cv::Mat();
    *image = cv::imread(filename, flags);
    return static_cast<CMat*>(image);
}

void opencv_mat_drop(CMat* cmat) {
    cv::Mat* mat = static_cast<cv::Mat*>(cmat);
    delete mat;
    cmat = nullptr;
}

void opencv_vec_of_rect_drop(CVecOfRect* v) {
    if (v->array != nullptr) {
        free(v->array);
        v->array = nullptr;
        v->size = 0;
    }
}

// =============================================================================
//  Imgproc
// =============================================================================
void opencv_rectangle(CMat* cmat, CRect crect) {
    cv::Mat* mat = static_cast<cv::Mat*>(cmat);
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::rectangle(*mat, rect, cv::Scalar(255, 0, 0, 255));
}

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================
void opencv_named_window(const char* const winname, int flags) {
    cv::namedWindow(winname, flags);
}

void opencv_imshow(const char* const winname, CMat* cmat) {
    cv::Mat* mat = static_cast<cv::Mat*>(cmat);
    if (mat != NULL) {
        cv::imshow(winname, *mat);
    }
}

int opencv_wait_key(int delay) {
    return cv::waitKey(delay);
}

// =============================================================================
//   VideoCapture
// =============================================================================
CVideoCapture* opencv_videocapture_new(int index) {
    cv::VideoCapture* cap = new cv::VideoCapture(index);
    return static_cast<CVideoCapture*>(cap);
}

bool opencv_videocapture_is_opened(const CVideoCapture* const ccap) {
    const cv::VideoCapture* const cap =
        static_cast<const cv::VideoCapture* const>(ccap);
    return cap->isOpened();
}

bool opencv_videocapture_read(CVideoCapture* ccap, CMat* cmat) {
    cv::VideoCapture* cap = static_cast<cv::VideoCapture*>(ccap);
    cv::Mat* mat = static_cast<cv::Mat*>(cmat);
    return cap->read(*mat);
}

void opencv_videocapture_drop(CVideoCapture* ccap) {
    cv::VideoCapture* cap = static_cast<cv::VideoCapture*>(ccap);
    delete cap;
    ccap = nullptr;
}

// =============================================================================
//   CascadeClassifier
// =============================================================================
CCascadeClassifier* opencv_cascade_classifier_new() {
    cv::CascadeClassifier* cc = new cv::CascadeClassifier();
    return static_cast<CCascadeClassifier*>(cc);
}

CCascadeClassifier* opencv_cascade_classifier_from_path(const char* const p) {
    cv::CascadeClassifier* cc = new cv::CascadeClassifier(p);
    return static_cast<CCascadeClassifier*>(cc);
}

void opencv_cascade_classifier_drop(CCascadeClassifier* cc) {
    cv::CascadeClassifier* cascade = static_cast<cv::CascadeClassifier*>(cc);
    delete cascade;
    cc = nullptr;
}

void opencv_cascade_classifier_detect(CCascadeClassifier* cc, CMat* cmat,
                                      CVecOfRect* vec_of_rect) {
    cv::CascadeClassifier* cascade = static_cast<cv::CascadeClassifier*>(cc);
    cv::Mat* image = static_cast<cv::Mat*>(cmat);
    std::vector<cv::Rect> objects;
    cascade->detectMultiScale(*image, objects);
    // Move objects to vec_of_rect
    size_t num = objects.size();
    vec_of_rect->array = (CRect*) malloc(num * sizeof(CRect));
    vec_of_rect->size = num;
    for (size_t i = 0; i < num; i++) {
        vec_of_rect->array[i].x = objects[i].x;
        vec_of_rect->array[i].y = objects[i].y;
        vec_of_rect->array[i].width = objects[i].width;
        vec_of_rect->array[i].height = objects[i].height;
    }
}

EXTERN_C_END
