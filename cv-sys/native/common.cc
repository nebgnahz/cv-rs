#include "common.hpp"

namespace cvsys {

CString::CString() : value(nullptr) {
}

CString::CString(const char* s) {
    if (s) {
        auto len = std::strlen(s);
        value = new char[len + 1];
        std::strcpy(value, s);
    } else {
        value = nullptr;
    }
}

void CString::drop_cpp() {
    if (value) {
        delete value;
    }
}

bool CString::is_str() const {
    return value != nullptr;
}

const char* CString::get_str() const {
    return value;
}

}  // namespace cvsys
