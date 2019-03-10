#include "highgui.hpp"

void cvsys_nat_named_window(const char* const winname, int flags) {
    cv::namedWindow(winname, flags);
}

void cvsys_nat_destroy_window(const char* const winname) {
    cv::destroyWindow(winname);
}

void cvsys_nat_set_mouse_callback(const char* const winname, cv::MouseCallback on_mouse, void* userdata) {
    cv::setMouseCallback(winname, on_mouse, userdata);
}

void cvsys_nat_imshow(const char* const winname, cv::Mat* mat) {
    if (mat != NULL) {
        cv::imshow(winname, *mat);
    }
}

int cvsys_nat_wait_key(int delay) {
    return cv::waitKey(delay);
}
