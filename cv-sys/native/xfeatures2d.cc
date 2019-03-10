#include "xfeatures2d.hpp"
#include "utils.hpp"

namespace cvsys {

cv::Ptr<cv::xfeatures2d::SURF>*
surf_new(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright) {
    auto result = cv::xfeatures2d::SURF::create(hessianThreshold, nOctaves, nOctaveLayers, extended, upright);
    return new cv::Ptr<cv::xfeatures2d::SURF>(result);
}
void surf_drop(cv::Ptr<cv::xfeatures2d::SURF>* detector) {
    delete detector;
    detector = nullptr;
}

void surf_detect_and_compute(cv::Ptr<cv::xfeatures2d::SURF>* detector,
                             cv::Mat* image,
                             cv::Mat* mask,
                             CVec<KeyPoint>* keypoints,
                             cv::Mat* descriptors,
                             bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    to_ffi(keypoints_vector, keypoints);
}

cv::Ptr<cv::xfeatures2d::SIFT>*
sift_new(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma) {
    auto result = cv::xfeatures2d::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma);
    return new cv::Ptr<cv::xfeatures2d::SIFT>(result);
}
void sift_drop(cv::Ptr<cv::xfeatures2d::SIFT>* detector) {
    delete detector;
    detector = nullptr;
}

void sift_detect_and_compute(cv::Ptr<cv::xfeatures2d::SIFT>* detector,
                             cv::Mat* image,
                             cv::Mat* mask,
                             CVec<KeyPoint>* keypoints,
                             cv::Mat* descriptors,
                             bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    to_ffi(keypoints_vector, keypoints);
}

}  // namespace cvsys
