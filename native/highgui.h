#ifndef CV_RS_HIGHGUI_H
#define CV_RS_HIGHGUI_H

#include <opencv2/highgui.hpp>

extern "C" {

void cv_named_window(const char* const winname, int flags);
void cv_destroy_window(const char* const winname);
void cv_set_mouse_callback(const char* const winname, cv::MouseCallback onMouse, void* userdata);
}

#endif  // CV_RS_HIGHGUI_H
