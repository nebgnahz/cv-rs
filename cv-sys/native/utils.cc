#include <opencv2/core.hpp>
#include <vector>

#include "common.hpp"
#include "utils.hpp"

void cvsys_to_ffi(const cv::Rect& source, Rect* dest) {
    dest->x = source.x;
    dest->y = source.y;
    dest->width = source.width;
    dest->height = source.height;
}

void cvsys_to_ffi(const cv::Point& source, Point2i* dest) {
    dest->x = source.x;
    dest->y = source.y;
};

void cvsys_to_ffi(const cv::KeyPoint& source, KeyPoint* dest) {
    dest->pt.x = source.pt.x;
    dest->pt.y = source.pt.y;
    dest->size = source.size;
    dest->angle = source.angle;
    dest->response = source.response;
    dest->octave = source.octave;
    dest->class_id = source.class_id;
}

void cvsys_to_ffi(const cv::DMatch& source, DMatch* dest) {
    dest->distance = source.distance;
    dest->imgIdx = source.imgIdx;
    dest->queryIdx = source.queryIdx;
    dest->trainIdx = source.trainIdx;
}

void cvsys_to_ffi(const std::string& source, CString* dest) {
    *dest = CString(source.c_str());
}

void ffi_to_cv(const cv::Mat& source, cv::Mat* dest) {
    *dest = source;
}
