use super::DomainEntry;

/// PERCEPT-1: Perception domain codebook (Registry ID 0x02)
pub const PERCEPT1_REGISTRY_ID: u8 = 0x02;
pub const PERCEPT1_NAME: &str = "PERCEPT-1";

pub static PERCEPT1_ENTRIES: &[DomainEntry] = &[
    // Object Detection (0x0000-0x002F)
    DomainEntry { code: 0x0000, mnemonic: "DETECTED_OBJECT", value_type: "STRUCT", unit: "", description: "Detected object with properties" },
    DomainEntry { code: 0x0001, mnemonic: "OBJECT_CLASS", value_type: "UINT16", unit: "", description: "Object class from taxonomy" },
    DomainEntry { code: 0x0002, mnemonic: "OBJECT_CONFIDENCE", value_type: "FLOAT16", unit: "", description: "Detection confidence 0.0-1.0" },
    DomainEntry { code: 0x0003, mnemonic: "BOUNDING_BOX_2D", value_type: "ARRAY<FLOAT32,4>", unit: "px", description: "2D bbox (x, y, width, height)" },
    DomainEntry { code: 0x0004, mnemonic: "BOUNDING_BOX_3D", value_type: "STRUCT", unit: "m", description: "3D bbox (center, dimensions, orientation)" },
    DomainEntry { code: 0x0005, mnemonic: "OBJECT_POSITION", value_type: "ARRAY<FLOAT32,3>", unit: "m", description: "Object centroid in 3D" },
    DomainEntry { code: 0x0006, mnemonic: "OBJECT_VELOCITY", value_type: "ARRAY<FLOAT32,3>", unit: "m/s", description: "Object velocity estimate" },
    DomainEntry { code: 0x0007, mnemonic: "OBJECT_ID", value_type: "UINT32", unit: "", description: "Tracking ID (persistent across frames)" },
    DomainEntry { code: 0x0008, mnemonic: "OBJECT_LIST", value_type: "LIST<DETECTED_OBJECT>", unit: "", description: "Collection of detections" },
    DomainEntry { code: 0x0009, mnemonic: "SEGMENTATION_MASK", value_type: "BYTES", unit: "", description: "Run-length encoded pixel mask" },
    DomainEntry { code: 0x000A, mnemonic: "KEYPOINT", value_type: "ARRAY<FLOAT32,3>", unit: "px", description: "2D keypoint (x, y, confidence)" },
    DomainEntry { code: 0x000B, mnemonic: "KEYPOINT_SET", value_type: "LIST<KEYPOINT>", unit: "", description: "Named set of keypoints (skeleton)" },
    DomainEntry { code: 0x000C, mnemonic: "OBJECT_LABEL", value_type: "STRING", unit: "", description: "Human-readable label" },

    // Spatial Relations (0x0030-0x004F)
    DomainEntry { code: 0x0030, mnemonic: "ABOVE", value_type: "NONE", unit: "", description: "Spatial: A is above B" },
    DomainEntry { code: 0x0031, mnemonic: "BELOW", value_type: "NONE", unit: "", description: "Spatial: A is below B" },
    DomainEntry { code: 0x0032, mnemonic: "LEFT_OF", value_type: "NONE", unit: "", description: "Spatial: A is left of B" },
    DomainEntry { code: 0x0033, mnemonic: "RIGHT_OF", value_type: "NONE", unit: "", description: "Spatial: A is right of B" },
    DomainEntry { code: 0x0034, mnemonic: "IN_FRONT_OF", value_type: "NONE", unit: "", description: "Spatial: A is in front of B" },
    DomainEntry { code: 0x0035, mnemonic: "BEHIND", value_type: "NONE", unit: "", description: "Spatial: A is behind B" },
    DomainEntry { code: 0x0036, mnemonic: "INSIDE", value_type: "NONE", unit: "", description: "Spatial: A is inside B" },
    DomainEntry { code: 0x0037, mnemonic: "OUTSIDE", value_type: "NONE", unit: "", description: "Spatial: A is outside B" },
    DomainEntry { code: 0x0038, mnemonic: "ADJACENT", value_type: "NONE", unit: "", description: "Spatial: A is adjacent to B" },
    DomainEntry { code: 0x0039, mnemonic: "FAR_FROM", value_type: "NONE", unit: "", description: "Spatial: A is far from B" },
    DomainEntry { code: 0x003A, mnemonic: "NEAR", value_type: "NONE", unit: "", description: "Spatial: A is near B" },
    DomainEntry { code: 0x003B, mnemonic: "ON_TOP_OF", value_type: "NONE", unit: "", description: "Spatial: A is resting on B" },
    DomainEntry { code: 0x003C, mnemonic: "ATTACHED_TO", value_type: "NONE", unit: "", description: "Spatial: A is physically attached to B" },

    // Visual Properties (0x0050-0x006F)
    DomainEntry { code: 0x0050, mnemonic: "COLOR_RGB", value_type: "ARRAY<UINT8,3>", unit: "", description: "Color as (R, G, B)" },
    DomainEntry { code: 0x0051, mnemonic: "COLOR_NAME", value_type: "UINT8", unit: "", description: "Named color index" },
    DomainEntry { code: 0x0052, mnemonic: "TEXTURE", value_type: "UINT8", unit: "", description: "Texture class" },
    DomainEntry { code: 0x0053, mnemonic: "MATERIAL", value_type: "UINT8", unit: "", description: "Material class" },
    DomainEntry { code: 0x0054, mnemonic: "SHAPE", value_type: "UINT8", unit: "", description: "Shape class" },
    DomainEntry { code: 0x0055, mnemonic: "SIZE_RELATIVE", value_type: "UINT8", unit: "", description: "Relative size" },
    DomainEntry { code: 0x0056, mnemonic: "BRIGHTNESS", value_type: "FLOAT16", unit: "lux", description: "Measured brightness" },
    DomainEntry { code: 0x0057, mnemonic: "TRANSPARENCY", value_type: "FLOAT16", unit: "", description: "Transparency 0.0-1.0" },

    // Sensor Data (0x0070-0x008F)
    DomainEntry { code: 0x0070, mnemonic: "LIDAR_SCAN", value_type: "LIST<ARRAY<FLOAT32,3>>", unit: "m", description: "Point cloud from LiDAR" },
    DomainEntry { code: 0x0071, mnemonic: "DEPTH_MAP", value_type: "STRUCT{w,h,data}", unit: "m", description: "Depth image" },
    DomainEntry { code: 0x0072, mnemonic: "CAMERA_INTRINSICS", value_type: "STRUCT", unit: "", description: "Camera calibration matrix" },
    DomainEntry { code: 0x0073, mnemonic: "CAMERA_EXTRINSICS", value_type: "STRUCT", unit: "", description: "Camera pose" },
    DomainEntry { code: 0x0074, mnemonic: "IMAGE_EMBEDDING", value_type: "ARRAY<FLOAT16,N>", unit: "", description: "Feature embedding vector" },
    DomainEntry { code: 0x0075, mnemonic: "AUDIO_LEVEL", value_type: "FLOAT16", unit: "dB", description: "Ambient audio level" },
    DomainEntry { code: 0x0076, mnemonic: "TEMPERATURE", value_type: "FLOAT16", unit: "K", description: "Measured temperature" },
    DomainEntry { code: 0x0077, mnemonic: "HUMIDITY", value_type: "FLOAT16", unit: "%", description: "Relative humidity" },
    DomainEntry { code: 0x0078, mnemonic: "PRESSURE", value_type: "FLOAT32", unit: "Pa", description: "Atmospheric pressure" },
    DomainEntry { code: 0x0079, mnemonic: "IMU_DATA", value_type: "STRUCT{accel,gyro,mag}", unit: "", description: "Inertial measurement unit" },
];
