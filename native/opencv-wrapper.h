#ifndef OPENCV_WRAPPER_H_
#define OPENCV_WRAPPER_H_

#ifdef __cplusplus
#define EXTERN_C_BEGIN  extern "C" {
#define EXTERN_C_END    }
#else
#define EXTERN_C_BEGIN
#define EXTERN_C_END
#endif

EXTERN_C_BEGIN

// =============================================================================
//   Core
// =============================================================================
typedef void CMat;

// The caller owns the returned data CMat
CMat* opencv_mat_new();

bool opencv_mat_valid(CMat* cmat);

// The caller owns the returned data CMat
CMat* opencv_imread(const char* const filename, int flags);

// Free a Mat object
void opencv_mat_drop(CMat* cmat);

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================
void opencv_named_window(const char* const winname, int flags);
void opencv_imshow(const char* const winname, CMat* mat);
int opencv_wait_key(int delay_in_millis);

// =============================================================================
//   VideoCapture
// =============================================================================
typedef void CVideoCapture;

CVideoCapture* opencv_videocapture_new(int index);
bool opencv_videocapture_is_opened(const CVideoCapture* const ccap);
bool opencv_videocapture_read(CVideoCapture* ccap, CMat* cmat);
void opencv_videocapture_drop(CVideoCapture* ccap);

EXTERN_C_END

#endif  // OPENCV_WRAPPER_H_
