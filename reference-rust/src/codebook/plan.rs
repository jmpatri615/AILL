use super::DomainEntry;

/// PLAN-1: Planning domain codebook (Registry ID 0x06)
pub const PLAN1_REGISTRY_ID: u8 = 0x06;
pub const PLAN1_NAME: &str = "PLAN-1";

pub static PLAN1_ENTRIES: &[DomainEntry] = &[
    DomainEntry { code: 0x0000, mnemonic: "TASK", value_type: "STRUCT{id,type,params}", unit: "", description: "Task definition" },
    DomainEntry { code: 0x0001, mnemonic: "TASK_ID", value_type: "UINT32", unit: "", description: "Unique task identifier" },
    DomainEntry { code: 0x0002, mnemonic: "TASK_STATUS", value_type: "UINT8", unit: "", description: "0=pending, 1=active, 2=complete, 3=failed, 4=cancelled" },
    DomainEntry { code: 0x0003, mnemonic: "TASK_PRIORITY", value_type: "UINT8", unit: "", description: "Task priority 0-7" },
    DomainEntry { code: 0x0004, mnemonic: "TASK_DEADLINE", value_type: "TIMESTAMP", unit: "", description: "Task completion deadline" },
    DomainEntry { code: 0x0005, mnemonic: "TASK_PROGRESS", value_type: "FLOAT16", unit: "%", description: "Completion percentage 0-100%" },
    DomainEntry { code: 0x0006, mnemonic: "SUBTASK", value_type: "STRUCT{id,parent_id}", unit: "", description: "Subtask with parent reference" },
    DomainEntry { code: 0x0007, mnemonic: "TASK_DEPENDENCY", value_type: "STRUCT{task_id,dep_id}", unit: "", description: "Task A depends on task B" },
    DomainEntry { code: 0x0008, mnemonic: "GOAL", value_type: "STRUCT{id,condition}", unit: "", description: "Goal as a boolean condition" },
    DomainEntry { code: 0x0009, mnemonic: "GOAL_STATUS", value_type: "UINT8", unit: "", description: "0=unachieved, 1=achieved, 2=impossible" },
    DomainEntry { code: 0x000A, mnemonic: "PLAN", value_type: "LIST<TASK>", unit: "", description: "Ordered plan (sequence of tasks)" },
    DomainEntry { code: 0x000B, mnemonic: "PLAN_COST", value_type: "FLOAT32", unit: "", description: "Estimated total plan cost" },
    DomainEntry { code: 0x000C, mnemonic: "PLAN_DURATION", value_type: "FLOAT32", unit: "s", description: "Estimated total plan duration" },
    DomainEntry { code: 0x000D, mnemonic: "ALLOCATE_TASK", value_type: "STRUCT{task_id,agent_id}", unit: "", description: "Assign task to agent" },
    DomainEntry { code: 0x000E, mnemonic: "RELEASE_TASK", value_type: "UINT32", unit: "", description: "Unassign/release a task" },
    DomainEntry { code: 0x000F, mnemonic: "REPLAN_REQUEST", value_type: "STRUCT{reason}", unit: "", description: "Request plan regeneration" },
    DomainEntry { code: 0x0010, mnemonic: "RESOURCE", value_type: "STRUCT{type,amount}", unit: "", description: "Resource requirement or availability" },
    DomainEntry { code: 0x0011, mnemonic: "RESOURCE_CONFLICT", value_type: "STRUCT{res,agents}", unit: "", description: "Resource contention report" },
    DomainEntry { code: 0x0012, mnemonic: "AUCTION_BID", value_type: "STRUCT{task_id,cost}", unit: "", description: "Bid on a task in task auction" },
    DomainEntry { code: 0x0013, mnemonic: "AUCTION_AWARD", value_type: "STRUCT{task_id,agent_id}", unit: "", description: "Award task to winning bidder" },
];
