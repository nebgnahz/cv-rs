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
}

#endif  // CV_RS_OBJDETECT_H
