#pragma once

#include <iostream>
#include <string_view>

// Minimal logging helpers used across the engine. Header-only so every
// translation unit can log without an extra link dependency.
namespace lh {

enum class LogLevel { Info, Warn, Error };

inline void log(LogLevel level, std::string_view message) {
    switch (level) {
        case LogLevel::Info:  std::cout << "[INFO]  " << message << '\n'; break;
        case LogLevel::Warn:  std::cout << "[WARN]  " << message << '\n'; break;
        case LogLevel::Error: std::cerr << "[ERROR] " << message << '\n'; break;
    }
}

inline void log_info(std::string_view message)  { log(LogLevel::Info, message); }
inline void log_warn(std::string_view message)  { log(LogLevel::Warn, message); }
inline void log_error(std::string_view message) { log(LogLevel::Error, message); }

} // namespace lh
