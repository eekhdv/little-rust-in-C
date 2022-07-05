cmake_minimum_required(VERSION 3.10)

if (NOT EXISTS ./deps)
	file(MAKE_DIRECTORY ./deps)

endif()

add_custom_target(my_rust_lib 
	COMMAND cargo build --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/my-rust/Cargo.toml --target-dir ${CMAKE_CURRENT_SOURCE_DIR}/deps)

# set the project name
project(main C)

# add the executable
add_executable(main ./src/main.c)

add_dependencies(main my_rust_lib)
target_link_libraries(main ${CMAKE_CURRENT_SOURCE_DIR}/deps/debug/libmy_rust.so)
