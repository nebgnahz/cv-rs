#ifndef CV_RS_VIDEO_H
#define CV_RS_VIDEO_H

#include "common.h"
#include <opencv2/video/tracking.hpp>

extern "C" {

void* cv_term_criteria_new(int type, int count, double epsilon);
void cv_term_criteria_drop(cv::TermCriteria* criteria);
RotatedRect cv_camshift(cv::Mat* back_project_image, Rect window, cv::TermCriteria* criteria);
}
#endif  // CV_RS_VIDEO_H
