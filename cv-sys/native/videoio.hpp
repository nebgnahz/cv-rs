#ifndef CV_RS_VIDEOIO_H
#define CV_RS_VIDEOIO_H

#include "common.hpp"
#include <opencv2/videoio.hpp>

cv::VideoCapture* cvsys_videocapture_new(int index);
cv::VideoCapture* cvsys_videocapture_from_file(const char* const filename);
cv::VideoCapture* cvsys_videocapture_from_gst_pipeline(const char* const pipeline);
bool cvsys_videocapture_is_opened(const cv::VideoCapture* const cap);
bool cvsys_videocapture_read(cv::VideoCapture* cap, cv::Mat* mat);
void cvsys_videocapture_drop(cv::VideoCapture* cap);
bool cvsys_videocapture_set(cv::VideoCapture* cap, int property, double value);
double cvsys_videocapture_get(cv::VideoCapture* cap, int property);

cv::VideoWriter* cvsys_videowriter_default();
cv::VideoWriter*
cvsys_videowriter_new(const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color);
void cvsys_videowriter_drop(cv::VideoWriter* writer);
bool cvsys_videowriter_open(
    cv::VideoWriter* writer, const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color);
bool cvsys_videowriter_is_opened(cv::VideoWriter* writer);
void cvsys_videowriter_write(cv::VideoWriter* writer, cv::Mat* mat);
bool cvsys_videowriter_set(cv::VideoWriter* writer, int property, double value);
double cvsys_videowriter_get(cv::VideoWriter* writer, int property);

#endif  // CV_RS_VIDEOIO_H
