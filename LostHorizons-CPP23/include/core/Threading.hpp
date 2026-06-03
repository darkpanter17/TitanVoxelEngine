#pragma once

#include <algorithm>
#include <atomic>
#include <cstddef>
#include <thread>
#include <vector>

// Lightweight parallel primitives used to spread independent, CPU-bound work
// (chunk generation and meshing) across hardware threads. Header-only so any
// translation unit can use it without an extra link dependency.
namespace lh {

// Resolve the worker thread count to use. `requested == 0` means "auto": use
// the hardware concurrency reported by the runtime (with a sane fallback of 1
// when that is unavailable). The result is always at least 1.
[[nodiscard]] inline unsigned resolve_thread_count(unsigned requested = 0) {
    if (requested > 0) {
        return requested;
    }
    const unsigned hw = std::thread::hardware_concurrency();
    return hw > 0 ? hw : 1u;
}

// Invoke `fn(i)` for every index `i` in `[0, count)`, distributing the work
// across `thread_count` workers using dynamic (atomic-counter) scheduling so
// uneven per-index cost stays balanced. `fn` must be safe to call concurrently
// for distinct indices. The calling thread participates as one of the workers.
//
// `fn` is taken by value-copyable reference and invoked as `fn(std::size_t)`.
template <typename Fn>
void parallel_for(std::size_t count, unsigned thread_count, Fn fn) {
    if (count == 0) {
        return;
    }

    const unsigned workers =
        std::max(1u, std::min<unsigned>(thread_count, static_cast<unsigned>(count)));

    if (workers == 1) {
        for (std::size_t i = 0; i < count; ++i) {
            fn(i);
        }
        return;
    }

    std::atomic<std::size_t> next{0};
    const auto run = [&]() {
        for (;;) {
            const std::size_t i = next.fetch_add(1, std::memory_order_relaxed);
            if (i >= count) {
                break;
            }
            fn(i);
        }
    };

    std::vector<std::jthread> pool;
    pool.reserve(workers - 1);
    for (unsigned t = 0; t + 1 < workers; ++t) {
        pool.emplace_back(run);
    }
    run(); // the calling thread is also a worker.
    // std::jthread joins on destruction, so all work is complete here.
}

} // namespace lh
