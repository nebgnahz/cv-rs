#ifndef CV_RS_TEXT_H
#define CV_RS_TEXT_H

#include "common.hpp"
#include <opencv2/core.hpp>
#include <opencv2/text/ocr.hpp>

namespace cvsys {

struct BaseOCR : cv::Ptr<cv::text::BaseOCR> {
    BaseOCR(cv::Ptr<cv::text::BaseOCR> p) : cv::Ptr<cv::text::BaseOCR>(p) {
    }
};
struct OCRTesseract : cv::Ptr<cv::text::OCRTesseract> {
    OCRTesseract(cv::Ptr<cv::text::OCRTesseract> p) : cv::Ptr<cv::text::OCRTesseract>(p) {
    }
};
struct OCRHMMDecoder : cv::Ptr<cv::text::OCRHMMDecoder> {
    OCRHMMDecoder(cv::Ptr<cv::text::OCRHMMDecoder> p) : cv::Ptr<cv::text::OCRHMMDecoder>(p) {
    }
};
struct OCRHolisticWordRecognizer : cv::Ptr<cv::text::OCRHolisticWordRecognizer> {
    OCRHolisticWordRecognizer(cv::Ptr<cv::text::OCRHolisticWordRecognizer> p)
        : cv::Ptr<cv::text::OCRHolisticWordRecognizer>(p) {
    }
};

void ocr_run(BaseOCR& ocr,
             cv::Mat& image,
             CString* output_text,
             CVec<Rect>* component_rects,
             CVec<CString>* component_texts,
             CVec<float>* component_confidences,
             int component_level);

Result<OCRTesseract*>
tesseract_new(const char* datapath, const char* language, const char* char_whitelist, int oem, int psmode);
void tesseract_drop(OCRTesseract* ocr);
Result<OCRHMMDecoder*> hmm_new(const char* classifier_filename,
                               const char* vocabulary,
                               cv::Mat& transition_probabilities_table,
                               cv::Mat& emission_probabilities_table,
                               cv::text::classifier_type classifier_type);
void hmm_drop(OCRHMMDecoder* ocr);
Result<OCRHolisticWordRecognizer*>
holistic_new(const char* archive_file, const char* weights_file, const char* words_file);
void holistic_drop(OCRHolisticWordRecognizer* ocr);

}  // namespace cvsys

#endif  // CV_RS_TEXT_H
