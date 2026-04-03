use super::{
    add_custom_sprint_section, add_template, add_template_sprint, add_template_sprint_sections,
};
use super::super::super::types::AppError;
use rusqlite::Connection;

pub fn seed(
    conn: &Connection,
    section_map: &std::collections::HashMap<String, i64>,
    sprint_map: &std::collections::HashMap<String, i64>,
) -> Result<(), AppError> {
    let planning = section_map.get("planning").copied().unwrap_or(0);
    let security = section_map.get("security").copied().unwrap_or(0);
    let testing = section_map.get("testing").copied().unwrap_or(0);
    let cicd = section_map.get("cicd").copied().unwrap_or(0);
    let docs = section_map.get("docs").copied().unwrap_or(0);
    let quality = section_map.get("quality").copied().unwrap_or(0);
    let performance = section_map.get("performance").copied().unwrap_or(0);
    let monitoring = section_map.get("monitoring").copied().unwrap_or(0);
    let database = section_map.get("database").copied().unwrap_or(0);

    let planning_setup = sprint_map.get("planning_setup").copied().unwrap_or(0);
    let security_quality = sprint_map.get("security_quality").copied().unwrap_or(0);
    let testing_qa = sprint_map.get("testing_qa").copied().unwrap_or(0);
    let cicd_deploy = sprint_map.get("cicd_deploy").copied().unwrap_or(0);
    let monitoring_ops = sprint_map.get("monitoring_ops").copied().unwrap_or(0);
    let perf_sprint = sprint_map.get("performance").copied().unwrap_or(0);
    let db_sprint = sprint_map.get("database").copied().unwrap_or(0);

    // === Embedded Systems & IoT ===
    let tpl = add_template(
        conn,
        "Embedded Systems & IoT",
        "Firmware, hardware integration, and IoT device development",
        "#14b8a6",
    )?;

    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Hardware Setup",
        "Hardware selection, toolchain, and project structure",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning])?;
    add_custom_sprint_section(
        conn,
        s1,
        "Hardware & Toolchain",
        "MCU selection, dev tools, and flash programming",
        "#14b8a6",
        &[
            (
                "Microcontroller or SoC selected",
                "ARM Cortex-M, RISC-V, ESP32, or similar with datasheet reviewed",
            ),
            (
                "Development board acquired and tested",
                "Blink test, serial output, and debug probe verified",
            ),
            (
                "Toolchain installed and configured",
                "GCC/Clang for target, OpenOCD, JTAG/SWD debugger",
            ),
            (
                "Build system configured",
                "CMake, Make, or platform-specific (PlatformIO, ESP-IDF)",
            ),
            (
                "Hardware abstraction layer planned",
                "HAL, BSP, or direct register access strategy decided",
            ),
        ],
    )?;

    let s2 = add_template_sprint(
        conn,
        tpl,
        "Firmware Foundation",
        "Boot process, clock config, and basic peripherals",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Boot & System Init",
        "Startup code and system configuration",
        "#06b6d4",
        &[
            (
                "Startup code and vector table configured",
                "Reset handler, interrupt vectors, and stack setup",
            ),
            (
                "Clock tree configured",
                "PLL, clock sources, and peripheral clock enables",
            ),
            (
                "Memory layout defined",
                "Linker script with flash, RAM, and peripheral regions",
            ),
            (
                "Watchdog timer configured",
                "System watchdog with proper feed intervals",
            ),
            (
                "Power management modes defined",
                "Sleep, deep sleep, and standby with wake sources",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Peripheral Drivers",
        "GPIO, UART, SPI, I2C, and ADC drivers",
        "#10b981",
        &[
            (
                "GPIO driver implemented",
                "Pin configuration, input/output, and interrupt handling",
            ),
            (
                "UART/serial communication configured",
                "Baud rate, parity, and DMA-based transfers",
            ),
            (
                "SPI/I2C bus drivers implemented",
                "Master/slave modes with error handling",
            ),
            (
                "ADC/DAC configured",
                "Sampling rate, resolution, and calibration",
            ),
            (
                "Timer and PWM drivers implemented",
                "Hardware timers, PWM output, and input capture",
            ),
        ],
    )?;

    let s3 = add_template_sprint(
        conn,
        tpl,
        "Sensor Integration",
        "Sensor drivers, data acquisition, and calibration",
        2,
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Sensor Drivers",
        "Hardware sensor integration and data processing",
        "#8b5cf6",
        &[
            (
                "Sensor communication protocol implemented",
                "I2C, SPI, or analog read for each sensor",
            ),
            (
                "Sensor data parsing and conversion",
                "Raw values to engineering units with calibration",
            ),
            (
                "Sensor fusion algorithm implemented",
                "Kalman filter, complementary filter, or similar",
            ),
            (
                "Sensor error handling and timeouts",
                "Stuck sensor detection and fallback values",
            ),
            (
                "Sampling rate and buffering configured",
                "DMA-based acquisition with ring buffers",
            ),
        ],
    )?;

    let s4 = add_template_sprint(
        conn,
        tpl,
        "Communication Protocol",
        "Network stack, IoT protocols, and data transmission",
        3,
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Network Stack",
        "Connectivity and protocol implementation",
        "#f59e0b",
        &[
            (
                "WiFi, BLE, or LoRa stack configured",
                "Radio initialization, connection, and reconnection",
            ),
            (
                "MQTT or CoAP client implemented",
                "Publish/subscribe with QoS levels and retained messages",
            ),
            (
                "TLS/DTLS encryption configured",
                "Certificate management and secure connections",
            ),
            (
                "OTA update mechanism designed",
                "Dual-bank firmware update with rollback capability",
            ),
            (
                "Data serialization format chosen",
                "Protocol Buffers, CBOR, or JSON for payload encoding",
            ),
        ],
    )?;

    let s5 = add_template_sprint(
        conn,
        tpl,
        "Power Management",
        "Battery optimization and power modes",
        4,
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Power Optimization",
        "Energy-efficient operation and battery management",
        "#ef4444",
        &[
            (
                "Sleep mode implementation",
                "Deep sleep with periodic wake for sensor reads",
            ),
            (
                "Dynamic frequency scaling configured",
                "Clock speed adjustment based on workload",
            ),
            (
                "Peripheral power gating implemented",
                "Power off unused peripherals between tasks",
            ),
            (
                "Battery monitoring implemented",
                "Voltage, current, and charge level with low-battery alerts",
            ),
            (
                "Power consumption profiled",
                "Current draw measured in all operating modes",
            ),
        ],
    )?;

    let s6 = add_template_sprint(
        conn,
        tpl,
        "Security Hardening",
        "Firmware security and device authentication",
        5,
    )?;
    add_template_sprint_sections(conn, s6, &[security])?;
    add_custom_sprint_section(
        conn,
        s6,
        "Embedded Security",
        "Device-level security measures",
        "#ef4444",
        &[
            (
                "Secure boot configured",
                "Signed firmware with hardware verification",
            ),
            (
                "Flash read protection enabled",
                "Debug port lockdown and readout protection",
            ),
            (
                "Unique device identity provisioned",
                "Hardware unique ID or secure element for auth",
            ),
            (
                "Firmware encryption implemented",
                "Encrypted firmware images with secure key storage",
            ),
            (
                "Input validation on all external data",
                "Sanitize UART, network, and sensor inputs",
            ),
        ],
    )?;

    let s7 = add_template_sprint(
        conn,
        tpl,
        "Testing & QA",
        "Hardware-in-loop testing and validation",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[testing])?;
    add_custom_sprint_section(
        conn,
        s7,
        "Embedded Testing",
        "Hardware-specific testing strategies",
        "#10b981",
        &[
            (
                "Unit tests for firmware modules",
                "Mocked hardware interfaces with test frameworks",
            ),
            (
                "Hardware-in-loop test bench configured",
                "Automated testing with real hardware and stimuli",
            ),
            (
                "Integration tests with cloud backend",
                "End-to-end data flow from device to cloud",
            ),
            (
                "Stress and endurance testing",
                "Long-running tests with edge case injection",
            ),
            (
                "EMC/EMI compliance testing planned",
                "Electromagnetic compatibility test procedures",
            ),
        ],
    )?;

    let s8 = add_template_sprint(
        conn,
        tpl,
        "Safety & Certification",
        "Functional safety and regulatory compliance",
        7,
    )?;
    add_custom_sprint_section(
        conn,
        s8,
        "Safety Compliance",
        "Industry standards and certification requirements",
        "#f97316",
        &[
            (
                "Safety requirements documented",
                "IEC 61508, ISO 26262, or industry-specific standards",
            ),
            (
                "FMEA analysis completed",
                "Failure mode and effects analysis for critical functions",
            ),
            (
                "Watchdog and fault detection verified",
                "System recovery from all identified failure modes",
            ),
            (
                "Regulatory certification testing",
                "FCC, CE, or other regional certifications",
            ),
            (
                "Safety documentation package prepared",
                "Design docs, test reports, and compliance evidence",
            ),
        ],
    )?;

    let s9 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Deployment",
        "Automated build, flash, and OTA pipeline",
        8,
    )?;
    add_template_sprint_sections(conn, s9, &[cicd, docs])?;

    let s10 = add_template_sprint(
        conn,
        tpl,
        "Monitoring & Operations",
        "Device telemetry and fleet management",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[monitoring, docs])?;

    Ok(())
}
