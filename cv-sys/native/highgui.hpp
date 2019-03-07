#ifndef CV_RS_HIGHGUI_H
#define CV_RS_HIGHGUI_H

#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>

extern "C" {

void cv_named_window(const char* const winname, int flags);
void cv_destroy_window(const char* const winname);
void cv_set_mouse_callback(const char* const winname, cv::MouseCallback onMouse, void* userdata);
void cv_imshow(const char* const winname, cv::Mat* mat);
int cv_wait_key(int delay_in_millis);
}

#endif  // CV_RS_HIGHGUI_H
