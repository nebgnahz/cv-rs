#include "xfeatures2d.hpp"
#include "utils.hpp"

namespace cvsys {

SURF* surf_new(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright) {
    auto result = cv::xfeatures2d::SURF::create(hessianThreshold, nOctaves, nOctaveLayers, extended, upright);
    return new SURF(result);
}
void surf_drop(SURF* detector) {
    delete detector;
}

void surf_detect_and_compute(SURF* detector,
                             cv::Mat* image,
                             cv::Mat* mask,
                             CVec<KeyPoint>* keypoints,
                             cv::Mat* descriptors,
                             bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    auto nat_detector = static_cast<cv::Ptr<cv::xfeatures2d::SURF>*>(detector);
    nat_detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    to_ffi(keypoints_vector, keypoints);
}

SIFT* sift_new(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma) {
    auto result = cv::xfeatures2d::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma);
    return new SIFT(result);
}
void sift_drop(SIFT* detector) {
    delete detector;
}

void sift_detect_and_compute(SIFT* detector,
                             cv::Mat* image,
                             cv::Mat* mask,
                             CVec<KeyPoint>* keypoints,
                             cv::Mat* descriptors,
                             bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    auto nat_detector = static_cast<cv::Ptr<cv::xfeatures2d::SIFT>*>(detector);
    nat_detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    to_ffi(keypoints_vector, keypoints);
}

}  // namespace cvsys
