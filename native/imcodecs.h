#ifndef CV_RS_IMCODECS_H
#define CV_RS_IMCODECS_H

#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>
#include <stddef.h>
#include <stdint.h>

extern "C" {

typedef struct {
    bool status;
    uint8_t* buf;
    size_t size;
} ImencodeResult;

void* cv_imread(const char* const filename, int flags);
void* cv_imdecode(const uint8_t* const buffer, size_t len, int flag);
ImencodeResult
cv_imencode(const char* const ext, const cv::Mat* const mat, const int* const flag_ptr, size_t flag_size);
}

#endif  // CV_RS_IMCODECS_H
