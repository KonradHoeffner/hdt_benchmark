cmake_minimum_required(VERSION 3.17)

project(
        b_rdf4cpp
        VERSION 1.0
        LANGUAGES C CXX)

set(CMAKE_VERBOSE_MAKEFILE ON)
include(cmake/boilerplate_init.cmake)
boilerplate_init()

OPTION(USE_CONAN "If available, use conan to retrieve dependencies." ON)
if (IS_TOP_LEVEL AND USE_CONAN)
    include(cmake/conan_cmake.cmake)
    set(CONAN_OPTIONS "")
    install_packages_via_conan("${CMAKE_SOURCE_DIR}/conanfile.txt" "${CONAN_OPTIONS}")
endif ()

find_package(rdf4cpp REQUIRED)

add_executable(run run.cpp)

target_include_directories(run PUBLIC
        ${CMAKE_SOURCE_DIR}) # for measures.h

target_link_libraries(run PRIVATE rdf4cpp::rdf4cpp)

set_target_properties(run PROPERTIES
        CXX_STANDARD 20
        CXX_EXTENSIONS OFF
        CXX_STANDARD_REQUIRED ON
)