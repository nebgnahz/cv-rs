#ifndef OPENCV_WRAPPER_H_
#define OPENCV_WRAPPER_H_

#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/objdetect.hpp>
#include <opencv2/video/tracking.hpp>
#include <opencv2/features2d.hpp>
#include <functional>
#include <stddef.h>
#include <stdint.h>

#define EXTERN_C_BEGIN extern "C" {
#define EXTERN_C_END }

// Caller is responsible for disposing `error` field
template<typename T>
struct Result
{
    T value;
    const char* error;

    static Result<T> FromFunction(std::function<T()> function)
    {
        Result<T> result = {};
        try
        {
            result.value = function();
        }
        catch( cv::Exception& e )
        {
            const char* err_msg = e.what();
            auto len = std::strlen(err_msg);
            auto retained_err = new char[len + 1];
            std::strcpy(retained_err, err_msg);
            result.error = retained_err;
        }
        return result;
    }
};

template<typename T>
struct CVec
{
    T* array;
    size_t size;
};

EXTERN_C_BEGIN

// =============================================================================
//   Core/Types
// =============================================================================
typedef struct {
    int32_t x;
    int32_t y;
} Point2i;

typedef struct {
    float x;
    float y;
} Point2f;

typedef struct {
    int width;
    int height;
} Size2i;

typedef struct {
    float width;
    float height;
} Size2f;

typedef struct {
    int32_t x;
    int32_t y;
    int32_t width;
    int32_t height;
} Rect;

typedef struct {
    Point2f center;
    Size2f size;
    float angle;
} RotatedRect;

typedef struct {
    int32_t v0;
    int32_t v1;
    int32_t v2;
    int32_t v3;
} Scalar;

typedef struct {
    bool status;
    uint8_t* buf;
    size_t size;
} ImencodeResult;

// The caller owns the returned data cv::Mat
void* cv_mat_new();
void* cv_mat_new_with_size(int rows, int cols, int type);
void* cv_mat_zeros(int rows, int cols, int type);
void* cv_mat_from_buffer(int rows, int cols, int type, const uint8_t* buf);

bool cv_mat_valid(cv::Mat* mat);

// The caller owns the returned cv::Mat
void* cv_mat_roi(cv::Mat* mat, Rect crect);

void cv_mat_logic_and(cv::Mat* image, const cv::Mat* const mask);
void cv_mat_flip(cv::Mat* image, int code);

// The caller owns the returned data cv::Mat
void* cv_imread(const char* const filename, int flags);

int cv_mat_rows(const cv::Mat* const mat);
int cv_mat_cols(const cv::Mat* const mat);
int cv_mat_depth(const cv::Mat* const mat);
int cv_mat_channels(const cv::Mat* const mat);
int cv_mat_type(const cv::Mat* const mat);
const uint8_t* cv_mat_data(const cv::Mat* const mat);
size_t cv_mat_total(const cv::Mat* const mat);
size_t cv_mat_elem_size(const cv::Mat* const mat);
size_t cv_mat_elem_size1(const cv::Mat* const mat);
size_t cv_mat_step1(const cv::Mat* const mat, int i);

// Free a Mat object
void cv_mat_drop(cv::Mat* mat);

void cv_vec_drop(CVec<void>* vec, unsigned int depth);
void c_drop(void* value);

// =============================================================================
//  core array
// =============================================================================
void cv_in_range(cv::Mat* mat, Scalar lowerb, Scalar upperb, cv::Mat* dst);
void cv_min_max_loc(const cv::Mat* const mat, double* min, double* max,
                    Point2i* minLoc, Point2i* maxLoc,
                    const cv::Mat* const cmask);
void cv_mix_channels(cv::Mat* mat, size_t nsrcs, cv::Mat* dst, size_t ndsts,
                     const int* from_to, size_t npairs);
void cv_normalize(cv::Mat* csrc, cv::Mat* cdst, double alpha, double beta,
                  int norm_type);
void cv_bitwise_and(const cv::Mat* const src1, const cv::Mat* const src2,
                    cv::Mat* dst);
void cv_bitwise_not(const cv::Mat* const src, cv::Mat* const dst);
void cv_bitwise_or(const cv::Mat* const src1, const cv::Mat* const src2,
                   cv::Mat* dst);
void cv_bitwise_xor(const cv::Mat* const src1, const cv::Mat* const src2,
                    cv::Mat* dst);
int cv_count_non_zero(const cv::Mat* const src);

// =============================================================================
//  Imgproc
// =============================================================================
void cv_line(cv::Mat* mat, Point2i pt1, Point2i pt2, Scalar color,
             int thickness, int linetype, int shift);
void cv_rectangle(cv::Mat* mat, Rect crect, Scalar color, int thickness,
                  int linetype);
void cv_ellipse(cv::Mat* mat, Point2i center, Size2i axes, double angle,
                double start_angle, double end_angle, Scalar color,
                int thickness, int linetype, int shift);

void cv_cvt_color(cv::Mat* mat, cv::Mat* output, int code);
void cv_pyr_down(cv::Mat* mat, cv::Mat* output);
void cv_resize(cv::Mat* from, cv::Mat* to, Size2i dsize, double fx, double fy,
               int interpolation);
void cv_calc_hist(const cv::Mat* const cimages, int nimages,
                  const int* channels, cv::Mat* mask, cv::Mat* hist, int dims,
                  const int* hist_size, const float** ranges);
void cv_calc_back_project(const cv::Mat* images, int nimages,
                          const int* channels, cv::Mat* hist,
                          cv::Mat* back_project, const float** ranges);

// =============================================================================
//  Imgcodecs
// =============================================================================
void* cv_imdecode(const uint8_t* const buffer, size_t len, int flag);
ImencodeResult cv_imencode(const char* const ext, const cv::Mat* const mat,
                           const int* const flag_ptr, size_t flag_size);

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================
void cv_named_window(const char* const winname, int flags);
void cv_destroy_window(const char* const winname);
void cv_imshow(const char* const winname, cv::Mat* mat);
int cv_wait_key(int delay_in_millis);

typedef void (*MouseCallback)(int e, int x, int y, int flags, void* data);
void cv_set_mouse_callback(const char* const winname, MouseCallback onMouse,
                           void* userdata);

// =============================================================================
//   VideoIO
// =============================================================================
void* cv_videocapture_new(int index);
void* cv_videocapture_from_file(const char* const filename);
bool cv_videocapture_is_opened(const cv::VideoCapture* const cap);
bool cv_videocapture_read(cv::VideoCapture* cap, cv::Mat* mat);
void cv_videocapture_drop(cv::VideoCapture* cap);
bool cv_videocapture_set(cv::VideoCapture* cap, int property, double value);
double cv_videocapture_get(cv::VideoCapture* cap, int property);

int cv_fourcc(char c1, char c2, char c3, char c4);

void* cv_videowriter_default();
void* cv_videowriter_new(const char* const path, int fourcc, double fps,
                                 Size2i frame_size, bool is_color);
void cv_videowriter_drop(cv::VideoWriter* writer);
bool cv_videowriter_open(cv::VideoWriter* writer, const char* const path,
                         int fourcc, double fps, Size2i frame_size,
                         bool is_color);
bool cv_videowriter_is_opened(cv::VideoWriter* writer);
void cv_videowriter_write(cv::VideoWriter* writer, cv::Mat* mat);
bool cv_videowriter_set(cv::VideoWriter* writer, int property, double value);
double cv_videowriter_get(cv::VideoWriter* writer, int property);

// =============================================================================
//   CascadeClassifier
// =============================================================================
void* cv_cascade_classifier_new();
void* cv_cascade_classifier_from_path(const char* const path);
bool cv_cascade_classifier_load(cv::CascadeClassifier* cc, const char* const path);
void cv_cascade_classifier_drop(cv::CascadeClassifier* cc);

// vec_of_rect is dynamically allocated, the caller should take ownership of it.
void cv_cascade_classifier_detect(cv::CascadeClassifier* cascade, cv::Mat* mat,
                                  CVec<Rect>* vec_of_rect, double scale_factor,
                                  int min_neighbors, int flags, Size2i min_size,
                                  Size2i max_size);

void* cv_hog_default_people_detector();
void* cv_hog_daimler_people_detector();
void cv_hog_detector_drop(std::vector<float>*);

void* cv_hog_new();
void cv_hog_drop(cv::HOGDescriptor*);
void cv_hog_set_svm_detector(cv::HOGDescriptor*, std::vector<float>*);
void cv_hog_detect(cv::HOGDescriptor*, cv::Mat*, CVec<Rect>* vec_detected,
                   CVec<double>* vec_weight, Size2i win_stride, Size2i padding,
                   double scale, double final_threshold, bool use_means_shift);

// =============================================================================
//   VideoTrack
// =============================================================================
void* cv_term_criteria_new(int type, int count, double epsilon);
void cv_term_criteria_drop(cv::TermCriteria* criteria);
RotatedRect cv_camshift(cv::Mat* back_project_image, Rect window,
                        cv::TermCriteria* criteria);

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
                  int edge_blur_size);
void cv_mser_drop(cv::Ptr<cv::MSER>* mser);
void cv_mser_detect_regions(cv::Ptr<cv::MSER>* mser, cv::Mat* image, CVec<CVec<Point2i>>* msers, CVec<Rect>* bboxes);

// =============================================================================
//   Other
// =============================================================================

void cv_compare_hist(cv::Mat* first_image, cv::Mat* second_image, int method, Result<double>* result);

EXTERN_C_END

#endif  // OPENCV_WRAPPER_H_
