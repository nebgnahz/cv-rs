#ifndef CV_RS_HIGHGUI_H
#define CV_RS_HIGHGUI_H

#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>

namespace cvsys {

void nat_named_window(const char* const winname, int flags);
void nat_destroy_window(const char* const winname);
void nat_set_mouse_callback(const char* const winname, cv::MouseCallback onMouse, void* userdata);
void nat_imshow(const char* const winname, cv::Mat* mat);
int nat_wait_key(int delay_in_millis);

}  // namespace cvsys

#endif  // CV_RS_HIGHGUI_H
