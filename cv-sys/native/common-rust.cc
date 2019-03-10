#include "common-rust.hpp"

namespace cvsys {

void vec_drop(CVec<void>* vec, unsigned int depth) {
    if (vec->array != nullptr) {
        if (depth > 1) {
            auto nestedVec = (CVec<void>*) vec->array;
            for (size_t i = 0; i < vec->size; ++i) {
                vec_drop(&nestedVec[i], depth - 1);
            }
        }
        free(vec->array);
        vec->array = nullptr;
        vec->size = 0;
    }
}

void c_drop(void* value) {
    free(value);
    value = nullptr;
}

}  // namespace cvsys
