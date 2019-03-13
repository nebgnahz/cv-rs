#ifndef CV_RS_XFEATURES2D_H
#define CV_RS_XFEATURES2D_H

#include "common.hpp"
#include <opencv2/core.hpp>
#include <opencv2/xfeatures2d.hpp>

namespace cvsys {

// =============================================================================
//   SURF
// =============================================================================

void* surf_new(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright);
void surf_drop(void* detector);
void surf_detect_and_compute(void* detector,
                             cv::Mat* image,
                             cv::Mat* mask,
                             CVec<KeyPoint>* keypoints,
                             cv::Mat* descriptors,
                             bool useProvidedKeypoints);

// =============================================================================
//   SIFT
// =============================================================================

void* sift_new(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma);
void sift_drop(void* detector);
void sift_detect_and_compute(void* detector,
                             cv::Mat* image,
                             cv::Mat* mask,
                             CVec<KeyPoint>* keypoints,
                             cv::Mat* descriptors,
                             bool useProvidedKeypoints);

}  // namespace cvsys

#endif  // CV_RS_XFEATURES2D_H
