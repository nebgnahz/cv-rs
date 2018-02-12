#ifndef CV_RS_COMMON_H
#define CV_RS_COMMON_H

#include <cstddef>
#include <cstdint>
#include <functional>
#include <opencv2/core.hpp>

typedef struct {
    int32_t x;
    int32_t y;
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
    int32_t x;
    int32_t y;
    int32_t width;
    int32_t height;
} Rect;

typedef struct {
    Point2f center;
    Size2f size;
    float angle;
} RotatedRect;

typedef struct {
    int32_t v0;
    int32_t v1;
    int32_t v2;
    int32_t v3;
} Scalar;

typedef struct {
    bool status;
    uint8_t* buf;
    size_t size;
} ImencodeResult;

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

typedef struct { const char* value; } CDisposableString;

// Caller is responsible for disposing `error` field
template <typename T>
struct Result {
    T value;
    const char* error;

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
        return Result<T>{value, error};
    }
};

template <typename T>
struct CVec {
    T* array;
    size_t size;
};
#endif  // CV_RS_COMMON_H
