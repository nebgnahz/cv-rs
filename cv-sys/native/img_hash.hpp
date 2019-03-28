#ifndef HASH_H_
#define HASH_H_

#include <opencv2/core.hpp>
#include <opencv2/img_hash.hpp>

namespace cvsys {

struct AverageHash : cv::Ptr<cv::img_hash::AverageHash> {
    AverageHash(cv::Ptr<cv::img_hash::AverageHash> p) : cv::Ptr<cv::img_hash::AverageHash>(p) {
    }
};

struct BlockMeanHash : cv::Ptr<cv::img_hash::BlockMeanHash> {
    BlockMeanHash(cv::Ptr<cv::img_hash::BlockMeanHash> p) : cv::Ptr<cv::img_hash::BlockMeanHash>(p) {
    }
};

struct ColorMomentHash : cv::Ptr<cv::img_hash::ColorMomentHash> {
    ColorMomentHash(cv::Ptr<cv::img_hash::ColorMomentHash> p) : cv::Ptr<cv::img_hash::ColorMomentHash>(p) {
    }
};

struct MarrHildrethHash : cv::Ptr<cv::img_hash::MarrHildrethHash> {
    MarrHildrethHash(cv::Ptr<cv::img_hash::MarrHildrethHash> p) : cv::Ptr<cv::img_hash::MarrHildrethHash>(p) {
    }
};

struct PHash : cv::Ptr<cv::img_hash::PHash> {
    PHash(cv::Ptr<cv::img_hash::PHash> p) : cv::Ptr<cv::img_hash::PHash>(p) {
    }
};

struct RadialVarianceHash : cv::Ptr<cv::img_hash::RadialVarianceHash> {
    RadialVarianceHash(cv::Ptr<cv::img_hash::RadialVarianceHash> p) : cv::Ptr<cv::img_hash::RadialVarianceHash>(p) {
    }
};

void hash_any_compute(PHash* phash, cv::Mat& mat, cv::Mat& result);
double hash_any_compare(PHash* phash, cv::Mat& lhs, cv::Mat& rhs);

AverageHash* average_hash_new();
void average_hash_drop(AverageHash* phash);

BlockMeanHash* block_mean_hash_new();
void block_mean_hash_drop(BlockMeanHash* phash);

ColorMomentHash* color_moment_hash_new();
void color_moment_hash_drop(ColorMomentHash* phash);

MarrHildrethHash* marr_hildreth_hash_new();
void marr_hildreth_hash_drop(MarrHildrethHash* phash);

PHash* phash_new();
void phash_drop(PHash* phash);

RadialVarianceHash* radial_variance_hash_new();
void radial_variance_hash_drop(RadialVarianceHash* phash);

}  // namespace cvsys

#endif
