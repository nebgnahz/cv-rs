#ifndef CV_RS_VIDEOIO_H
#define CV_RS_VIDEOIO_H

#include <opencv2/videoio.hpp>

extern "C" {

void* cv_videocapture_new(int index);
void* cv_videocapture_from_file(const char* const filename);
bool cv_videocapture_is_opened(const cv::VideoCapture* const cap);
bool cv_videocapture_read(cv::VideoCapture* cap, cv::Mat* mat);
void cv_videocapture_drop(cv::VideoCapture* cap);
bool cv_videocapture_set(cv::VideoCapture* cap, int property, double value);
double cv_videocapture_get(cv::VideoCapture* cap, int property);
}
#endif  // CV_RS_VIDEOIO_H
