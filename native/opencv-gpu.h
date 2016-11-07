#ifndef OPENCV_GPU_H_
#define OPENCV_GPU_H_

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
//   Basic
// =============================================================================
typedef struct _GpuMat GpuMat;
GpuMat* cv_gpu_mat_default();
void cv_gpu_mat_drop(GpuMat*);
void cv_gpu_mat_upload(GpuMat*, CMat*);
CMat* cv_mat_from_gpu_mat(GpuMat*);
GpuMat* cv_gpu_mat_from_mat(CMat*);

// =============================================================================
//   Hog
// =============================================================================
typedef struct _GpuHog GpuHog;
GpuHog* cv_gpu_hog_default();
void cv_gpu_hog_drop(GpuHog*);
void cv_gpu_hog_set_detector(GpuHog*, SvmDetector*);
void cv_gpu_hog_detect(GpuHog*, GpuMat*, VecRect*);

EXTERN_C_END

#endif  // OPENCV_GPU_H_
