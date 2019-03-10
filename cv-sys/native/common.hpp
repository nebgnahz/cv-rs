#ifndef CV_RS_COMMON_H
#define CV_RS_COMMON_H

#include <cstddef>
#include <functional>
#include <opencv2/core.hpp>

namespace cvsys {

typedef struct {
    int x;
    int y;
} Point2i;

typedef struct {
    float x;
    float y;
} Point2f;

typedef struct {
    int width;
    int height;
} Size2i;

typedef struct {
    float width;
    float height;
} Size2f;

typedef struct {
    int x;
    int y;
    int width;
    int height;
} Rect;

typedef struct {
    Point2f center;
    Size2f size;
    float angle;
} RotatedRect;

typedef struct {
    int v0;
    int v1;
    int v2;
    int v3;
} Scalar;

typedef struct {
    Point2f pt;
    float size;
    float angle;
    float response;
    int octave;
    int class_id;
} KeyPoint;

typedef struct {
    float distance;
    int imgIdx;
    int queryIdx;
    int trainIdx;
} DMatch;

struct CString {
    char* value;

    CString(const char* s) {
        if (s) {
            auto len = std::strlen(s);
            value = new char[len + 1];
            std::strcpy(value, s);
        } else {
            value = nullptr;
        }
    }
    ~CString() {
        if (value) {
            delete value;
        }
    }

    bool is_str() const;

    const char* get_str() const;
};

// Caller is responsible for disposing `error` field
template <typename T>
struct Result {
    T value;
    CString error;

    static Result<T> FromFunction(std::function<T()> function) {
        T value;
        CString error(nullptr);
        try {
            value = function();
        } catch (cv::Exception& e) {
            error = CString(e.what());
        }
        return Result<T>{value, error};
    }
};

// Bindings generation needs to call the destructor which frees the string memory.
struct EmptyResult {
    CString error;

    static EmptyResult FromFunction(std::function<void()> function) {
        CString error(nullptr);

        try {
            function();
        } catch (cv::Exception& e) {
            error = CString(e.what());
        }

        return EmptyResult{error};
    }
};

template <typename T>
struct CVec {
    T* array;
    size_t size;
};

template <typename T>
struct COption {
    bool hasValue;
    T value;
};

}  // namespace cvsys

#endif  // CV_RS_COMMON_H
