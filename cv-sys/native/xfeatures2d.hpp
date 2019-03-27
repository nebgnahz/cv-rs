#ifndef CV_RS_XFEATURES2D_H
#define CV_RS_XFEATURES2D_H

#include "common.hpp"
#include <opencv2/core.hpp>
#include <opencv2/xfeatures2d.hpp>

namespace cvsys {

// =============================================================================
//   SURF
// =============================================================================

struct SURF : cv::Ptr<cv::xfeatures2d::SURF> {
    SURF(cv::Ptr<cv::xfeatures2d::SURF> p) : cv::Ptr<cv::xfeatures2d::SURF>(p) {
    }
};

SURF* surf_new(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright);
void surf_drop(SURF* detector);
void surf_detect_and_compute(SURF* detector,
                             cv::Mat* image,
                             cv::Mat* mask,
                             CVec<KeyPoint>* keypoints,
                             cv::Mat* descriptors,
                             bool useProvidedKeypoints);

// =============================================================================
//   SIFT
// =============================================================================

struct SIFT : cv::Ptr<cv::xfeatures2d::SIFT> {
    SIFT(cv::Ptr<cv::xfeatures2d::SIFT> p) : cv::Ptr<cv::xfeatures2d::SIFT>(p) {
    }
};

SIFT* sift_new(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma);
void sift_drop(SIFT* detector);
void sift_detect_and_compute(SIFT* detector,
                             cv::Mat* image,
                             cv::Mat* mask,
                             CVec<KeyPoint>* keypoints,
                             cv::Mat* descriptors,
                             bool useProvidedKeypoints);

}  // namespace cvsys

#endif  // CV_RS_XFEATURES2D_H
