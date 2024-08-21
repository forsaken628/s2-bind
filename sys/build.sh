set -e
mkdir -p ./abseil-src/build
cmake -S ./abseil-src/source -B ./abseil-src/build -D CMAKE_PREFIX_PATH=./intall -D CMAKE_INSTALL_PREFIX=./install -D ABSL_ENABLE_INSTALL=ON -D CMAKE_POSITION_INDEPENDENT_CODE=ON -D CMAKE_CXX_STANDARD=14
cmake --build ./abseil-src/build
mkdir -p ./install
cmake --build ./abseil-src/build --target ./install
mkdir -p ./s2-src/build
cmake -S ./s2-src/source -B ./s2-src/build -D CMAKE_PREFIX_PATH=./intall -D CMAKE_INSTALL_PREFIX=./install -D CMAKE_CXX_STANDARD=14 -D BUILD_TESTS=OFF -D BUILD_SHARED_LIBS=OFF
cmake --build ./s2-src/build
cmake --build ./s2-src/build --target ./install
