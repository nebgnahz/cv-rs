#ifndef UTILS_H_
#define UTILS_H_

#include "utils.h"
#include "common.h"
#include <opencv2/core.hpp>
#include <vector>

void cv_to_ffi(const cv::Rect& source, Rect* dest) {
    dest->x = source.x;
    dest->y = source.y;
    dest->width = source.width;
    dest->height = source.height;
}

void cv_to_ffi(const cv::Point& source, Point2i* dest) {
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

void cv_to_ffi(const std::vector<double>& source, CVec<double>* dest) {
    size_t num = source.size();
    dest->size = num;
    dest->array = (double*)malloc(num * sizeof(double));
    ::memcpy(dest->array, source.data(), num * sizeof(double));
}

template <typename T, typename U>
void cv_to_ffi(const std::vector<T>& source, CVec<U>* dest) {
    size_t num = source.size();
    dest->size = num;
    dest->array = (U*)malloc(num * sizeof(U));
    for (size_t i = 0; i < num; i++) {
        cv_to_ffi(source[i], &dest->array[i]);
    }
}
#endif
