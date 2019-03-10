#ifndef CV_RS_COMMON_RUST_H
#define CV_RS_COMMON_RUST_H

#include "common.hpp"

namespace cvsys {

void vec_drop(CVec<void>* vec, unsigned int depth);
void c_drop(void* value);

}  // namespace cvsys

#endif  // CV_RS_COMMON_RUST_H
