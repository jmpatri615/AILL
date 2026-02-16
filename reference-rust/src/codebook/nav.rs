use super::DomainEntry;

/// NAV-1: Navigation domain codebook (Registry ID 0x01)
pub const NAV1_REGISTRY_ID: u8 = 0x01;
pub const NAV1_NAME: &str = "NAV-1";

pub static NAV1_ENTRIES: &[DomainEntry] = &[
    // Coordinate and Pose (0x0000-0x002F)
    DomainEntry { code: 0x0000, mnemonic: "POSITION_3D", value_type: "ARRAY<FLOAT32,3>", unit: "m", description: "3D position (x, y, z)" },
    DomainEntry { code: 0x0001, mnemonic: "POSITION_2D", value_type: "ARRAY<FLOAT32,2>", unit: "m", description: "2D position (x, y)" },
    DomainEntry { code: 0x0002, mnemonic: "HEADING", value_type: "FLOAT32", unit: "rad", description: "Heading angle from North" },
    DomainEntry { code: 0x0003, mnemonic: "ORIENTATION_QUAT", value_type: "ARRAY<FLOAT32,4>", unit: "", description: "Quaternion (w, x, y, z)" },
    DomainEntry { code: 0x0004, mnemonic: "ORIENTATION_EULER", value_type: "ARRAY<FLOAT32,3>", unit: "rad", description: "Euler angles (roll, pitch, yaw)" },
    DomainEntry { code: 0x0005, mnemonic: "VELOCITY_3D", value_type: "ARRAY<FLOAT32,3>", unit: "m/s", description: "Linear velocity vector" },
    DomainEntry { code: 0x0006, mnemonic: "VELOCITY_SCALAR", value_type: "FLOAT32", unit: "m/s", description: "Scalar speed" },
    DomainEntry { code: 0x0007, mnemonic: "ANGULAR_VEL", value_type: "ARRAY<FLOAT32,3>", unit: "rad/s", description: "Angular velocity" },
    DomainEntry { code: 0x0008, mnemonic: "ACCELERATION_3D", value_type: "ARRAY<FLOAT32,3>", unit: "m/s^2", description: "Linear acceleration" },
    DomainEntry { code: 0x0009, mnemonic: "POSE_6DOF", value_type: "STRUCT{pos,orient}", unit: "", description: "Full 6DOF pose" },
    DomainEntry { code: 0x000A, mnemonic: "LATITUDE", value_type: "FLOAT64", unit: "deg", description: "WGS84 latitude" },
    DomainEntry { code: 0x000B, mnemonic: "LONGITUDE", value_type: "FLOAT64", unit: "deg", description: "WGS84 longitude" },
    DomainEntry { code: 0x000C, mnemonic: "ALTITUDE_MSL", value_type: "FLOAT32", unit: "m", description: "Altitude above mean sea level" },
    DomainEntry { code: 0x000D, mnemonic: "ALTITUDE_AGL", value_type: "FLOAT32", unit: "m", description: "Altitude above ground level" },
    DomainEntry { code: 0x000E, mnemonic: "GPS_FIX", value_type: "STRUCT", unit: "", description: "Complete GPS fix record" },
    DomainEntry { code: 0x000F, mnemonic: "COORDINATE_FRAME", value_type: "UINT8", unit: "", description: "Coord frame ID" },

    // Waypoint and Path (0x0030-0x005F)
    DomainEntry { code: 0x0030, mnemonic: "WAYPOINT", value_type: "STRUCT{id,pos,rad}", unit: "", description: "Named waypoint" },
    DomainEntry { code: 0x0031, mnemonic: "WAYPOINT_ID", value_type: "UINT16", unit: "", description: "Waypoint identifier" },
    DomainEntry { code: 0x0032, mnemonic: "PATH", value_type: "LIST<WAYPOINT>", unit: "", description: "Ordered waypoint sequence" },
    DomainEntry { code: 0x0033, mnemonic: "PATH_SEGMENT", value_type: "STRUCT", unit: "", description: "Segment with curvature" },
    DomainEntry { code: 0x0034, mnemonic: "CURRENT_WAYPOINT", value_type: "UINT16", unit: "", description: "Current target waypoint index" },
    DomainEntry { code: 0x0035, mnemonic: "DISTANCE_TO_WP", value_type: "FLOAT32", unit: "m", description: "Distance to current waypoint" },
    DomainEntry { code: 0x0036, mnemonic: "ETA", value_type: "FLOAT32", unit: "s", description: "Estimated time of arrival" },
    DomainEntry { code: 0x0037, mnemonic: "PATH_COMPLETE", value_type: "BOOL", unit: "", description: "Path completion flag" },
    DomainEntry { code: 0x0038, mnemonic: "PATH_DEVIATION", value_type: "FLOAT32", unit: "m", description: "Cross-track error" },
    DomainEntry { code: 0x0039, mnemonic: "GEOFENCE", value_type: "LIST<POSITION_2D>", unit: "", description: "Restricted area polygon" },
    DomainEntry { code: 0x003A, mnemonic: "GEOFENCE_STATUS", value_type: "UINT8", unit: "", description: "Geofence relation status" },
    DomainEntry { code: 0x003B, mnemonic: "HOME_POSITION", value_type: "POSITION_3D", unit: "m", description: "Designated home position" },

    // Obstacle and Environment (0x0060-0x008F)
    DomainEntry { code: 0x0060, mnemonic: "OBSTACLE", value_type: "STRUCT", unit: "", description: "Detected obstacle" },
    DomainEntry { code: 0x0061, mnemonic: "OBSTACLE_TYPE", value_type: "UINT8", unit: "", description: "Obstacle classification" },
    DomainEntry { code: 0x0062, mnemonic: "OBSTACLE_SIZE", value_type: "ARRAY<FLOAT32,3>", unit: "m", description: "Bounding box dimensions" },
    DomainEntry { code: 0x0063, mnemonic: "OBSTACLE_LIST", value_type: "LIST<OBSTACLE>", unit: "", description: "Collection of obstacles" },
    DomainEntry { code: 0x0064, mnemonic: "CLEARANCE", value_type: "FLOAT32", unit: "m", description: "Min clearance to nearest obstacle" },
    DomainEntry { code: 0x0065, mnemonic: "COLLISION_RISK", value_type: "FLOAT16", unit: "", description: "Collision probability 0.0-1.0" },
    DomainEntry { code: 0x0066, mnemonic: "TERRAIN_TYPE", value_type: "UINT8", unit: "", description: "Surface type code" },
    DomainEntry { code: 0x0067, mnemonic: "SLOPE_ANGLE", value_type: "FLOAT16", unit: "rad", description: "Ground slope" },
    DomainEntry { code: 0x0068, mnemonic: "VISIBILITY", value_type: "FLOAT32", unit: "m", description: "Visibility range" },
    DomainEntry { code: 0x0069, mnemonic: "OCCUPANCY_GRID", value_type: "STRUCT", unit: "", description: "2D occupancy grid map" },

    // Motion Commands (0x0090-0x00BF)
    DomainEntry { code: 0x0090, mnemonic: "GOTO", value_type: "POSITION_3D", unit: "m", description: "Navigate to position" },
    DomainEntry { code: 0x0091, mnemonic: "GOTO_WAYPOINT", value_type: "UINT16", unit: "", description: "Navigate to waypoint ID" },
    DomainEntry { code: 0x0092, mnemonic: "FOLLOW_PATH", value_type: "PATH", unit: "", description: "Execute path" },
    DomainEntry { code: 0x0093, mnemonic: "STOP", value_type: "NONE", unit: "", description: "Halt all movement" },
    DomainEntry { code: 0x0094, mnemonic: "HOLD_POSITION", value_type: "NONE", unit: "", description: "Station-keeping" },
    DomainEntry { code: 0x0095, mnemonic: "SET_VELOCITY", value_type: "VELOCITY_3D", unit: "m/s", description: "Set desired velocity" },
    DomainEntry { code: 0x0096, mnemonic: "SET_HEADING", value_type: "FLOAT32", unit: "rad", description: "Turn to heading" },
    DomainEntry { code: 0x0097, mnemonic: "ORBIT", value_type: "STRUCT", unit: "", description: "Orbit a point" },
    DomainEntry { code: 0x0098, mnemonic: "FOLLOW_AGENT", value_type: "STRUCT{uuid,dist}", unit: "", description: "Follow another agent" },
    DomainEntry { code: 0x0099, mnemonic: "RETURN_HOME", value_type: "NONE", unit: "", description: "Navigate to home" },
    DomainEntry { code: 0x009A, mnemonic: "AVOID", value_type: "STRUCT{pos,radius}", unit: "", description: "Add exclusion zone" },
    DomainEntry { code: 0x009B, mnemonic: "FORMATION", value_type: "STRUCT{type,slot}", unit: "", description: "Join formation" },
];
