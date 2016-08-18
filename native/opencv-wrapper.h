#ifndef OPENCV_WRAPPER_H_
#define OPENCV_WRAPPER_H_

typedef void CMat;

#ifdef __cplusplus
extern "C" {
#endif

// The caller owns the returned data CMat
CMat* opencv_imread(const char* const filename, int flags);

#ifdef __cplusplus
}  // extern "C"
#endif


#endif  // OPENCV_WRAPPER_H_
