#include "features2d.h"
#include "utils.h"

extern "C" {

void* cv_mser_new(int delta,
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

void cv_mser_drop(cv::Ptr<cv::MSER>* detector) {
    delete detector;
    detector = nullptr;
}

void cv_mser_detect_regions(cv::Ptr<cv::MSER>* detector,
                            cv::Mat* image,
                            CVec<CVec<Point2i>>* msers,
                            CVec<Rect>* bboxes) {
    std::vector<std::vector<cv::Point>> msers_vector;
    std::vector<cv::Rect> bboxes_vector;

    detector->get()->detectRegions(*image, msers_vector, bboxes_vector);

    cv_to_ffi(msers_vector, msers);
    cv_to_ffi(bboxes_vector, bboxes);
}

void cv_mser_detect_and_compute(cv::Ptr<cv::MSER>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    cv_to_ffi(keypoints_vector, keypoints);
}

void* cv_surf_new(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright) {
    auto result = cv::xfeatures2d::SURF::create(hessianThreshold, nOctaves, nOctaveLayers, extended, upright);
    return new cv::Ptr<cv::xfeatures2d::SURF>(result);
}
void cv_surf_drop(cv::Ptr<cv::xfeatures2d::SURF>* detector) {
    delete detector;
    detector = nullptr;
}

void cv_surf_detect_and_compute(cv::Ptr<cv::xfeatures2d::SURF>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    cv_to_ffi(keypoints_vector, keypoints);
}

void* cv_sift_new(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma) {
    auto result = cv::xfeatures2d::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma);
    return new cv::Ptr<cv::xfeatures2d::SIFT>(result);
}
void cv_sift_drop(cv::Ptr<cv::xfeatures2d::SIFT>* detector) {
    delete detector;
    detector = nullptr;
}

void cv_sift_detect_and_compute(cv::Ptr<cv::xfeatures2d::SIFT>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    cv_to_ffi(keypoints_vector, keypoints);
}

void* cv_matcher_new(const char* descriptorMatcherType) {
    auto result = cv::DescriptorMatcher::create(descriptorMatcherType);
    return new cv::Ptr<cv::DescriptorMatcher>(result);
}

void cv_matcher_drop(cv::Ptr<cv::DescriptorMatcher>* descriptorMatcher) {
    delete descriptorMatcher;
    descriptorMatcher = nullptr;
}

void cv_matcher_add(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher, CVec<cv::Mat*>& descriptors) {
    std::vector<cv::Mat> descriptors_vector;
    ffi_to_cv(descriptors, &descriptors_vector);
    descriptorMatcher.get()->add(descriptors_vector);
}

void cv_matcher_train(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher) {
    descriptorMatcher.get()->train();
}

bool cv_matcher_is_empty(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher) {
    return descriptorMatcher.get()->empty();
}

void cv_matcher_match(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                      cv::Mat& queryDescriptors,
                      CVec<DMatch>* matches) {
    std::vector<cv::DMatch> matches_vector;
    descriptorMatcher.get()->match(queryDescriptors, matches_vector);
    cv_to_ffi(matches_vector, matches);
}

void cv_matcher_match_two(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                          cv::Mat& queryDescriptors,
                          cv::Mat& trainDescriptors,
                          CVec<DMatch>* matches) {
    std::vector<cv::DMatch> matches_vector;
    descriptorMatcher.get()->match(queryDescriptors, trainDescriptors, matches_vector);
    cv_to_ffi(matches_vector, matches);
}

void cv_matcher_knn_match(cv::Ptr<cv::DescriptorMatcher>& descriptorMatcher,
                          cv::Mat& queryDescriptors,
                          int k,
                          CVec<CVec<DMatch>>* matches) {
    std::vector<std::vector<cv::DMatch>> matches_vector;
    descriptorMatcher.get()->knnMatch(queryDescriptors, matches_vector, k);
    cv_to_ffi(matches_vector, matches);
}

void* cv_bow_trainer_new(int clusterCount, const cv::TermCriteria& termcrit, int attempts, int flags) {
    return new cv::BOWKMeansTrainer(clusterCount, termcrit, attempts, flags);
}

void cv_bow_trainer_drop(cv::BOWKMeansTrainer* trainer) {
    delete trainer;
    trainer = nullptr;
}

void cv_bow_trainer_add(cv::BOWKMeansTrainer& trainer, cv::Mat& descriptors) {
    trainer.add(descriptors);
}

void* cv_bow_trainer_cluster(cv::BOWKMeansTrainer& trainer) {
    cv::Mat* mat = new cv::Mat();
    *mat = trainer.cluster();
    return (mat);
}
}
