#ifndef PHASH_H_
#define PHASH_H_

#include <opencv2/core.hpp>
#include <opencv2/img_hash.hpp>

extern "C" {

void* cv_phash_new();
void cv_phash_drop(cv::Ptr<cv::img_hash::PHash>* phash);
void cv_phash_compute(cv::Ptr<cv::img_hash::PHash>* phash, cv::Mat& mat, cv::Mat& result);
void cv_phash_compare(cv::Ptr<cv::img_hash::PHash>* phash, cv::Mat& lhs, cv::Mat& rhs);
}
#endif
