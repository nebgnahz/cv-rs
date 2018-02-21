#ifndef CV_RS_OBJDETECT_H
#define CV_RS_OBJDETECT_H

#include "common.h"
#include <opencv2/objdetect.hpp>

extern "C" {

void* cv_cascade_classifier_new();
void* cv_cascade_classifier_from_path(const char* const path);
bool cv_cascade_classifier_load(cv::CascadeClassifier* cc, const char* const path);
void cv_cascade_classifier_drop(cv::CascadeClassifier* cc);
void cv_cascade_classifier_detect(cv::CascadeClassifier* cascade,
                                  cv::Mat* mat,
                                  CVec<Rect>* vec_of_rect,
                                  double scale_factor,
                                  int min_neighbors,
                                  int flags,
                                  Size2i min_size,
                                  Size2i max_size);

void* cv_hog_new();
void cv_hog_drop(cv::HOGDescriptor*);
void* cv_hog_default_people_detector();
void* cv_hog_daimler_people_detector();
void cv_hog_detector_drop(std::vector<float>*);
void cv_hog_set_svm_detector(cv::HOGDescriptor*, std::vector<float>*);
void cv_hog_detect(cv::HOGDescriptor*,
                   cv::Mat*,
                   CVec<Rect>* vec_detected,
                   CVec<double>* vec_weight,
                   Size2i win_stride,
                   Size2i padding,
                   double scale,
                   double final_threshold,
                   bool use_means_shift);
}

#endif  // CV_RS_OBJDETECT_H
