cmake_minimum_required(VERSION 3.20)

project(LostHorizons
    VERSION 0.1.0
    DESCRIPTION "Lost Horizons C++23 voxel engine"
    LANGUAGES CXX)

# ---- C++ standard -----------------------------------------------------------
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
    set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type" FORCE)
endif()

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# ---- Dependencies -----------------------------------------------------------
find_package(Vulkan REQUIRED)
find_package(glfw3 REQUIRED)

# glm is header-only; prefer its CMake package but fall back to a header search.
find_package(glm QUIET)
if(NOT glm_FOUND)
    find_path(GLM_INCLUDE_DIR glm/glm.hpp REQUIRED)
endif()

# EnTT (header-only ECS) fetched at configure time.
include(FetchContent)
FetchContent_Declare(
    EnTT
    GIT_REPOSITORY https://github.com/skypjack/entt.git
    GIT_TAG v3.13.2
    GIT_SHALLOW TRUE)
FetchContent_MakeAvailable(EnTT)

# ---- Shader compilation -----------------------------------------------------
if(DEFINED Vulkan_GLSLANG_VALIDATOR_EXECUTABLE)
    set(GLSLANG_VALIDATOR ${Vulkan_GLSLANG_VALIDATOR_EXECUTABLE})
else()
    find_program(GLSLANG_VALIDATOR glslangValidator REQUIRED)
endif()

set(SHADER_OUTPUT_DIR ${CMAKE_BINARY_DIR}/shaders)
file(MAKE_DIRECTORY ${SHADER_OUTPUT_DIR})

set(SHADER_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/shaders/voxel.vert
    ${CMAKE_CURRENT_SOURCE_DIR}/shaders/voxel.frag)

set(SHADER_BINARIES)
foreach(SHADER ${SHADER_SOURCES})
    get_filename_component(SHADER_NAME ${SHADER} NAME)
    set(SPV ${SHADER_OUTPUT_DIR}/${SHADER_NAME}.spv)
    add_custom_command(
        OUTPUT ${SPV}
        COMMAND ${GLSLANG_VALIDATOR} -V ${SHADER} -o ${SPV}
        DEPENDS ${SHADER}
        COMMENT "Compiling shader ${SHADER_NAME}"
        VERBATIM)
    list(APPEND SHADER_BINARIES ${SPV})
endforeach()

add_custom_target(shaders DEPENDS ${SHADER_BINARIES})

# ---- Engine target ----------------------------------------------------------
add_executable(${PROJECT_NAME}
    src/main.cpp
    src/core/Window.cpp
    src/core/Engine.cpp
    src/voxel/TerrainGenerator.cpp
    src/voxel/ChunkMesher.cpp
    src/voxel/World.cpp
    src/rendering/VulkanRenderer_Init.cpp
    src/rendering/VulkanRenderer_Swapchain.cpp
    src/rendering/VulkanRenderer_Pipeline.cpp
    src/rendering/VulkanRenderer_Render.cpp)

add_dependencies(${PROJECT_NAME} shaders)

target_include_directories(${PROJECT_NAME} PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include)
if(GLM_INCLUDE_DIR)
    target_include_directories(${PROJECT_NAME} PRIVATE ${GLM_INCLUDE_DIR})
endif()

target_link_libraries(${PROJECT_NAME} PRIVATE
    Vulkan::Vulkan
    glfw
    EnTT::EnTT)
if(glm_FOUND)
    target_link_libraries(${PROJECT_NAME} PRIVATE glm::glm)
endif()

target_compile_definitions(${PROJECT_NAME} PRIVATE
    LH_SHADER_DIR="${SHADER_OUTPUT_DIR}"
    GLM_FORCE_RADIANS
    GLM_FORCE_DEPTH_ZERO_TO_ONE)

if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE /W4)
else()
    target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -Wpedantic)
endif()
