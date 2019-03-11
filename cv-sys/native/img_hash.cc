#include <opencv2/core.hpp>
#include <opencv2/img_hash.hpp>

namespace cvsys {

void hash_any_compute(void* hash, cv::Mat& mat, cv::Mat& result) {
    cv::Ptr<cv::img_hash::PHash>* base = static_cast<cv::Ptr<cv::img_hash::PHash>*>(hash);
    base->get()->compute(mat, result);
}

double hash_any_compare(void* hash, cv::Mat& lhs, cv::Mat& rhs) {
    cv::Ptr<cv::img_hash::PHash>* base = static_cast<cv::Ptr<cv::img_hash::PHash>*>(hash);
    return base->get()->compare(lhs, rhs);
}

cv::Ptr<cv::img_hash::AverageHash>* average_hash_new() {
    cv::Ptr<cv::img_hash::AverageHash> result = cv::img_hash::AverageHash::create();
    return new cv::Ptr<cv::img_hash::AverageHash>(result);
}

void average_hash_drop(cv::Ptr<cv::img_hash::AverageHash>* hash) {
    delete hash;
    hash = nullptr;
}

cv::Ptr<cv::img_hash::BlockMeanHash>* block_mean_hash_new() {
    cv::Ptr<cv::img_hash::BlockMeanHash> result = cv::img_hash::BlockMeanHash::create();
    return new cv::Ptr<cv::img_hash::BlockMeanHash>(result);
}

void block_mean_hash_drop(cv::Ptr<cv::img_hash::BlockMeanHash>* hash) {
    delete hash;
    hash = nullptr;
}

cv::Ptr<cv::img_hash::ColorMomentHash>* color_moment_hash_new() {
    cv::Ptr<cv::img_hash::ColorMomentHash> result = cv::img_hash::ColorMomentHash::create();
    return new cv::Ptr<cv::img_hash::ColorMomentHash>(result);
}

void color_moment_hash_drop(cv::Ptr<cv::img_hash::ColorMomentHash>* hash) {
    delete hash;
    hash = nullptr;
}

cv::Ptr<cv::img_hash::MarrHildrethHash>* marr_hildreth_hash_new() {
    cv::Ptr<cv::img_hash::MarrHildrethHash> result = cv::img_hash::MarrHildrethHash::create();
    return new cv::Ptr<cv::img_hash::MarrHildrethHash>(result);
}
void marr_hildreth_hash_drop(cv::Ptr<cv::img_hash::MarrHildrethHash>* hash) {
    delete hash;
    hash = nullptr;
}

cv::Ptr<cv::img_hash::PHash>* phash_new() {
    cv::Ptr<cv::img_hash::PHash> result = cv::img_hash::PHash::create();
    return new cv::Ptr<cv::img_hash::PHash>(result);
}

void phash_drop(cv::Ptr<cv::img_hash::PHash>* hash) {
    delete hash;
    hash = nullptr;
}

cv::Ptr<cv::img_hash::RadialVarianceHash>* radial_variance_hash_new() {
    cv::Ptr<cv::img_hash::RadialVarianceHash> result = cv::img_hash::RadialVarianceHash::create();
    return new cv::Ptr<cv::img_hash::RadialVarianceHash>(result);
}

void radial_variance_hash_drop(cv::Ptr<cv::img_hash::RadialVarianceHash>* hash) {
    delete hash;
    hash = nullptr;
}

}  // namespace cvsys
