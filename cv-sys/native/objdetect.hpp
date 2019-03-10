#ifndef CV_RS_OBJDETECT_H
#define CV_RS_OBJDETECT_H

#include "common.hpp"
#include <opencv2/objdetect.hpp>

namespace cvsys {

cv::CascadeClassifier* cascade_classifier_new();
cv::CascadeClassifier* cascade_classifier_from_path(const char* const path);
bool cascade_classifier_load(cv::CascadeClassifier* cc, const char* const path);
void cascade_classifier_drop(cv::CascadeClassifier* cc);
void cascade_classifier_detect(cv::CascadeClassifier* cascade,
                               cv::Mat* mat,
                               CVec<Rect>* vec_of_rect,
                               double scale_factor,
                               int min_neighbors,
                               int flags,
                               Size2i min_size,
                               Size2i max_size);

cv::HOGDescriptor* hog_new();
void hog_drop(cv::HOGDescriptor*);
std::vector<float>* hog_default_people_detector();
std::vector<float>* hog_daimler_people_detector();
void hog_detector_drop(std::vector<float>*);
void hog_set_svm_detector(cv::HOGDescriptor*, std::vector<float>*);
void hog_detect(cv::HOGDescriptor*,
                cv::Mat*,
                CVec<Rect>* vec_detected,
                CVec<double>* vec_weight,
                Size2i win_stride,
                Size2i padding,
                double scale,
                double final_threshold,
                bool use_means_shift);

}  // namespace cvsys

#endif  // CV_RS_OBJDETECT_H
