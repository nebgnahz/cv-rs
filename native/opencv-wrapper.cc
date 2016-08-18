#include "opencv-wrapper.h"
#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>

extern "C" {
    CMat* opencv_imread(const char* const filename, int flags) {
        cv::Mat* image = new cv::Mat();
        *image = cv::imread(filename, flags);
        return (CMat*) image;
    }

    void opencv_mat_free(CMat* cmat) {
        cv::Mat* mat = (cv::Mat*) cmat;
        delete mat;
        mat = NULL;
    }

    void opencv_named_window(const char* const winname, int flags) {
        cv::namedWindow(winname, flags);
    }

    void opencv_imshow(const char* const winname, CMat *cmat) {
        cv::Mat* mat = (cv::Mat*) cmat;
        if (mat != NULL) {
            cv::imshow(winname, *mat);
        }
    }

    int opencv_wait_key(int delay) {
        cv::waitKey(delay);
    }
}
