cmake_minimum_required(VERSION 3.10)
project(CxxRust)

set(CMAKE_CXX_STANDARD 14)

# 添加C++源代码
add_executable(${PROJECT_NAME} src/main.cpp)

# 设置Rust库的路径
set(RUST_LIB_DIR ${CMAKE_SOURCE_DIR}/rust/target/debug)

# 查找生成的Rust静态库或动态库
find_library(RUST_LIB my_rust_lib PATHS ${RUST_LIB_DIR} NO_DEFAULT_PATH)

# 链接Rust库到C++可执行文件
target_link_libraries(${PROJECT_NAME} PRIVATE ${RUST_LIB})

# 包含Rust生成的头文件（如果有）
# include_directories(${RUST_HEADER_DIR})  # 如果有头文件需要包含，取消注释并设置路径