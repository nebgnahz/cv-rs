#ifndef CV_RS_XFEATURES2D_H
#define CV_RS_XFEATURES2D_H

#include "common.hpp"
#include <opencv2/core.hpp>
#include <opencv2/xfeatures2d.hpp>

// =============================================================================
//   SURF
// =============================================================================

cv::Ptr<cv::xfeatures2d::SURF>*
cvsys_surf_new(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright);
void cvsys_surf_drop(cv::Ptr<cv::xfeatures2d::SURF>* detector);
void cvsys_surf_detect_and_compute(cv::Ptr<cv::xfeatures2d::SURF>* detector,
                                   cv::Mat* image,
                                   cv::Mat* mask,
                                   CVec<KeyPoint>* keypoints,
                                   cv::Mat* descriptors,
                                   bool useProvidedKeypoints);

// =============================================================================
//   SIFT
// =============================================================================

cv::Ptr<cv::xfeatures2d::SIFT>*
cvsys_sift_new(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma);
void cvsys_sift_drop(cv::Ptr<cv::xfeatures2d::SIFT>* detector);
void cvsys_sift_detect_and_compute(cv::Ptr<cv::xfeatures2d::SIFT>* detector,
                                   cv::Mat* image,
                                   cv::Mat* mask,
                                   CVec<KeyPoint>* keypoints,
                                   cv::Mat* descriptors,
                                   bool useProvidedKeypoints);

#endif  // CV_RS_XFEATURES2D_H
