#include "videoio.h"

extern "C" {

void* cv_videocapture_new(int index) {
    return new cv::VideoCapture(index);
}

void* cv_videocapture_from_file(const char* const filename) {
    return new cv::VideoCapture(filename);
}

bool cv_videocapture_is_opened(const cv::VideoCapture* const cap) {
    return cap->isOpened();
}

bool cv_videocapture_read(cv::VideoCapture* cap, cv::Mat* mat) {
    return cap->read(*mat);
}

void cv_videocapture_drop(cv::VideoCapture* cap) {
    delete cap;
    cap = nullptr;
}

bool cv_videocapture_set(cv::VideoCapture* cap, int property, double value) {
    return cap->set(property, value);
}

double cv_videocapture_get(cv::VideoCapture* cap, int property) {
    return cap->get(property);
}
}
