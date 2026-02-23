use super::DomainEntry;

/// SAFETY-1: Safety, emergency, and regulatory compliance (Registry ID 0x07)
pub const SAFETY1_REGISTRY_ID: u8 = 0x07;
pub const SAFETY1_NAME: &str = "SAFETY-1";

pub static SAFETY1_ENTRIES: &[DomainEntry] = &[
    // Emergency Levels and Alerts (0x0000-0x001F)
    DomainEntry { code: 0x0000, mnemonic: "EMERGENCY_LEVEL", value_type: "UINT8", unit: "", description: "0=clear, 1=caution, 2=warning, 3=danger, 4=critical, 5=catastrophic" },
    DomainEntry { code: 0x0001, mnemonic: "EMERGENCY_TYPE", value_type: "UINT8", unit: "", description: "0=collision, 1=fire, 2=flood, 3=structural, 4=chemical, 5=electrical, 6=medical, 7=security, 8=loss_of_control" },
    DomainEntry { code: 0x0002, mnemonic: "EMERGENCY_DECLARE", value_type: "STRUCT{level,type,pos,desc}", unit: "", description: "Declare emergency with location and description" },
    DomainEntry { code: 0x0003, mnemonic: "EMERGENCY_CLEAR", value_type: "STRUCT{type}", unit: "", description: "Declare emergency condition resolved" },
    DomainEntry { code: 0x0004, mnemonic: "MAYDAY", value_type: "STRUCT{agent,pos,nature}", unit: "", description: "Distress call: agent in immediate danger" },
    DomainEntry { code: 0x0005, mnemonic: "PAN_PAN", value_type: "STRUCT{agent,pos,nature}", unit: "", description: "Urgency call: agent needs assistance" },
    DomainEntry { code: 0x0006, mnemonic: "ALL_STOP", value_type: "NONE", unit: "", description: "Immediate halt command to all agents" },
    DomainEntry { code: 0x0007, mnemonic: "RESUME_OPERATIONS", value_type: "NONE", unit: "", description: "Resume normal operations after all-stop" },
    DomainEntry { code: 0x0008, mnemonic: "EVACUATION_ORDER", value_type: "STRUCT{zone,rally_point}", unit: "", description: "Order to evacuate zone to rally point" },
    DomainEntry { code: 0x0009, mnemonic: "SHELTER_IN_PLACE", value_type: "STRUCT{zone,duration}", unit: "", description: "Order to hold position and wait" },
    DomainEntry { code: 0x000A, mnemonic: "DISTRESS_BEACON", value_type: "STRUCT{uuid,pos,ts}", unit: "", description: "Periodic emergency beacon until rescued/resolved" },

    // Human Safety (0x0020-0x002F)
    DomainEntry { code: 0x0020, mnemonic: "HUMAN_DETECTED", value_type: "STRUCT{pos,distance,conf}", unit: "", description: "Human presence detected near agent" },
    DomainEntry { code: 0x0021, mnemonic: "HUMAN_PROXIMITY", value_type: "FLOAT32", unit: "m", description: "Distance to nearest detected human" },
    DomainEntry { code: 0x0022, mnemonic: "HUMAN_IN_WORKSPACE", value_type: "BOOL", unit: "", description: "Human has entered robot workspace" },
    DomainEntry { code: 0x0023, mnemonic: "SAFETY_ZONE", value_type: "UINT8", unit: "", description: "0=safe (>2m), 1=warning (1-2m), 2=protective (<1m), 3=danger (<0.5m)" },
    DomainEntry { code: 0x0024, mnemonic: "SPEED_LIMIT", value_type: "FLOAT32", unit: "m/s", description: "Current speed limit for human safety" },
    DomainEntry { code: 0x0025, mnemonic: "FORCE_LIMIT", value_type: "FLOAT32", unit: "N", description: "Current force limit for human safety" },
    DomainEntry { code: 0x0026, mnemonic: "PROTECTIVE_STOP", value_type: "STRUCT{reason,pos}", unit: "", description: "Safety-rated protective stop engaged" },
    DomainEntry { code: 0x0027, mnemonic: "SAFETY_STOP_CLEAR", value_type: "NONE", unit: "", description: "Protective stop condition resolved" },
    DomainEntry { code: 0x0028, mnemonic: "PERSON_TRACKING", value_type: "LIST<STRUCT{id,pos,vel}>", unit: "", description: "All tracked persons with trajectories" },
    DomainEntry { code: 0x0029, mnemonic: "PERSON_PREDICTED", value_type: "STRUCT{id,pred_pos,horizon}", unit: "", description: "Predicted person position at time horizon" },
    DomainEntry { code: 0x002A, mnemonic: "COLLABORATIVE_MODE", value_type: "UINT8", unit: "", description: "0=separated, 1=coexistence, 2=cooperation, 3=collaboration (ISO 10218)" },
    DomainEntry { code: 0x002B, mnemonic: "SAFETY_RATED_SPEED", value_type: "FLOAT32", unit: "m/s", description: "Safety-rated monitored speed (ISO/TS 15066)" },
    DomainEntry { code: 0x002C, mnemonic: "POWER_FORCE_LIMIT", value_type: "STRUCT{body_part,max_force}", unit: "N", description: "ISO/TS 15066 per-body-part force limits" },

    // Fault and Failure (0x0040-0x004F)
    DomainEntry { code: 0x0040, mnemonic: "FAULT_DETECTED", value_type: "STRUCT{system,code,severity}", unit: "", description: "System fault detected" },
    DomainEntry { code: 0x0041, mnemonic: "FAULT_CLEARED", value_type: "STRUCT{system,code}", unit: "", description: "Fault condition resolved" },
    DomainEntry { code: 0x0042, mnemonic: "FAILSAFE_ACTIVE", value_type: "STRUCT{type}", unit: "", description: "Failsafe mode engaged: 0=soft_stop, 1=safe_park, 2=return_home, 3=power_off, 4=controlled_descent" },
    DomainEntry { code: 0x0043, mnemonic: "REDUNDANCY_STATUS", value_type: "STRUCT{system,primary,backup}", unit: "", description: "Redundant system health" },
    DomainEntry { code: 0x0044, mnemonic: "WATCHDOG_TRIP", value_type: "STRUCT{module,last_seen}", unit: "", description: "Watchdog timer expired for module" },
    DomainEntry { code: 0x0045, mnemonic: "COMM_LOST", value_type: "STRUCT{agent,duration}", unit: "", description: "Communication lost with agent" },
    DomainEntry { code: 0x0046, mnemonic: "COMM_RESTORED", value_type: "STRUCT{agent}", unit: "", description: "Communication restored with agent" },
    DomainEntry { code: 0x0047, mnemonic: "GPS_LOST", value_type: "NONE", unit: "", description: "GPS signal lost" },
    DomainEntry { code: 0x0048, mnemonic: "GPS_RESTORED", value_type: "STRUCT{accuracy}", unit: "m", description: "GPS signal restored with accuracy" },
    DomainEntry { code: 0x0049, mnemonic: "SENSOR_FAULT", value_type: "STRUCT{sensor_id,type}", unit: "", description: "Sensor fault: 0=degraded, 1=failed, 2=inconsistent, 3=stuck" },
    DomainEntry { code: 0x004A, mnemonic: "ACTUATOR_FAULT", value_type: "STRUCT{actuator_id,type}", unit: "", description: "Actuator fault: 0=degraded, 1=locked, 2=runaway, 3=disconnected" },
    DomainEntry { code: 0x004B, mnemonic: "POWER_FAULT", value_type: "STRUCT{type,details}", unit: "", description: "Power system fault: 0=brownout, 1=overcurrent, 2=cell_imbalance, 3=thermal_runaway" },
    DomainEntry { code: 0x004C, mnemonic: "ESTOP_PRESSED", value_type: "STRUCT{agent,source}", unit: "", description: "Emergency stop button activated" },
    DomainEntry { code: 0x004D, mnemonic: "ESTOP_RELEASED", value_type: "STRUCT{agent}", unit: "", description: "Emergency stop button released" },

    // Geofence and Regulatory (0x0060-0x006F)
    DomainEntry { code: 0x0060, mnemonic: "GEOFENCE_BREACH", value_type: "STRUCT{fence_id,pos}", unit: "", description: "Agent has breached geofence boundary" },
    DomainEntry { code: 0x0061, mnemonic: "ALTITUDE_LIMIT", value_type: "FLOAT32", unit: "m", description: "Maximum permitted altitude" },
    DomainEntry { code: 0x0062, mnemonic: "ALTITUDE_BREACH", value_type: "STRUCT{current,limit}", unit: "m", description: "Agent exceeds altitude limit" },
    DomainEntry { code: 0x0063, mnemonic: "SPEED_BREACH", value_type: "STRUCT{current,limit}", unit: "m/s", description: "Agent exceeds speed limit" },
    DomainEntry { code: 0x0064, mnemonic: "RESTRICTED_ZONE", value_type: "STRUCT{id,polygon,floor,ceiling}", unit: "", description: "Defined restricted zone" },
    DomainEntry { code: 0x0065, mnemonic: "ZONE_ENTERED", value_type: "STRUCT{zone_id}", unit: "", description: "Agent entered restricted zone" },
    DomainEntry { code: 0x0066, mnemonic: "ZONE_EXITED", value_type: "STRUCT{zone_id}", unit: "", description: "Agent exited restricted zone" },
    DomainEntry { code: 0x0067, mnemonic: "FLIGHT_AUTH", value_type: "STRUCT{area,start,end,auth_id}", unit: "", description: "Regulatory flight authorization" },
    DomainEntry { code: 0x0068, mnemonic: "REMOTE_ID", value_type: "STRUCT{uuid,pos,alt,vel,pilot_pos}", unit: "", description: "Remote identification broadcast (FAA compliance)" },
    DomainEntry { code: 0x0069, mnemonic: "NOISE_LIMIT", value_type: "FLOAT16", unit: "dB_SPL", description: "Maximum permitted noise level" },
    DomainEntry { code: 0x006A, mnemonic: "OPERATING_HOURS", value_type: "STRUCT{start,end}", unit: "", description: "Permitted operating time window" },
    DomainEntry { code: 0x006B, mnemonic: "WEATHER_LIMIT", value_type: "STRUCT{max_wind,min_vis,max_rain}", unit: "", description: "Weather operating limits" },
    DomainEntry { code: 0x006C, mnemonic: "WEATHER_ABORT", value_type: "STRUCT{condition}", unit: "", description: "Weather exceeds operating limits" },

    // Safety Monitoring (0x0080-0x008F)
    DomainEntry { code: 0x0080, mnemonic: "SAFETY_SCORE", value_type: "FLOAT16", unit: "", description: "Overall safety score 0.0-1.0" },
    DomainEntry { code: 0x0081, mnemonic: "RISK_ASSESSMENT", value_type: "STRUCT{hazard,probability,severity}", unit: "", description: "Risk assessment for hazard" },
    DomainEntry { code: 0x0082, mnemonic: "MITIGATION_ACTIVE", value_type: "STRUCT{risk_id,measure}", unit: "", description: "Active risk mitigation measure" },
    DomainEntry { code: 0x0083, mnemonic: "SAFETY_LOG", value_type: "STRUCT{event,ts,details}", unit: "", description: "Safety event log entry" },
    DomainEntry { code: 0x0084, mnemonic: "NEAR_MISS", value_type: "STRUCT{type,agents,min_dist}", unit: "", description: "Near-miss incident report" },
    DomainEntry { code: 0x0085, mnemonic: "INCIDENT_REPORT", value_type: "STRUCT{type,agents,pos,ts,desc}", unit: "", description: "Post-incident report" },
    DomainEntry { code: 0x0086, mnemonic: "SAFE_LANDING_SITES", value_type: "LIST<STRUCT{pos,quality}>", unit: "", description: "Available emergency landing sites" },
    DomainEntry { code: 0x0087, mnemonic: "ESCAPE_ROUTE", value_type: "LIST<POSITION_3D>", unit: "", description: "Planned escape route from current position" },
    DomainEntry { code: 0x0088, mnemonic: "BATTERY_RESERVE", value_type: "FLOAT16", unit: "%", description: "Battery reserved for safe return" },
    DomainEntry { code: 0x0089, mnemonic: "POINT_OF_NO_RETURN", value_type: "STRUCT{pos,time}", unit: "", description: "Must-decide point for safe return" },
    DomainEntry { code: 0x008A, mnemonic: "CONTINGENCY_PLAN", value_type: "STRUCT{trigger,action}", unit: "", description: "If-trigger-then-action safety plan" },
    DomainEntry { code: 0x008B, mnemonic: "BLACK_BOX_MARK", value_type: "STRUCT{event,ts}", unit: "", description: "Mark event in flight recorder / black box" },
];
