#ifndef CV_RS_TEXT_H
#define CV_RS_TEXT_H

#include "common.hpp"
#include <opencv2/core.hpp>
#include <opencv2/text/ocr.hpp>

void cv_ocr_run(cv::Ptr<cv::text::BaseOCR>& ocr,
                cv::Mat& image,
                CString* output_text,
                CVec<Rect>* component_rects,
                CVec<CString>* component_texts,
                CVec<float>* component_confidences,
                int component_level);

void cv_tesseract_new(
    const char* datapath, const char* language, const char* char_whitelist, int oem, int psmode, Result<void*>* result);
void cv_tesseract_drop(cv::Ptr<cv::text::OCRTesseract>* ocr);
void cv_hmm_new(const char* classifier_filename,
                const char* vocabulary,
                cv::Mat& transition_probabilities_table,
                cv::Mat& emission_probabilities_table,
                cv::text::classifier_type classifier_type,
                Result<void*>* result);
void cv_hmm_drop(cv::Ptr<cv::text::OCRHMMDecoder>* ocr);
void cv_holistic_new(const char* archive_file, const char* weights_file, const char* words_file, Result<void*>* result);
void cv_holistic_drop(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* ocr);

#endif  // CV_RS_TEXT_H
