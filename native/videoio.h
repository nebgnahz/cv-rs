#ifndef CV_RS_VIDEOIO_H
#define CV_RS_VIDEOIO_H

#include "common.h"
#include <opencv2/videoio.hpp>

extern "C" {

void* cv_videocapture_new(int index);
void* cv_videocapture_from_file(const char* const filename);
void* cv_videocapture_from_gst_pipeline(const char* const pipeline);
bool cv_videocapture_is_opened(const cv::VideoCapture* const cap);
bool cv_videocapture_read(cv::VideoCapture* cap, cv::Mat* mat);
void cv_videocapture_drop(cv::VideoCapture* cap);
bool cv_videocapture_set(cv::VideoCapture* cap, int property, double value);
double cv_videocapture_get(cv::VideoCapture* cap, int property);

void* cv_videowriter_default();
void* cv_videowriter_new(const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color);
void cv_videowriter_drop(cv::VideoWriter* writer);
bool cv_videowriter_open(
    cv::VideoWriter* writer, const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color);
bool cv_videowriter_is_opened(cv::VideoWriter* writer);
void cv_videowriter_write(cv::VideoWriter* writer, cv::Mat* mat);
bool cv_videowriter_set(cv::VideoWriter* writer, int property, double value);
double cv_videowriter_get(cv::VideoWriter* writer, int property);
}
#endif  // CV_RS_VIDEOIO_H
