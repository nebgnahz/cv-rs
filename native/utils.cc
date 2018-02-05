#ifndef UTILS_H_
#define UTILS_H_

#include <vector>
#include <opencv2/core.hpp>
#include "common.h"
#include "utils.h"

void cv_to_ffi(const cv::Rect& source, Rect* dest){
    dest->x = source.x;
    dest->y = source.y;
    dest->width = source.width;
    dest->height = source.height;
}

void cv_to_ffi(const cv::Point& source, Point2i* dest){
    dest->x = source.x;
    dest->y = source.y;
};

void cv_to_ffi(const cv::KeyPoint& source, KeyPoint* dest) {
    dest->pt.x = dest->pt.x;
    dest->pt.y = dest->pt.y;
    dest->size = source.size;
    dest->angle = source.angle;
    dest->response = source.response;
    dest->octave = source.octave;
    dest->class_id = source.class_id;
}

void cv_to_ffi(const cv::DMatch& source, DMatch* dest){
    dest->distance = dest->distance;
    dest->imgIdx = dest->imgIdx;
    dest->queryIdx = source.queryIdx;
    dest->trainIdx = source.trainIdx;
}

void cv_to_ffi(const std::vector<double>& source, CVec<double>* dest) {
    size_t num = source.size();
    dest->size = num;
    dest->array = (double*) malloc(num * sizeof(double));
    ::memcpy(dest->array, source.data(), num * sizeof(double));
}

void ffi_to_cv(const cv::Mat& source, cv::Mat* dest) {
    *dest = source;
}
#endif
