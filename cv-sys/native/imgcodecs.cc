#include "imgcodecs.hpp"

cv::Mat* cvsys_nat_imread(const char* const filename, int flags) {
    cv::Mat* image = new cv::Mat();
    *image = cv::imread(filename, flags);
    return (image);
}

cv::Mat* cvsys_nat_imdecode(const uint8_t* const buffer, size_t len, int flag) {
    cv::Mat* dst = new cv::Mat();
    std::vector<uchar> input(buffer, buffer + len);
    cv::imdecode(cv::Mat(input), flag, dst);
    return (dst);
}

void cvsys_nat_imencode(const char* const ext,
                        const cv::Mat* const image,
                        const int* const flag_ptr,
                        size_t flag_size,
                        COption<CVec<uint8_t>>* result) {
    std::vector<uchar> buf;
    std::vector<int> params(flag_ptr, flag_ptr + flag_size);
    bool r = cv::imencode(ext, *image, buf, params);
    if (r) {
        CVec<uint8_t> cvec;
        cvsys_to_ffi(buf, &cvec);
        *result = COption<CVec<uint8_t>>{true, cvec};
    } else {
        *result = COption<CVec<uint8_t>>{false, CVec<uint8_t>()};
    }
}
