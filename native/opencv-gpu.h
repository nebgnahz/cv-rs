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
void cv_gpu_mat_upload(GpuMat*, CvMatrix*);
CvMatrix* cv_mat_from_gpu_mat(GpuMat*);
GpuMat* cv_gpu_mat_from_mat(CvMatrix*);

// =============================================================================
//   Hog
// =============================================================================
typedef struct _GpuHog GpuHog;
GpuHog* cv_gpu_hog_default();
GpuHog* cv_gpu_hog_new(CSize2i win_size, CSize2i block_size,
                       CSize2i block_stride, CSize2i cell_size, int32_t nbins);
void cv_gpu_hog_drop(GpuHog*);
void cv_gpu_hog_set_detector(GpuHog*, SvmDetector*);
void cv_gpu_hog_detect(GpuHog*, GpuMat*, VecRect*);

void cv_gpu_hog_set_gamma_correction(GpuHog*, bool gamma);
void cv_gpu_hog_set_group_threshold(GpuHog*, int32_t group_threshold);
void cv_gpu_hog_set_hit_threshold(GpuHog*, double hit_threshold);
void cv_gpu_hog_set_l2hys_threshold(GpuHog*, double l2hys_threshold);
void cv_gpu_hog_set_num_levels(GpuHog*, size_t num_levels);
void cv_gpu_hog_set_scale_factor(GpuHog*, double scale_factor);
void cv_gpu_hog_set_win_sigma(GpuHog*, double win_sigma);
void cv_gpu_hog_set_win_stride(GpuHog*, CSize2i win_stride);

bool cv_gpu_hog_get_gamma_correction(GpuHog*);
int32_t cv_gpu_hog_get_group_threshold(GpuHog*);
double cv_gpu_hog_get_hit_threshold(GpuHog*);
double cv_gpu_hog_get_l2hys_threshold(GpuHog*);
size_t cv_gpu_hog_get_num_levels(GpuHog*);
double cv_gpu_hog_get_scale_factor(GpuHog*);
double cv_gpu_hog_get_win_sigma(GpuHog*);
CSize2i cv_gpu_hog_get_win_stride(GpuHog*);

EXTERN_C_END

#endif  // OPENCV_GPU_H_
