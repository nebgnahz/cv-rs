cmake_minimum_required (VERSION 2.8.12.2)
project(CVSYS)

add_subdirectory(opencv)

# Found from https://stackoverflow.com/a/39127212.
# This is needed to recursively resolve the LINK_LIBRARIES that Cargo needs.
function(get_link_libraries OUTPUT_LIST TARGET)
    get_target_property(IMPORTED ${TARGET} IMPORTED)
    list(APPEND VISITED_TARGETS ${TARGET})
    if (IMPORTED)
        get_target_property(LIBS ${TARGET} INTERFACE_LINK_LIBRARIES)
    else()
        get_target_property(LIBS ${TARGET} LINK_LIBRARIES)
    endif()
    set(LIB_FILES "")
    foreach(LIB ${LIBS})
        if (TARGET ${LIB})
            list(FIND VISITED_TARGETS ${LIB} VISITED)
            if (${VISITED} EQUAL -1)
                get_target_property(LIB_FILE ${LIB} LOCATION)
                get_link_libraries(LINK_LIB_FILES ${LIB})
                list(APPEND LIB_FILES ${LIB_FILE} ${LINK_LIB_FILES})
            endif()
        endif()
    endforeach()
    set(VISITED_TARGETS ${VISITED_TARGETS} PARENT_SCOPE)
    set(${OUTPUT_LIST} ${LIB_FILES} PARENT_SCOPE)
endfunction()

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

get_link_libraries(link_libs_list cvsys)
file(WRITE "${CMAKE_BINARY_DIR}/link_libs_list.txt" "${link_libs_list}")
