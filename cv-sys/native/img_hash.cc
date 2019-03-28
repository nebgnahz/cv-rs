#include "img_hash.hpp"

namespace cvsys {

void hash_any_compute(PHash* hash, cv::Mat& mat, cv::Mat& result) {
    hash->get()->compute(mat, result);
}

double hash_any_compare(PHash* hash, cv::Mat& lhs, cv::Mat& rhs) {
    return hash->get()->compare(lhs, rhs);
}

AverageHash* average_hash_new() {
    cv::Ptr<cv::img_hash::AverageHash> result = cv::img_hash::AverageHash::create();
    return new AverageHash(result);
}

void average_hash_drop(AverageHash* hash) {
    delete hash;
}

BlockMeanHash* block_mean_hash_new() {
    cv::Ptr<cv::img_hash::BlockMeanHash> result = cv::img_hash::BlockMeanHash::create();
    return new BlockMeanHash(result);
}

void block_mean_hash_drop(BlockMeanHash* hash) {
    delete hash;
}

ColorMomentHash* color_moment_hash_new() {
    cv::Ptr<cv::img_hash::ColorMomentHash> result = cv::img_hash::ColorMomentHash::create();
    return new ColorMomentHash(result);
}

void color_moment_hash_drop(ColorMomentHash* hash) {
    delete hash;
}

MarrHildrethHash* marr_hildreth_hash_new() {
    cv::Ptr<cv::img_hash::MarrHildrethHash> result = cv::img_hash::MarrHildrethHash::create();
    return new MarrHildrethHash(result);
}
void marr_hildreth_hash_drop(MarrHildrethHash* hash) {
    delete hash;
}

PHash* phash_new() {
    cv::Ptr<cv::img_hash::PHash> result = cv::img_hash::PHash::create();
    return new PHash(result);
}

void phash_drop(PHash* hash) {
    delete hash;
}

RadialVarianceHash* radial_variance_hash_new() {
    cv::Ptr<cv::img_hash::RadialVarianceHash> result = cv::img_hash::RadialVarianceHash::create();
    return new RadialVarianceHash(result);
}

void radial_variance_hash_drop(RadialVarianceHash* hash) {
    delete hash;
}

}  // namespace cvsys
