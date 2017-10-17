#ifndef OPENCV_WRAPPER_H_
#define OPENCV_WRAPPER_H_

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
#define EXTERN_C_BEGIN extern "C" {
#define EXTERN_C_END }
#else
#define EXTERN_C_BEGIN
#define EXTERN_C_END
#endif

EXTERN_C_BEGIN

// =============================================================================
//   Core/Types
// =============================================================================
typedef struct _CvMatrixrix CvMatrix;

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
    Rect* array;
    size_t size;
} VecRect;

typedef struct {
    double* array;
    size_t size;
} VecDouble;

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

// The caller owns the returned data CvMatrix
CvMatrix* cv_mat_new();
CvMatrix* cv_mat_new_with_size(int rows, int cols, int type);
CvMatrix* cv_mat_zeros(int rows, int cols, int type);
CvMatrix* cv_mat_from_buffer(int rows, int cols, int type, const uint8_t* buf);

bool cv_mat_valid(CvMatrix* cmat);

// The caller owns the returned CvMatrix
CvMatrix* cv_mat_roi(CvMatrix* cmat, Rect crect);

void cv_mat_logic_and(CvMatrix* image, const CvMatrix* const mask);
void cv_mat_flip(CvMatrix* image, int code);

// The caller owns the returned data CvMatrix
CvMatrix* cv_imread(const char* const filename, int flags);

int cv_mat_rows(const CvMatrix* const cmat);
int cv_mat_cols(const CvMatrix* const cmat);
int cv_mat_depth(const CvMatrix* const cmat);
int cv_mat_channels(const CvMatrix* const cmat);
int cv_mat_type(const CvMatrix* const cmat);
const uint8_t* cv_mat_data(const CvMatrix* const cmat);
size_t cv_mat_total(const CvMatrix* const cmat);
size_t cv_mat_elem_size(const CvMatrix* const cmat);

// Free a Mat object
void cv_mat_drop(CvMatrix* cmat);

void cv_vec_of_rect_drop(VecRect* v);

// =============================================================================
//  core array
// =============================================================================
void cv_in_range(CvMatrix* cmat, Scalar lowerb, Scalar upperb, CvMatrix* dst);
void cv_mix_channels(CvMatrix* cmat, size_t nsrcs, CvMatrix* dst, size_t ndsts,
                     const int* from_to, size_t npairs);
void cv_normalize(CvMatrix* csrc, CvMatrix* cdst, double alpha, double beta,
                  int norm_type);
void cv_bitwise_and(const CvMatrix* const src1, const CvMatrix* const src2,
                    CvMatrix* dst);
void cv_bitwise_not(const CvMatrix* const src, CvMatrix* const dst);
void cv_bitwise_or(const CvMatrix* const src1, const CvMatrix* const src2,
                   CvMatrix* dst);
void cv_bitwise_xor(const CvMatrix* const src1, const CvMatrix* const src2,
                    CvMatrix* dst);
int cv_count_non_zero(const CvMatrix* const src);

// =============================================================================
//  Imgproc
// =============================================================================
void cv_rectangle(CvMatrix* cmat, Rect crect, Scalar color, int thickness,
                  int linetype);
void cv_ellipse(CvMatrix* cmat, Point2i center, Size2i axes, double angle,
                double start_angle, double end_angle, Scalar color,
                int thickness, int linetype, int shift);

void cv_cvt_color(CvMatrix* cmat, CvMatrix* output, int code);
void cv_pyr_down(CvMatrix* cmat, CvMatrix* output);
void cv_resize(CvMatrix* from, CvMatrix* to, Size2i dsize, double fx, double fy,
               int interpolation);
void cv_calc_hist(const CvMatrix* const cimages, int nimages,
                  const int* channels, CvMatrix* mask, CvMatrix* hist, int dims,
                  const int* hist_size, const float** ranges);
void cv_calc_back_project(const CvMatrix* images, int nimages,
                          const int* channels, CvMatrix* hist,
                          CvMatrix* back_project, const float** ranges);

// =============================================================================
//  Imgcodecs
// =============================================================================
CvMatrix* cv_imdecode(const uint8_t* const buffer, size_t len, int flag);
ImencodeResult cv_imencode(const char* const ext, const CvMatrix* const cmat,
                           const int* const flag_ptr, size_t flag_size);

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================
void cv_named_window(const char* const winname, int flags);
void cv_destroy_window(const char* const winname);
void cv_imshow(const char* const winname, CvMatrix* mat);
int cv_wait_key(int delay_in_millis);

typedef void (*MouseCallback)(int e, int x, int y, int flags, void* data);
void cv_set_mouse_callback(const char* const winname, MouseCallback onMouse,
                           void* userdata);

// =============================================================================
//   VideoIO
// =============================================================================
typedef struct _CVideoCapture CVideoCapture;

CVideoCapture* cv_videocapture_new(int index);
CVideoCapture* cv_videocapture_from_file(const char* const filename);
bool cv_videocapture_is_opened(const CVideoCapture* const ccap);
bool cv_videocapture_read(CVideoCapture* ccap, CvMatrix* cmat);
void cv_videocapture_drop(CVideoCapture* ccap);
bool cv_videocapture_set(CVideoCapture* ccap, int property, double value);
double cv_videocapture_get(CVideoCapture* ccap, int property);

typedef struct _CVideoWriter CVideoWriter;

int cv_fourcc(char c1, char c2, char c3, char c4);

CVideoWriter* cv_videowriter_default();
CVideoWriter* cv_videowriter_new(const char* const path, int fourcc, double fps,
                                 Size2i frame_size, bool is_color);
void cv_videowriter_drop(CVideoWriter* writer);
bool cv_videowriter_open(CVideoWriter* writer, const char* const path,
                         int fourcc, double fps, Size2i frame_size,
                         bool is_color);
bool cv_videowriter_is_opened(CVideoWriter* writer);
void cv_videowriter_write(CVideoWriter* writer, CvMatrix* cmat);
bool cv_videowriter_set(CVideoWriter* writer, int property, double value);
double cv_videowriter_get(CVideoWriter* writer, int property);

// =============================================================================
//   CascadeClassifier
// =============================================================================
typedef struct _CCascadeClassifier CCascadeClassifier;
CCascadeClassifier* cv_cascade_classifier_new();
CCascadeClassifier* cv_cascade_classifier_from_path(const char* const path);
bool cv_cascade_classifier_load(CCascadeClassifier* cc, const char* const path);
void cv_cascade_classifier_drop(CCascadeClassifier* cc);

// vec_of_rect is dynamically allocated, the caller should take ownership of it.
void cv_cascade_classifier_detect(CCascadeClassifier* cc, CvMatrix* cmat,
                                  VecRect* vec_of_rect, double scale_factor,
                                  int min_neighbors, int flags, Size2i min_size,
                                  Size2i max_size);

typedef struct _SvmDetector SvmDetector;
SvmDetector* cv_hog_default_people_detector();
SvmDetector* cv_hog_daimler_people_detector();
void cv_hog_detector_drop(SvmDetector*);

typedef struct _HogDescriptor HogDescriptor;
HogDescriptor* cv_hog_new();
void cv_hog_drop(HogDescriptor*);
void cv_hog_set_svm_detector(HogDescriptor*, SvmDetector*);
void cv_hog_detect(HogDescriptor*, CvMatrix*, VecRect* vec_detected,
                   VecDouble* vec_weight, Size2i win_stride, Size2i padding,
                   double scale, double final_threshold, bool use_means_shift);

// =============================================================================
//   VideoTrack
// =============================================================================
typedef struct _CTermCriteria CTermCriteria;
CTermCriteria* cv_term_criteria_new(int type, int count, double epsilon);
void cv_term_criteria_drop(CTermCriteria* c_criteria);
RotatedRect cv_camshift(CvMatrix* back_project_image, Rect window,
                        CTermCriteria* term_criteria);

EXTERN_C_END

#endif  // OPENCV_WRAPPER_H_
