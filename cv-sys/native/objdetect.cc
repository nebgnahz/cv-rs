#include "objdetect.hpp"
#include "utils.hpp"

namespace cvsys {

cv::CascadeClassifier* cascade_classifier_new() {
    return new cv::CascadeClassifier();
}

cv::CascadeClassifier* cascade_classifier_from_path(const char* const p) {
    return new cv::CascadeClassifier(p);
}

bool cascade_classifier_load(cv::CascadeClassifier* cascade, const char* const p) {
    return cascade->load(p);
}

void cascade_classifier_drop(cv::CascadeClassifier* cascade) {
    delete cascade;
    cascade = nullptr;
}

void cascade_classifier_detect(cv::CascadeClassifier* cascade,
                               cv::Mat* image,
                               CVec<Rect>* vec_of_rect,
                               double scale_factor,
                               int min_neighbors,
                               int flags,
                               Size2i min_size,
                               Size2i max_size) {
    std::vector<cv::Rect> objects;

    cv::Size native_min_size(min_size.width, min_size.height);
    cv::Size native_max_size(max_size.width, max_size.height);
    cascade->detectMultiScale(*image, objects, scale_factor, min_neighbors, flags, native_min_size, native_max_size);
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

cv::HOGDescriptor* hog_new() {
    return new cv::HOGDescriptor();
}

void hog_drop(cv::HOGDescriptor* hog) {
    delete hog;
    hog = nullptr;
}

std::vector<float>* hog_default_people_detector() {
    return new std::vector<float>(cv::HOGDescriptor::getDefaultPeopleDetector());
}

std::vector<float>* hog_daimler_people_detector() {
    return new std::vector<float>(cv::HOGDescriptor::getDaimlerPeopleDetector());
}

void hog_detector_drop(std::vector<float>* detector) {
    delete detector;
    detector = nullptr;
}

void hog_set_svm_detector(cv::HOGDescriptor* hog, std::vector<float>* detector) {
    hog->setSVMDetector(*detector);
}

void hog_detect(cv::HOGDescriptor* hog,
                cv::Mat* image,
                CVec<Rect>* vec_rect,
                CVec<double>* vec_weight,
                Size2i nat_win_stride,
                Size2i nat_padding,
                double scale,
                double final_threshold,
                bool use_means_shift) {
    // convert all types

    std::vector<cv::Rect> objects;
    std::vector<double> weights;
    cv::Size win_stride(nat_win_stride.width, nat_win_stride.height);
    cv::Size padding(nat_padding.width, nat_padding.height);

    // Call the function
    hog->detectMultiScale(*image, objects, weights, 0.1, win_stride, padding, scale, final_threshold, use_means_shift);

    // Prepare the results
    to_ffi(objects, vec_rect);
    to_ffi(weights, vec_weight);
}

}  // namespace cvsys
