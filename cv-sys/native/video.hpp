#ifndef CV_RS_VIDEO_H
#define CV_RS_VIDEO_H

#include "common.hpp"
#include <opencv2/video/tracking.hpp>

namespace cvsys {

cv::TermCriteria* term_criteria_new(int type, int count, double epsilon);
void term_criteria_drop(cv::TermCriteria* criteria);
RotatedRect camshift(cv::Mat* back_project_image, Rect window, cv::TermCriteria* criteria);

}  // namespace cvsys

#endif  // CV_RS_VIDEO_H
