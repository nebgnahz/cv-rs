#include "video.hpp"

cv::TermCriteria* cvsys_term_criteria_new(int type, int count, double epsilon) {
    return new cv::TermCriteria(type, count, epsilon);
}

void cvsys_term_criteria_drop(cv::TermCriteria* criteria) {
    delete criteria;
    criteria = nullptr;
}

RotatedRect cvsys_camshift(cv::Mat* bp_image, Rect crect, cv::TermCriteria* criteria) {
    cv::Rect rect(crect.x, crect.y, crect.width, crect.height);
    cv::RotatedRect rr = cv::CamShift(*bp_image, rect, *criteria);
    RotatedRect c_rr;
    c_rr.center.x = rr.center.x;
    c_rr.center.y = rr.center.y;
    c_rr.size.width = rr.size.width;
    c_rr.size.height = rr.size.height;
    c_rr.angle = rr.angle;
    return c_rr;
}
