#include "videoio.hpp"

namespace cvsys {

cv::VideoCapture* videocapture_new(int index) {
    return new cv::VideoCapture(index);
}

cv::VideoCapture* videocapture_from_file(const char* const filename) {
    return new cv::VideoCapture(filename);
}

cv::VideoCapture* videocapture_from_gst_pipeline(const char* const pipeline) {
    return new cv::VideoCapture(pipeline, cv::CAP_GSTREAMER);
}

bool videocapture_is_opened(const cv::VideoCapture* const cap) {
    return cap->isOpened();
}

bool videocapture_read(cv::VideoCapture* cap, cv::Mat* mat) {
    return cap->read(*mat);
}

void videocapture_drop(cv::VideoCapture* cap) {
    delete cap;
    cap = nullptr;
}

bool videocapture_set(cv::VideoCapture* cap, int property, double value) {
    return cap->set(property, value);
}

double videocapture_get(cv::VideoCapture* cap, int property) {
    return cap->get(property);
}

cv::VideoWriter* videowriter_default() {
    return new cv::VideoWriter();
}

cv::VideoWriter* videowriter_new(const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color) {
    cv::Size native_frame_size(frame_size.width, frame_size.height);
    cv::VideoWriter* writer = new cv::VideoWriter(path, fourcc, fps, native_frame_size, is_color);
    return writer;
}

void videowriter_drop(cv::VideoWriter* writer) {
    delete writer;
    writer = nullptr;
}

bool videowriter_open(
    cv::VideoWriter* writer, const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color) {
    cv::Size native_frame_size(frame_size.width, frame_size.height);
    return writer->open(path, fourcc, fps, native_frame_size, is_color);
}

bool videowriter_is_opened(cv::VideoWriter* writer) {
    return writer->isOpened();
}

void videowriter_write(cv::VideoWriter* writer, cv::Mat* mat) {
    (*writer) << (*mat);
}

bool videowriter_set(cv::VideoWriter* writer, int property, double value) {
    return writer->set(property, value);
}

double videowriter_get(cv::VideoWriter* writer, int property) {
    return writer->get(property);
}

}  // namespace cvsys
