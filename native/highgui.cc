#include "highgui.h"

extern "C" {
void cv_named_window(const char* const winname, int flags) {
    cv::namedWindow(winname, flags);
}

void cv_destroy_window(const char* const winname) {
    cv::destroyWindow(winname);
}

void cv_set_mouse_callback(const char* const winname, cv::MouseCallback on_mouse, void* userdata) {
    cv::setMouseCallback(winname, on_mouse, userdata);
}
}
