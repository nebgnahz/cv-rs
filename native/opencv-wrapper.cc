#include "opencv-wrapper.h"
#include "utils.h"

#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/objdetect.hpp>
#include <opencv2/video/tracking.hpp>

EXTERN_C_BEGIN

// =============================================================================
//   Core
// =============================================================================
CvMatrix* cv_mat_new() {
    cv::Mat* image = new cv::Mat();
    return reinterpret_cast<CvMatrix*>(image);
}

CvMatrix* cv_mat_new_with_size(int rows, int cols, int type) {
    return reinterpret_cast<CvMatrix*>(new cv::Mat(rows, cols, type));
}

CvMatrix* cv_mat_zeros(int rows, int cols, int type) {
    cv::Mat* mat = new cv::Mat();
    *mat = cv::Mat::zeros(rows, cols, type);
    return reinterpret_cast<CvMatrix*>(mat);
}

CvMatrix* cv_mat_from_buffer(int rows, int cols, int type, const uint8_t* buf) {
    return reinterpret_cast<CvMatrix*>(
        new cv::Mat(rows, cols, type,
                    const_cast<void*>(reinterpret_cast<const void*>(buf))));
}

bool cv_mat_is_valid(CvMatrix* cmat) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    return mat->data != NULL;
}

CvMatrix* cv_mat_roi(CvMatrix* cmat, Rect crect) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::Mat* dst = new cv::Mat(*mat, rect);
    return reinterpret_cast<CvMatrix*>(dst);
}

void cv_mat_logic_and(CvMatrix* cimage, const CvMatrix* const cmask) {
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cimage);
    const cv::Mat* mask = reinterpret_cast<const cv::Mat*>(cmask);
    (*image) &= (*mask);
}

void cv_mat_flip(CvMatrix* cimage, int code) {
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cimage);
    cv::flip(*image, *image, code);
}

CvMatrix* cv_imread(const char* const filename, int flags) {
    cv::Mat* image = new cv::Mat();
    *image = cv::imread(filename, flags);
    return reinterpret_cast<CvMatrix*>(image);
}

int cv_mat_cols(const CvMatrix* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->cols;
}

int cv_mat_rows(const CvMatrix* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->rows;
}

int cv_mat_depth(const CvMatrix* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->depth();
}

int cv_mat_channels(const CvMatrix* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->channels();
}

int cv_mat_type(const CvMatrix* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->type();
}

const uint8_t* cv_mat_data(const CvMatrix* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->data;
}

size_t cv_mat_total(const CvMatrix* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->total();
}

size_t cv_mat_elem_size(const CvMatrix* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->elemSize();
}

size_t cv_mat_elem_size1(const CvMatrix* const cmat) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->elemSize1();
}

size_t cv_mat_step1(const CvMatrix* const cmat, int i) {
    return (reinterpret_cast<const cv::Mat* const>(cmat))->step1(i);
}

void cv_mat_drop(CvMatrix* cmat) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    delete mat;
    cmat = nullptr;
}

void cv_vec_of_rect_drop(VecRect* v) {
    if (v->array != nullptr) {
        free(v->array);
        v->array = nullptr;
        v->size = 0;
    }
}

// =============================================================================
//  core array
// =============================================================================
void cv_in_range(CvMatrix* cmat, Scalar lowerb, Scalar upperb, CvMatrix* cdst) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Scalar lb(lowerb.v0, lowerb.v1, lowerb.v2);
    cv::Scalar ub(upperb.v0, upperb.v1, upperb.v2);
    cv::Mat* dst = reinterpret_cast<cv::Mat*>(cdst);
    cv::inRange(*mat, lb, ub, *dst);
}

void cv_mix_channels(CvMatrix* cmat, size_t nsrcs, CvMatrix* dst, size_t ndsts,
                     const int* from_to, size_t npairs) {
    cv::Mat* from = reinterpret_cast<cv::Mat*>(cmat);
    cv::Mat* to = reinterpret_cast<cv::Mat*>(dst);
    cv::mixChannels(from, nsrcs, to, ndsts, from_to, npairs);
}

void cv_normalize(CvMatrix* csrc, CvMatrix* cdst, double alpha, double beta,
                  int norm_type) {
    cv::Mat* src = reinterpret_cast<cv::Mat*>(csrc);
    cv::Mat* dst = reinterpret_cast<cv::Mat*>(cdst);
    cv::normalize(*src, *dst, alpha, beta, norm_type);
}

void cv_bitwise_and(const CvMatrix* const csrc1, const CvMatrix* const csrc2,
                    CvMatrix* cdst) {
    const cv::Mat* src1 = reinterpret_cast<const cv::Mat*>(csrc1);
    const cv::Mat* src2 = reinterpret_cast<const cv::Mat*>(csrc2);
    cv::Mat* dst = reinterpret_cast<cv::Mat*>(cdst);

    cv::bitwise_and(*src1, *src2, *dst);
}

void cv_bitwise_not(const CvMatrix* const csrc, CvMatrix* const cdst) {
    const cv::Mat* src = reinterpret_cast<const cv::Mat*>(csrc);
    cv::Mat* dst = reinterpret_cast<cv::Mat*>(cdst);

    cv::bitwise_not(*src, *dst);
}

void cv_bitwise_or(const CvMatrix* const csrc1, const CvMatrix* const csrc2,
                   CvMatrix* cdst) {
    const cv::Mat* src1 = reinterpret_cast<const cv::Mat*>(csrc1);
    const cv::Mat* src2 = reinterpret_cast<const cv::Mat*>(csrc2);
    cv::Mat* dst = reinterpret_cast<cv::Mat*>(cdst);

    cv::bitwise_or(*src1, *src2, *dst);
}

void cv_bitwise_xor(const CvMatrix* const csrc1, const CvMatrix* const csrc2,
                    CvMatrix* cdst) {
    const cv::Mat* src1 = reinterpret_cast<const cv::Mat*>(csrc1);
    const cv::Mat* src2 = reinterpret_cast<const cv::Mat*>(csrc2);
    cv::Mat* dst = reinterpret_cast<cv::Mat*>(cdst);

    cv::bitwise_xor(*src1, *src2, *dst);
}

int cv_count_non_zero(const CvMatrix* const csrc) {
    const cv::Mat* src = reinterpret_cast<const cv::Mat*>(csrc);
    return cv::countNonZero(*src);
}

// =============================================================================
//  Imgproc
// =============================================================================
void cv_line(CvMatrix* cmat, Point2i pt1, Point2i pt2, Scalar color,
             int thickness, int linetype, int shift) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Point point1(pt1.x, pt1.y);
    cv::Point point2(pt2.x, pt2.y);
    cv::Scalar colour(color.v0, color.v1, color.v2, color.v3);
    cv::line(*mat, point1, point2, colour, thickness, linetype, shift);
}

void cv_rectangle(CvMatrix* cmat, Rect crect, Scalar color, int thickness,
                  int linetype) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::Scalar colour(color.v0, color.v1, color.v2, color.v3);
    cv::rectangle(*mat, rect, colour, thickness, linetype);
}

void cv_ellipse(CvMatrix* cmat, Point2i center, Size2i axes, double angle,
                double start_angle, double end_angle, Scalar color,
                int thickness, int linetype, int shift) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Point cv_center(center.x, center.y);
    cv::Size cv_axes(axes.width, axes.height);
    cv::Scalar cv_color(color.v0, color.v1, color.v2, color.v3);

    cv::ellipse(*mat, cv_center, cv_axes, angle, start_angle, end_angle,
                cv_color, thickness, linetype, shift);
}

void cv_cvt_color(CvMatrix* cmat, CvMatrix* output, int code) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Mat* out = reinterpret_cast<cv::Mat*>(output);
    cv::cvtColor(*mat, *out, code);
}

void cv_pyr_down(CvMatrix* cmat, CvMatrix* output) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    cv::Mat* out = reinterpret_cast<cv::Mat*>(output);
    cv::pyrDown(*mat, *out);
}

void cv_resize(CvMatrix* from, CvMatrix* to, Size2i dsize, double fx, double fy,
               int interpolation) {
    cv::Mat* cv_from = reinterpret_cast<cv::Mat*>(from);
    cv::Mat* cv_to = reinterpret_cast<cv::Mat*>(to);
    cv::Size cv_dsize(dsize.width, dsize.height);
    cv::resize(*cv_from, *cv_to, cv_dsize, fx, fy, interpolation);
}

void cv_calc_hist(const CvMatrix* cimages, int nimages, const int* channels,
                  CvMatrix* cmask, CvMatrix* chist, int dims,
                  const int* hist_size, const float** ranges) {
    const cv::Mat* images = reinterpret_cast<const cv::Mat*>(cimages);
    cv::Mat* mask = reinterpret_cast<cv::Mat*>(cmask);
    cv::Mat* hist = reinterpret_cast<cv::Mat*>(chist);
    cv::calcHist(images, nimages, channels, *mask, *hist, dims, hist_size,
                 ranges);
}

void cv_calc_back_project(const CvMatrix* cimages, int nimages,
                          const int* channels, CvMatrix* chist,
                          CvMatrix* cback_project, const float** ranges) {
    const cv::Mat* images = reinterpret_cast<const cv::Mat*>(cimages);
    cv::Mat* hist = reinterpret_cast<cv::Mat*>(chist);
    cv::Mat* back_project = reinterpret_cast<cv::Mat*>(cback_project);
    cv::calcBackProject(images, nimages, channels, *hist, *back_project,
                        ranges);
}

// =============================================================================
//  Imgcodecs
// =============================================================================
CvMatrix* cv_imdecode(const uint8_t* const buffer, size_t len, int flag) {
    cv::Mat* dst = new cv::Mat();
    std::vector<uchar> input(buffer, buffer + len);
    cv::imdecode(cv::Mat(input), flag, dst);
    return reinterpret_cast<CvMatrix*>(dst);
}

// The caller is responsible for the allocated buffer
ImencodeResult cv_imencode(const char* const ext, const CvMatrix* const cmat,
                           const int* const flag_ptr, size_t flag_size) {
    const cv::Mat* image = reinterpret_cast<const cv::Mat*>(cmat);
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

void cv_imshow(const char* const winname, CvMatrix* cmat) {
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    if (mat != NULL) {
        cv::imshow(winname, *mat);
    }
}

int cv_wait_key(int delay) {
    return cv::waitKey(delay);
}

void cv_set_mouse_callback(const char* const winname, MouseCallback on_mouse,
                           void* userdata) {
    cv::setMouseCallback(winname, on_mouse, userdata);
}

// =============================================================================
//   VideoCapture
// =============================================================================
CVideoCapture* cv_videocapture_new(int index) {
    cv::VideoCapture* cap = new cv::VideoCapture(index);
    return reinterpret_cast<CVideoCapture*>(cap);
}

CVideoCapture* cv_videocapture_from_file(const char* const filename) {
    cv::VideoCapture* cap = new cv::VideoCapture(filename);
    return reinterpret_cast<CVideoCapture*>(cap);
}

bool cv_videocapture_is_opened(const CVideoCapture* const ccap) {
    const cv::VideoCapture* const cap =
        reinterpret_cast<const cv::VideoCapture* const>(ccap);
    return cap->isOpened();
}

bool cv_videocapture_read(CVideoCapture* ccap, CvMatrix* cmat) {
    cv::VideoCapture* cap = reinterpret_cast<cv::VideoCapture*>(ccap);
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    return cap->read(*mat);
}

void cv_videocapture_drop(CVideoCapture* ccap) {
    cv::VideoCapture* cap = reinterpret_cast<cv::VideoCapture*>(ccap);
    delete cap;
    ccap = nullptr;
}

bool cv_videocapture_set(CVideoCapture* ccap, int property, double value) {
    cv::VideoCapture* cap = reinterpret_cast<cv::VideoCapture*>(ccap);
    return cap->set(property, value);
}

double cv_videocapture_get(CVideoCapture* ccap, int property) {
    cv::VideoCapture* cap = reinterpret_cast<cv::VideoCapture*>(ccap);
    return cap->get(property);
}

// =============================================================================
//   VideoWriter
// =============================================================================
/// http://www.fourcc.org/codecs.php
int cv_fourcc(char c1, char c2, char c3, char c4) {
    return (((c1) &255) + (((c2) &255) << 8) + (((c3) &255) << 16) +
            (((c4) &255) << 24));
}

CVideoWriter* cv_videowriter_default() {
    cv::VideoWriter* writer = new cv::VideoWriter();
    return reinterpret_cast<CVideoWriter*>(writer);
}

CVideoWriter* cv_videowriter_new(const char* const path, int fourcc, double fps,
                                 Size2i frame_size, bool is_color) {
    cv::Size cv_frame_size(frame_size.width, frame_size.height);
    cv::VideoWriter* writer =
        new cv::VideoWriter(path, fourcc, fps, cv_frame_size, is_color);
    return reinterpret_cast<CVideoWriter*>(writer);
}

void cv_videowriter_drop(CVideoWriter* writer) {
    cv::VideoWriter* cv_writer = reinterpret_cast<cv::VideoWriter*>(writer);
    delete cv_writer;
    cv_writer = nullptr;
}

bool cv_videowriter_open(CVideoWriter* writer, const char* const path,
                         int fourcc, double fps, Size2i frame_size,
                         bool is_color) {
    cv::VideoWriter* cv_writer = reinterpret_cast<cv::VideoWriter*>(writer);
    cv::Size cv_frame_size(frame_size.width, frame_size.height);
    return cv_writer->open(path, fourcc, fps, cv_frame_size, is_color);
}

bool cv_videowriter_is_opened(CVideoWriter* writer) {
    cv::VideoWriter* cv_writer = reinterpret_cast<cv::VideoWriter*>(writer);
    return cv_writer->isOpened();
}

void cv_videowriter_write(CVideoWriter* writer, CvMatrix* cmat) {
    cv::VideoWriter* cv_writer = reinterpret_cast<cv::VideoWriter*>(writer);
    cv::Mat* mat = reinterpret_cast<cv::Mat*>(cmat);
    (*cv_writer) << (*mat);
}

bool cv_videowriter_set(CVideoWriter* writer, int property, double value) {
    cv::VideoWriter* cv_writer = reinterpret_cast<cv::VideoWriter*>(writer);
    return cv_writer->set(property, value);
}

double cv_videowriter_get(CVideoWriter* writer, int property) {
    cv::VideoWriter* cv_writer = reinterpret_cast<cv::VideoWriter*>(writer);
    return cv_writer->get(property);
}

// =============================================================================
//   CascadeClassifier
// =============================================================================
CCascadeClassifier* cv_cascade_classifier_new() {
    cv::CascadeClassifier* cc = new cv::CascadeClassifier();
    return reinterpret_cast<CCascadeClassifier*>(cc);
}

bool cv_cascade_classifier_load(CCascadeClassifier* cc, const char* const p) {
    cv::CascadeClassifier* cascade =
        reinterpret_cast<cv::CascadeClassifier*>(cc);
    return cascade->load(p);
}

CCascadeClassifier* cv_cascade_classifier_from_path(const char* const p) {
    cv::CascadeClassifier* cc = new cv::CascadeClassifier(p);
    return reinterpret_cast<CCascadeClassifier*>(cc);
}

void cv_cascade_classifier_drop(CCascadeClassifier* cc) {
    cv::CascadeClassifier* cascade =
        reinterpret_cast<cv::CascadeClassifier*>(cc);
    delete cascade;
    cc = nullptr;
}

void cv_cascade_classifier_detect(CCascadeClassifier* cc, CvMatrix* cmat,
                                  VecRect* vec_of_rect, double scale_factor,
                                  int min_neighbors, int flags, Size2i min_size,
                                  Size2i max_size) {
    cv::CascadeClassifier* cascade =
        reinterpret_cast<cv::CascadeClassifier*>(cc);
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cmat);
    std::vector<cv::Rect> objects;

    cv::Size cv_min_size(min_size.width, min_size.height);
    cv::Size cv_max_size(max_size.width, max_size.height);
    cascade->detectMultiScale(*image, objects, scale_factor, min_neighbors,
                              flags, cv_min_size, cv_max_size);
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

SvmDetector* cv_hog_default_people_detector() {
    std::vector<float>* detector =
        new std::vector<float>(cv::HOGDescriptor::getDefaultPeopleDetector());
    return reinterpret_cast<SvmDetector*>(detector);
}

SvmDetector* cv_hog_daimler_people_detector() {
    std::vector<float>* detector =
        new std::vector<float>(cv::HOGDescriptor::getDaimlerPeopleDetector());
    return reinterpret_cast<SvmDetector*>(detector);
}

void cv_hog_detector_drop(SvmDetector* detector) {
    std::vector<float>* cv_detector =
        reinterpret_cast<std::vector<float>*>(detector);
    delete cv_detector;
    cv_detector = nullptr;
}

HogDescriptor* cv_hog_new() {
    return reinterpret_cast<HogDescriptor*>(new cv::HOGDescriptor());
}

void cv_hog_drop(HogDescriptor* hog) {
    cv::HOGDescriptor* cv_hog = reinterpret_cast<cv::HOGDescriptor*>(hog);
    delete cv_hog;
    cv_hog = nullptr;
}

void cv_hog_set_svm_detector(HogDescriptor* hog, SvmDetector* detector) {
    cv::HOGDescriptor* cv_hog = reinterpret_cast<cv::HOGDescriptor*>(hog);
    std::vector<float>* cv_detector =
        reinterpret_cast<std::vector<float>*>(detector);
    cv_hog->setSVMDetector(*cv_detector);
}

void cv_hog_detect(HogDescriptor* hog, CvMatrix* cmat, VecRect* vec_rect,
                   VecDouble* vec_weight, Size2i win_stride, Size2i padding,
                   double scale, double final_threshold, bool use_means_shift) {
    // convert all types
    cv::HOGDescriptor* cv_hog = reinterpret_cast<cv::HOGDescriptor*>(hog);
    cv::Mat* image = reinterpret_cast<cv::Mat*>(cmat);
    std::vector<cv::Rect> objects;
    std::vector<double> weights;
    cv::Size cv_win_stride(win_stride.width, win_stride.height);
    cv::Size cv_padding(padding.width, padding.height);

    // Call the function
    cv_hog->detectMultiScale(*image, objects, weights, 0.1, cv_win_stride,
                             cv_padding, scale, final_threshold,
                             use_means_shift);

    // Prepare the results
    vec_rect_cxx_to_c(objects, vec_rect);
    vec_double_cxx_to_c(weights, vec_weight);
}

// =============================================================================
//  Object Tracking
// =============================================================================
CTermCriteria* cv_term_criteria_new(int type, int count, double epsilon) {
    cv::TermCriteria* criteria = new cv::TermCriteria(type, count, epsilon);
    return reinterpret_cast<CTermCriteria*>(criteria);
}

void cv_term_criteria_drop(CTermCriteria* c_criteria) {
    cv::TermCriteria* criteria =
        reinterpret_cast<cv::TermCriteria*>(c_criteria);
    delete criteria;
    c_criteria = nullptr;
}

RotatedRect cv_camshift(CvMatrix* c_bp_image, Rect crect,
                        CTermCriteria* c_criteria) {
    cv::Mat* bp_image = reinterpret_cast<cv::Mat*>(c_bp_image);
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::TermCriteria* criteria =
        reinterpret_cast<cv::TermCriteria*>(c_criteria);
    cv::RotatedRect rr = cv::CamShift(*bp_image, rect, *criteria);
    RotatedRect c_rr;
    c_rr.center.x = rr.center.x;
    c_rr.center.y = rr.center.y;
    c_rr.size.width = rr.size.width;
    c_rr.size.height = rr.size.height;
    c_rr.angle = rr.angle;
    return c_rr;
}

EXTERN_C_END
