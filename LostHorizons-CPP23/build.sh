#!/usr/bin/env bash
# Build script for Lost Horizons C++23 voxel engine.
set -euo pipefail

cd "$(dirname "$0")"

BUILD_TYPE="${BUILD_TYPE:-Release}"
CXX_COMPILER="${CXX:-g++-13}"

echo "=== Lost Horizons C++23 Build ==="
echo "Build type:  ${BUILD_TYPE}"
echo "Compiler:    ${CXX_COMPILER}"

GENERATOR=""
if command -v ninja >/dev/null 2>&1; then
    GENERATOR="-G Ninja"
fi

cmake -S . -B build ${GENERATOR} \
    -DCMAKE_BUILD_TYPE="${BUILD_TYPE}" \
    -DCMAKE_CXX_COMPILER="${CXX_COMPILER}"

cmake --build build --parallel

echo "Build complete!"
echo "Run with: ./build/bin/LostHorizons"
