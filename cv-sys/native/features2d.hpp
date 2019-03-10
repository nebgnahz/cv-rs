#ifndef CV_RS_FEATURES2D_H
#define CV_RS_FEATURES2D_H

#include "common.hpp"
#include <opencv2/core.hpp>
#include <opencv2/features2d.hpp>

// =============================================================================
//   MSER
// =============================================================================
cv::Ptr<cv::MSER>* cvsys_mser_new(int delta,
                                  int min_area,
                                  int max_area,
                                  double max_variation,
                                  double min_diversity,
                                  int max_evolution,
                                  double area_threshold,
                                  double min_margin,
                                  int edge_blur_size);
void cvsys_mser_drop(cv::Ptr<cv::MSER>* detector);
void cvsys_mser_detect_regions(cv::Ptr<cv::MSER>* detector,
                               cv::Mat* image,
                               CVec<CVec<Point2i>>* msers,
                               CVec<Rect>* bboxes);
void cvsys_mser_detect_and_compute(cv::Ptr<cv::MSER>* detector,
                                   cv::Mat* image,
                                   cv::Mat* mask,
                                   CVec<KeyPoint>* keypoints,
                                   cv::Mat* descriptors,
                                   bool useProvidedKeypoints);

// =============================================================================
//   DESCRIPTOR MATCHER
// =============================================================================

cv::Ptr<cv::DescriptorMatcher>* cvsys_matcher_new(const char* descriptorMatcherType);
void cvsys_matcher_drop(cv::Ptr<cv::DescriptorMatcher>* descriptorMatcher);
void cvsys_matcher_add(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher, cv::Mat* const* descriptors, size_t len);
void cvsys_matcher_train(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher);
bool cvsys_matcher_is_empty(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher);
void cvsys_matcher_match(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                         cv::Mat& queryDescriptors,
                         CVec<DMatch>* matches);
void cvsys_matcher_match_two(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                             cv::Mat& queryDescriptors,
                             cv::Mat& trainDescriptors,
                             CVec<DMatch>* matches);
void cvsys_matcher_knn_match(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                             cv::Mat& queryDescriptors,
                             int k,
                             CVec<CVec<DMatch>>* matches);

cv::BOWKMeansTrainer*
cvsys_bow_trainer_new(int clusterCount, const cv::TermCriteria& termcrit, int attempts, int flags);
void cvsys_bow_trainer_drop(cv::BOWKMeansTrainer* trainer);
void cvsys_bow_trainer_add(cv::BOWKMeansTrainer& trainer, cv::Mat& descriptors);
cv::Mat* cvsys_bow_trainer_cluster(cv::BOWKMeansTrainer& trainer);

#endif  // CV_RS_FEATURES2D_H
