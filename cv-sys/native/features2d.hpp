#ifndef CV_RS_FEATURES2D_H
#define CV_RS_FEATURES2D_H

#include "common.hpp"
#include <opencv2/core.hpp>
#include <opencv2/features2d.hpp>

namespace cvsys {

// =============================================================================
//   MSER
// =============================================================================

struct MSER : cv::Ptr<cv::MSER> {
    MSER(cv::Ptr<cv::MSER> p) : cv::Ptr<cv::MSER>(p) {
    }
};

MSER* mser_new(int delta,
               int min_area,
               int max_area,
               double max_variation,
               double min_diversity,
               int max_evolution,
               double area_threshold,
               double min_margin,
               int edge_blur_size);
void mser_drop(MSER* detector);
void mser_detect_regions(MSER* detector, cv::Mat* image, CVec<CVec<Point2i>>* msers, CVec<Rect>* bboxes);
void mser_detect_and_compute(MSER* detector,
                             cv::Mat* image,
                             cv::Mat* mask,
                             CVec<KeyPoint>* keypoints,
                             cv::Mat* descriptors,
                             bool useProvidedKeypoints);

// =============================================================================
//   DESCRIPTOR MATCHER
// =============================================================================

struct DescriptorMatcher : cv::Ptr<cv::DescriptorMatcher> {
    DescriptorMatcher(cv::Ptr<cv::DescriptorMatcher> p) : cv::Ptr<cv::DescriptorMatcher>(p) {
    }
};

DescriptorMatcher* matcher_new(const char* descriptorMatcherType);
void matcher_drop(DescriptorMatcher* descriptorMatcher);
void matcher_add(DescriptorMatcher& descriptorMatcher, cv::Mat* const* descriptors, size_t len);
void matcher_train(DescriptorMatcher& descriptorMatcher);
bool matcher_is_empty(DescriptorMatcher& descriptorMatcher);
void matcher_match(DescriptorMatcher& descriptorMatcher, cv::Mat& queryDescriptors, CVec<DMatch>* matches);
void matcher_match_two(DescriptorMatcher& descriptorMatcher,
                       cv::Mat& queryDescriptors,
                       cv::Mat& trainDescriptors,
                       CVec<DMatch>* matches);
void matcher_knn_match(DescriptorMatcher& descriptorMatcher,
                       cv::Mat& queryDescriptors,
                       int k,
                       CVec<CVec<DMatch>>* matches);

// =============================================================================
//   BoW
// =============================================================================

cv::BOWKMeansTrainer* bow_trainer_new(int clusterCount, const cv::TermCriteria& termcrit, int attempts, int flags);
void bow_trainer_drop(cv::BOWKMeansTrainer* trainer);
void bow_trainer_add(cv::BOWKMeansTrainer& trainer, cv::Mat& descriptors);
cv::Mat* bow_trainer_cluster(cv::BOWKMeansTrainer& trainer);

}  // namespace cvsys

#endif  // CV_RS_FEATURES2D_H
