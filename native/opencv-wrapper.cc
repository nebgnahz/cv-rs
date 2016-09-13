#include "opencv-wrapper.h"
#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/objdetect.hpp>
#include <opencv2/video/tracking.hpp>

EXTERN_C_BEGIN

// =============================================================================
//   Core
// =============================================================================
CMat* opencv_mat_new() {
    cv::Mat* image = new cv::Mat();
    return reinterpret_cast<CMat*>(image);
}

CMat* opencv_mat_new_with_size(int rows, int cols, int type) {
    return reinterpret_cast<CMat*>(new cv::Mat(rows, cols, type));
}

bool opencv_mat_is_valid(CMat* cmat) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    return mat->data != NULL;
}

CMat* opencv_mat_roi(CMat* cmat, CRect crect) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::Mat* dst = new cv::Mat(*mat, rect);
    return reinterpret_cast<CMat*>(dst);
}

void opencv_mat_logic_and(CMat* cimage, const CMat* const cmask) {
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cimage);
    const cv::Mat* mask = reinterpret_cast<const cv::Mat*>(cmask);
    (*image) &= (*mask);
}

void opencv_mat_flip(CMat* cimage, int code) {
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cimage);
    cv::flip(*image, *image, code);
}

CMat* opencv_imread(const char* const filename, int flags) {
    cv::Mat* image = new cv::Mat();
    *image = cv::imread(filename, flags);
    return reinterpret_cast<CMat*>(image);
}

int opencv_mat_cols(const CMat* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->cols;
}

int opencv_mat_rows(const CMat* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->rows;
}

int opencv_mat_depth(const CMat* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->depth();
}

void opencv_mat_drop(CMat* cmat) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
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
//  core array
// =============================================================================
void opencv_in_range(CMat* cmat, CScalar lowerb, CScalar upperb, CMat* cdst) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Scalar lb(lowerb.v0, lowerb.v1, lowerb.v2);
    cv::Scalar ub(upperb.v0, upperb.v1, upperb.v2);
    cv::Mat* dst = reinterpret_cast<cv::Mat*>(cdst);
    cv::inRange(*mat, lb, ub, *dst);
}

void opencv_mix_channels(CMat* cmat, size_t nsrcs, CMat* dst, size_t ndsts,
                         const int* from_to, size_t npairs) {
    cv::Mat* from = reinterpret_cast<cv::Mat*>(cmat);
    cv::Mat* to = reinterpret_cast<cv::Mat*>(dst);
    cv::mixChannels(from, nsrcs, to, ndsts, from_to, npairs);
}

void opencv_normalize(CMat* csrc, CMat* cdst, double alpha, double beta,
                      int norm_type) {
    cv::Mat* src = reinterpret_cast<cv::Mat*>(csrc);
    cv::Mat* dst = reinterpret_cast<cv::Mat*>(cdst);
    cv::normalize(*src, *dst, alpha, beta, norm_type);
}

// =============================================================================
//  Imgproc
// =============================================================================
void opencv_rectangle(CMat* cmat, CRect crect) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::rectangle(*mat, rect, cv::Scalar(255, 0, 0, 255));
}

void opencv_cvt_color(CMat* cmat, CMat* output, int code) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Mat* out = reinterpret_cast<cv::Mat*>(output);
    cv::cvtColor(*mat, *out, code);
}

void opencv_calc_hist(const CMat* cimages, int nimages,
                      const int* channels, CMat* cmask, CMat* chist, int dims,
                      const int* hist_size, const float** ranges) {
    const cv::Mat* images = reinterpret_cast<const cv::Mat*>(cimages);
    cv::Mat* mask = reinterpret_cast<cv::Mat*>(cmask);
    cv::Mat* hist = reinterpret_cast<cv::Mat*>(chist);
    cv::calcHist(images, nimages, channels, *mask, *hist, dims, hist_size,
                 ranges);
}

void opencv_calc_back_project(const CMat* cimages, int nimages,
                              const int* channels, CMat* chist,
                              CMat* cback_project, const float** ranges) {
    const cv::Mat* images = reinterpret_cast<const cv::Mat*>(cimages);
    cv::Mat* hist = reinterpret_cast<cv::Mat*>(chist);
    cv::Mat* back_project = reinterpret_cast<cv::Mat*>(cback_project);
    cv::calcBackProject(images, nimages, channels, *hist, *back_project,
                        ranges);
}

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================
void opencv_named_window(const char* const winname, int flags) {
    cv::namedWindow(winname, flags);
}

void opencv_destroy_window(const char* const winname) {
    cv::destroyWindow(winname);
}

void opencv_imshow(const char* const winname, CMat* cmat) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    if (mat != NULL) {
        cv::imshow(winname, *mat);
    }
}

int opencv_wait_key(int delay) {
    return cv::waitKey(delay);
}

void opencv_set_mouse_callback(const char* const winname,
                               MouseCallback on_mouse,
                               void* userdata) {
    cv::setMouseCallback(winname, on_mouse, userdata);
}

// =============================================================================
//   VideoCapture
// =============================================================================
CVideoCapture* opencv_videocapture_new(int index) {
    cv::VideoCapture* cap = new cv::VideoCapture(index);
    return reinterpret_cast<CVideoCapture*>(cap);
}

bool opencv_videocapture_is_opened(const CVideoCapture* const ccap) {
    const cv::VideoCapture* const cap =
        reinterpret_cast<const cv::VideoCapture* const>(ccap);
    return cap->isOpened();
}

bool opencv_videocapture_read(CVideoCapture* ccap, CMat* cmat) {
    cv::VideoCapture* cap = reinterpret_cast<cv::VideoCapture*>(ccap);
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    return cap->read(*mat);
}

void opencv_videocapture_drop(CVideoCapture* ccap) {
    cv::VideoCapture* cap = reinterpret_cast<cv::VideoCapture*>(ccap);
    delete cap;
    ccap = nullptr;
}

// =============================================================================
//   CascadeClassifier
// =============================================================================
CCascadeClassifier* opencv_cascade_classifier_new() {
    cv::CascadeClassifier* cc = new cv::CascadeClassifier();
    return reinterpret_cast<CCascadeClassifier*>(cc);
}

bool opencv_cascade_classifier_load(CCascadeClassifier* cc,
                                    const char* const p) {
    cv::CascadeClassifier* cascade = reinterpret_cast<cv::CascadeClassifier*>(cc);
    return cascade->load(p);
}

CCascadeClassifier* opencv_cascade_classifier_from_path(const char* const p) {
    cv::CascadeClassifier* cc = new cv::CascadeClassifier(p);
    return reinterpret_cast<CCascadeClassifier*>(cc);
}

void opencv_cascade_classifier_drop(CCascadeClassifier* cc) {
    cv::CascadeClassifier* cascade = reinterpret_cast<cv::CascadeClassifier*>(cc);
    delete cascade;
    cc = nullptr;
}

void opencv_cascade_classifier_detect(CCascadeClassifier* cc, CMat* cmat,
                                      CVecOfRect* vec_of_rect,
                                      double scale_factor, int min_neighbors,
                                      int flags, CSize2i min_size,
                                      CSize2i max_size) {
    cv::CascadeClassifier* cascade = reinterpret_cast<cv::CascadeClassifier*>(cc);
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cmat);
    std::vector<cv::Rect> objects;

    cv::Size cv_min_size(min_size.width, min_size.height);
    cv::Size cv_max_size(max_size.width, max_size.height);
    cascade->detectMultiScale(*image, objects, scale_factor, min_neighbors,
                              flags, cv_min_size, cv_max_size);
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

// =============================================================================
//  Object Tracking
// =============================================================================
CTermCriteria* opencv_term_criteria_new(int type, int count, double epsilon) {
    cv::TermCriteria* criteria = new cv::TermCriteria(type, count, epsilon);
    return reinterpret_cast<CTermCriteria*>(criteria);
}

void opencv_term_criteria_drop(CTermCriteria* c_criteria) {
    cv::TermCriteria* criteria = reinterpret_cast<cv::TermCriteria*>(c_criteria);
    delete criteria;
    c_criteria = nullptr;
}

CRotatedRect opencv_camshift(CMat* c_bp_image, CRect crect,
                             CTermCriteria* c_criteria) {
    cv::Mat* bp_image = reinterpret_cast<cv::Mat*>(c_bp_image);
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::TermCriteria* criteria = reinterpret_cast<cv::TermCriteria*>(c_criteria);
    cv::RotatedRect rr = cv::CamShift(*bp_image, rect, *criteria);
    CRotatedRect c_rr;
    c_rr.center.x = rr.center.x;
    c_rr.center.y = rr.center.y;
    c_rr.size.width = rr.size.width;
    c_rr.size.height = rr.size.height;
    c_rr.angle = rr.angle;
    return c_rr;
}



EXTERN_C_END
