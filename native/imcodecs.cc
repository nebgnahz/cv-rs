#include "imcodecs.h"

extern "C" {

void* cv_imread(const char* const filename, int flags) {
    cv::Mat* image = new cv::Mat();
    *image = cv::imread(filename, flags);
    return (image);
}

void* cv_imdecode(const uint8_t* const buffer, size_t len, int flag) {
    cv::Mat* dst = new cv::Mat();
    std::vector<uchar> input(buffer, buffer + len);
    cv::imdecode(cv::Mat(input), flag, dst);
    return (dst);
}

// TODO: replace raw pointer return with CResult because it's memory leak
ImencodeResult
cv_imencode(const char* const ext, const cv::Mat* const image, const int* const flag_ptr, size_t flag_size) {
    std::vector<uchar> buf;
    std::vector<int> params(flag_ptr, flag_ptr + flag_size);
    bool r = cv::imencode(ext, *image, buf, params);

    int size = buf.size();
    uint8_t* buffer = new uint8_t[size];
    std::copy(buf.begin(), buf.begin() + size, buffer);

    ImencodeResult result;
    result.status = r;
    result.size = size;
    result.buf = buffer;
    return result;
}
}
