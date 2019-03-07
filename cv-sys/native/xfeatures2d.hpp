#ifndef CV_RS_FEATURES2D_H
#define CV_RS_FEATURES2D_H

#include "common.hpp"
#include <opencv2/core.hpp>
#include <opencv2/xfeatures2d.hpp>

extern "C" {

// =============================================================================
//   SURF
// =============================================================================

void* cv_surf_new(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright);
void cv_surf_drop(cv::Ptr<cv::xfeatures2d::SURF>* detector);
void cv_surf_detect_and_compute(cv::Ptr<cv::xfeatures2d::SURF>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints);

// =============================================================================
//   SIFT
// =============================================================================

void* cv_sift_new(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma);
void cv_sift_drop(cv::Ptr<cv::xfeatures2d::SIFT>* detector);
void cv_sift_detect_and_compute(cv::Ptr<cv::xfeatures2d::SIFT>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints);
}

#endif  // CV_RS_FEATURES2D_H
