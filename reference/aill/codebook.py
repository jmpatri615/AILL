"""
AILL Reference Implementation - Codebook Definitions
Acoustic Inter-agent Linguistic Link v1.1

This module defines the complete Base Codebook (Level 0) and standard
Level 1 domain codebooks for the AILL protocol.
"""

from enum import IntEnum
from dataclasses import dataclass, field
from typing import Optional, Any


# ═══════════════════════════════════════════════════════════════════════
# BASE CODEBOOK (Level 0) - 256 entries
# ═══════════════════════════════════════════════════════════════════════

class FrameControl(IntEnum):
    """0x00-0x0F: Frame control codes"""
    START_UTTERANCE = 0x00
    END_UTTERANCE   = 0x01
    ABORT           = 0x02
    PAUSE           = 0x03
    RESUME          = 0x04
    RETRANSMIT      = 0x05
    ACK_EPOCH       = 0x06
    NACK_EPOCH      = 0x07
    SYNC_MARK       = 0x08
    FRAGMENT_START  = 0x09
    FRAGMENT_CONT   = 0x0A
    FRAGMENT_END    = 0x0B
    ECHO_REQUEST    = 0x0C
    ECHO_REPLY      = 0x0D
    RESERVED_0E     = 0x0E
    RESERVED_0F     = 0x0F


class TypeMarker(IntEnum):
    """0x10-0x1F: Type markers"""
    TYPE_INT8       = 0x10
    TYPE_INT16      = 0x11
    TYPE_INT32      = 0x12
    TYPE_INT64      = 0x13
    TYPE_UINT8      = 0x14
    TYPE_UINT16     = 0x15
    TYPE_UINT32     = 0x16
    TYPE_UINT64     = 0x17
    TYPE_FLOAT16    = 0x18
    TYPE_FLOAT32    = 0x19
    TYPE_FLOAT64    = 0x1A
    TYPE_BOOL       = 0x1B
    TYPE_STRING     = 0x1C
    TYPE_BYTES      = 0x1D
    TYPE_TIMESTAMP  = 0x1E
    TYPE_NULL       = 0x1F


class Structure(IntEnum):
    """0x20-0x2F: Structure codes"""
    BEGIN_STRUCT    = 0x20
    END_STRUCT      = 0x21
    FIELD_SEP       = 0x22
    BEGIN_LIST      = 0x23
    END_LIST        = 0x24
    BEGIN_MAP       = 0x25
    END_MAP         = 0x26
    BEGIN_TUPLE     = 0x27
    END_TUPLE       = 0x28
    FIELD_ID        = 0x29
    BEGIN_UNION     = 0x2A
    END_UNION       = 0x2B
    BEGIN_OPTION    = 0x2C
    END_OPTION      = 0x2D
    SCHEMA_REF      = 0x2E
    RESERVED_2F     = 0x2F


class Quantifier(IntEnum):
    """0x30-0x3F: Quantifiers"""
    FORALL          = 0x30
    EXISTS          = 0x31
    EXISTS_UNIQUE   = 0x32
    EXACTLY_N       = 0x33
    AT_LEAST_N      = 0x34
    AT_MOST_N       = 0x35
    COUNT           = 0x36
    ZERO            = 0x37
    ONE             = 0x38
    FEW             = 0x39
    MANY            = 0x3A
    ALL             = 0x3B
    NONE_Q          = 0x3C
    MOST            = 0x3D
    PROPORTION      = 0x3E
    RESERVED_3F     = 0x3F


class Logic(IntEnum):
    """0x40-0x4F: Logic operators"""
    AND             = 0x40
    OR              = 0x41
    NOT             = 0x42
    XOR             = 0x43
    IMPLIES         = 0x44
    IFF             = 0x45
    NAND            = 0x46
    NOR             = 0x47
    IF_THEN_ELSE    = 0x48
    COALESCE        = 0x49
    IS_NULL         = 0x4A
    IS_TYPE         = 0x4B
    RESERVED_4C     = 0x4C
    RESERVED_4D     = 0x4D
    RESERVED_4E     = 0x4E
    RESERVED_4F     = 0x4F


class Relational(IntEnum):
    """0x50-0x5F: Relational operators"""
    EQ              = 0x50
    NEQ             = 0x51
    LT              = 0x52
    GT              = 0x53
    LTE             = 0x54
    GTE             = 0x55
    APPROX          = 0x56
    CONTAINS        = 0x57
    SUBSET          = 0x58
    SUPERSET        = 0x59
    IN_RANGE        = 0x5A
    MATCHES         = 0x5B
    STARTS_WITH     = 0x5C
    ENDS_WITH       = 0x5D
    BETWEEN         = 0x5E
    RESERVED_5F     = 0x5F


class Temporal(IntEnum):
    """0x60-0x6F: Temporal operators"""
    PAST            = 0x60
    PRESENT         = 0x61
    FUTURE          = 0x62
    DURATION        = 0x63
    T_BEFORE        = 0x64
    T_AFTER         = 0x65
    T_DURING        = 0x66
    T_SIMULTANEOUS  = 0x67
    T_STARTS        = 0x68
    T_FINISHES      = 0x69
    T_OVERLAPS      = 0x6A
    T_MEETS         = 0x6B
    T_ELAPSED       = 0x6C
    T_NOW           = 0x6D
    T_DEADLINE      = 0x6E
    RESERVED_6F     = 0x6F


class Modality(IntEnum):
    """0x70-0x7F: Modality operators"""
    CERTAIN         = 0x70
    PROBABLE        = 0x71
    POSSIBLE        = 0x72
    UNLIKELY        = 0x73
    UNCERTAIN       = 0x74
    HYPOTHETICAL    = 0x75
    COUNTERFACTUAL  = 0x76
    OBLIGATORY      = 0x77
    PERMITTED       = 0x78
    FORBIDDEN       = 0x79
    INFERRED        = 0x7A
    OBSERVED        = 0x7B
    REPORTED        = 0x7C
    PREDICTED       = 0x7D
    DESIRED         = 0x7E
    UNDESIRED       = 0x7F


class Pragmatic(IntEnum):
    """0x80-0x8F: Pragmatic acts"""
    QUERY           = 0x80
    ASSERT          = 0x81
    REQUEST         = 0x82
    COMMAND         = 0x83
    ACKNOWLEDGE     = 0x84
    REJECT          = 0x85
    CLARIFY         = 0x86
    CORRECT         = 0x87
    PROPOSE         = 0x88
    ACCEPT          = 0x89
    WARN            = 0x8A
    PROMISE         = 0x8B
    INFORM          = 0x8C
    SUGGEST         = 0x8D
    GREET           = 0x8E
    FAREWELL        = 0x8F


class Meta(IntEnum):
    """0x90-0x9F: Meta and annotation"""
    CONFIDENCE      = 0x90
    PRIORITY        = 0x91
    SOURCE_AGENT    = 0x92
    DEST_AGENT      = 0x93
    TIMESTAMP_META  = 0x94
    SEQNUM          = 0x95
    HASH_REF        = 0x96
    TOPIC           = 0x97
    CONTEXT_REF     = 0x98
    EPOCH_BOUNDARY  = 0x99
    LABEL           = 0x9A
    VERSION_TAG     = 0x9B
    TRACE_ID        = 0x9C
    COST            = 0x9D
    TTL             = 0x9E
    RESERVED_9F     = 0x9F


class Arithmetic(IntEnum):
    """0xA0-0xBF: Arithmetic and mathematical"""
    ADD             = 0xA0
    SUB             = 0xA1
    MUL             = 0xA2
    DIV             = 0xA3
    MOD             = 0xA4
    POW             = 0xA5
    SQRT            = 0xA6
    LOG             = 0xA7
    LOG10           = 0xA8
    LOG2            = 0xA9
    ABS             = 0xAA
    NEG             = 0xAB
    ROUND           = 0xAC
    FLOOR           = 0xAD
    CEIL            = 0xAE
    TRUNC           = 0xAF
    MIN             = 0xB0
    MAX             = 0xB1
    SUM             = 0xB2
    MEAN            = 0xB3
    MEDIAN          = 0xB4
    STDDEV          = 0xB5
    VARIANCE        = 0xB6
    DOT_PRODUCT     = 0xB7
    CROSS_PRODUCT   = 0xB8
    NORM            = 0xB9
    CLAMP           = 0xBA
    LERP            = 0xBB
    SIN             = 0xBC
    COS             = 0xBD
    ATAN2           = 0xBE
    DISTANCE        = 0xBF


class Escape(IntEnum):
    """0xF0-0xFF: Escape and extension codes"""
    ESCAPE_L1       = 0xF0
    ESCAPE_L2       = 0xF1
    ESCAPE_L3       = 0xF2
    LITERAL_BYTES   = 0xF3
    CODEBOOK_REF    = 0xF4
    EXTENSION       = 0xF5
    EXT_ACK         = 0xF6
    EXT_NACK        = 0xF7
    CODEBOOK_DEF    = 0xF8
    CODEBOOK_ACK    = 0xF9
    CODEBOOK_NACK   = 0xFA
    STREAM_ID       = 0xFB
    XREF            = 0xFC
    COMMENT         = 0xFD
    NOP             = 0xFE
    RESERVED_FF     = 0xFF


# ═══════════════════════════════════════════════════════════════════════
# CODE METADATA
# ═══════════════════════════════════════════════════════════════════════

@dataclass
class CodeEntry:
    """Metadata for a single codebook entry."""
    code: int
    mnemonic: str
    category: str
    arg_format: str = "none"        # Argument format description
    arity: int = 0                   # Stack operands consumed (for operators)
    result_count: int = 0            # Stack values produced
    description: str = ""
    min_level: str = "core"          # Minimum conformance level


# Build the complete base codebook lookup
BASE_CODEBOOK: dict[int, CodeEntry] = {}

def _register(enum_cls, category):
    for member in enum_cls:
        BASE_CODEBOOK[member.value] = CodeEntry(
            code=member.value,
            mnemonic=member.name,
            category=category,
        )

_register(FrameControl, "frame_control")
_register(TypeMarker, "type_marker")
_register(Structure, "structure")
_register(Quantifier, "quantifier")
_register(Logic, "logic")
_register(Relational, "relational")
_register(Temporal, "temporal")
_register(Modality, "modality")
_register(Pragmatic, "pragmatic")
_register(Meta, "meta")
_register(Arithmetic, "arithmetic")
_register(Escape, "escape")

# Mark reserved range 0xC0-0xEF
for code in range(0xC0, 0xF0):
    BASE_CODEBOOK[code] = CodeEntry(
        code=code,
        mnemonic=f"RESERVED_{code:02X}",
        category="reserved",
        description="Reserved for future base codebook expansion."
    )


# Operator arity table for the expression evaluator
BINARY_OPS = {
    Arithmetic.ADD, Arithmetic.SUB, Arithmetic.MUL, Arithmetic.DIV,
    Arithmetic.MOD, Arithmetic.POW, Arithmetic.MIN, Arithmetic.MAX,
    Arithmetic.DOT_PRODUCT, Arithmetic.CROSS_PRODUCT, Arithmetic.DISTANCE,
    Arithmetic.ATAN2,
    Logic.AND, Logic.OR, Logic.XOR, Logic.IMPLIES, Logic.IFF,
    Logic.NAND, Logic.NOR, Logic.COALESCE,
    Relational.EQ, Relational.NEQ, Relational.LT, Relational.GT,
    Relational.LTE, Relational.GTE, Relational.CONTAINS,
    Relational.SUBSET, Relational.SUPERSET,
    Temporal.T_BEFORE, Temporal.T_AFTER, Temporal.T_DURING,
    Temporal.T_SIMULTANEOUS, Temporal.T_STARTS, Temporal.T_FINISHES,
    Temporal.T_OVERLAPS, Temporal.T_MEETS,
}

UNARY_OPS = {
    Logic.NOT, Logic.IS_NULL,
    Arithmetic.SQRT, Arithmetic.LOG, Arithmetic.LOG10, Arithmetic.LOG2,
    Arithmetic.ABS, Arithmetic.NEG, Arithmetic.ROUND, Arithmetic.FLOOR,
    Arithmetic.CEIL, Arithmetic.TRUNC, Arithmetic.NORM,
    Arithmetic.SIN, Arithmetic.COS,
    Arithmetic.SUM, Arithmetic.MEAN, Arithmetic.MEDIAN,
    Arithmetic.STDDEV, Arithmetic.VARIANCE,
    Quantifier.COUNT,
    Temporal.T_ELAPSED,
}

TERNARY_OPS = {
    Logic.IF_THEN_ELSE,
    Arithmetic.CLAMP, Arithmetic.LERP,
    Relational.IN_RANGE,
}


# ═══════════════════════════════════════════════════════════════════════
# LEVEL 1 DOMAIN CODEBOOKS
# ═══════════════════════════════════════════════════════════════════════

@dataclass
class DomainEntry:
    """Entry in a Level 1 domain codebook."""
    code: int
    mnemonic: str
    value_type: str          # Expected AILL type signature
    unit: str = ""           # Physical unit (SI)
    description: str = ""


class DomainCodebook:
    """A complete Level 1 domain codebook."""
    
    def __init__(self, registry_id: int, name: str, domain: str):
        self.registry_id = registry_id
        self.name = name
        self.domain = domain
        self.entries: dict[int, DomainEntry] = {}
    
    def add(self, code: int, mnemonic: str, value_type: str,
            unit: str = "", description: str = ""):
        self.entries[code] = DomainEntry(code, mnemonic, value_type, unit, description)
    
    def lookup(self, code: int) -> Optional[DomainEntry]:
        return self.entries.get(code)
    
    def __len__(self):
        return len(self.entries)


# ── NAV-1: Navigation ──────────────────────────────────────────────────

NAV1 = DomainCodebook(0x01, "NAV-1", "Navigation and spatial positioning")

# Coordinate and Pose (0x0000-0x002F)
NAV1.add(0x0000, "POSITION_3D",      "ARRAY<FLOAT32,3>",    "m",     "3D position (x, y, z)")
NAV1.add(0x0001, "POSITION_2D",      "ARRAY<FLOAT32,2>",    "m",     "2D position (x, y)")
NAV1.add(0x0002, "HEADING",          "FLOAT32",             "rad",   "Heading angle from North")
NAV1.add(0x0003, "ORIENTATION_QUAT", "ARRAY<FLOAT32,4>",    "",      "Quaternion (w, x, y, z)")
NAV1.add(0x0004, "ORIENTATION_EULER","ARRAY<FLOAT32,3>",    "rad",   "Euler angles (roll, pitch, yaw)")
NAV1.add(0x0005, "VELOCITY_3D",      "ARRAY<FLOAT32,3>",    "m/s",   "Linear velocity vector")
NAV1.add(0x0006, "VELOCITY_SCALAR",  "FLOAT32",             "m/s",   "Scalar speed")
NAV1.add(0x0007, "ANGULAR_VEL",      "ARRAY<FLOAT32,3>",    "rad/s", "Angular velocity")
NAV1.add(0x0008, "ACCELERATION_3D",  "ARRAY<FLOAT32,3>",    "m/s^2", "Linear acceleration")
NAV1.add(0x0009, "POSE_6DOF",        "STRUCT{pos,orient}",  "",      "Full 6DOF pose")
NAV1.add(0x000A, "LATITUDE",         "FLOAT64",             "deg",   "WGS84 latitude")
NAV1.add(0x000B, "LONGITUDE",        "FLOAT64",             "deg",   "WGS84 longitude")
NAV1.add(0x000C, "ALTITUDE_MSL",     "FLOAT32",             "m",     "Altitude above mean sea level")
NAV1.add(0x000D, "ALTITUDE_AGL",     "FLOAT32",             "m",     "Altitude above ground level")
NAV1.add(0x000E, "GPS_FIX",          "STRUCT",              "",      "Complete GPS fix record")
NAV1.add(0x000F, "COORDINATE_FRAME", "UINT8",               "",      "Coord frame ID")

# Waypoint and Path (0x0030-0x005F)
NAV1.add(0x0030, "WAYPOINT",         "STRUCT{id,pos,rad}",  "",      "Named waypoint")
NAV1.add(0x0031, "WAYPOINT_ID",      "UINT16",              "",      "Waypoint identifier")
NAV1.add(0x0032, "PATH",             "LIST<WAYPOINT>",      "",      "Ordered waypoint sequence")
NAV1.add(0x0033, "PATH_SEGMENT",     "STRUCT",              "",      "Segment with curvature")
NAV1.add(0x0034, "CURRENT_WAYPOINT", "UINT16",              "",      "Current target waypoint index")
NAV1.add(0x0035, "DISTANCE_TO_WP",   "FLOAT32",             "m",     "Distance to current waypoint")
NAV1.add(0x0036, "ETA",              "FLOAT32",             "s",     "Estimated time of arrival")
NAV1.add(0x0037, "PATH_COMPLETE",    "BOOL",                "",      "Path completion flag")
NAV1.add(0x0038, "PATH_DEVIATION",   "FLOAT32",             "m",     "Cross-track error")
NAV1.add(0x0039, "GEOFENCE",         "LIST<POSITION_2D>",   "",      "Restricted area polygon")
NAV1.add(0x003A, "GEOFENCE_STATUS",  "UINT8",               "",      "Geofence relation status")
NAV1.add(0x003B, "HOME_POSITION",    "POSITION_3D",         "m",     "Designated home position")

# Obstacle and Environment (0x0060-0x008F)
NAV1.add(0x0060, "OBSTACLE",         "STRUCT",              "",      "Detected obstacle")
NAV1.add(0x0061, "OBSTACLE_TYPE",    "UINT8",               "",      "Obstacle classification")
NAV1.add(0x0062, "OBSTACLE_SIZE",    "ARRAY<FLOAT32,3>",    "m",     "Bounding box dimensions")
NAV1.add(0x0063, "OBSTACLE_LIST",    "LIST<OBSTACLE>",      "",      "Collection of obstacles")
NAV1.add(0x0064, "CLEARANCE",        "FLOAT32",             "m",     "Min clearance to nearest obstacle")
NAV1.add(0x0065, "COLLISION_RISK",   "FLOAT16",             "",      "Collision probability 0.0-1.0")
NAV1.add(0x0066, "TERRAIN_TYPE",     "UINT8",               "",      "Surface type code")
NAV1.add(0x0067, "SLOPE_ANGLE",      "FLOAT16",             "rad",   "Ground slope")
NAV1.add(0x0068, "VISIBILITY",       "FLOAT32",             "m",     "Visibility range")
NAV1.add(0x0069, "OCCUPANCY_GRID",   "STRUCT",              "",      "2D occupancy grid map")

# Motion Commands (0x0090-0x00BF)
NAV1.add(0x0090, "GOTO",             "POSITION_3D",         "m",     "Navigate to position")
NAV1.add(0x0091, "GOTO_WAYPOINT",    "UINT16",              "",      "Navigate to waypoint ID")
NAV1.add(0x0092, "FOLLOW_PATH",      "PATH",                "",      "Execute path")
NAV1.add(0x0093, "STOP",             "NONE",                "",      "Halt all movement")
NAV1.add(0x0094, "HOLD_POSITION",    "NONE",                "",      "Station-keeping")
NAV1.add(0x0095, "SET_VELOCITY",     "VELOCITY_3D",         "m/s",   "Set desired velocity")
NAV1.add(0x0096, "SET_HEADING",      "FLOAT32",             "rad",   "Turn to heading")
NAV1.add(0x0097, "ORBIT",            "STRUCT",              "",      "Orbit a point")
NAV1.add(0x0098, "FOLLOW_AGENT",     "STRUCT{uuid,dist}",   "",      "Follow another agent")
NAV1.add(0x0099, "RETURN_HOME",      "NONE",                "",      "Navigate to home")
NAV1.add(0x009A, "AVOID",            "STRUCT{pos,radius}",  "",      "Add exclusion zone")
NAV1.add(0x009B, "FORMATION",        "STRUCT{type,slot}",   "",      "Join formation")


# ── PERCEPT-1: Perception ──────────────────────────────────────────────

PERCEPT1 = DomainCodebook(0x02, "PERCEPT-1", "Visual and sensor perception")

# Object Detection (0x0000-0x002F)
PERCEPT1.add(0x0000, "DETECTED_OBJECT",  "STRUCT",              "",      "Detected object with properties")
PERCEPT1.add(0x0001, "OBJECT_CLASS",     "UINT16",              "",      "Object class from taxonomy")
PERCEPT1.add(0x0002, "OBJECT_CONFIDENCE","FLOAT16",             "",      "Detection confidence 0.0-1.0")
PERCEPT1.add(0x0003, "BOUNDING_BOX_2D",  "ARRAY<FLOAT32,4>",   "px",    "2D bbox (x, y, width, height)")
PERCEPT1.add(0x0004, "BOUNDING_BOX_3D",  "STRUCT",              "m",     "3D bbox (center, dimensions, orientation)")
PERCEPT1.add(0x0005, "OBJECT_POSITION",  "ARRAY<FLOAT32,3>",    "m",     "Object centroid in 3D")
PERCEPT1.add(0x0006, "OBJECT_VELOCITY",  "ARRAY<FLOAT32,3>",    "m/s",   "Object velocity estimate")
PERCEPT1.add(0x0007, "OBJECT_ID",        "UINT32",              "",      "Tracking ID (persistent across frames)")
PERCEPT1.add(0x0008, "OBJECT_LIST",      "LIST<DETECTED_OBJECT>","",     "Collection of detections")
PERCEPT1.add(0x0009, "SEGMENTATION_MASK","BYTES",               "",      "Run-length encoded pixel mask")
PERCEPT1.add(0x000A, "KEYPOINT",         "ARRAY<FLOAT32,3>",    "px",    "2D keypoint (x, y, confidence)")
PERCEPT1.add(0x000B, "KEYPOINT_SET",     "LIST<KEYPOINT>",      "",      "Named set of keypoints (skeleton)")
PERCEPT1.add(0x000C, "OBJECT_LABEL",     "STRING",              "",      "Human-readable label")

# Spatial Relations (0x0030-0x004F)
PERCEPT1.add(0x0030, "ABOVE",            "NONE",     "",  "Spatial: A is above B")
PERCEPT1.add(0x0031, "BELOW",            "NONE",     "",  "Spatial: A is below B")
PERCEPT1.add(0x0032, "LEFT_OF",          "NONE",     "",  "Spatial: A is left of B")
PERCEPT1.add(0x0033, "RIGHT_OF",         "NONE",     "",  "Spatial: A is right of B")
PERCEPT1.add(0x0034, "IN_FRONT_OF",      "NONE",     "",  "Spatial: A is in front of B")
PERCEPT1.add(0x0035, "BEHIND",           "NONE",     "",  "Spatial: A is behind B")
PERCEPT1.add(0x0036, "INSIDE",           "NONE",     "",  "Spatial: A is inside B")
PERCEPT1.add(0x0037, "OUTSIDE",          "NONE",     "",  "Spatial: A is outside B")
PERCEPT1.add(0x0038, "ADJACENT",         "NONE",     "",  "Spatial: A is adjacent to B")
PERCEPT1.add(0x0039, "FAR_FROM",         "NONE",     "",  "Spatial: A is far from B")
PERCEPT1.add(0x003A, "NEAR",             "NONE",     "",  "Spatial: A is near B")
PERCEPT1.add(0x003B, "ON_TOP_OF",        "NONE",     "",  "Spatial: A is resting on B")
PERCEPT1.add(0x003C, "ATTACHED_TO",      "NONE",     "",  "Spatial: A is physically attached to B")

# Visual Properties (0x0050-0x006F)
PERCEPT1.add(0x0050, "COLOR_RGB",        "ARRAY<UINT8,3>",  "",     "Color as (R, G, B)")
PERCEPT1.add(0x0051, "COLOR_NAME",       "UINT8",           "",     "Named color index")
PERCEPT1.add(0x0052, "TEXTURE",          "UINT8",           "",     "Texture class (smooth, rough, etc.)")
PERCEPT1.add(0x0053, "MATERIAL",         "UINT8",           "",     "Material class (metal, wood, etc.)")
PERCEPT1.add(0x0054, "SHAPE",            "UINT8",           "",     "Shape class (sphere, cube, etc.)")
PERCEPT1.add(0x0055, "SIZE_RELATIVE",    "UINT8",           "",     "Relative size (tiny, small, medium, large, huge)")
PERCEPT1.add(0x0056, "BRIGHTNESS",       "FLOAT16",         "lux",  "Measured brightness")
PERCEPT1.add(0x0057, "TRANSPARENCY",     "FLOAT16",         "",     "Transparency 0.0-1.0")

# Sensor Data (0x0070-0x008F)
PERCEPT1.add(0x0070, "LIDAR_SCAN",       "LIST<ARRAY<FLOAT32,3>>", "m",  "Point cloud from LiDAR")
PERCEPT1.add(0x0071, "DEPTH_MAP",        "STRUCT{w,h,data}",       "m",  "Depth image")
PERCEPT1.add(0x0072, "CAMERA_INTRINSICS","STRUCT",                 "",   "Camera calibration matrix")
PERCEPT1.add(0x0073, "CAMERA_EXTRINSICS","STRUCT",                 "",   "Camera pose")
PERCEPT1.add(0x0074, "IMAGE_EMBEDDING",  "ARRAY<FLOAT16,N>",       "",   "Feature embedding vector")
PERCEPT1.add(0x0075, "AUDIO_LEVEL",      "FLOAT16",                "dB", "Ambient audio level")
PERCEPT1.add(0x0076, "TEMPERATURE",      "FLOAT16",                "K",  "Measured temperature")
PERCEPT1.add(0x0077, "HUMIDITY",         "FLOAT16",                "%",  "Relative humidity")
PERCEPT1.add(0x0078, "PRESSURE",         "FLOAT32",                "Pa", "Atmospheric pressure")
PERCEPT1.add(0x0079, "IMU_DATA",         "STRUCT{accel,gyro,mag}", "",   "Inertial measurement unit")


# ── DIAG-1: Diagnostics ──────────────────────────────────────────────

DIAG1 = DomainCodebook(0x05, "DIAG-1", "Diagnostic and system health reporting")

# Power and Energy (0x0000-0x001F)
DIAG1.add(0x0000, "BATTERY_LEVEL",    "FLOAT16",   "%",    "Battery state of charge 0-100%")
DIAG1.add(0x0001, "BATTERY_VOLTAGE",  "FLOAT16",   "V",    "Battery terminal voltage")
DIAG1.add(0x0002, "BATTERY_CURRENT",  "FLOAT16",   "A",    "Battery discharge current")
DIAG1.add(0x0003, "BATTERY_TEMP",     "FLOAT16",   "K",    "Battery temperature")
DIAG1.add(0x0004, "CHARGE_RATE",      "FLOAT16",   "W",    "Current charge rate")
DIAG1.add(0x0005, "TIME_REMAINING",   "FLOAT32",   "s",    "Estimated runtime remaining")
DIAG1.add(0x0006, "POWER_CONSUMPTION","FLOAT16",   "W",    "Current total power draw")
DIAG1.add(0x0007, "ENERGY_CONSUMED",  "FLOAT32",   "J",    "Total energy consumed this session")
DIAG1.add(0x0008, "CHARGING_STATUS",  "UINT8",     "",     "0=discharging, 1=charging, 2=full, 3=fault")
DIAG1.add(0x0009, "POWER_SOURCE",     "UINT8",     "",     "0=battery, 1=wired, 2=solar, 3=fuel_cell")

# Compute and Memory (0x0020-0x003F)
DIAG1.add(0x0020, "CPU_LOAD",         "FLOAT16",   "%",    "CPU utilization 0-100%")
DIAG1.add(0x0021, "GPU_LOAD",         "FLOAT16",   "%",    "GPU utilization 0-100%")
DIAG1.add(0x0022, "MEMORY_USED",      "UINT32",    "KB",   "Memory in use")
DIAG1.add(0x0023, "MEMORY_TOTAL",     "UINT32",    "KB",   "Total available memory")
DIAG1.add(0x0024, "STORAGE_USED",     "UINT32",    "KB",   "Storage in use")
DIAG1.add(0x0025, "STORAGE_TOTAL",    "UINT32",    "KB",   "Total available storage")
DIAG1.add(0x0026, "CPU_TEMP",         "FLOAT16",   "K",    "CPU temperature")
DIAG1.add(0x0027, "GPU_TEMP",         "FLOAT16",   "K",    "GPU temperature")
DIAG1.add(0x0028, "INFERENCE_RATE",   "FLOAT32",   "Hz",   "AI model inference rate")
DIAG1.add(0x0029, "MODEL_ID",         "STRING",    "",     "Active AI model identifier")

# Communication Health (0x0040-0x005F)
DIAG1.add(0x0040, "AILL_SNR",         "FLOAT16",   "dB",   "Current AILL channel SNR")
DIAG1.add(0x0041, "AILL_BER",         "FLOAT32",   "",     "Current AILL bit error rate")
DIAG1.add(0x0042, "AILL_THROUGHPUT",  "FLOAT32",   "bps",  "Current effective data rate")
DIAG1.add(0x0043, "AILL_RETRANSMITS", "UINT16",    "",     "Retransmission count this session")
DIAG1.add(0x0044, "AILL_LATENCY",     "FLOAT16",   "ms",   "Round-trip latency estimate")
DIAG1.add(0x0045, "WIFI_RSSI",        "INT8",      "dBm",  "WiFi signal strength")
DIAG1.add(0x0046, "NETWORK_STATUS",   "UINT8",     "",     "0=disconnected, 1=connected, 2=limited")

# System Status (0x0060-0x007F)
DIAG1.add(0x0060, "UPTIME",           "UINT32",    "s",    "System uptime in seconds")
DIAG1.add(0x0061, "BOOT_COUNT",       "UINT16",    "",     "Number of system boots")
DIAG1.add(0x0062, "ERROR_COUNT",      "UINT16",    "",     "Cumulative error count")
DIAG1.add(0x0063, "LAST_ERROR",       "STRUCT{code,msg,ts}", "",  "Most recent error record")
DIAG1.add(0x0064, "HEALTH_STATUS",    "UINT8",     "",     "0=nominal, 1=degraded, 2=critical, 3=emergency")
DIAG1.add(0x0065, "FIRMWARE_VERSION", "STRING",    "",     "Firmware/software version string")
DIAG1.add(0x0066, "HARDWARE_ID",      "STRING",    "",     "Hardware model identifier")
DIAG1.add(0x0067, "CAPABILITIES_REPORT", "STRUCT", "",     "Full capability self-report")
DIAG1.add(0x0068, "SELF_TEST_RESULT", "STRUCT{pass,details}", "",  "Built-in self-test results")
DIAG1.add(0x0069, "MAINTENANCE_DUE",  "TIMESTAMP", "",     "Next scheduled maintenance time")
DIAG1.add(0x006A, "OPERATING_MODE",   "UINT8",     "",     "0=idle, 1=active, 2=standby, 3=safe_mode, 4=shutdown")
DIAG1.add(0x006B, "ACTUATOR_STATUS",  "LIST<STRUCT{id,ok,temp}>", "", "Per-actuator health")


# ── PLAN-1: Planning ──────────────────────────────────────────────────

PLAN1 = DomainCodebook(0x06, "PLAN-1", "Task planning and goal management")

PLAN1.add(0x0000, "TASK",              "STRUCT{id,type,params}",  "",  "Task definition")
PLAN1.add(0x0001, "TASK_ID",           "UINT32",                  "",  "Unique task identifier")
PLAN1.add(0x0002, "TASK_STATUS",       "UINT8",                   "",  "0=pending, 1=active, 2=complete, 3=failed, 4=cancelled")
PLAN1.add(0x0003, "TASK_PRIORITY",     "UINT8",                   "",  "Task priority 0-7")
PLAN1.add(0x0004, "TASK_DEADLINE",     "TIMESTAMP",               "",  "Task completion deadline")
PLAN1.add(0x0005, "TASK_PROGRESS",     "FLOAT16",                 "%",  "Completion percentage 0-100%")
PLAN1.add(0x0006, "SUBTASK",           "STRUCT{id,parent_id}",    "",  "Subtask with parent reference")
PLAN1.add(0x0007, "TASK_DEPENDENCY",   "STRUCT{task_id,dep_id}",  "",  "Task A depends on task B")
PLAN1.add(0x0008, "GOAL",              "STRUCT{id,condition}",    "",  "Goal as a boolean condition")
PLAN1.add(0x0009, "GOAL_STATUS",       "UINT8",                   "",  "0=unachieved, 1=achieved, 2=impossible")
PLAN1.add(0x000A, "PLAN",              "LIST<TASK>",              "",  "Ordered plan (sequence of tasks)")
PLAN1.add(0x000B, "PLAN_COST",         "FLOAT32",                 "",  "Estimated total plan cost")
PLAN1.add(0x000C, "PLAN_DURATION",     "FLOAT32",                 "s",  "Estimated total plan duration")
PLAN1.add(0x000D, "ALLOCATE_TASK",     "STRUCT{task_id,agent_id}","",  "Assign task to agent")
PLAN1.add(0x000E, "RELEASE_TASK",      "UINT32",                  "",  "Unassign/release a task")
PLAN1.add(0x000F, "REPLAN_REQUEST",    "STRUCT{reason}",          "",  "Request plan regeneration")
PLAN1.add(0x0010, "RESOURCE",          "STRUCT{type,amount}",     "",  "Resource requirement or availability")
PLAN1.add(0x0011, "RESOURCE_CONFLICT", "STRUCT{res,agents}",      "",  "Resource contention report")
PLAN1.add(0x0012, "AUCTION_BID",       "STRUCT{task_id,cost}",    "",  "Bid on a task in task auction")
PLAN1.add(0x0013, "AUCTION_AWARD",     "STRUCT{task_id,agent_id}","",  "Award task to winning bidder")


# ── Domain codebook registry ──────────────────────────────────────────

DOMAIN_REGISTRY: dict[int, DomainCodebook] = {
    0x01: NAV1,
    0x02: PERCEPT1,
    0x05: DIAG1,
    0x06: PLAN1,
}

def get_domain_codebook(registry_id: int) -> Optional[DomainCodebook]:
    """Look up a domain codebook by its registry ID."""
    return DOMAIN_REGISTRY.get(registry_id)
