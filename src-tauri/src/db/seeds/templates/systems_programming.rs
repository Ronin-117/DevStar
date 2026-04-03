use super::{
    add_custom_sprint_section, add_template, add_template_sprint, add_template_sprint_sections,
};
use super::super::super::types::AppError;
use rusqlite::Connection;

pub fn seed(
    conn: &Connection,
    section_map: &std::collections::HashMap<String, i64>,
    _sprint_map: &std::collections::HashMap<String, i64>,
) -> Result<(), AppError> {
    let planning = section_map.get("planning").copied().unwrap_or(0);
    let security = section_map.get("security").copied().unwrap_or(0);
    let testing = section_map.get("testing").copied().unwrap_or(0);
    let cicd = section_map.get("cicd").copied().unwrap_or(0);
    let docs = section_map.get("docs").copied().unwrap_or(0);
    let quality = section_map.get("quality").copied().unwrap_or(0);
    let performance = section_map.get("performance").copied().unwrap_or(0);
    let monitoring = section_map.get("monitoring").copied().unwrap_or(0);

    let tpl = add_template(
        conn,
        "Systems Programming",
        "Operating systems, kernels, drivers, and low-level systems software",
        "#64748b",
    )?;

    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Architecture",
        "System design and architecture decisions",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning, quality])?;
    add_custom_sprint_section(
        conn,
        s1,
        "System Architecture",
        "Low-level system design and constraints",
        "#64748b",
        &[
            (
                "System requirements documented",
                "Performance targets, memory limits, and latency requirements",
            ),
            (
                "Architecture diagram created",
                "Kernel space, user space, and hardware boundary mapped",
            ),
            (
                "Memory model defined",
                "Virtual memory layout, page tables, and allocation strategy",
            ),
            (
                "Concurrency model chosen",
                "Locks, lock-free, or actor-based concurrency",
            ),
            (
                "Target platforms and ABIs specified",
                "Architecture, OS, calling conventions, and alignment",
            ),
        ],
    )?;

    let s2 = add_template_sprint(
        conn,
        tpl,
        "Core System Foundation",
        "Boot process, memory management, and initialization",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Memory Management",
        "Allocation, paging, and memory safety",
        "#06b6d4",
        &[
            (
                "Memory allocator implemented or configured",
                "Buddy system, slab, or jemalloc/tcmalloc",
            ),
            (
                "Virtual memory management configured",
                "Page fault handling, TLB management, and swapping",
            ),
            (
                "Memory safety analysis performed",
                "Static analysis, sanitizers, and bounds checking",
            ),
            (
                "Memory leak detection configured",
                "Valgrind, ASan, or custom leak tracking",
            ),
            (
                "Memory-mapped I/O implemented",
                "Device memory mapping with proper barriers",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Initialization & Boot",
        "System startup and hardware initialization",
        "#14b8a6",
        &[
            (
                "Boot sequence implemented",
                "Early init, hardware detection, and kernel handoff",
            ),
            (
                "Hardware abstraction layer created",
                "Platform-independent interface to hardware",
            ),
            (
                "Interrupt controller configured",
                "IRQ routing, priority, and handler registration",
            ),
            (
                "Timer subsystem initialized",
                "System tick, high-resolution timers, and scheduling",
            ),
            (
                "Early console/logging enabled",
                "Serial output or framebuffer for early debugging",
            ),
        ],
    )?;

    let s3 = add_template_sprint(
        conn,
        tpl,
        "Concurrency & Scheduling",
        "Threading, synchronization, and process management",
        2,
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Threading & Synchronization",
        "Multi-threading primitives and patterns",
        "#8b5cf6",
        &[
            (
                "Thread creation and management implemented",
                "Thread pools, lifecycle, and cleanup",
            ),
            (
                "Mutex and lock primitives implemented",
                "Spinlocks, mutexes, rwlocks, and futexes",
            ),
            (
                "Condition variables and semaphores",
                "Wait/notify patterns with spurious wake handling",
            ),
            (
                "Lock-free data structures implemented",
                "Atomic operations, CAS loops, and memory ordering",
            ),
            (
                "Deadlock detection or prevention",
                "Lock ordering, timeout-based, or wait-for graph",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Process Management",
        "Process lifecycle and IPC",
        "#a855f7",
        &[
            (
                "Process creation and termination",
                "Fork/exec or equivalent with resource cleanup",
            ),
            (
                "Inter-process communication implemented",
                "Pipes, shared memory, message queues, or sockets",
            ),
            (
                "Signal handling configured",
                "Signal masks, handlers, and safe async-signal operations",
            ),
            (
                "Resource limits enforced",
                "CPU time, memory, file descriptors, and process count",
            ),
            (
                "Process scheduling implemented",
                "Priority-based, round-robin, or CFS-like scheduler",
            ),
        ],
    )?;

    let s4 = add_template_sprint(
        conn,
        tpl,
        "Device Drivers & I/O",
        "Hardware drivers and I/O subsystems",
        3,
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Driver Development",
        "Hardware device driver implementation",
        "#f59e0b",
        &[
            (
                "Driver framework initialized",
                "Platform-specific driver registration and binding",
            ),
            (
                "Device detection and enumeration",
                "PCI, USB, ACPI, or device tree enumeration",
            ),
            (
                "DMA engine configured",
                "Direct memory access with proper cache coherency",
            ),
            (
                "Interrupt handler implemented",
                "Top-half/bottom-half or threaded interrupt handling",
            ),
            (
                "Power management for devices",
                "Suspend, resume, and runtime PM for devices",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "I/O Subsystems",
        "File systems, block devices, and network I/O",
        "#f97316",
        &[
            (
                "Block device layer implemented",
                "Request queue, elevator, and I/O scheduling",
            ),
            (
                "File system interface defined",
                "VFS or equivalent abstraction layer",
            ),
            (
                "Network stack configured",
                "Socket interface, protocol handlers, and NIC driver",
            ),
            (
                "Asynchronous I/O supported",
                "io_uring, epoll, kqueue, or IOCP integration",
            ),
            (
                "Buffer management optimized",
                "Page cache, buffer pool, and zero-copy I/O",
            ),
        ],
    )?;

    let s5 = add_template_sprint(
        conn,
        tpl,
        "Security Hardening",
        "Kernel security and exploit mitigation",
        4,
    )?;
    add_template_sprint_sections(conn, s5, &[security])?;
    add_custom_sprint_section(
        conn,
        s5,
        "Systems Security",
        "Low-level security measures",
        "#ef4444",
        &[
            (
                "ASLR enabled",
                "Address space layout randomization for all mappings",
            ),
            (
                "Stack canaries configured",
                "Stack smashing protection with canary validation",
            ),
            ("NX/DEP enabled", "Non-executable stack and data segments"),
            (
                "Kernel hardening applied",
                "KASLR, SMEP, SMAP, and kernel page table isolation",
            ),
            (
                "Syscall filtering implemented",
                "seccomp, capsicum, or similar syscall sandboxing",
            ),
        ],
    )?;

    let s6 = add_template_sprint(
        conn,
        tpl,
        "Testing & Validation",
        "Kernel testing, fuzzing, and verification",
        5,
    )?;
    add_template_sprint_sections(conn, s6, &[testing])?;
    add_custom_sprint_section(
        conn,
        s6,
        "Systems Testing",
        "Low-level testing strategies",
        "#10b981",
        &[
            (
                "Unit tests for core modules",
                "Memory management, scheduling, and IPC tests",
            ),
            (
                "Kernel fuzzing configured",
                "syzkaller or similar for syscall fuzzing",
            ),
            (
                "Stress testing completed",
                "Memory pressure, CPU saturation, and I/O storms",
            ),
            (
                "Hardware compatibility testing",
                "Multiple CPU architectures and device configurations",
            ),
            (
                "Formal verification where applicable",
                "Model checking for critical algorithms",
            ),
        ],
    )?;

    let s7 = add_template_sprint(
        conn,
        tpl,
        "Performance Optimization",
        "Latency, throughput, and resource efficiency",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[performance])?;
    add_custom_sprint_section(
        conn,
        s7,
        "Systems Performance",
        "Low-level optimization techniques",
        "#f97316",
        &[
            (
                "Cache line optimization applied",
                "Data structure alignment and false sharing elimination",
            ),
            (
                "Branch prediction optimization",
                "Likely/unlikely hints and branch-free code paths",
            ),
            (
                "Lock contention profiling",
                "Hot lock identification and lock-free alternatives",
            ),
            (
                "Syscall overhead minimized",
                "Batched syscalls, io_uring, or vDSO optimizations",
            ),
            (
                "Interrupt latency measured and optimized",
                "Interrupt coalescing and priority tuning",
            ),
        ],
    )?;

    let s8 = add_template_sprint(
        conn,
        tpl,
        "Debugging & Diagnostics",
        "Debug infrastructure and crash analysis",
        7,
    )?;
    add_custom_sprint_section(
        conn,
        s8,
        "Debug Infrastructure",
        "Debugging tools and crash handling",
        "#64748b",
        &[
            (
                "Kernel debugger configured",
                "KGDB, LLDB, or JTAG-based debugging setup",
            ),
            (
                "Crash dump mechanism implemented",
                "Panic handler with core dump and stack trace",
            ),
            (
                "Tracing infrastructure enabled",
                "ftrace, eBPF, or DTrace for runtime analysis",
            ),
            (
                "Performance counters accessible",
                "Hardware PMU access with perf integration",
            ),
            (
                "Logging subsystem configured",
                "Ring buffer logging with log levels and filtering",
            ),
        ],
    )?;

    let s9 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Build System",
        "Cross-compilation and automated testing",
        8,
    )?;
    add_template_sprint_sections(conn, s9, &[cicd, docs])?;
    add_custom_sprint_section(
        conn,
        s9,
        "Build System",
        "Cross-platform build and toolchain",
        "#06b6d4",
        &[
            (
                "Cross-compilation toolchain configured",
                "Target-specific compilers, linkers, and sysroots",
            ),
            (
                "Build system automated",
                "Make, CMake, or custom with dependency tracking",
            ),
            (
                "Multi-architecture builds supported",
                "x86_64, ARM64, RISC-V build targets",
            ),
            (
                "Static analysis integrated",
                "Coverity, clang-tidy, or sparse in CI pipeline",
            ),
            (
                "Binary size optimization applied",
                "LTO, strip, and dead code elimination",
            ),
        ],
    )?;

    let s10 = add_template_sprint(
        conn,
        tpl,
        "Documentation & Release",
        "Kernel docs, changelog, and release process",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[monitoring, docs])?;

    Ok(())
}
