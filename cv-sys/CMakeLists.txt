cmake_minimum_required (VERSION 2.8.12.2)
project(CVSYS)

set(MODULES
    ${CV_CORE_MODULES}
    ${CV_CONTRIB_MODULES}
)

string(REGEX REPLACE "([^;]+)" "native/\\1.hpp;native/\\1.cc" module_srcs "${MODULES}")

set(SRC
    native/common.cc
    native/common.hpp
    native/common-rust.cc
    native/common-rust.hpp
    native/core/mat.cc
    native/core/mat.hpp
    native/utils.cc
    native/utils.hpp
    ${module_srcs}
)

add_library(cvsys ${SRC})

# Add prebuilt OpenCV include directory (there is only one).
target_include_directories(cvsys PUBLIC "${CVSYS_INCLUDE_DIR}")

install(TARGETS cvsys
        RUNTIME DESTINATION bin
        LIBRARY DESTINATION lib
        ARCHIVE DESTINATION lib)
