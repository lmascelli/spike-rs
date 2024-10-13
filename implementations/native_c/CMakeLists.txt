cmake_minimum_required(VERSION 3.23)

project(pycode_h5 LANGUAGES C)

add_library(pycode_h5 STATIC
  "src/pycode_h5.h" "src/pycode_h5.c"
)
# set(HDF5_INCLUDE_DIR "/home/leonardo/Documents/unige/hdf5/1.14.5/include")
set(HDF5_INCLUDE_DIR $ENV{HDF5_INCLUDE_DIR})
target_include_directories(pycode_h5 PRIVATE ${HDF5_INCLUDE_DIR})

install(TARGETS pycode_h5)
