#include "objdetect.h"

extern "C" {

void* cv_cascade_classifier_new() {
    return new cv::CascadeClassifier();
}

bool cv_cascade_classifier_load(cv::CascadeClassifier* cascade, const char* const p) {
    return cascade->load(p);
}

void* cv_cascade_classifier_from_path(const char* const p) {
    return new cv::CascadeClassifier(p);
}

void cv_cascade_classifier_drop(cv::CascadeClassifier* cascade) {
    delete cascade;
    cascade = nullptr;
}

void cv_cascade_classifier_detect(cv::CascadeClassifier* cascade,
                                  cv::Mat* image,
                                  CVec<Rect>* vec_of_rect,
                                  double scale_factor,
                                  int min_neighbors,
                                  int flags,
                                  Size2i min_size,
                                  Size2i max_size) {
    std::vector<cv::Rect> objects;

    cv::Size cv_min_size(min_size.width, min_size.height);
    cv::Size cv_max_size(max_size.width, max_size.height);
    cascade->detectMultiScale(*image, objects, scale_factor, min_neighbors, flags, cv_min_size, cv_max_size);
    // Move objects to vec_of_rect
    size_t num = objects.size();
    vec_of_rect->array = (Rect*) malloc(num * sizeof(Rect));
    vec_of_rect->size = num;
    for (size_t i = 0; i < num; i++) {
        vec_of_rect->array[i].x = objects[i].x;
        vec_of_rect->array[i].y = objects[i].y;
        vec_of_rect->array[i].width = objects[i].width;
        vec_of_rect->array[i].height = objects[i].height;
    }
}
}
