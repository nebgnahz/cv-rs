#include "highgui.hpp"

namespace cvsys {

void nat_named_window(const char* const winname, int flags) {
    cv::namedWindow(winname, flags);
}

void nat_destroy_window(const char* const winname) {
    cv::destroyWindow(winname);
}

void nat_set_mouse_callback(const char* const winname,
                            void (*on_mouse)(int event, int x, int y, int flags, void* userdata),
                            void* userdata) {
    cv::setMouseCallback(winname, on_mouse, userdata);
}

void nat_imshow(const char* const winname, cv::Mat* mat) {
    if (mat != NULL) {
        cv::imshow(winname, *mat);
    }
}

int nat_wait_key(int delay) {
    return cv::waitKey(delay);
}

}  // namespace cvsys
