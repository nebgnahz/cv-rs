#ifndef CV_RS_VIDEO_H
#define CV_RS_VIDEO_H

#include "common.h"
#include <opencv2/video/tracking.hpp>
#include <opencv2/video/background_segm.hpp>

extern "C" {
    void* cv_term_criteria_new(int type, int count, double epsilon);
    void cv_term_criteria_drop(cv::TermCriteria* criteria);
    RotatedRect cv_camshift(cv::Mat* back_project_image, Rect window, cv::TermCriteria* criteria);

    // BackgroundSubtractorKNN

    void* cv_create_background_subtractor_knn(
        int history,
        double dist2Threshold,
        bool detectShadows
    );

    void* cv_create_background_subtractor_knn_default();

    void cv_background_subtractor_knn_drop(cv::Ptr<cv::BackgroundSubtractorKNN>* background_subtractor);

    void cv_background_subtractor_knn_apply(
        cv::Ptr<cv::BackgroundSubtractorKNN> background_subtractor,
        cv::Mat* input_image,
        cv::Mat* output_foreground,
        double learning_rate
    );

    void cv_background_subtractor_knn_get_background_image(
        cv::Ptr<cv::BackgroundSubtractorKNN> background_subtractor,
        cv::Mat* output_background
    );

    // BackgroundSubtractorMOG2

    void* cv_create_background_subtractor_mog2(
        int history,
        double dist2Threshold,
        bool detectShadows
    );

    void* cv_create_background_subtractor_mog2_default();

    void cv_background_subtractor_mog2_drop(cv::Ptr<cv::BackgroundSubtractorMOG2>* background_subtractor);

    void cv_background_subtractor_mog2_apply(
        cv::Ptr<cv::BackgroundSubtractorMOG2> background_subtractor,
        cv::Mat* input_image,
        cv::Mat* output_foreground,
        double learning_rate
    );

    void cv_background_subtractor_mog2_get_background_image(
        cv::Ptr<cv::BackgroundSubtractorMOG2> background_subtractor,
        cv::Mat* output_background
    );

}
#endif  // CV_RS_VIDEO_H
