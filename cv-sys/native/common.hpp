#ifndef CV_RS_COMMON_H
#define CV_RS_COMMON_H

#include <cstddef>
#include <functional>
#include <opencv2/core.hpp>

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

typedef struct {
    const char* value;
} CDisposableString;

// Caller is responsible for disposing `error` field
template <typename T>
struct Result {
    T value;
    CDisposableString error;

    static Result<T> FromFunction(std::function<T()> function) {
        T value;
        char* error = nullptr;
        try {
            value = function();
        } catch (cv::Exception& e) {
            const char* err_msg = e.what();
            auto len = std::strlen(err_msg);
            error = new char[len + 1];
            std::strcpy(error, err_msg);
        }
        return Result<T>{value, CDisposableString{error}};
    }
};

// Caller is responsible for disposing `error` field
struct EmptyResult {
    CDisposableString error;

    static EmptyResult FromFunction(std::function<void()> function) {
        char* error = nullptr;

        try {
            function();
        } catch (cv::Exception& e) {
            const char* err_msg = e.what();
            auto len = std::strlen(err_msg);
            error = new char[len + 1];
            std::strcpy(error, err_msg);
        }

        return EmptyResult{CDisposableString{error}};
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
#endif  // CV_RS_COMMON_H
