use super::DomainEntry;

/// MANIP-1: Robotic manipulation and grasping (Registry ID 0x03)
pub const MANIP1_REGISTRY_ID: u8 = 0x03;
pub const MANIP1_NAME: &str = "MANIP-1";

pub static MANIP1_ENTRIES: &[DomainEntry] = &[
    // Gripper and End Effector (0x0000-0x001F)
    DomainEntry { code: 0x0000, mnemonic: "GRIPPER_STATE", value_type: "UINT8", unit: "", description: "0=open, 1=closing, 2=closed, 3=opening, 4=holding, 5=error" },
    DomainEntry { code: 0x0001, mnemonic: "GRIPPER_WIDTH", value_type: "FLOAT32", unit: "m", description: "Current gripper aperture width" },
    DomainEntry { code: 0x0002, mnemonic: "GRIPPER_FORCE", value_type: "FLOAT32", unit: "N", description: "Current gripper force" },
    DomainEntry { code: 0x0003, mnemonic: "GRIPPER_SET_WIDTH", value_type: "FLOAT32", unit: "m", description: "Commanded gripper width" },
    DomainEntry { code: 0x0004, mnemonic: "GRIPPER_SET_FORCE", value_type: "FLOAT32", unit: "N", description: "Commanded gripper force limit" },
    DomainEntry { code: 0x0005, mnemonic: "TOOL_TYPE", value_type: "UINT8", unit: "", description: "0=parallel_jaw, 1=vacuum, 2=magnetic, 3=soft, 4=finger_3, 5=hook, 6=scoop, 7=custom" },
    DomainEntry { code: 0x0006, mnemonic: "TOOL_CENTER_POINT", value_type: "ARRAY<FLOAT32,3>", unit: "m", description: "Tool center point (TCP) in end-effector frame" },
    DomainEntry { code: 0x0007, mnemonic: "TOOL_CHANGE_REQ", value_type: "UINT8", unit: "", description: "Request tool change to specified tool type" },
    DomainEntry { code: 0x0008, mnemonic: "TOOL_CHANGE_ACK", value_type: "UINT8", unit: "", description: "Tool change completed" },
    DomainEntry { code: 0x0009, mnemonic: "SUCTION_PRESSURE", value_type: "FLOAT32", unit: "Pa", description: "Vacuum gripper suction pressure" },
    DomainEntry { code: 0x000A, mnemonic: "SUCTION_STATUS", value_type: "UINT8", unit: "", description: "0=off, 1=engaged, 2=leak, 3=lost_seal" },
    DomainEntry { code: 0x000B, mnemonic: "FINGER_POSITIONS", value_type: "LIST<FLOAT32>", unit: "rad", description: "Per-finger joint positions" },
    DomainEntry { code: 0x000C, mnemonic: "FINGER_FORCES", value_type: "LIST<FLOAT32>", unit: "N", description: "Per-finger contact forces" },
    DomainEntry { code: 0x000D, mnemonic: "TACTILE_ARRAY", value_type: "STRUCT{rows,cols,data}", unit: "Pa", description: "Tactile sensor pad readings" },

    // Joint Space (0x0020-0x002F)
    DomainEntry { code: 0x0020, mnemonic: "JOINT_POSITIONS", value_type: "LIST<FLOAT32>", unit: "rad", description: "All joint angles" },
    DomainEntry { code: 0x0021, mnemonic: "JOINT_VELOCITIES", value_type: "LIST<FLOAT32>", unit: "rad/s", description: "All joint angular velocities" },
    DomainEntry { code: 0x0022, mnemonic: "JOINT_TORQUES", value_type: "LIST<FLOAT32>", unit: "Nm", description: "All joint torques" },
    DomainEntry { code: 0x0023, mnemonic: "JOINT_LIMITS", value_type: "LIST<STRUCT{min,max}>", unit: "rad", description: "Joint angle limits" },
    DomainEntry { code: 0x0024, mnemonic: "JOINT_TARGET", value_type: "LIST<FLOAT32>", unit: "rad", description: "Commanded joint positions" },
    DomainEntry { code: 0x0025, mnemonic: "JOINT_TRAJECTORY", value_type: "LIST<STRUCT{time,positions}>", unit: "", description: "Time-parameterized joint trajectory" },
    DomainEntry { code: 0x0026, mnemonic: "JOINT_IMPEDANCE", value_type: "STRUCT{stiffness,damping}", unit: "", description: "Joint impedance parameters" },
    DomainEntry { code: 0x0027, mnemonic: "DOF_COUNT", value_type: "UINT8", unit: "", description: "Number of degrees of freedom" },
    DomainEntry { code: 0x0028, mnemonic: "DH_PARAMETERS", value_type: "LIST<STRUCT{a,alpha,d,theta}>", unit: "", description: "Denavit-Hartenberg kinematic parameters" },
    DomainEntry { code: 0x0029, mnemonic: "SINGULARITY_PROXIMITY", value_type: "FLOAT16", unit: "", description: "Distance to kinematic singularity 0.0-1.0" },

    // Cartesian Space (0x0040-0x004F)
    DomainEntry { code: 0x0040, mnemonic: "EE_POSE", value_type: "STRUCT{pos,orient}", unit: "", description: "End-effector pose in base frame" },
    DomainEntry { code: 0x0041, mnemonic: "EE_VELOCITY", value_type: "STRUCT{linear,angular}", unit: "", description: "End-effector twist (linear + angular velocity)" },
    DomainEntry { code: 0x0042, mnemonic: "EE_WRENCH", value_type: "STRUCT{force,torque}", unit: "", description: "End-effector wrench (force + torque)" },
    DomainEntry { code: 0x0043, mnemonic: "CARTESIAN_TARGET", value_type: "STRUCT{pos,orient}", unit: "", description: "Commanded end-effector pose" },
    DomainEntry { code: 0x0044, mnemonic: "CARTESIAN_PATH", value_type: "LIST<STRUCT{pos,orient,time}>", unit: "", description: "Cartesian trajectory waypoints" },
    DomainEntry { code: 0x0045, mnemonic: "WORKSPACE_LIMIT", value_type: "STRUCT{min,max}", unit: "m", description: "Reachable workspace bounding box" },
    DomainEntry { code: 0x0046, mnemonic: "COMPLIANCE_FRAME", value_type: "STRUCT{pos,orient}", unit: "", description: "Reference frame for compliance control" },
    DomainEntry { code: 0x0047, mnemonic: "IMPEDANCE_PARAMS", value_type: "STRUCT{mass,damping,stiffness}", unit: "", description: "Cartesian impedance parameters" },
    DomainEntry { code: 0x0048, mnemonic: "FORCE_THRESHOLD", value_type: "STRUCT{force,torque}", unit: "", description: "Force/torque thresholds for safety stop" },

    // Grasp Planning (0x0060-0x006F)
    DomainEntry { code: 0x0060, mnemonic: "GRASP_POSE", value_type: "STRUCT{pos,orient,width}", unit: "", description: "Planned grasp pose" },
    DomainEntry { code: 0x0061, mnemonic: "GRASP_QUALITY", value_type: "FLOAT16", unit: "", description: "Grasp quality metric 0.0-1.0" },
    DomainEntry { code: 0x0062, mnemonic: "GRASP_TYPE", value_type: "UINT8", unit: "", description: "0=power, 1=precision, 2=pinch, 3=wrap, 4=hook, 5=lateral, 6=spherical" },
    DomainEntry { code: 0x0063, mnemonic: "GRASP_LIST", value_type: "LIST<STRUCT{pose,quality,type}>", unit: "", description: "Ranked list of candidate grasps" },
    DomainEntry { code: 0x0064, mnemonic: "GRASP_EXECUTE", value_type: "STRUCT{grasp_id}", unit: "", description: "Command: execute specified grasp" },
    DomainEntry { code: 0x0065, mnemonic: "GRASP_RESULT", value_type: "UINT8", unit: "", description: "0=success, 1=slip, 2=miss, 3=collision, 4=force_limit" },
    DomainEntry { code: 0x0066, mnemonic: "APPROACH_VECTOR", value_type: "ARRAY<FLOAT32,3>", unit: "", description: "Approach direction for grasp" },
    DomainEntry { code: 0x0067, mnemonic: "RETREAT_VECTOR", value_type: "ARRAY<FLOAT32,3>", unit: "", description: "Retreat direction after grasp" },
    DomainEntry { code: 0x0068, mnemonic: "OBJECT_MASS", value_type: "FLOAT32", unit: "kg", description: "Estimated mass of grasped object" },
    DomainEntry { code: 0x0069, mnemonic: "CENTER_OF_MASS", value_type: "ARRAY<FLOAT32,3>", unit: "m", description: "Estimated CoM of grasped object" },
    DomainEntry { code: 0x006A, mnemonic: "INERTIA_TENSOR", value_type: "ARRAY<FLOAT32,9>", unit: "kg*m^2", description: "Estimated rotational inertia of object" },

    // Manipulation Actions (0x0080-0x008F)
    DomainEntry { code: 0x0080, mnemonic: "PICK", value_type: "STRUCT{object_id,grasp}", unit: "", description: "Pick up object with grasp plan" },
    DomainEntry { code: 0x0081, mnemonic: "PLACE", value_type: "STRUCT{object_id,target_pose}", unit: "", description: "Place object at target pose" },
    DomainEntry { code: 0x0082, mnemonic: "PUSH", value_type: "STRUCT{object_id,direction,dist}", unit: "", description: "Push object in direction" },
    DomainEntry { code: 0x0083, mnemonic: "PULL", value_type: "STRUCT{object_id,direction,dist}", unit: "", description: "Pull object in direction" },
    DomainEntry { code: 0x0084, mnemonic: "ROTATE_OBJECT", value_type: "STRUCT{object_id,axis,angle}", unit: "", description: "Rotate held object about axis" },
    DomainEntry { code: 0x0085, mnemonic: "INSERT", value_type: "STRUCT{peg_id,hole_pose,tol}", unit: "", description: "Peg-in-hole insertion" },
    DomainEntry { code: 0x0086, mnemonic: "SCREW", value_type: "STRUCT{fastener,direction,torque}", unit: "", description: "Screw/unscrew operation" },
    DomainEntry { code: 0x0087, mnemonic: "POUR", value_type: "STRUCT{source,target,amount}", unit: "", description: "Pour from container to target" },
    DomainEntry { code: 0x0088, mnemonic: "WIPE", value_type: "STRUCT{surface,pattern,force}", unit: "", description: "Wiping/cleaning motion" },
    DomainEntry { code: 0x0089, mnemonic: "HANDOVER", value_type: "STRUCT{object_id,to_agent}", unit: "", description: "Hand object to another agent" },
    DomainEntry { code: 0x008A, mnemonic: "RECEIVE_OBJECT", value_type: "STRUCT{from_agent}", unit: "", description: "Ready to receive object from agent" },
    DomainEntry { code: 0x008B, mnemonic: "STACK", value_type: "STRUCT{object_id,on_top_of}", unit: "", description: "Stack object on another" },
    DomainEntry { code: 0x008C, mnemonic: "UNSTACK", value_type: "STRUCT{object_id}", unit: "", description: "Remove top object from stack" },
    DomainEntry { code: 0x008D, mnemonic: "ALIGN", value_type: "STRUCT{object_id,reference}", unit: "", description: "Align object to reference" },
    DomainEntry { code: 0x008E, mnemonic: "FOLD", value_type: "STRUCT{object_id,fold_line,angle}", unit: "", description: "Fold deformable object" },
    DomainEntry { code: 0x008F, mnemonic: "CUT", value_type: "STRUCT{tool,path,depth}", unit: "", description: "Cutting operation along path" },

    // Contact and Force Control (0x00A0-0x00AF)
    DomainEntry { code: 0x00A0, mnemonic: "FORCE_MODE", value_type: "UINT8", unit: "", description: "0=position, 1=force, 2=impedance, 3=admittance, 4=hybrid" },
    DomainEntry { code: 0x00A1, mnemonic: "TARGET_FORCE", value_type: "ARRAY<FLOAT32,3>", unit: "N", description: "Commanded contact force" },
    DomainEntry { code: 0x00A2, mnemonic: "TARGET_TORQUE", value_type: "ARRAY<FLOAT32,3>", unit: "Nm", description: "Commanded contact torque" },
    DomainEntry { code: 0x00A3, mnemonic: "CONTACT_STATE", value_type: "UINT8", unit: "", description: "0=free, 1=approaching, 2=contact, 3=stable, 4=sliding, 5=stuck" },
    DomainEntry { code: 0x00A4, mnemonic: "FORCE_ERROR", value_type: "ARRAY<FLOAT32,6>", unit: "", description: "Force/torque tracking error" },
    DomainEntry { code: 0x00A5, mnemonic: "COMPLIANCE_AXES", value_type: "ARRAY<BOOL,6>", unit: "", description: "Which axes are compliant (force-controlled)" },
    DomainEntry { code: 0x00A6, mnemonic: "STIFFNESS_MATRIX", value_type: "ARRAY<FLOAT32,36>", unit: "", description: "6x6 Cartesian stiffness matrix" },
    DomainEntry { code: 0x00A7, mnemonic: "DAMPING_MATRIX", value_type: "ARRAY<FLOAT32,36>", unit: "", description: "6x6 Cartesian damping matrix" },

    // Deformable Object Handling (0x00B0-0x00BF)
    DomainEntry { code: 0x00B0, mnemonic: "DEFORM_MODEL", value_type: "STRUCT{type,params}", unit: "", description: "Deformable object model (FEM, mass-spring, etc.)" },
    DomainEntry { code: 0x00B1, mnemonic: "DEFORM_STATE", value_type: "LIST<ARRAY<FLOAT32,3>>", unit: "m", description: "Current deformation state (node positions)" },
    DomainEntry { code: 0x00B2, mnemonic: "STRETCH_LIMIT", value_type: "FLOAT32", unit: "", description: "Maximum allowable stretch ratio" },
    DomainEntry { code: 0x00B3, mnemonic: "STIFFNESS_EST", value_type: "FLOAT32", unit: "N/m", description: "Estimated object stiffness" },
    DomainEntry { code: 0x00B4, mnemonic: "ROPE_CONFIG", value_type: "LIST<ARRAY<FLOAT32,3>>", unit: "m", description: "Rope/cable configuration (ordered points)" },
    DomainEntry { code: 0x00B5, mnemonic: "CLOTH_CORNERS", value_type: "LIST<ARRAY<FLOAT32,3>>", unit: "m", description: "Cloth corner positions" },
    DomainEntry { code: 0x00B6, mnemonic: "KNOT_TYPE", value_type: "UINT8", unit: "", description: "0=none, 1=overhand, 2=bowline, 3=cleat_hitch, 4=unknown" },
];
