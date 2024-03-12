# Toolchain file should be provided
cmake_minimum_required(VERSION 3.26.0)

set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)
# Macos variables
set(CMAKE_OSX_ARCHITECTURES "arm64")
set(CMAKE_OSX_DEPLOYMENT_TARGET "10.12")
# All variables should be defined before project() function
project(detector-lib-bridge VERSION 1.1 LANGUAGES CXX C)

# Define build type
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(buildtype "debug")
else()
    set(buildtype "release")
endif()

# Add detector-lib library
add_subdirectory("detector-lib")
set(detector_include "detector-lib/src")

# Define our sources
file(GLOB sources "source/bridge.cpp")
set(include "source")

# Define our library
add_library(${PROJECT_NAME} STATIC ${sources})

target_include_directories(${PROJECT_NAME} PUBLIC ${include} PRIVATE ${detector_include} )
target_link_libraries(${PROJECT_NAME} PRIVATE detector-lib)

add_executable(test_detector "source/main.c")
target_include_directories(test_detector PRIVATE ${include})
target_link_libraries(test_detector PRIVATE ${PROJECT_NAME})
