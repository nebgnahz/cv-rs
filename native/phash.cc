#include <opencv2/core.hpp>
#include <opencv2/img_hash.hpp>

extern "C" {

void* cv_phash_new() {
    cv::Ptr<cv::img_hash::PHash> result = cv::img_hash::PHash::create();
    return new cv::Ptr<cv::img_hash::PHash>(result);
}

void cv_phash_drop(cv::Ptr<cv::img_hash::PHash>* phash) {
    delete phash;
    phash = nullptr;
}

void cv_phash_compute(cv::Ptr<cv::img_hash::PHash>* phash, cv::Mat& mat, cv::Mat& result) {
    phash->get()->compute(mat, result);
}

double cv_phash_compare(cv::Ptr<cv::img_hash::PHash>* phash, cv::Mat& lhs, cv::Mat& rhs) {
    return phash->get()->compare(lhs, rhs);
}
}