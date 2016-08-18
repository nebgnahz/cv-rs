#include "opencv-wrapper.h"
#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>

extern "C" {
    CMat* opencv_imread(const char* const filename, int flags) {
        cv::Mat* image = new cv::Mat();
        *image = cv::imread(filename, flags);
        return (CMat*) image;
    }

    void opencv_mat_free(CMat* mat) {
        delete mat;
        mat = NULL;
    }
}
