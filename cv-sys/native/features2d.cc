#include "features2d.hpp"
#include "utils.hpp"

namespace cvsys {

cv::Ptr<cv::MSER>* mser_new(int delta,
                            int min_area,
                            int max_area,
                            double max_variation,
                            double min_diversity,
                            int max_evolution,
                            double area_threshold,
                            double min_margin,
                            int edge_blur_size) {
    cv::Ptr<cv::MSER> result = cv::MSER::create(delta,
                                                min_area,
                                                max_area,
                                                max_variation,
                                                min_diversity,
                                                max_evolution,
                                                area_threshold,
                                                min_margin,
                                                edge_blur_size);
    return new cv::Ptr<cv::MSER>(result);
}

void mser_drop(cv::Ptr<cv::MSER>* detector) {
    delete detector;
    detector = nullptr;
}

void mser_detect_regions(cv::Ptr<cv::MSER>* detector, cv::Mat* image, CVec<CVec<Point2i>>* msers, CVec<Rect>* bboxes) {
    std::vector<std::vector<cv::Point>> msers_vector;
    std::vector<cv::Rect> bboxes_vector;

    detector->get()->detectRegions(*image, msers_vector, bboxes_vector);

    to_ffi(msers_vector, msers);
    to_ffi(bboxes_vector, bboxes);
}

void mser_detect_and_compute(cv::Ptr<cv::MSER>* detector,
                             cv::Mat* image,
                             cv::Mat* mask,
                             CVec<KeyPoint>* keypoints,
                             cv::Mat* descriptors,
                             bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    to_ffi(keypoints_vector, keypoints);
}

cv::Ptr<cv::DescriptorMatcher>* matcher_new(const char* descriptorMatcherType) {
    auto result = cv::DescriptorMatcher::create(descriptorMatcherType);
    return new cv::Ptr<cv::DescriptorMatcher>(result);
}

void matcher_drop(cv::Ptr<cv::DescriptorMatcher>* descriptorMatcher) {
    delete descriptorMatcher;
    descriptorMatcher = nullptr;
}

void matcher_add(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher, cv::Mat* const* descriptors, size_t len) {
    std::vector<cv::Mat> descriptors_vector;
    for (size_t i = 0; i < len; i++) {
        descriptors_vector.emplace_back(*descriptors[i]);
    }
    descriptorMatcher.get()->add(descriptors_vector);
}

void matcher_train(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher) {
    descriptorMatcher.get()->train();
}

bool matcher_is_empty(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher) {
    return descriptorMatcher.get()->empty();
}

void matcher_match(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                   cv::Mat& queryDescriptors,
                   CVec<DMatch>* matches) {
    std::vector<cv::DMatch> matches_vector;
    descriptorMatcher.get()->match(queryDescriptors, matches_vector);
    to_ffi(matches_vector, matches);
}

void matcher_match_two(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                       cv::Mat& queryDescriptors,
                       cv::Mat& trainDescriptors,
                       CVec<DMatch>* matches) {
    std::vector<cv::DMatch> matches_vector;
    descriptorMatcher.get()->match(queryDescriptors, trainDescriptors, matches_vector);
    to_ffi(matches_vector, matches);
}

void matcher_knn_match(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                       cv::Mat& queryDescriptors,
                       int k,
                       CVec<CVec<DMatch>>* matches) {
    std::vector<std::vector<cv::DMatch>> matches_vector;
    descriptorMatcher.get()->knnMatch(queryDescriptors, matches_vector, k);
    to_ffi(matches_vector, matches);
}

cv::BOWKMeansTrainer* bow_trainer_new(int clusterCount, const cv::TermCriteria& termcrit, int attempts, int flags) {
    return new cv::BOWKMeansTrainer(clusterCount, termcrit, attempts, flags);
}

void bow_trainer_drop(cv::BOWKMeansTrainer* trainer) {
    delete trainer;
    trainer = nullptr;
}

void bow_trainer_add(cv::BOWKMeansTrainer& trainer, cv::Mat& descriptors) {
    trainer.add(descriptors);
}

cv::Mat* bow_trainer_cluster(cv::BOWKMeansTrainer& trainer) {
    cv::Mat* mat = new cv::Mat();
    *mat = trainer.cluster();
    return (mat);
}

}  // namespace cvsys
