#include "common.hpp"

namespace cvsys {

bool CString::is_str() const {
    return value != nullptr;
}

const char* CString::get_str() const {
    return value;
}

}  // namespace cvsys
