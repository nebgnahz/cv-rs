#include "text.hpp"
#include "utils.hpp"

extern "C" {
void cv_ocr_run(cv::Ptr<cv::text::BaseOCR>& ocr,
                cv::Mat& image,
                CDisposableString* output_text,
                CVec<Rect>* component_rects,
                CVec<CDisposableString>* component_texts,
                CVec<float>* component_confidences,
                int component_level) {
    std::string output;
    std::vector<cv::Rect> boxes;
    std::vector<std::string> words;
    std::vector<float> confidences;
    ocr.get()->run(image, output, &boxes, &words, &confidences, component_level);

    cv_to_ffi(output, output_text);
    cv_to_ffi(boxes, component_rects);
    cv_to_ffi(words, component_texts);
    cv_to_ffi(confidences, component_confidences);
}

void cv_tesseract_new(const char* datapath,
                      const char* language,
                      const char* char_whitelist,
                      int oem,
                      int psmode,
                      Result<void*>* result) {
    *result = Result<void*>::FromFunction([datapath, language, char_whitelist, oem, psmode]() {
        auto result = cv::text::OCRTesseract::create(datapath, language, char_whitelist, oem, psmode);
        return new cv::Ptr<cv::text::OCRTesseract>(result);
    });
}

void cv_tesseract_drop(cv::Ptr<cv::text::OCRTesseract>* ocr) {
    delete ocr;
    ocr = nullptr;
}

void cv_hmm_new(const char* classifier_filename,
                const char* vocabulary,
                cv::Mat& transition_probabilities_table,
                cv::Mat& emission_probabilities_table,
                cv::text::classifier_type classifier_type,
                Result<void*>* result) {
    *result = Result<void*>::FromFunction([classifier_filename,
                                           vocabulary,
                                           transition_probabilities_table,
                                           emission_probabilities_table,
                                           classifier_type]() {
        std::string voc(vocabulary);
        auto classifier = cv::text::loadOCRHMMClassifier(classifier_filename, classifier_type);
        auto result = cv::text::OCRHMMDecoder::create(
            classifier, voc, transition_probabilities_table, emission_probabilities_table);
        return new cv::Ptr<cv::text::OCRHMMDecoder>(result);
    });
}

void cv_hmm_drop(cv::Ptr<cv::text::OCRHMMDecoder>* ocr) {
    delete ocr;
    ocr = nullptr;
}

void cv_holistic_new(const char* archive_file,
                     const char* weights_file,
                     const char* words_file,
                     Result<void*>* result) {
    *result = Result<void*>::FromFunction([archive_file, weights_file, words_file]() {
        auto result = cv::text::OCRHolisticWordRecognizer::create(archive_file, weights_file, words_file);
        return new cv::Ptr<cv::text::OCRHolisticWordRecognizer>(result);
    });
}

void cv_holistic_drop(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* ocr) {
    delete ocr;
    ocr = nullptr;
}
}
