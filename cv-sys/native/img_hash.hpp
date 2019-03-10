#ifndef HASH_H_
#define HASH_H_

#include <opencv2/core.hpp>
#include <opencv2/img_hash.hpp>

namespace cvsys {

void hash_compute(cv::Ptr<cv::img_hash::ImgHashBase>* phash, cv::Mat& mat, cv::Mat& result);
double hash_compare(cv::Ptr<cv::img_hash::ImgHashBase>* phash, cv::Mat& lhs, cv::Mat& rhs);

cv::Ptr<cv::img_hash::AverageHash>* average_hash_new();
void average_hash_drop(cv::Ptr<cv::img_hash::AverageHash>* phash);

cv::Ptr<cv::img_hash::BlockMeanHash>* block_mean_hash_new();
void block_mean_hash_drop(cv::Ptr<cv::img_hash::BlockMeanHash>* phash);

cv::Ptr<cv::img_hash::ColorMomentHash>* color_moment_hash_new();
void color_moment_hash_drop(cv::Ptr<cv::img_hash::ColorMomentHash>* phash);

cv::Ptr<cv::img_hash::MarrHildrethHash>* marr_hildreth_hash_new();
void marr_hildreth_hash_drop(cv::Ptr<cv::img_hash::MarrHildrethHash>* phash);

cv::Ptr<cv::img_hash::PHash>* phash_new();
void phash_drop(cv::Ptr<cv::img_hash::PHash>* phash);

cv::Ptr<cv::img_hash::RadialVarianceHash>* radial_variance_hash_new();
void radial_variance_hash_drop(cv::Ptr<cv::img_hash::RadialVarianceHash>* phash);

}  // namespace cvsys

#endif
