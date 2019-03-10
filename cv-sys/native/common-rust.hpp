#ifndef CV_RS_COMMON_RUST_H
#define CV_RS_COMMON_RUST_H

#include "common.hpp"

void cv_vec_drop(CVec<void>* vec, unsigned int depth);
void c_drop(void* value);

#endif  // CV_RS_COMMON_RUST_H
