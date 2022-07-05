cmake_minimum_required(VERSION 3.10)

add_custom_target(test_rust COMMAND cargo build --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/my-rust/Cargo.toml)


# set the project name
project(main C)

# add the executable
add_executable(main ./src/main.c)

add_dependencies(main test_rust)
target_link_libraries(main ${CMAKE_CURRENT_SOURCE_DIR}/my-rust/target/debug/libmy_rust.so)
