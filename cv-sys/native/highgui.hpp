#ifndef CV_RS_HIGHGUI_H
#define CV_RS_HIGHGUI_H

#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>

void cvsys_nat_named_window(const char* const winname, int flags);
void cvsys_nat_destroy_window(const char* const winname);
void cvsys_nat_set_mouse_callback(const char* const winname, cv::MouseCallback onMouse, void* userdata);
void cvsys_nat_imshow(const char* const winname, cv::Mat* mat);
int cvsys_nat_wait_key(int delay_in_millis);

#endif  // CV_RS_HIGHGUI_H
