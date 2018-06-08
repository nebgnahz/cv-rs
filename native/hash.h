#ifndef HASH_H_
#define HASH_H_

#include <opencv2/core.hpp>
#include <opencv2/img_hash.hpp>

extern "C" {

void cv_hash_compute(cv::Ptr<cv::img_hash::ImgHashBase>* phash, cv::Mat& mat, cv::Mat& result);
void cv_hash_compare(cv::Ptr<cv::img_hash::ImgHashBase>* phash, cv::Mat& lhs, cv::Mat& rhs);

void* cv_average_hash_new();
void cv_average_hash_drop(cv::Ptr<cv::img_hash::AverageHash>* phash);

void* cv_block_mean_hash_new();
void cv_block_mean_hash_drop(cv::Ptr<cv::img_hash::BlockMeanHash>* phash);

void* cv_color_moment_hash_new();
void cv_color_moment_hash_drop(cv::Ptr<cv::img_hash::ColorMomentHash>* phash);

void* cv_marr_hildreth_hash_new();
void cv_marr_hildreth_hash_drop(cv::Ptr<cv::img_hash::MarrHildrethHash>* phash);

void* cv_phash_new();
void cv_phash_drop(cv::Ptr<cv::img_hash::PHash>* phash);

void* cv_radial_variance_hash_new();
void cv_radial_variance_hash_drop(cv::Ptr<cv::img_hash::RadialVarianceHash>* phash);
}
#endif
