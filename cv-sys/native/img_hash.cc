#include <opencv2/core.hpp>
#include <opencv2/img_hash.hpp>

void cvsys_hash_compute(cv::Ptr<cv::img_hash::PHash>* hash, cv::Mat& mat, cv::Mat& result) {
    hash->get()->compute(mat, result);
}

double cvsys_hash_compare(cv::Ptr<cv::img_hash::PHash>* hash, cv::Mat& lhs, cv::Mat& rhs) {
    return hash->get()->compare(lhs, rhs);
}

cv::Ptr<cv::img_hash::AverageHash>* cvsys_average_hash_new() {
    cv::Ptr<cv::img_hash::AverageHash> result = cv::img_hash::AverageHash::create();
    return new cv::Ptr<cv::img_hash::AverageHash>(result);
}

void cvsys_average_hash_drop(cv::Ptr<cv::img_hash::AverageHash>* hash) {
    delete hash;
    hash = nullptr;
}

cv::Ptr<cv::img_hash::BlockMeanHash>* cvsys_block_mean_hash_new() {
    cv::Ptr<cv::img_hash::BlockMeanHash> result = cv::img_hash::BlockMeanHash::create();
    return new cv::Ptr<cv::img_hash::BlockMeanHash>(result);
}

void cvsys_block_mean_hash_drop(cv::Ptr<cv::img_hash::BlockMeanHash>* hash) {
    delete hash;
    hash = nullptr;
}

cv::Ptr<cv::img_hash::ColorMomentHash>* cvsys_color_moment_hash_new() {
    cv::Ptr<cv::img_hash::ColorMomentHash> result = cv::img_hash::ColorMomentHash::create();
    return new cv::Ptr<cv::img_hash::ColorMomentHash>(result);
}

void cvsys_color_moment_hash_drop(cv::Ptr<cv::img_hash::ColorMomentHash>* hash) {
    delete hash;
    hash = nullptr;
}

cv::Ptr<cv::img_hash::MarrHildrethHash>* cvsys_marr_hildreth_hash_new() {
    cv::Ptr<cv::img_hash::MarrHildrethHash> result = cv::img_hash::MarrHildrethHash::create();
    return new cv::Ptr<cv::img_hash::MarrHildrethHash>(result);
}
void cvsys_marr_hildreth_hash_drop(cv::Ptr<cv::img_hash::MarrHildrethHash>* hash) {
    delete hash;
    hash = nullptr;
}

cv::Ptr<cv::img_hash::PHash>* cvsys_phash_new() {
    cv::Ptr<cv::img_hash::PHash> result = cv::img_hash::PHash::create();
    return new cv::Ptr<cv::img_hash::PHash>(result);
}

void cvsys_phash_drop(cv::Ptr<cv::img_hash::PHash>* hash) {
    delete hash;
    hash = nullptr;
}

cv::Ptr<cv::img_hash::RadialVarianceHash>* cvsys_radial_variance_hash_new() {
    cv::Ptr<cv::img_hash::RadialVarianceHash> result = cv::img_hash::RadialVarianceHash::create();
    return new cv::Ptr<cv::img_hash::RadialVarianceHash>(result);
}

void cvsys_radial_variance_hash_drop(cv::Ptr<cv::img_hash::RadialVarianceHash>* hash) {
    delete hash;
    hash = nullptr;
}
