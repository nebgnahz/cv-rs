#ifndef CV_RS_FEATURES2D_H
#define CV_RS_FEATURES2D_H

#include "common.h"
#include <opencv2/core.hpp>
#include <opencv2/features2d.hpp>
#include <opencv2/xfeatures2d.hpp>

extern "C" {

// =============================================================================
//   MSER
// =============================================================================
void* cv_mser_new(int delta,
                  int min_area,
                  int max_area,
                  double max_variation,
                  double min_diversity,
                  int max_evolution,
                  double area_threshold,
                  double min_margin,
                  int edge_blur_size);
void cv_mser_drop(cv::Ptr<cv::MSER>* detector);
void cv_mser_detect_regions(cv::Ptr<cv::MSER>* detector,
                            cv::Mat* image,
                            CVec<CVec<Point2i>>* msers,
                            CVec<Rect>* bboxes);
void cv_mser_detect_and_compute(cv::Ptr<cv::MSER>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints);

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

// =============================================================================
//   DESCRIPTOR MATCHER
// =============================================================================

void* cv_matcher_new(const char* descriptorMatcherType);
void cv_matcher_drop(cv::Ptr<cv::DescriptorMatcher>* descriptorMatcher);
void cv_matcher_add(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher, CVec<cv::Mat*>& descriptors);
void cv_matcher_train(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher);
bool cv_matcher_is_empty(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher);
void cv_matcher_match(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                      cv::Mat& queryDescriptors,
                      CVec<DMatch>* matches);
void cv_matcher_match_two(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                          cv::Mat& queryDescriptors,
                          cv::Mat& trainDescriptors,
                          CVec<DMatch>* matches);
void cv_matcher_knn_match(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                          cv::Mat& queryDescriptors,
                          int k,
                          CVec<CVec<DMatch>>* matches);

void* cv_bow_trainer_new(int clusterCount, const cv::TermCriteria& termcrit, int attempts, int flags);
void cv_bow_trainer_drop(cv::BOWKMeansTrainer* trainer);
void cv_bow_trainer_add(cv::BOWKMeansTrainer& trainer, cv::Mat& descriptors);
void* cv_bow_trainer_cluster(cv::BOWKMeansTrainer& trainer);
}

#endif  // CV_RS_FEATURES2D_H
