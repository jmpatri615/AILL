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

# Mapping and Localization (0x00C0-0x00EF)
NAV1.add(0x00C0, "MAP_ORIGIN",        "POSITION_3D",         "m",     "Origin of the local map frame")
NAV1.add(0x00C1, "MAP_RESOLUTION",    "FLOAT32",             "m",     "Grid cell size")
NAV1.add(0x00C2, "MAP_DIMENSIONS",    "ARRAY<UINT16,3>",     "",      "Grid dimensions (nx, ny, nz)")
NAV1.add(0x00C3, "MAP_UPDATE",        "STRUCT{region,cells}", "",     "Incremental map patch")
NAV1.add(0x00C4, "MAP_VERSION",       "UINT32",              "",      "Map revision counter")
NAV1.add(0x00C5, "LOCALIZATION_CONF", "FLOAT16",             "",      "Localization confidence 0.0-1.0")
NAV1.add(0x00C6, "POSE_COVARIANCE",   "ARRAY<FLOAT32,36>",   "",      "6x6 pose uncertainty covariance matrix")
NAV1.add(0x00C7, "LANDMARK",          "STRUCT{id,pos,desc}",  "",     "Recognized environmental landmark")
NAV1.add(0x00C8, "LANDMARK_LIST",     "LIST<LANDMARK>",      "",      "Collection of observed landmarks")
NAV1.add(0x00C9, "LOOP_CLOSURE",      "STRUCT{from,to,tf}",  "",      "Loop closure detection with transform")
NAV1.add(0x00CA, "RELOCALIZE",        "NONE",                "",      "Trigger relocalization procedure")
NAV1.add(0x00CB, "LOCALIZATION_MODE", "UINT8",               "",      "0=SLAM, 1=known_map, 2=GPS_primary, 3=visual_odom, 4=dead_reckoning")
NAV1.add(0x00CC, "ODOMETRY_DRIFT",    "FLOAT32",             "m",     "Estimated cumulative drift from odometry")
NAV1.add(0x00CD, "VISUAL_FEATURES",   "UINT16",              "",      "Number of tracked visual features")
NAV1.add(0x00CE, "MAP_MERGE_REQ",     "STRUCT{agent,hash}",  "",      "Request to merge map from another agent")
NAV1.add(0x00CF, "MAP_MERGE_ACK",     "STRUCT{agent,tf}",    "",      "Acknowledge merge with alignment transform")

# Coordinate Transforms (0x00F0-0x010F)
NAV1.add(0x00F0, "TRANSFORM_3D",      "STRUCT{rot,trans}",   "",      "Rigid body transform (rotation matrix + translation)")
NAV1.add(0x00F1, "TRANSFORM_QUAT",    "STRUCT{quat,trans}",  "",      "Quaternion-based rigid body transform")
NAV1.add(0x00F2, "FRAME_PARENT",      "UINT8",               "",      "Parent frame ID in transform tree")
NAV1.add(0x00F3, "FRAME_CHILD",       "UINT8",               "",      "Child frame ID in transform tree")
NAV1.add(0x00F4, "TF_TREE",           "LIST<STRUCT{parent,child,tf}>", "", "Complete transform tree snapshot")
NAV1.add(0x00F5, "TF_LOOKUP",         "STRUCT{from,to}",     "",      "Request transform between two frames")
NAV1.add(0x00F6, "TF_RESULT",         "TRANSFORM_QUAT",     "",      "Result of a transform lookup")
NAV1.add(0x00F7, "DATUM_WGS84",       "STRUCT{lat,lon,alt}", "",      "WGS84 datum point for local frame")
NAV1.add(0x00F8, "UTM_ZONE",          "STRUCT{zone,band}",   "",      "UTM zone number and latitude band")
NAV1.add(0x00F9, "MAGNETIC_DECLINATION","FLOAT16",            "rad",   "Local magnetic declination")

# Multi-Agent Spatial Coordination (0x0110-0x013F)
NAV1.add(0x0110, "SWARM_CENTER",      "POSITION_3D",         "m",     "Centroid of all agents in swarm")
NAV1.add(0x0111, "SWARM_RADIUS",      "FLOAT32",             "m",     "Bounding radius of swarm")
NAV1.add(0x0112, "AGENT_POSITIONS",   "LIST<STRUCT{uuid,pos}>", "",   "Positions of all known agents")
NAV1.add(0x0113, "SEPARATION_DIST",   "FLOAT32",             "m",     "Minimum inter-agent separation distance")
NAV1.add(0x0114, "COLLISION_ALERT",   "STRUCT{agent,ttc}",   "",      "Inter-agent collision alert with time-to-collision")
NAV1.add(0x0115, "ZONE_CLAIM",        "STRUCT{agent,polygon}","",     "Agent claims exclusive operating zone")
NAV1.add(0x0116, "ZONE_RELEASE",      "STRUCT{agent,zone_id}","",     "Agent releases zone claim")
NAV1.add(0x0117, "RENDEZVOUS_POINT",  "STRUCT{pos,time}",    "",      "Designated meeting point with time")
NAV1.add(0x0118, "CONVOY_JOIN",       "STRUCT{leader,pos,slot}","",   "Join a convoy behind specified leader")
NAV1.add(0x0119, "CONVOY_LEAVE",      "NONE",                "",      "Depart from current convoy")
NAV1.add(0x011A, "COVERAGE_MAP",      "STRUCT{grid,visited}","",      "Coverage completion map for area search")
NAV1.add(0x011B, "ASSIGN_SECTOR",     "STRUCT{agent,polygon}","",     "Assign search sector to agent")
NAV1.add(0x011C, "SECTOR_COMPLETE",   "STRUCT{agent,sector_id}","",   "Report sector search complete")
NAV1.add(0x011D, "RELATIVE_BEARING",  "STRUCT{agent,bearing,range}","","Bearing and range to another agent")
NAV1.add(0x011E, "FORMATION_OFFSET",  "STRUCT{slot,offset}",  "",     "Position offset within formation for assigned slot")
NAV1.add(0x011F, "TRAFFIC_DECONFLICT","STRUCT{agent,corridor}","",    "Corridor assignment for traffic deconfliction")

# Advanced Path Planning (0x0140-0x015F)
NAV1.add(0x0140, "SPLINE_PATH",       "STRUCT{ctrl_pts,order}","",    "Spline-based smooth path (control points + order)")
NAV1.add(0x0141, "DUBINS_PATH",       "STRUCT{start,end,radius}","",  "Dubins path for non-holonomic vehicles")
NAV1.add(0x0142, "VELOCITY_PROFILE",  "LIST<STRUCT{dist,vel}>","",    "Speed profile along path")
NAV1.add(0x0143, "ALTITUDE_PROFILE",  "LIST<STRUCT{dist,alt}>","",    "Altitude profile along path")
NAV1.add(0x0144, "NO_FLY_ZONE",       "STRUCT{polygon,floor,ceiling}","","3D restricted airspace volume")
NAV1.add(0x0145, "DYNAMIC_OBSTACLE",  "STRUCT{id,pos,vel,pred_path}","","Obstacle with predicted trajectory")
NAV1.add(0x0146, "REPLAN_TRIGGER",    "UINT8",               "",      "0=obstacle, 1=path_blocked, 2=priority_change, 3=battery_low, 4=weather")
NAV1.add(0x0147, "SEARCH_PATTERN",    "UINT8",               "",      "0=lawnmower, 1=spiral, 2=sector, 3=random_walk, 4=levy_flight")
NAV1.add(0x0148, "LOITER",            "STRUCT{center,radius,alt,duration}","","Loiter (circle) at location for duration")
NAV1.add(0x0149, "LANDING_ZONE",      "STRUCT{pos,heading,slope,clear}","","Designated landing area with surface info")
NAV1.add(0x014A, "TAKEOFF",           "STRUCT{alt}",         "",      "Takeoff to specified altitude")
NAV1.add(0x014B, "LAND",              "STRUCT{zone_id}",     "",      "Land at designated landing zone")


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

# Scene Understanding (0x0090-0x00AF)
PERCEPT1.add(0x0090, "SCENE_GRAPH",      "LIST<STRUCT{subj,rel,obj}>", "", "Scene graph: subject-relation-object triples")
PERCEPT1.add(0x0091, "ROOM_TYPE",        "UINT8",           "",     "0=unknown, 1=corridor, 2=room, 3=outdoor, 4=stairwell, 5=elevator, 6=garage, 7=warehouse")
PERCEPT1.add(0x0092, "FLOOR_LEVEL",      "INT8",            "",     "Building floor number (-N for basement)")
PERCEPT1.add(0x0093, "SURFACE_NORMAL",   "ARRAY<FLOAT32,3>","",     "Dominant surface normal vector")
PERCEPT1.add(0x0094, "PLANE_SEGMENT",    "STRUCT{normal,d,bounds}", "", "Detected planar surface segment")
PERCEPT1.add(0x0095, "PLANE_LIST",       "LIST<PLANE_SEGMENT>",     "", "All detected planar surfaces")
PERCEPT1.add(0x0096, "SEMANTIC_LABEL",   "STRUCT{region,class,conf}","", "Semantic segmentation label for a region")
PERCEPT1.add(0x0097, "SCENE_COMPLEXITY", "FLOAT16",         "",     "Scene complexity score 0.0-1.0")
PERCEPT1.add(0x0098, "CLUTTER_DENSITY",  "FLOAT16",         "",     "Object density per cubic meter")
PERCEPT1.add(0x0099, "TRAVERSABILITY",   "FLOAT16",         "",     "Surface traversability score 0.0-1.0")
PERCEPT1.add(0x009A, "DOOR_STATE",       "STRUCT{pos,state}","",    "Door: 0=closed, 1=open, 2=ajar, 3=locked")
PERCEPT1.add(0x009B, "OPENING",          "STRUCT{pos,width,height}","m", "Passable opening (doorway, gap)")
PERCEPT1.add(0x009C, "STAIRS",           "STRUCT{pos,direction,count}","","Detected staircase with step count")
PERCEPT1.add(0x009D, "RAMP",             "STRUCT{pos,slope,width}","",   "Detected ramp or incline")
PERCEPT1.add(0x009E, "SIGN_TEXT",        "STRUCT{pos,text,lang}","",     "Detected and OCR'd sign text")
PERCEPT1.add(0x009F, "QR_CODE",          "STRUCT{pos,data}",  "",       "Detected QR code with decoded data")

# Event Detection (0x00B0-0x00CF)
PERCEPT1.add(0x00B0, "MOTION_DETECTED",  "STRUCT{region,magnitude}","", "Motion detected in field of view")
PERCEPT1.add(0x00B1, "OBJECT_APPEARED",  "STRUCT{id,class,pos}","",     "New object entered field of view")
PERCEPT1.add(0x00B2, "OBJECT_DISAPPEARED","STRUCT{id,last_pos}","",     "Tracked object left field of view")
PERCEPT1.add(0x00B3, "OBJECT_STOPPED",   "STRUCT{id,pos,duration}","",  "Moving object has stopped")
PERCEPT1.add(0x00B4, "OBJECT_PICKED_UP", "STRUCT{id,agent}",  "",       "Object was picked up")
PERCEPT1.add(0x00B5, "OBJECT_PLACED",    "STRUCT{id,surface}","",       "Object was placed on surface")
PERCEPT1.add(0x00B6, "GESTURE_DETECTED", "STRUCT{type,agent,conf}","",  "Human gesture recognized")
PERCEPT1.add(0x00B7, "GESTURE_TYPE",     "UINT8",             "",       "0=wave, 1=point, 2=stop, 3=come, 4=thumbs_up, 5=thumbs_down, 6=nod, 7=shake_head")
PERCEPT1.add(0x00B8, "FACE_DETECTED",    "STRUCT{bbox,landmarks,id}","","Detected human face with optional ID")
PERCEPT1.add(0x00B9, "FACE_EXPRESSION",  "UINT8",             "",       "0=neutral, 1=happy, 2=sad, 3=angry, 4=surprised, 5=fearful, 6=disgusted")
PERCEPT1.add(0x00BA, "PERSON_POSE",      "LIST<KEYPOINT>",    "",       "Full body skeleton keypoints")
PERCEPT1.add(0x00BB, "ACTIVITY_CLASS",   "UINT8",             "",       "0=standing, 1=walking, 2=running, 3=sitting, 4=lying, 5=falling, 6=working, 7=waving")
PERCEPT1.add(0x00BC, "CROWD_DENSITY",    "FLOAT16",           "1/m^2", "People per square meter in region")
PERCEPT1.add(0x00BD, "ANOMALY_DETECTED", "STRUCT{type,pos,conf}","",    "Anomalous event or state detected")
PERCEPT1.add(0x00BE, "LIGHT_CHANGE",     "STRUCT{before,after}","lux",  "Significant illumination change")
PERCEPT1.add(0x00BF, "OCCLUSION",        "STRUCT{obj_id,pct}","",       "Object partially occluded (percent hidden)")

# Audio Perception (0x00D0-0x00DF)
PERCEPT1.add(0x00D0, "SOUND_EVENT",      "STRUCT{class,dir,level}","",  "Detected sound event")
PERCEPT1.add(0x00D1, "SOUND_CLASS",      "UINT8",             "",       "0=speech, 1=alarm, 2=impact, 3=engine, 4=music, 5=animal, 6=footsteps, 7=glass_break")
PERCEPT1.add(0x00D2, "SOUND_DIRECTION",  "ARRAY<FLOAT32,2>",  "rad",   "Azimuth and elevation of sound source")
PERCEPT1.add(0x00D3, "SOUND_LEVEL",      "FLOAT16",           "dB_SPL","Sound pressure level")
PERCEPT1.add(0x00D4, "SPEECH_DETECTED",  "STRUCT{dir,lang,dur}","",    "Speech activity detected")
PERCEPT1.add(0x00D5, "SPEECH_TEXT",       "STRUCT{text,lang,conf}","",  "Speech-to-text transcription result")
PERCEPT1.add(0x00D6, "SPEAKER_ID",       "STRUCT{uuid,conf}",  "",     "Identified speaker (voice print match)")
PERCEPT1.add(0x00D7, "AMBIENT_NOISE",    "FLOAT16",           "dB_SPL","Background noise floor level")
PERCEPT1.add(0x00D8, "ALARM_ACTIVE",     "STRUCT{type,pos,level}","",  "Active alarm detected (fire, security, etc.)")

# Tactile and Force Sensing (0x00E0-0x00EF)
PERCEPT1.add(0x00E0, "CONTACT_DETECTED", "STRUCT{pos,normal,force}","", "Physical contact detected")
PERCEPT1.add(0x00E1, "CONTACT_FORCE",    "ARRAY<FLOAT32,3>",  "N",     "Contact force vector")
PERCEPT1.add(0x00E2, "CONTACT_AREA",     "FLOAT32",           "m^2",   "Estimated contact patch area")
PERCEPT1.add(0x00E3, "SURFACE_FRICTION", "FLOAT16",           "",      "Estimated surface friction coefficient")
PERCEPT1.add(0x00E4, "VIBRATION",        "STRUCT{freq,amplitude}","",   "Detected vibration (frequency and amplitude)")
PERCEPT1.add(0x00E5, "LOAD_CELL",        "STRUCT{id,force}",   "N",    "Load cell reading")
PERCEPT1.add(0x00E6, "TORQUE_SENSOR",    "STRUCT{id,torque}",  "Nm",   "Torque sensor reading")
PERCEPT1.add(0x00E7, "SLIP_DETECTED",    "STRUCT{gripper,obj}","",     "Object slippage detected at gripper")
PERCEPT1.add(0x00E8, "PROXIMITY_SENSOR", "STRUCT{id,range}",   "m",    "Proximity sensor reading")

# Environmental Sensing (0x00F0-0x00FF)
PERCEPT1.add(0x00F0, "GAS_CONCENTRATION","STRUCT{gas,ppm}",    "ppm",  "Gas concentration (CO, CO2, CH4, etc.)")
PERCEPT1.add(0x00F1, "RADIATION_LEVEL",  "FLOAT32",            "uSv/h","Radiation dose rate")
PERCEPT1.add(0x00F2, "WIND_SPEED",       "FLOAT32",            "m/s",  "Measured wind speed")
PERCEPT1.add(0x00F3, "WIND_DIRECTION",   "FLOAT32",            "rad",  "Wind direction (from)")
PERCEPT1.add(0x00F4, "RAIN_RATE",        "FLOAT16",            "mm/h", "Precipitation rate")
PERCEPT1.add(0x00F5, "UV_INDEX",         "FLOAT16",            "",     "Ultraviolet radiation index")
PERCEPT1.add(0x00F6, "AIR_QUALITY_INDEX","UINT16",             "",     "Air quality index (0-500)")
PERCEPT1.add(0x00F7, "DUST_DENSITY",     "FLOAT32",            "ug/m^3","Particulate matter concentration")
PERCEPT1.add(0x00F8, "MAGNETIC_FIELD",   "ARRAY<FLOAT32,3>",   "uT",   "Local magnetic field vector")
PERCEPT1.add(0x00F9, "LIGHT_SPECTRUM",   "STRUCT{wavelengths,intensities}","","Spectral light measurement")


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

# Thermal Management (0x0080-0x0097)
DIAG1.add(0x0080, "THERMAL_MAP",      "LIST<STRUCT{zone,temp}>", "K", "Temperature readings by zone")
DIAG1.add(0x0081, "HOT_SPOT",         "STRUCT{zone,temp,trend}", "",  "Thermal hot spot alert")
DIAG1.add(0x0082, "COOLING_STATUS",   "UINT8",     "",     "0=passive, 1=fan_low, 2=fan_high, 3=liquid, 4=emergency_shutdown")
DIAG1.add(0x0083, "THERMAL_THROTTLE", "BOOL",      "",     "True if performance is thermally throttled")
DIAG1.add(0x0084, "AMBIENT_TEMP",     "FLOAT16",   "K",    "External ambient temperature")
DIAG1.add(0x0085, "INTERNAL_TEMP",    "FLOAT16",   "K",    "Internal chassis temperature")
DIAG1.add(0x0086, "MOTOR_TEMP",       "STRUCT{id,temp}",   "K",    "Per-motor temperature reading")
DIAG1.add(0x0087, "HEATER_STATUS",    "STRUCT{id,on,power}","",    "Heater element status")

# Actuator Detail (0x0098-0x00AF)
DIAG1.add(0x0098, "MOTOR_CURRENT",    "STRUCT{id,amps}",    "A",    "Per-motor current draw")
DIAG1.add(0x0099, "MOTOR_RPM",        "STRUCT{id,rpm}",     "rpm",  "Per-motor rotational speed")
DIAG1.add(0x009A, "MOTOR_POSITION",   "STRUCT{id,angle}",   "rad",  "Per-motor shaft position")
DIAG1.add(0x009B, "MOTOR_FAULT",      "STRUCT{id,code}",    "",     "Motor fault: 0=ok, 1=overcurrent, 2=overheat, 3=stall, 4=encoder_fail, 5=comm_fail")
DIAG1.add(0x009C, "SERVO_POSITION",   "STRUCT{id,actual,target}","rad","Servo actual vs target position")
DIAG1.add(0x009D, "SERVO_LOAD",       "STRUCT{id,load_pct}", "%",   "Servo load as percent of max")
DIAG1.add(0x009E, "HYDRAULIC_PRESSURE","STRUCT{id,pressure}","Pa",   "Hydraulic system pressure")
DIAG1.add(0x009F, "PNEUMATIC_PRESSURE","STRUCT{id,pressure}","Pa",   "Pneumatic system pressure")
DIAG1.add(0x00A0, "BRAKE_STATUS",     "STRUCT{id,engaged}", "",     "Brake engagement status")
DIAG1.add(0x00A1, "CLUTCH_STATUS",    "STRUCT{id,engaged}", "",     "Clutch engagement status")
DIAG1.add(0x00A2, "JOINT_TORQUE",     "STRUCT{id,torque}",  "Nm",   "Measured joint torque")
DIAG1.add(0x00A3, "JOINT_BACKLASH",   "STRUCT{id,angle}",   "rad",  "Measured joint backlash")
DIAG1.add(0x00A4, "WEAR_INDICATOR",   "STRUCT{component,pct}","%",  "Component wear level percentage")

# Software and AI Diagnostics (0x00B0-0x00CF)
DIAG1.add(0x00B0, "PROCESS_LIST",     "LIST<STRUCT{pid,name,cpu,mem}>","","Running processes")
DIAG1.add(0x00B1, "THREAD_COUNT",     "UINT16",    "",     "Active thread count")
DIAG1.add(0x00B2, "QUEUE_DEPTH",      "STRUCT{name,depth}","",     "Message queue occupancy")
DIAG1.add(0x00B3, "LATENCY_HIST",     "LIST<STRUCT{bucket_ms,count}>","","Latency histogram for processing pipeline")
DIAG1.add(0x00B4, "MODEL_CONFIDENCE", "FLOAT16",   "",     "Current AI model output confidence")
DIAG1.add(0x00B5, "MODEL_LATENCY",    "FLOAT16",   "ms",   "AI model inference latency")
DIAG1.add(0x00B6, "PERCEPTION_FPS",   "FLOAT16",   "Hz",   "Perception pipeline frame rate")
DIAG1.add(0x00B7, "PLANNING_CYCLE",   "FLOAT16",   "ms",   "Planning loop cycle time")
DIAG1.add(0x00B8, "CONTROL_CYCLE",    "FLOAT16",   "ms",   "Control loop cycle time")
DIAG1.add(0x00B9, "WATCHDOG_STATUS",  "UINT8",     "",     "0=ok, 1=warning, 2=tripped")
DIAG1.add(0x00BA, "LOG_ENTRY",        "STRUCT{level,source,msg}","","Diagnostic log entry")
DIAG1.add(0x00BB, "LOG_LEVEL",        "UINT8",     "",     "0=trace, 1=debug, 2=info, 3=warn, 4=error, 5=fatal")
DIAG1.add(0x00BC, "CRASH_REPORT",     "STRUCT{time,module,backtrace}","","Software crash report")
DIAG1.add(0x00BD, "PARAM_VALUE",      "STRUCT{name,value}", "",     "Runtime configuration parameter")
DIAG1.add(0x00BE, "PARAM_SET",        "STRUCT{name,value}", "",     "Request to change runtime parameter")
DIAG1.add(0x00BF, "PARAM_ACK",        "STRUCT{name,ok}",    "",     "Acknowledge parameter change")

# Lifecycle and Fleet (0x00D0-0x00DF)
DIAG1.add(0x00D0, "FLIGHT_HOURS",     "FLOAT32",   "h",    "Total operational flight/run hours")
DIAG1.add(0x00D1, "CYCLE_COUNT",      "UINT32",    "",     "Total motor/actuator power cycles")
DIAG1.add(0x00D2, "LAST_CALIBRATION", "TIMESTAMP", "",     "Timestamp of last sensor calibration")
DIAG1.add(0x00D3, "CALIBRATION_DUE",  "TIMESTAMP", "",     "Next required calibration")
DIAG1.add(0x00D4, "REPLACEMENT_PART", "STRUCT{part_id,urgency}","", "Part approaching end of life")
DIAG1.add(0x00D5, "FLEET_ID",         "STRING",    "",     "Fleet assignment identifier")
DIAG1.add(0x00D6, "DEPLOYMENT_ID",    "STRING",    "",     "Current deployment/mission identifier")
DIAG1.add(0x00D7, "OTA_STATUS",       "UINT8",     "",     "Over-the-air update: 0=none, 1=available, 2=downloading, 3=ready, 4=applying, 5=failed")
DIAG1.add(0x00D8, "OTA_VERSION",      "STRING",    "",     "Available OTA update version string")
DIAG1.add(0x00D9, "STORAGE_HEALTH",   "UINT8",     "%",    "Storage medium health (SSD wear level)")


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

# Negotiation and Commitment (0x0020-0x003F)
PLAN1.add(0x0020, "OFFER",            "STRUCT{id,terms}",    "",  "Offer terms for negotiation")
PLAN1.add(0x0021, "COUNTER_OFFER",    "STRUCT{orig_id,new_terms}","","Counter-proposal to an offer")
PLAN1.add(0x0022, "ACCEPT_OFFER",     "STRUCT{offer_id}",    "",  "Accept a specific offer")
PLAN1.add(0x0023, "REJECT_OFFER",     "STRUCT{offer_id,reason}","","Reject an offer with reason")
PLAN1.add(0x0024, "COMMITMENT",       "STRUCT{task_id,agent,deadline}","","Binding commitment to complete task")
PLAN1.add(0x0025, "COMMITMENT_CANCEL","STRUCT{commit_id,reason}","","Cancel a commitment (with penalty if applicable)")
PLAN1.add(0x0026, "PROMISE_DELIVERY", "STRUCT{what,when,where}","","Promise to deliver result at time and place")
PLAN1.add(0x0027, "CAPABILITY_QUERY", "STRUCT{task_type}",   "",  "Ask what agents can perform task type")
PLAN1.add(0x0028, "CAPABILITY_RESPONSE","STRUCT{agent,can,cost}","","Response: can perform, estimated cost")
PLAN1.add(0x0029, "VOTE_REQUEST",     "STRUCT{proposal_id,options}","","Request vote on proposal")
PLAN1.add(0x002A, "VOTE_CAST",        "STRUCT{proposal_id,choice}","","Cast vote on proposal")
PLAN1.add(0x002B, "VOTE_RESULT",      "STRUCT{proposal_id,outcome}","","Announce voting result")
PLAN1.add(0x002C, "CONSENSUS_REACHED","STRUCT{topic,value}", "",  "Group consensus reached on topic")
PLAN1.add(0x002D, "ARBITRATION_REQ",  "STRUCT{dispute,parties}","","Request third-party arbitration")

# Temporal Planning (0x0040-0x005F)
PLAN1.add(0x0040, "TIME_WINDOW",      "STRUCT{earliest,latest}","","Acceptable time window for action")
PLAN1.add(0x0041, "SCHEDULE",         "LIST<STRUCT{task,start,end}>","","Scheduled sequence of tasks with times")
PLAN1.add(0x0042, "SCHEDULE_CONFLICT","STRUCT{task_a,task_b,overlap}","","Two tasks conflict in time")
PLAN1.add(0x0043, "MILESTONE",        "STRUCT{id,condition,deadline}","","Named checkpoint in plan")
PLAN1.add(0x0044, "MILESTONE_REACHED","STRUCT{id,actual_time}","",  "Report milestone completion")
PLAN1.add(0x0045, "CRITICAL_PATH",    "LIST<TASK_ID>",       "",  "Tasks on the critical path (zero slack)")
PLAN1.add(0x0046, "SLACK_TIME",       "STRUCT{task_id,slack}","s", "Available slack time for task")
PLAN1.add(0x0047, "TEMPORAL_CONSTRAINT","STRUCT{before,after,gap}","","Task A must complete >= gap before task B")
PLAN1.add(0x0048, "RECURRING_TASK",   "STRUCT{task,interval,count}","","Repeating task definition")
PLAN1.add(0x0049, "PREEMPT_TASK",     "STRUCT{running_id,new_id}","","Interrupt current task for higher priority")
PLAN1.add(0x004A, "RESUME_TASK",      "STRUCT{task_id}",     "",  "Resume a previously preempted task")

# Behavior and Intent (0x0060-0x007F)
PLAN1.add(0x0060, "INTENT",           "STRUCT{action,target,purpose}","","Declared intent (transparent planning)")
PLAN1.add(0x0061, "INTENT_CONFLICT",  "STRUCT{agent_a,agent_b,type}","","Detected intent conflict between agents")
PLAN1.add(0x0062, "YIELD",            "STRUCT{to_agent,context}","",   "Yield priority to another agent")
PLAN1.add(0x0063, "REQUEST_YIELD",    "STRUCT{from_agent,reason}","",  "Ask another agent to yield")
PLAN1.add(0x0064, "BEHAVIOR_MODE",    "UINT8",               "",  "0=normal, 1=cautious, 2=aggressive, 3=energy_saving, 4=exploration, 5=return_to_base")
PLAN1.add(0x0065, "RISK_TOLERANCE",   "FLOAT16",             "",  "Risk acceptance level 0.0 (risk-averse) to 1.0 (risk-seeking)")
PLAN1.add(0x0066, "EXPLANATION",      "STRUCT{decision,factors}","","Explain reasoning behind a decision")
PLAN1.add(0x0067, "UNCERTAINTY_MAP",  "STRUCT{region,entropy}","",  "Spatial uncertainty for exploration planning")
PLAN1.add(0x0068, "INFORMATION_GAIN", "STRUCT{action,expected_bits}","","Expected information gain from action")
PLAN1.add(0x0069, "UTILITY",          "STRUCT{outcome,value}","",  "Utility value for an outcome")
PLAN1.add(0x006A, "CONSTRAINT",       "STRUCT{type,params}", "",  "Planning constraint (spatial, temporal, resource)")
PLAN1.add(0x006B, "CONSTRAINT_VIOLATED","STRUCT{constraint_id,severity}","","Report constraint violation")

# Workflow and State Machine (0x0080-0x0097)
PLAN1.add(0x0080, "STATE_MACHINE",    "STRUCT{id,states,transitions}","","State machine definition")
PLAN1.add(0x0081, "CURRENT_STATE",    "STRUCT{machine_id,state}","",   "Current state in a state machine")
PLAN1.add(0x0082, "STATE_TRANSITION", "STRUCT{from,to,trigger}","",    "State transition event")
PLAN1.add(0x0083, "WORKFLOW",         "STRUCT{id,steps}",     "",      "Multi-step workflow definition")
PLAN1.add(0x0084, "WORKFLOW_STEP",    "STRUCT{id,action,next}","",     "Single step in a workflow")
PLAN1.add(0x0085, "WORKFLOW_STATUS",  "STRUCT{wf_id,step_id,pct}","",  "Current workflow progress")
PLAN1.add(0x0086, "CONDITIONAL_STEP", "STRUCT{condition,if_true,if_false}","","Branching step in workflow")
PLAN1.add(0x0087, "PARALLEL_STEPS",   "LIST<STRUCT{step_id,agent}>","","Steps to execute in parallel")
PLAN1.add(0x0088, "SYNC_BARRIER",     "STRUCT{barrier_id,agents}","",  "All agents must reach barrier before proceeding")
PLAN1.add(0x0089, "BARRIER_REACHED",  "STRUCT{barrier_id,agent}","",   "Agent arrived at sync barrier")


# ═══════════════════════════════════════════════════════════════════════
# NEW DOMAIN CODEBOOKS
# ═══════════════════════════════════════════════════════════════════════

# ── MANIP-1: Robotic Manipulation ─────────────────────────────────────

MANIP1 = DomainCodebook(0x03, "MANIP-1", "Robotic manipulation and grasping")

# Gripper and End Effector (0x0000-0x001F)
MANIP1.add(0x0000, "GRIPPER_STATE",    "UINT8",               "",     "0=open, 1=closing, 2=closed, 3=opening, 4=holding, 5=error")
MANIP1.add(0x0001, "GRIPPER_WIDTH",    "FLOAT32",             "m",    "Current gripper aperture width")
MANIP1.add(0x0002, "GRIPPER_FORCE",    "FLOAT32",             "N",    "Current gripper force")
MANIP1.add(0x0003, "GRIPPER_SET_WIDTH","FLOAT32",             "m",    "Commanded gripper width")
MANIP1.add(0x0004, "GRIPPER_SET_FORCE","FLOAT32",             "N",    "Commanded gripper force limit")
MANIP1.add(0x0005, "TOOL_TYPE",        "UINT8",               "",     "0=parallel_jaw, 1=vacuum, 2=magnetic, 3=soft, 4=finger_3, 5=hook, 6=scoop, 7=custom")
MANIP1.add(0x0006, "TOOL_CENTER_POINT","ARRAY<FLOAT32,3>",    "m",    "Tool center point (TCP) in end-effector frame")
MANIP1.add(0x0007, "TOOL_CHANGE_REQ",  "UINT8",               "",     "Request tool change to specified tool type")
MANIP1.add(0x0008, "TOOL_CHANGE_ACK",  "UINT8",               "",     "Tool change completed")
MANIP1.add(0x0009, "SUCTION_PRESSURE", "FLOAT32",             "Pa",   "Vacuum gripper suction pressure")
MANIP1.add(0x000A, "SUCTION_STATUS",   "UINT8",               "",     "0=off, 1=engaged, 2=leak, 3=lost_seal")
MANIP1.add(0x000B, "FINGER_POSITIONS", "LIST<FLOAT32>",       "rad",  "Per-finger joint positions")
MANIP1.add(0x000C, "FINGER_FORCES",    "LIST<FLOAT32>",       "N",    "Per-finger contact forces")
MANIP1.add(0x000D, "TACTILE_ARRAY",    "STRUCT{rows,cols,data}","Pa", "Tactile sensor pad readings")

# Joint Space (0x0020-0x003F)
MANIP1.add(0x0020, "JOINT_POSITIONS",  "LIST<FLOAT32>",       "rad",  "All joint angles")
MANIP1.add(0x0021, "JOINT_VELOCITIES", "LIST<FLOAT32>",       "rad/s","All joint angular velocities")
MANIP1.add(0x0022, "JOINT_TORQUES",    "LIST<FLOAT32>",       "Nm",   "All joint torques")
MANIP1.add(0x0023, "JOINT_LIMITS",     "LIST<STRUCT{min,max}>","rad", "Joint angle limits")
MANIP1.add(0x0024, "JOINT_TARGET",     "LIST<FLOAT32>",       "rad",  "Commanded joint positions")
MANIP1.add(0x0025, "JOINT_TRAJECTORY", "LIST<STRUCT{time,positions}>","","Time-parameterized joint trajectory")
MANIP1.add(0x0026, "JOINT_IMPEDANCE",  "STRUCT{stiffness,damping}","", "Joint impedance parameters")
MANIP1.add(0x0027, "DOF_COUNT",        "UINT8",               "",     "Number of degrees of freedom")
MANIP1.add(0x0028, "DH_PARAMETERS",    "LIST<STRUCT{a,alpha,d,theta}>","","Denavit-Hartenberg kinematic parameters")
MANIP1.add(0x0029, "SINGULARITY_PROXIMITY","FLOAT16",         "",     "Distance to kinematic singularity 0.0-1.0")

# Cartesian Space (0x0040-0x005F)
MANIP1.add(0x0040, "EE_POSE",          "STRUCT{pos,orient}",  "",     "End-effector pose in base frame")
MANIP1.add(0x0041, "EE_VELOCITY",      "STRUCT{linear,angular}","",   "End-effector twist (linear + angular velocity)")
MANIP1.add(0x0042, "EE_WRENCH",        "STRUCT{force,torque}","",     "End-effector wrench (force + torque)")
MANIP1.add(0x0043, "CARTESIAN_TARGET", "STRUCT{pos,orient}",  "",     "Commanded end-effector pose")
MANIP1.add(0x0044, "CARTESIAN_PATH",   "LIST<STRUCT{pos,orient,time}>","","Cartesian trajectory waypoints")
MANIP1.add(0x0045, "WORKSPACE_LIMIT",  "STRUCT{min,max}",     "m",    "Reachable workspace bounding box")
MANIP1.add(0x0046, "COMPLIANCE_FRAME", "STRUCT{pos,orient}",  "",     "Reference frame for compliance control")
MANIP1.add(0x0047, "IMPEDANCE_PARAMS", "STRUCT{mass,damping,stiffness}","","Cartesian impedance parameters")
MANIP1.add(0x0048, "FORCE_THRESHOLD",  "STRUCT{force,torque}","",     "Force/torque thresholds for safety stop")

# Grasp Planning (0x0060-0x007F)
MANIP1.add(0x0060, "GRASP_POSE",       "STRUCT{pos,orient,width}","", "Planned grasp pose")
MANIP1.add(0x0061, "GRASP_QUALITY",    "FLOAT16",             "",     "Grasp quality metric 0.0-1.0")
MANIP1.add(0x0062, "GRASP_TYPE",       "UINT8",               "",     "0=power, 1=precision, 2=pinch, 3=wrap, 4=hook, 5=lateral, 6=spherical")
MANIP1.add(0x0063, "GRASP_LIST",       "LIST<STRUCT{pose,quality,type}>","","Ranked list of candidate grasps")
MANIP1.add(0x0064, "GRASP_EXECUTE",    "STRUCT{grasp_id}",    "",     "Command: execute specified grasp")
MANIP1.add(0x0065, "GRASP_RESULT",     "UINT8",               "",     "0=success, 1=slip, 2=miss, 3=collision, 4=force_limit")
MANIP1.add(0x0066, "APPROACH_VECTOR",  "ARRAY<FLOAT32,3>",    "",     "Approach direction for grasp")
MANIP1.add(0x0067, "RETREAT_VECTOR",   "ARRAY<FLOAT32,3>",    "",     "Retreat direction after grasp")
MANIP1.add(0x0068, "OBJECT_MASS",      "FLOAT32",             "kg",   "Estimated mass of grasped object")
MANIP1.add(0x0069, "CENTER_OF_MASS",   "ARRAY<FLOAT32,3>",    "m",    "Estimated CoM of grasped object")
MANIP1.add(0x006A, "INERTIA_TENSOR",   "ARRAY<FLOAT32,9>",    "kg*m^2","Estimated rotational inertia of object")

# Manipulation Actions (0x0080-0x009F)
MANIP1.add(0x0080, "PICK",             "STRUCT{object_id,grasp}","",   "Pick up object with grasp plan")
MANIP1.add(0x0081, "PLACE",            "STRUCT{object_id,target_pose}","","Place object at target pose")
MANIP1.add(0x0082, "PUSH",             "STRUCT{object_id,direction,dist}","","Push object in direction")
MANIP1.add(0x0083, "PULL",             "STRUCT{object_id,direction,dist}","","Pull object in direction")
MANIP1.add(0x0084, "ROTATE_OBJECT",    "STRUCT{object_id,axis,angle}","","Rotate held object about axis")
MANIP1.add(0x0085, "INSERT",           "STRUCT{peg_id,hole_pose,tol}","","Peg-in-hole insertion")
MANIP1.add(0x0086, "SCREW",            "STRUCT{fastener,direction,torque}","","Screw/unscrew operation")
MANIP1.add(0x0087, "POUR",             "STRUCT{source,target,amount}","","Pour from container to target")
MANIP1.add(0x0088, "WIPE",             "STRUCT{surface,pattern,force}","","Wiping/cleaning motion")
MANIP1.add(0x0089, "HANDOVER",         "STRUCT{object_id,to_agent}","","Hand object to another agent")
MANIP1.add(0x008A, "RECEIVE_OBJECT",   "STRUCT{from_agent}",  "",     "Ready to receive object from agent")
MANIP1.add(0x008B, "STACK",            "STRUCT{object_id,on_top_of}","","Stack object on another")
MANIP1.add(0x008C, "UNSTACK",          "STRUCT{object_id}",   "",     "Remove top object from stack")
MANIP1.add(0x008D, "ALIGN",            "STRUCT{object_id,reference}","","Align object to reference")
MANIP1.add(0x008E, "FOLD",             "STRUCT{object_id,fold_line,angle}","","Fold deformable object")
MANIP1.add(0x008F, "CUT",              "STRUCT{tool,path,depth}","",   "Cutting operation along path")

# Contact and Force Control (0x00A0-0x00AF)
MANIP1.add(0x00A0, "FORCE_MODE",       "UINT8",               "",     "0=position, 1=force, 2=impedance, 3=admittance, 4=hybrid")
MANIP1.add(0x00A1, "TARGET_FORCE",     "ARRAY<FLOAT32,3>",    "N",    "Commanded contact force")
MANIP1.add(0x00A2, "TARGET_TORQUE",    "ARRAY<FLOAT32,3>",    "Nm",   "Commanded contact torque")
MANIP1.add(0x00A3, "CONTACT_STATE",    "UINT8",               "",     "0=free, 1=approaching, 2=contact, 3=stable, 4=sliding, 5=stuck")
MANIP1.add(0x00A4, "FORCE_ERROR",      "ARRAY<FLOAT32,6>",    "",     "Force/torque tracking error")
MANIP1.add(0x00A5, "COMPLIANCE_AXES",  "ARRAY<BOOL,6>",       "",     "Which axes are compliant (force-controlled)")
MANIP1.add(0x00A6, "STIFFNESS_MATRIX", "ARRAY<FLOAT32,36>",   "",     "6x6 Cartesian stiffness matrix")
MANIP1.add(0x00A7, "DAMPING_MATRIX",   "ARRAY<FLOAT32,36>",   "",     "6x6 Cartesian damping matrix")

# Deformable Object Handling (0x00B0-0x00BF)
MANIP1.add(0x00B0, "DEFORM_MODEL",     "STRUCT{type,params}", "",     "Deformable object model (FEM, mass-spring, etc.)")
MANIP1.add(0x00B1, "DEFORM_STATE",     "LIST<ARRAY<FLOAT32,3>>","m",  "Current deformation state (node positions)")
MANIP1.add(0x00B2, "STRETCH_LIMIT",    "FLOAT32",             "",     "Maximum allowable stretch ratio")
MANIP1.add(0x00B3, "STIFFNESS_EST",    "FLOAT32",             "N/m",  "Estimated object stiffness")
MANIP1.add(0x00B4, "ROPE_CONFIG",      "LIST<ARRAY<FLOAT32,3>>","m",  "Rope/cable configuration (ordered points)")
MANIP1.add(0x00B5, "CLOTH_CORNERS",    "LIST<ARRAY<FLOAT32,3>>","m",  "Cloth corner positions")
MANIP1.add(0x00B6, "KNOT_TYPE",        "UINT8",               "",     "0=none, 1=overhand, 2=bowline, 3=cleat_hitch, 4=unknown")


# ── COMM-1: Inter-agent Communication ────────────────────────────────

COMM1 = DomainCodebook(0x04, "COMM-1", "Inter-agent communication and social protocols")

# Agent Identity and Discovery (0x0000-0x001F)
COMM1.add(0x0000, "AGENT_UUID",       "BYTES(16)",           "",     "128-bit agent unique identifier")
COMM1.add(0x0001, "AGENT_NAME",       "STRING",              "",     "Human-readable agent name")
COMM1.add(0x0002, "AGENT_TYPE",       "UINT8",               "",     "0=ground_robot, 1=aerial, 2=underwater, 3=manipulator, 4=humanoid, 5=vehicle, 6=sensor_node, 7=base_station")
COMM1.add(0x0003, "AGENT_ROLE",       "UINT8",               "",     "0=worker, 1=leader, 2=scout, 3=relay, 4=supervisor, 5=medic, 6=transport, 7=sentinel")
COMM1.add(0x0004, "TEAM_ID",          "UINT16",              "",     "Team/group membership identifier")
COMM1.add(0x0005, "AUTHORITY_LEVEL",   "UINT8",               "",     "Command authority 0 (none) to 7 (supreme)")
COMM1.add(0x0006, "DISCOVERY_BEACON",  "STRUCT{uuid,type,caps}","",   "Periodic presence announcement")
COMM1.add(0x0007, "PEER_LIST",        "LIST<STRUCT{uuid,name,type}>","","Known peers in communication range")
COMM1.add(0x0008, "HEARTBEAT",        "STRUCT{uuid,ts,health}","",    "Periodic liveness signal")
COMM1.add(0x0009, "AGENT_DEPARTED",   "STRUCT{uuid,reason}",  "",     "Agent leaving communication group")
COMM1.add(0x000A, "AGENT_JOINED",     "STRUCT{uuid,caps}",    "",     "New agent entered communication range")
COMM1.add(0x000B, "IDENTITY_VERIFY",  "STRUCT{uuid,challenge}","",    "Identity verification challenge")
COMM1.add(0x000C, "IDENTITY_RESPONSE","STRUCT{uuid,signature}","",    "Identity verification response")
COMM1.add(0x000D, "TRUST_LEVEL",      "STRUCT{uuid,level}",   "",     "Trust assessment for agent (0.0-1.0)")

# Message Routing (0x0020-0x003F)
COMM1.add(0x0020, "UNICAST",          "STRUCT{dest_uuid}",    "",     "Directed message to single agent")
COMM1.add(0x0021, "MULTICAST",        "STRUCT{dest_list}",    "",     "Directed to set of agents")
COMM1.add(0x0022, "BROADCAST",        "NONE",                 "",     "Sent to all agents in range")
COMM1.add(0x0023, "RELAY_REQUEST",    "STRUCT{dest,via}",     "",     "Request message relay through intermediary")
COMM1.add(0x0024, "RELAY_ACK",        "STRUCT{msg_id}",       "",     "Relay node confirms forwarding")
COMM1.add(0x0025, "MESH_ROUTE",       "LIST<UINT128>",        "",     "Explicit route through mesh network (UUID list)")
COMM1.add(0x0026, "HOP_COUNT",        "UINT8",                "",     "Number of relay hops traversed")
COMM1.add(0x0027, "MSG_ID",           "UINT64",               "",     "Unique message identifier for dedup")
COMM1.add(0x0028, "REPLY_TO",         "UINT64",               "",     "Message ID this is replying to")
COMM1.add(0x0029, "THREAD_ID",        "UINT64",               "",     "Conversation thread identifier")
COMM1.add(0x002A, "PRIORITY_OVERRIDE","UINT8",                "",     "Override message priority (0-7)")
COMM1.add(0x002B, "EXPIRY_TIME",      "TIMESTAMP",            "",     "Message expires after this time")

# Channel Management (0x0040-0x005F)
COMM1.add(0x0040, "CHANNEL_BUSY",     "NONE",                 "",     "Carrier sense: channel occupied")
COMM1.add(0x0041, "CHANNEL_CLEAR",    "NONE",                 "",     "Carrier sense: channel free")
COMM1.add(0x0042, "TX_REQUEST",       "STRUCT{duration_ms}",  "",     "Request to transmit for N ms")
COMM1.add(0x0043, "TX_GRANT",         "STRUCT{slot_start,duration}","","Permission to transmit in time slot")
COMM1.add(0x0044, "TX_DENY",          "STRUCT{reason}",       "",     "Transmission request denied")
COMM1.add(0x0045, "TDMA_SCHEDULE",    "LIST<STRUCT{agent,slot,dur}>","","Time-division schedule assignment")
COMM1.add(0x0046, "INTERFERENCE_REPORT","STRUCT{freq,level,direction}","","Detected RF/acoustic interference")
COMM1.add(0x0047, "CHANNEL_SWITCH",   "STRUCT{new_band,time}","",     "Request/announce band change")
COMM1.add(0x0048, "SILENCE_PERIOD",   "STRUCT{start,duration}","",    "Request radio silence period")
COMM1.add(0x0049, "ENCRYPTION_MODE",  "UINT8",                "",     "0=none, 1=AES128, 2=AES256, 3=ChaCha20")
COMM1.add(0x004A, "KEY_EXCHANGE",     "STRUCT{type,pubkey}",  "",     "Cryptographic key exchange")
COMM1.add(0x004B, "SESSION_KEY",      "BYTES",                "",     "Encrypted session key delivery")

# Status and Social (0x0060-0x007F)
COMM1.add(0x0060, "STATUS_UPDATE",    "STRUCT{agent,status,detail}","","General status broadcast")
COMM1.add(0x0061, "HELP_REQUEST",     "STRUCT{type,urgency,pos}","",  "Request assistance from peers")
COMM1.add(0x0062, "HELP_OFFER",       "STRUCT{to_agent,eta}", "",     "Offer to assist another agent")
COMM1.add(0x0063, "HELP_DECLINE",     "STRUCT{to_agent,reason}","",   "Decline assistance offer")
COMM1.add(0x0064, "SITUATION_REPORT", "STRUCT{summary,threats,assets}","","Comprehensive situation report")
COMM1.add(0x0065, "INFORMATION_SHARE","STRUCT{topic,data}",   "",     "Proactive information sharing")
COMM1.add(0x0066, "ATTENTION_ALERT",  "STRUCT{target,urgency}","",    "Request another agent's attention")
COMM1.add(0x0067, "THANK",            "STRUCT{to_agent,reason}","",   "Social: express gratitude")
COMM1.add(0x0068, "APOLOGY",          "STRUCT{to_agent,context}","",  "Social: express regret for error")
COMM1.add(0x0069, "HUMOR_MARKER",     "NONE",                 "",     "Indicates non-literal/playful intent")
COMM1.add(0x006A, "SARCASM_MARKER",   "NONE",                 "",     "Indicates opposite-meaning intent")
COMM1.add(0x006B, "PING",             "STRUCT{dest_uuid}",    "",     "Lightweight liveness check")
COMM1.add(0x006C, "PONG",             "STRUCT{src_uuid,latency}","",  "Liveness response with measured latency")

# Data Synchronization (0x0080-0x0097)
COMM1.add(0x0080, "SYNC_REQUEST",     "STRUCT{dataset,version}","",   "Request data synchronization")
COMM1.add(0x0081, "SYNC_OFFER",       "STRUCT{dataset,version,hash}","","Offer dataset for sync")
COMM1.add(0x0082, "SYNC_DIFF",        "STRUCT{dataset,changes}","",   "Incremental dataset update")
COMM1.add(0x0083, "SYNC_ACK",         "STRUCT{dataset,version}","",   "Acknowledge sync complete")
COMM1.add(0x0084, "BLACKBOARD_PUT",   "STRUCT{key,value}",    "",     "Write to shared blackboard")
COMM1.add(0x0085, "BLACKBOARD_GET",   "STRUCT{key}",          "",     "Read from shared blackboard")
COMM1.add(0x0086, "BLACKBOARD_VALUE", "STRUCT{key,value,ts}", "",     "Blackboard read response")
COMM1.add(0x0087, "BLACKBOARD_SUBSCRIBE","STRUCT{key_pattern}","",    "Subscribe to blackboard changes")
COMM1.add(0x0088, "BLACKBOARD_NOTIFY","STRUCT{key,value,ts}", "",     "Notification of blackboard change")
COMM1.add(0x0089, "EVENT_PUBLISH",    "STRUCT{topic,payload}","",     "Publish event to topic")
COMM1.add(0x008A, "EVENT_SUBSCRIBE",  "STRUCT{topic}",        "",     "Subscribe to event topic")
COMM1.add(0x008B, "EVENT_UNSUBSCRIBE","STRUCT{topic}",        "",     "Unsubscribe from event topic")


# ── SAFETY-1: Safety and Emergency Protocols ─────────────────────────

SAFETY1 = DomainCodebook(0x07, "SAFETY-1", "Safety, emergency, and regulatory compliance")

# Emergency Levels and Alerts (0x0000-0x001F)
SAFETY1.add(0x0000, "EMERGENCY_LEVEL", "UINT8",               "",     "0=clear, 1=caution, 2=warning, 3=danger, 4=critical, 5=catastrophic")
SAFETY1.add(0x0001, "EMERGENCY_TYPE",  "UINT8",               "",     "0=collision, 1=fire, 2=flood, 3=structural, 4=chemical, 5=electrical, 6=medical, 7=security, 8=loss_of_control")
SAFETY1.add(0x0002, "EMERGENCY_DECLARE","STRUCT{level,type,pos,desc}","","Declare emergency with location and description")
SAFETY1.add(0x0003, "EMERGENCY_CLEAR", "STRUCT{type}",         "",     "Declare emergency condition resolved")
SAFETY1.add(0x0004, "MAYDAY",          "STRUCT{agent,pos,nature}","",  "Distress call: agent in immediate danger")
SAFETY1.add(0x0005, "PAN_PAN",         "STRUCT{agent,pos,nature}","",  "Urgency call: agent needs assistance")
SAFETY1.add(0x0006, "ALL_STOP",        "NONE",                 "",     "Immediate halt command to all agents")
SAFETY1.add(0x0007, "RESUME_OPERATIONS","NONE",                "",     "Resume normal operations after all-stop")
SAFETY1.add(0x0008, "EVACUATION_ORDER","STRUCT{zone,rally_point}","",  "Order to evacuate zone to rally point")
SAFETY1.add(0x0009, "SHELTER_IN_PLACE","STRUCT{zone,duration}","",     "Order to hold position and wait")
SAFETY1.add(0x000A, "DISTRESS_BEACON", "STRUCT{uuid,pos,ts}",  "",    "Periodic emergency beacon until rescued/resolved")

# Human Safety (0x0020-0x003F)
SAFETY1.add(0x0020, "HUMAN_DETECTED",  "STRUCT{pos,distance,conf}","", "Human presence detected near agent")
SAFETY1.add(0x0021, "HUMAN_PROXIMITY", "FLOAT32",             "m",    "Distance to nearest detected human")
SAFETY1.add(0x0022, "HUMAN_IN_WORKSPACE","BOOL",               "",    "Human has entered robot workspace")
SAFETY1.add(0x0023, "SAFETY_ZONE",     "UINT8",               "",     "0=safe (>2m), 1=warning (1-2m), 2=protective (<1m), 3=danger (<0.5m)")
SAFETY1.add(0x0024, "SPEED_LIMIT",     "FLOAT32",             "m/s",  "Current speed limit for human safety")
SAFETY1.add(0x0025, "FORCE_LIMIT",     "FLOAT32",             "N",    "Current force limit for human safety")
SAFETY1.add(0x0026, "PROTECTIVE_STOP", "STRUCT{reason,pos}",   "",    "Safety-rated protective stop engaged")
SAFETY1.add(0x0027, "SAFETY_STOP_CLEAR","NONE",                "",    "Protective stop condition resolved")
SAFETY1.add(0x0028, "PERSON_TRACKING", "LIST<STRUCT{id,pos,vel}>","", "All tracked persons with trajectories")
SAFETY1.add(0x0029, "PERSON_PREDICTED","STRUCT{id,pred_pos,horizon}","","Predicted person position at time horizon")
SAFETY1.add(0x002A, "COLLABORATIVE_MODE","UINT8",              "",    "0=separated, 1=coexistence, 2=cooperation, 3=collaboration (ISO 10218)")
SAFETY1.add(0x002B, "SAFETY_RATED_SPEED","FLOAT32",           "m/s", "Safety-rated monitored speed (ISO/TS 15066)")
SAFETY1.add(0x002C, "POWER_FORCE_LIMIT","STRUCT{body_part,max_force}","N","ISO/TS 15066 per-body-part force limits")

# Fault and Failure (0x0040-0x005F)
SAFETY1.add(0x0040, "FAULT_DETECTED",  "STRUCT{system,code,severity}","","System fault detected")
SAFETY1.add(0x0041, "FAULT_CLEARED",   "STRUCT{system,code}",  "",     "Fault condition resolved")
SAFETY1.add(0x0042, "FAILSAFE_ACTIVE", "STRUCT{type}",         "",     "Failsafe mode engaged: 0=soft_stop, 1=safe_park, 2=return_home, 3=power_off, 4=controlled_descent")
SAFETY1.add(0x0043, "REDUNDANCY_STATUS","STRUCT{system,primary,backup}","","Redundant system health")
SAFETY1.add(0x0044, "WATCHDOG_TRIP",   "STRUCT{module,last_seen}","",  "Watchdog timer expired for module")
SAFETY1.add(0x0045, "COMM_LOST",       "STRUCT{agent,duration}","",    "Communication lost with agent")
SAFETY1.add(0x0046, "COMM_RESTORED",   "STRUCT{agent}",        "",     "Communication restored with agent")
SAFETY1.add(0x0047, "GPS_LOST",        "NONE",                 "",     "GPS signal lost")
SAFETY1.add(0x0048, "GPS_RESTORED",    "STRUCT{accuracy}",     "m",    "GPS signal restored with accuracy")
SAFETY1.add(0x0049, "SENSOR_FAULT",    "STRUCT{sensor_id,type}","",    "Sensor fault: 0=degraded, 1=failed, 2=inconsistent, 3=stuck")
SAFETY1.add(0x004A, "ACTUATOR_FAULT",  "STRUCT{actuator_id,type}","",  "Actuator fault: 0=degraded, 1=locked, 2=runaway, 3=disconnected")
SAFETY1.add(0x004B, "POWER_FAULT",     "STRUCT{type,details}", "",     "Power system fault: 0=brownout, 1=overcurrent, 2=cell_imbalance, 3=thermal_runaway")
SAFETY1.add(0x004C, "ESTOP_PRESSED",   "STRUCT{agent,source}", "",     "Emergency stop button activated")
SAFETY1.add(0x004D, "ESTOP_RELEASED",  "STRUCT{agent}",        "",     "Emergency stop button released")

# Geofence and Regulatory (0x0060-0x007F)
SAFETY1.add(0x0060, "GEOFENCE_BREACH", "STRUCT{fence_id,pos}", "",     "Agent has breached geofence boundary")
SAFETY1.add(0x0061, "ALTITUDE_LIMIT",  "FLOAT32",             "m",    "Maximum permitted altitude")
SAFETY1.add(0x0062, "ALTITUDE_BREACH", "STRUCT{current,limit}","m",    "Agent exceeds altitude limit")
SAFETY1.add(0x0063, "SPEED_BREACH",    "STRUCT{current,limit}","m/s",  "Agent exceeds speed limit")
SAFETY1.add(0x0064, "RESTRICTED_ZONE", "STRUCT{id,polygon,floor,ceiling}","","Defined restricted zone")
SAFETY1.add(0x0065, "ZONE_ENTERED",    "STRUCT{zone_id}",      "",     "Agent entered restricted zone")
SAFETY1.add(0x0066, "ZONE_EXITED",     "STRUCT{zone_id}",      "",     "Agent exited restricted zone")
SAFETY1.add(0x0067, "FLIGHT_AUTH",     "STRUCT{area,start,end,auth_id}","","Regulatory flight authorization")
SAFETY1.add(0x0068, "REMOTE_ID",       "STRUCT{uuid,pos,alt,vel,pilot_pos}","","Remote identification broadcast (FAA compliance)")
SAFETY1.add(0x0069, "NOISE_LIMIT",     "FLOAT16",             "dB_SPL","Maximum permitted noise level")
SAFETY1.add(0x006A, "OPERATING_HOURS", "STRUCT{start,end}",    "",     "Permitted operating time window")
SAFETY1.add(0x006B, "WEATHER_LIMIT",   "STRUCT{max_wind,min_vis,max_rain}","","Weather operating limits")
SAFETY1.add(0x006C, "WEATHER_ABORT",   "STRUCT{condition}",    "",     "Weather exceeds operating limits")

# Safety Monitoring (0x0080-0x0097)
SAFETY1.add(0x0080, "SAFETY_SCORE",    "FLOAT16",             "",     "Overall safety score 0.0-1.0")
SAFETY1.add(0x0081, "RISK_ASSESSMENT", "STRUCT{hazard,probability,severity}","","Risk assessment for hazard")
SAFETY1.add(0x0082, "MITIGATION_ACTIVE","STRUCT{risk_id,measure}","",  "Active risk mitigation measure")
SAFETY1.add(0x0083, "SAFETY_LOG",      "STRUCT{event,ts,details}","",  "Safety event log entry")
SAFETY1.add(0x0084, "NEAR_MISS",       "STRUCT{type,agents,min_dist}","","Near-miss incident report")
SAFETY1.add(0x0085, "INCIDENT_REPORT", "STRUCT{type,agents,pos,ts,desc}","","Post-incident report")
SAFETY1.add(0x0086, "SAFE_LANDING_SITES","LIST<STRUCT{pos,quality}>","","Available emergency landing sites")
SAFETY1.add(0x0087, "ESCAPE_ROUTE",    "LIST<POSITION_3D>",   "",     "Planned escape route from current position")
SAFETY1.add(0x0088, "BATTERY_RESERVE", "FLOAT16",             "%",    "Battery reserved for safe return")
SAFETY1.add(0x0089, "POINT_OF_NO_RETURN","STRUCT{pos,time}",  "",     "Must-decide point for safe return")
SAFETY1.add(0x008A, "CONTINGENCY_PLAN","STRUCT{trigger,action}","",   "If-trigger-then-action safety plan")
SAFETY1.add(0x008B, "BLACK_BOX_MARK",  "STRUCT{event,ts}",    "",     "Mark event in flight recorder / black box")


# ── Domain codebook registry ──────────────────────────────────────────

DOMAIN_REGISTRY: dict[int, DomainCodebook] = {
    0x01: NAV1,
    0x02: PERCEPT1,
    0x03: MANIP1,
    0x04: COMM1,
    0x05: DIAG1,
    0x06: PLAN1,
    0x07: SAFETY1,
}

def get_domain_codebook(registry_id: int) -> Optional[DomainCodebook]:
    """Look up a domain codebook by its registry ID."""
    return DOMAIN_REGISTRY.get(registry_id)
