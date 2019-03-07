cmake_minimum_required (VERSION 2.8.12.2)
project(CVSYS)

add_subdirectory(opencv)

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

string(REGEX REPLACE "([^;]+)" "opencv/modules/\\1/include" core_module_includes "${CV_CORE_MODULES}")
string(REGEX REPLACE "([^;]+)" "opencv_contrib/modules/\\1/include" contrib_module_includes "${CV_CONTRIB_MODULES}")

target_include_directories(cvsys PUBLIC ${core_module_includes} ${contrib_module_includes})
target_include_directories(cvsys PUBLIC ${CMAKE_BINARY_DIR})

string(REGEX REPLACE "([^;]+)" "opencv_\\1" module_targets "${MODULES}")

target_link_libraries(cvsys
    ${module_targets}
)