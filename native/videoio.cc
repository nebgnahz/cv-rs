#include "videoio.h"

extern "C" {

void* cv_videocapture_new(int index) {
    return new cv::VideoCapture(index);
}

void* cv_videocapture_from_file(const char* const filename) {
    return new cv::VideoCapture(filename);
}

void* cv_videocapture_from_gst_pipeline(const char* const pipeline) {
    return new cv::VideoCapture(pipeline, cv::CAP_GSTREAMER);
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

void* cv_videowriter_default() {
    return new cv::VideoWriter();
}

void* cv_videowriter_new(const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color) {
    cv::Size cv_frame_size(frame_size.width, frame_size.height);
    cv::VideoWriter* writer = new cv::VideoWriter(path, fourcc, fps, cv_frame_size, is_color);
    return writer;
}

void cv_videowriter_drop(cv::VideoWriter* writer) {
    delete writer;
    writer = nullptr;
}

bool cv_videowriter_open(
    cv::VideoWriter* writer, const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color) {
    cv::Size cv_frame_size(frame_size.width, frame_size.height);
    return writer->open(path, fourcc, fps, cv_frame_size, is_color);
}

bool cv_videowriter_is_opened(cv::VideoWriter* writer) {
    return writer->isOpened();
}

void cv_videowriter_write(cv::VideoWriter* writer, cv::Mat* mat) {
    (*writer) << (*mat);
}

bool cv_videowriter_set(cv::VideoWriter* writer, int property, double value) {
    return writer->set(property, value);
}

double cv_videowriter_get(cv::VideoWriter* writer, int property) {
    return writer->get(property);
}
}
