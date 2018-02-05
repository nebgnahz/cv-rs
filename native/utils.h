#ifndef UTILS_H_
#define UTILS_H_

#include <vector>
#include <opencv2/core.hpp>
#include "common.h"

void cv_to_ffi(const cv::Rect& source, Rect* dest);
void cv_to_ffi(const cv::Point& source, Point2i* dest);
void cv_to_ffi(const cv::KeyPoint& source, KeyPoint* dest);
void cv_to_ffi(const cv::DMatch& source, DMatch* dest);
void cv_to_ffi(const std::vector<double>& source, CVec<double>* dest);

template <typename T, typename U>
void cv_to_ffi(const std::vector<T>& source, CVec<U>* dest)
{
    size_t num = source.size();
    dest->size = num;
    dest->array = (U*) malloc(num * sizeof(U));
    for (size_t i = 0; i < num; i++) {
        cv_to_ffi(source[i], &dest->array[i]);
    }
}
#endif  // UTILS_H_
