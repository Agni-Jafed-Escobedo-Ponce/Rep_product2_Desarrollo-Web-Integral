cmake_minimum_required(VERSION 3.15)
project(clisoap2)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_definitions(-D_WIN32_WINNT=0x0A00)

find_package(CURL REQUIRED)

add_executable(clisoap2 main.cpp)
target_link_libraries(clisoap2 PRIVATE CURL::libcurl ws2_32 crypt32)