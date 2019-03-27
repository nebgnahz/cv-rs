#include "text.hpp"
#include "utils.hpp"

namespace cvsys {

void ocr_run(BaseOCR& ocr,
             cv::Mat& image,
             CString* output_text,
             CVec<Rect>* component_rects,
             CVec<CString>* component_texts,
             CVec<float>* component_confidences,
             int component_level) {
    std::string output;
    std::vector<cv::Rect> boxes;
    std::vector<std::string> words;
    std::vector<float> confidences;
    ocr.get()->run(image, output, &boxes, &words, &confidences, component_level);

    to_ffi(output, output_text);
    to_ffi(boxes, component_rects);
    to_ffi(words, component_texts);
    to_ffi(confidences, component_confidences);
}

Result<OCRTesseract*>
tesseract_new(const char* datapath, const char* language, const char* char_whitelist, int oem, int psmode) {
    return Result<OCRTesseract*>([datapath, language, char_whitelist, oem, psmode]() {
        auto result = cv::text::OCRTesseract::create(datapath, language, char_whitelist, oem, psmode);
        return new OCRTesseract(result);
    });
}

void tesseract_drop(OCRTesseract* ocr) {
    delete ocr;
}

Result<OCRHMMDecoder*> hmm_new(const char* classifier_filename,
                               const char* vocabulary,
                               cv::Mat& transition_probabilities_table,
                               cv::Mat& emission_probabilities_table,
                               cv::text::classifier_type classifier_type) {
    return Result<OCRHMMDecoder*>([classifier_filename,
                                   vocabulary,
                                   transition_probabilities_table,
                                   emission_probabilities_table,
                                   classifier_type] {
        std::string voc(vocabulary);
        auto classifier = cv::text::loadOCRHMMClassifier(classifier_filename, classifier_type);
        auto result = cv::text::OCRHMMDecoder::create(
            classifier, voc, transition_probabilities_table, emission_probabilities_table);
        return new OCRHMMDecoder(result);
    });
}

void hmm_drop(OCRHMMDecoder* ocr) {
    delete ocr;
}

Result<OCRHolisticWordRecognizer*>
holistic_new(const char* archive_file, const char* weights_file, const char* words_file) {
    return Result<OCRHolisticWordRecognizer*>([archive_file, weights_file, words_file] {
        auto result = cv::text::OCRHolisticWordRecognizer::create(archive_file, weights_file, words_file);
        return new OCRHolisticWordRecognizer(result);
    });
}

void holistic_drop(OCRHolisticWordRecognizer* ocr) {
    delete ocr;
}

}  // namespace cvsys
