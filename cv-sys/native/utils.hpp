#ifndef UTILS_H_
#define UTILS_H_

#include <opencv2/core.hpp>
#include <vector>

#include "common.hpp"

namespace cvsys {

void to_ffi(const cv::Rect& source, Rect* dest);
void to_ffi(const cv::Point& source, Point2i* dest);
void to_ffi(const cv::KeyPoint& source, KeyPoint* dest);
void to_ffi(const cv::DMatch& source, DMatch* dest);
void to_ffi(const std::string& source, CString* dest);

template <typename T>
void to_ffi(const std::vector<T>& source, CVec<T>* dest) {
    size_t num = source.size();
    dest->size = num;
    dest->array = (T*) malloc(num * sizeof(T));
    ::memcpy(dest->array, source.data(), num * sizeof(T));
}

template <typename T, typename U>
void to_ffi(const std::vector<T>& source, CVec<U>* dest) {
    size_t num = source.size();
    dest->size = num;
    dest->array = (U*) malloc(num * sizeof(U));
    for (size_t i = 0; i < num; i++) {
        to_ffi(source[i], &dest->array[i]);
    }
}

void ffi_to_cv(const cv::Mat& source, cv::Mat* dest);

template <typename T, typename U>
void ffi_to_cv(const CVec<U*>& source, std::vector<T>* dest) {
    dest->reserve(source.size);
    for (size_t i = 0; i < source.size; i++) {
        T* cell = new T();
        ffi_to_cv(*source.array[i], cell);
        dest->push_back(*cell);
    }
}

}  // namespace cvsys

#endif  // UTILS_H_
