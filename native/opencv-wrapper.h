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
typedef struct _CMat CMat;

typedef struct {
    int32_t x;
    int32_t y;
} CPoint2i;

typedef struct {
    float x;
    float y;
} CPoint2f;

typedef struct {
    int width;
    int height;
} CSize2i;

typedef struct {
    float width;
    float height;
} CSize2f;

typedef struct {
    int32_t x;
    int32_t y;
    int32_t width;
    int32_t height;
} CRect;

typedef struct {
    CPoint2f center;
    CSize2f size;
    float angle;
} CRotatedRect;

typedef struct {
    CRect* array;
    size_t size;
} CVecOfRect;

typedef struct {
    int32_t v0;
    int32_t v1;
    int32_t v2;
    int32_t v3;
} CScalar;

// The caller owns the returned data CMat
CMat* opencv_mat_new();
CMat* opencv_mat_new_with_size(int rows, int cols, int type);

bool opencv_mat_valid(CMat* cmat);

// The caller owns the returned CMat
CMat* opencv_mat_roi(CMat* cmat, CRect crect);

void opencv_mat_logic_and(CMat* image, const CMat* const mask);

// The caller owns the returned data CMat
CMat* opencv_imread(const char* const filename, int flags);

int opencv_mat_rows(const CMat* const cmat);
int opencv_mat_cols(const CMat* const cmat);
int opencv_mat_depth(const CMat* const cmat);

// Free a Mat object
void opencv_mat_drop(CMat* cmat);

void opencv_vec_of_rect_drop(CVecOfRect* v);

// =============================================================================
//  core array
// =============================================================================
void opencv_in_range(CMat* cmat, CScalar lowerb, CScalar upperb, CMat* dst);
void opencv_mix_channels(CMat* cmat, size_t nsrcs, CMat* dst, size_t ndsts,
                         const int* from_to, size_t npairs);
void opencv_normalize(CMat* csrc, CMat* cdst, double alpha, double beta,
                      int norm_type);

// =============================================================================
//  Imgproc
// =============================================================================
void opencv_rectangle(CMat* cmat, CRect crect);
void opencv_cvt_color(CMat* cmat, CMat* output, int code);
void opencv_calc_hist(const CMat* const cimages, int nimages,
                      const int* channels, CMat* mask, CMat* hist, int dims,
                      const int* hist_size, const float** ranges);
void opencv_calc_back_project(const CMat* images, int nimages,
                              const int* channels, CMat* hist,
                              CMat* back_project, const float** ranges);

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================
void opencv_named_window(const char* const winname, int flags);
void opencv_imshow(const char* const winname, CMat* mat);
int opencv_wait_key(int delay_in_millis);

typedef void(* MouseCallback)(int e, int x, int y, int flags, void *data);
void opencv_set_mouse_callback(const char* const winname, MouseCallback onMouse,
                               void* userdata);

// =============================================================================
//   VideoCapture
// =============================================================================
typedef struct _CVideoCapture CVideoCapture;

CVideoCapture* opencv_videocapture_new(int index);
bool opencv_videocapture_is_opened(const CVideoCapture* const ccap);
bool opencv_videocapture_read(CVideoCapture* ccap, CMat* cmat);
void opencv_videocapture_drop(CVideoCapture* ccap);

// =============================================================================
//   CascadeClassifier
// =============================================================================
typedef struct _CCascadeClassifier CCascadeClassifier;
CCascadeClassifier* opencv_cascade_classifier_new();
CCascadeClassifier* opencv_cascade_classifier_from_path(const char* const path);
bool opencv_cascade_classifier_load(CCascadeClassifier* cc,
                                    const char* const path);
void opencv_cascade_classifier_drop(CCascadeClassifier* cc);

// vec_of_rect is dynamically allocated, the caller should take ownership of it.
void opencv_cascade_classifier_detect(CCascadeClassifier* cc, CMat* cmat,
                                      CVecOfRect* vec_of_rect,
                                      double scale_factor, int min_neighbors,
                                      int flags, CSize2i min_size,
                                      CSize2i max_size);

// =============================================================================
//   VideoTrack
// =============================================================================
typedef struct _CTermCriteria CTermCriteria;
CTermCriteria* opencv_term_criteria_new(int type, int count, double epsilon);
void opencv_term_criteria_drop(CTermCriteria* c_criteria);
CRotatedRect opencv_camshift(CMat* back_project_image, CRect window,
                             CTermCriteria* term_criteria);

EXTERN_C_END

#endif  // OPENCV_WRAPPER_H_
