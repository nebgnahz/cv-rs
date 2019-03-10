#ifndef CV_RS_VIDEOIO_H
#define CV_RS_VIDEOIO_H

#include "common.hpp"
#include <opencv2/videoio.hpp>

namespace cvsys {

cv::VideoCapture* videocapture_new(int index);
cv::VideoCapture* videocapture_from_file(const char* const filename);
cv::VideoCapture* videocapture_from_gst_pipeline(const char* const pipeline);
bool videocapture_is_opened(const cv::VideoCapture* const cap);
bool videocapture_read(cv::VideoCapture* cap, cv::Mat* mat);
void videocapture_drop(cv::VideoCapture* cap);
bool videocapture_set(cv::VideoCapture* cap, int property, double value);
double videocapture_get(cv::VideoCapture* cap, int property);

cv::VideoWriter* videowriter_default();
cv::VideoWriter* videowriter_new(const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color);
void videowriter_drop(cv::VideoWriter* writer);
bool videowriter_open(
    cv::VideoWriter* writer, const char* const path, int fourcc, double fps, Size2i frame_size, bool is_color);
bool videowriter_is_opened(cv::VideoWriter* writer);
void videowriter_write(cv::VideoWriter* writer, cv::Mat* mat);
bool videowriter_set(cv::VideoWriter* writer, int property, double value);
double videowriter_get(cv::VideoWriter* writer, int property);

}  // namespace cvsys

#endif  // CV_RS_VIDEOIO_H
