cmake_minimum_required(VERSION 3.15)
project(conintl)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_definitions(-D_WIN32_WINNT=0x0A00)

add_executable(conintl main.cpp)
target_link_libraries(conintl PRIVATE ws2_32)