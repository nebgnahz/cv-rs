#ifndef OPENCV_WRAPPER_H_
#define OPENCV_WRAPPER_H_

typedef void CMat;

#ifdef __cplusplus
extern "C" {
#endif

// The caller owns the returned data CMat
CMat* opencv_imread(const char* const filename, int flags);

// Free a Mat object
void opencv_mat_free(CMat* mat);

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================
void opencv_named_window(const char* const winname, int flags);
void opencv_imshow(const char* const winname, CMat* mat);
int opencv_wait_key(int delay_in_millis);

#ifdef __cplusplus
}  // extern "C"
#endif


#endif  // OPENCV_WRAPPER_H_
