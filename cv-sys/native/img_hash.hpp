#ifndef HASH_H_
#define HASH_H_

#include <opencv2/core.hpp>
#include <opencv2/img_hash.hpp>

void cvsys_hash_compute(cv::Ptr<cv::img_hash::ImgHashBase>* phash, cv::Mat& mat, cv::Mat& result);
double cvsys_hash_compare(cv::Ptr<cv::img_hash::ImgHashBase>* phash, cv::Mat& lhs, cv::Mat& rhs);

cv::Ptr<cv::img_hash::AverageHash>* cvsys_average_hash_new();
void cvsys_average_hash_drop(cv::Ptr<cv::img_hash::AverageHash>* phash);

cv::Ptr<cv::img_hash::BlockMeanHash>* cvsys_block_mean_hash_new();
void cvsys_block_mean_hash_drop(cv::Ptr<cv::img_hash::BlockMeanHash>* phash);

cv::Ptr<cv::img_hash::ColorMomentHash>* cvsys_color_moment_hash_new();
void cvsys_color_moment_hash_drop(cv::Ptr<cv::img_hash::ColorMomentHash>* phash);

cv::Ptr<cv::img_hash::MarrHildrethHash>* cvsys_marr_hildreth_hash_new();
void cvsys_marr_hildreth_hash_drop(cv::Ptr<cv::img_hash::MarrHildrethHash>* phash);

cv::Ptr<cv::img_hash::PHash>* cvsys_phash_new();
void cvsys_phash_drop(cv::Ptr<cv::img_hash::PHash>* phash);

cv::Ptr<cv::img_hash::RadialVarianceHash>* cvsys_radial_variance_hash_new();
void cvsys_radial_variance_hash_drop(cv::Ptr<cv::img_hash::RadialVarianceHash>* phash);

#endif
