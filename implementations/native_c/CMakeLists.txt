cmake_minimum_required(VERSION 3.23)

project(pycode_h5 LANGUAGES C)

add_library(pycode_h5
  "src/pycode_h5.h" "src/pycode_h5.c"
)
target_link_libraries(pycode_h5 PRIVATE hdf5)

add_executable(Test
  "test/main.c"
)
target_link_libraries(Test PRIVATE pycode_h5)
install(TARGETS pycode_h5 Test)
