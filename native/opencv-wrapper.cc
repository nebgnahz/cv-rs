#include "opencv-wrapper.h"
#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>

extern "C" {
    CMat* opencv_mat_new() {
        cv::Mat* image = new cv::Mat();
        return static_cast<CMat*>(image);
    }

    bool opencv_mat_is_valid(CMat* cmat) {
        cv::Mat* mat = static_cast<cv::Mat*>(cmat);
        return mat->data != NULL;
    }

    CMat* opencv_imread(const char* const filename, int flags) {
        cv::Mat* image = new cv::Mat();
        *image = cv::imread(filename, flags);
        return static_cast<CMat*>(image);
    }

    void opencv_mat_drop(CMat* cmat) {
        cv::Mat* mat = static_cast<cv::Mat*>(cmat);
        delete mat;
        cmat = nullptr;
    }

    void opencv_named_window(const char* const winname, int flags) {
        cv::namedWindow(winname, flags);
    }

    void opencv_imshow(const char* const winname, CMat *cmat) {
        cv::Mat* mat = static_cast<cv::Mat*>(cmat);
        if (mat != NULL) {
            cv::imshow(winname, *mat);
        }
    }

    int opencv_wait_key(int delay) {
        return cv::waitKey(delay);
    }

    CVideoCapture* opencv_videocapture_new(int index) {
        cv::VideoCapture* cap = new cv::VideoCapture(index);
        return static_cast<CVideoCapture*>(cap);
    }

    bool opencv_videocapture_is_opened(const CVideoCapture* const ccap) {
        const cv::VideoCapture* const cap = static_cast<const cv::VideoCapture* const>(ccap);
        return cap->isOpened();
    }

    bool opencv_videocapture_read(CVideoCapture* ccap, CMat* cmat) {
        cv::VideoCapture* cap = static_cast<cv::VideoCapture*>(ccap);
        cv::Mat* mat = static_cast<cv::Mat*>(cmat);
        return cap->read(*mat);
    }

    void opencv_videocapture_drop(CVideoCapture* ccap) {
        cv::VideoCapture* cap = static_cast<cv::VideoCapture*>(ccap);
        delete cap;
        ccap = nullptr;
    }
}
