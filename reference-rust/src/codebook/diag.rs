use super::DomainEntry;

/// DIAG-1: Diagnostics domain codebook (Registry ID 0x05)
pub const DIAG1_REGISTRY_ID: u8 = 0x05;
pub const DIAG1_NAME: &str = "DIAG-1";

pub static DIAG1_ENTRIES: &[DomainEntry] = &[
    // Power and Energy (0x0000-0x001F)
    DomainEntry { code: 0x0000, mnemonic: "BATTERY_LEVEL", value_type: "FLOAT16", unit: "%", description: "Battery state of charge 0-100%" },
    DomainEntry { code: 0x0001, mnemonic: "BATTERY_VOLTAGE", value_type: "FLOAT16", unit: "V", description: "Battery terminal voltage" },
    DomainEntry { code: 0x0002, mnemonic: "BATTERY_CURRENT", value_type: "FLOAT16", unit: "A", description: "Battery discharge current" },
    DomainEntry { code: 0x0003, mnemonic: "BATTERY_TEMP", value_type: "FLOAT16", unit: "K", description: "Battery temperature" },
    DomainEntry { code: 0x0004, mnemonic: "CHARGE_RATE", value_type: "FLOAT16", unit: "W", description: "Current charge rate" },
    DomainEntry { code: 0x0005, mnemonic: "TIME_REMAINING", value_type: "FLOAT32", unit: "s", description: "Estimated runtime remaining" },
    DomainEntry { code: 0x0006, mnemonic: "POWER_CONSUMPTION", value_type: "FLOAT16", unit: "W", description: "Current total power draw" },
    DomainEntry { code: 0x0007, mnemonic: "ENERGY_CONSUMED", value_type: "FLOAT32", unit: "J", description: "Total energy consumed this session" },
    DomainEntry { code: 0x0008, mnemonic: "CHARGING_STATUS", value_type: "UINT8", unit: "", description: "0=discharging, 1=charging, 2=full, 3=fault" },
    DomainEntry { code: 0x0009, mnemonic: "POWER_SOURCE", value_type: "UINT8", unit: "", description: "0=battery, 1=wired, 2=solar, 3=fuel_cell" },

    // Compute and Memory (0x0020-0x003F)
    DomainEntry { code: 0x0020, mnemonic: "CPU_LOAD", value_type: "FLOAT16", unit: "%", description: "CPU utilization 0-100%" },
    DomainEntry { code: 0x0021, mnemonic: "GPU_LOAD", value_type: "FLOAT16", unit: "%", description: "GPU utilization 0-100%" },
    DomainEntry { code: 0x0022, mnemonic: "MEMORY_USED", value_type: "UINT32", unit: "KB", description: "Memory in use" },
    DomainEntry { code: 0x0023, mnemonic: "MEMORY_TOTAL", value_type: "UINT32", unit: "KB", description: "Total available memory" },
    DomainEntry { code: 0x0024, mnemonic: "STORAGE_USED", value_type: "UINT32", unit: "KB", description: "Storage in use" },
    DomainEntry { code: 0x0025, mnemonic: "STORAGE_TOTAL", value_type: "UINT32", unit: "KB", description: "Total available storage" },
    DomainEntry { code: 0x0026, mnemonic: "CPU_TEMP", value_type: "FLOAT16", unit: "K", description: "CPU temperature" },
    DomainEntry { code: 0x0027, mnemonic: "GPU_TEMP", value_type: "FLOAT16", unit: "K", description: "GPU temperature" },
    DomainEntry { code: 0x0028, mnemonic: "INFERENCE_RATE", value_type: "FLOAT32", unit: "Hz", description: "AI model inference rate" },
    DomainEntry { code: 0x0029, mnemonic: "MODEL_ID", value_type: "STRING", unit: "", description: "Active AI model identifier" },

    // Communication Health (0x0040-0x005F)
    DomainEntry { code: 0x0040, mnemonic: "AILL_SNR", value_type: "FLOAT16", unit: "dB", description: "Current AILL channel SNR" },
    DomainEntry { code: 0x0041, mnemonic: "AILL_BER", value_type: "FLOAT32", unit: "", description: "Current AILL bit error rate" },
    DomainEntry { code: 0x0042, mnemonic: "AILL_THROUGHPUT", value_type: "FLOAT32", unit: "bps", description: "Current effective data rate" },
    DomainEntry { code: 0x0043, mnemonic: "AILL_RETRANSMITS", value_type: "UINT16", unit: "", description: "Retransmission count this session" },
    DomainEntry { code: 0x0044, mnemonic: "AILL_LATENCY", value_type: "FLOAT16", unit: "ms", description: "Round-trip latency estimate" },
    DomainEntry { code: 0x0045, mnemonic: "WIFI_RSSI", value_type: "INT8", unit: "dBm", description: "WiFi signal strength" },
    DomainEntry { code: 0x0046, mnemonic: "NETWORK_STATUS", value_type: "UINT8", unit: "", description: "0=disconnected, 1=connected, 2=limited" },

    // System Status (0x0060-0x007F)
    DomainEntry { code: 0x0060, mnemonic: "UPTIME", value_type: "UINT32", unit: "s", description: "System uptime in seconds" },
    DomainEntry { code: 0x0061, mnemonic: "BOOT_COUNT", value_type: "UINT16", unit: "", description: "Number of system boots" },
    DomainEntry { code: 0x0062, mnemonic: "ERROR_COUNT", value_type: "UINT16", unit: "", description: "Cumulative error count" },
    DomainEntry { code: 0x0063, mnemonic: "LAST_ERROR", value_type: "STRUCT{code,msg,ts}", unit: "", description: "Most recent error record" },
    DomainEntry { code: 0x0064, mnemonic: "HEALTH_STATUS", value_type: "UINT8", unit: "", description: "0=nominal, 1=degraded, 2=critical, 3=emergency" },
    DomainEntry { code: 0x0065, mnemonic: "FIRMWARE_VERSION", value_type: "STRING", unit: "", description: "Firmware/software version string" },
    DomainEntry { code: 0x0066, mnemonic: "HARDWARE_ID", value_type: "STRING", unit: "", description: "Hardware model identifier" },
    DomainEntry { code: 0x0067, mnemonic: "CAPABILITIES_REPORT", value_type: "STRUCT", unit: "", description: "Full capability self-report" },
    DomainEntry { code: 0x0068, mnemonic: "SELF_TEST_RESULT", value_type: "STRUCT{pass,details}", unit: "", description: "Built-in self-test results" },
    DomainEntry { code: 0x0069, mnemonic: "MAINTENANCE_DUE", value_type: "TIMESTAMP", unit: "", description: "Next scheduled maintenance time" },
    DomainEntry { code: 0x006A, mnemonic: "OPERATING_MODE", value_type: "UINT8", unit: "", description: "0=idle, 1=active, 2=standby, 3=safe_mode, 4=shutdown" },
    DomainEntry { code: 0x006B, mnemonic: "ACTUATOR_STATUS", value_type: "LIST<STRUCT{id,ok,temp}>", unit: "", description: "Per-actuator health" },
];
