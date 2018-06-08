#ifndef HASH_H_
#define HASH_H_

#include <opencv2/core.hpp>
#include <opencv2/img_hash.hpp>

extern "C" {

void* cv_phash_new();
void cv_phash_drop(cv::Ptr<cv::img_hash::PHash>* phash);
void cv_hash_compute(cv::Ptr<cv::img_hash::PHash>* phash, cv::Mat& mat, cv::Mat& result);
void cv_hash_compare(cv::Ptr<cv::img_hash::PHash>* phash, cv::Mat& lhs, cv::Mat& rhs);
}
#endif
