#include <opencv2/core.hpp>
#include <vector>

#include "common.h"
#include "utils.h"

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

void cv_to_ffi(const cv::DMatch& source, DMatch* dest) {
    dest->distance = dest->distance;
    dest->imgIdx = dest->imgIdx;
    dest->queryIdx = source.queryIdx;
    dest->trainIdx = source.trainIdx;
}

void cv_to_ffi(const std::string& source, CDisposableString* dest) {
    char* result = new char[source.length() + 1];
    strcpy(result, source.c_str());
    dest->value = result;
}

void ffi_to_cv(const cv::Mat& source, cv::Mat* dest) {
    *dest = source;
}
