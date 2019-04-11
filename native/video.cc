#include "video.h"

extern "C" {

void* cv_term_criteria_new(int type, int count, double epsilon) {
    return new cv::TermCriteria(type, count, epsilon);
}

void cv_term_criteria_drop(cv::TermCriteria* criteria) {
    delete criteria;
    criteria = nullptr;
}

RotatedRect cv_camshift(cv::Mat* bp_image, Rect crect, cv::TermCriteria* criteria) {
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

// BackgroundSubtractorKNN

void* cv_create_background_subtractor_knn(int history, double dist2Threshold, bool detectShadows) {
    cv::Ptr<cv::BackgroundSubtractorKNN> background_subtractor =
        cv::createBackgroundSubtractorKNN(history, dist2Threshold, detectShadows);
    return new cv::Ptr<cv::BackgroundSubtractorKNN>(background_subtractor);
};

void* cv_create_background_subtractor_knn_default() {
    cv::Ptr<cv::BackgroundSubtractorKNN> background_subtractor = cv::createBackgroundSubtractorKNN();
    return new cv::Ptr<cv::BackgroundSubtractorKNN>(background_subtractor);
};

void cv_background_subtractor_knn_drop(cv::Ptr<cv::BackgroundSubtractorKNN>* background_subtractor) {
    delete background_subtractor;
    background_subtractor = nullptr;
}

void cv_background_subtractor_knn_apply(cv::Ptr<cv::BackgroundSubtractorKNN> background_subtractor,
                                        cv::Mat* input_image,
                                        cv::Mat* output_foreground,
                                        double learning_rate) {
    background_subtractor->apply(*input_image, *output_foreground, learning_rate);
};

void cv_background_subtractor_knn_get_background_image(cv::Ptr<cv::BackgroundSubtractorKNN> background_subtractor,
                                                       cv::Mat* output_background) {
    background_subtractor->getBackgroundImage(*output_background);
};

// BackgroundSubtractorMOG2

void* cv_create_background_subtractor_mog2(int history, double dist2Threshold, bool detectShadows) {
    cv::Ptr<cv::BackgroundSubtractorMOG2> background_subtractor =
        cv::createBackgroundSubtractorMOG2(history, dist2Threshold, detectShadows);
    return new cv::Ptr<cv::BackgroundSubtractorMOG2>(background_subtractor);
};

void* cv_create_background_subtractor_mog2_default() {
    cv::Ptr<cv::BackgroundSubtractorMOG2> background_subtractor = cv::createBackgroundSubtractorMOG2();
    return new cv::Ptr<cv::BackgroundSubtractorMOG2>(background_subtractor);
};

void cv_background_subtractor_mog2_drop(cv::Ptr<cv::BackgroundSubtractorMOG2>* background_subtractor) {
    delete background_subtractor;
    background_subtractor = nullptr;
}

void cv_background_subtractor_mog2_apply(cv::Ptr<cv::BackgroundSubtractorMOG2> background_subtractor,
                                         cv::Mat* input_image,
                                         cv::Mat* output_foreground,
                                         double learning_rate) {
    background_subtractor->apply(*input_image, *output_foreground, learning_rate);
};

void cv_background_subtractor_mog2_get_background_image(cv::Ptr<cv::BackgroundSubtractorMOG2> background_subtractor,
                                                        cv::Mat* output_background) {
    background_subtractor->getBackgroundImage(*output_background);
};
}