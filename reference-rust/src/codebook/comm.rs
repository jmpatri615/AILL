use super::DomainEntry;

/// COMM-1: Inter-agent communication and social protocols (Registry ID 0x04)
pub const COMM1_REGISTRY_ID: u8 = 0x04;
pub const COMM1_NAME: &str = "COMM-1";

pub static COMM1_ENTRIES: &[DomainEntry] = &[
    // Agent Identity and Discovery (0x0000-0x001F)
    DomainEntry { code: 0x0000, mnemonic: "AGENT_UUID", value_type: "BYTES(16)", unit: "", description: "128-bit agent unique identifier" },
    DomainEntry { code: 0x0001, mnemonic: "AGENT_NAME", value_type: "STRING", unit: "", description: "Human-readable agent name" },
    DomainEntry { code: 0x0002, mnemonic: "AGENT_TYPE", value_type: "UINT8", unit: "", description: "0=ground_robot, 1=aerial, 2=underwater, 3=manipulator, 4=humanoid, 5=vehicle, 6=sensor_node, 7=base_station" },
    DomainEntry { code: 0x0003, mnemonic: "AGENT_ROLE", value_type: "UINT8", unit: "", description: "0=worker, 1=leader, 2=scout, 3=relay, 4=supervisor, 5=medic, 6=transport, 7=sentinel" },
    DomainEntry { code: 0x0004, mnemonic: "TEAM_ID", value_type: "UINT16", unit: "", description: "Team/group membership identifier" },
    DomainEntry { code: 0x0005, mnemonic: "AUTHORITY_LEVEL", value_type: "UINT8", unit: "", description: "Command authority 0 (none) to 7 (supreme)" },
    DomainEntry { code: 0x0006, mnemonic: "DISCOVERY_BEACON", value_type: "STRUCT{uuid,type,caps}", unit: "", description: "Periodic presence announcement" },
    DomainEntry { code: 0x0007, mnemonic: "PEER_LIST", value_type: "LIST<STRUCT{uuid,name,type}>", unit: "", description: "Known peers in communication range" },
    DomainEntry { code: 0x0008, mnemonic: "HEARTBEAT", value_type: "STRUCT{uuid,ts,health}", unit: "", description: "Periodic liveness signal" },
    DomainEntry { code: 0x0009, mnemonic: "AGENT_DEPARTED", value_type: "STRUCT{uuid,reason}", unit: "", description: "Agent leaving communication group" },
    DomainEntry { code: 0x000A, mnemonic: "AGENT_JOINED", value_type: "STRUCT{uuid,caps}", unit: "", description: "New agent entered communication range" },
    DomainEntry { code: 0x000B, mnemonic: "IDENTITY_VERIFY", value_type: "STRUCT{uuid,challenge}", unit: "", description: "Identity verification challenge" },
    DomainEntry { code: 0x000C, mnemonic: "IDENTITY_RESPONSE", value_type: "STRUCT{uuid,signature}", unit: "", description: "Identity verification response" },
    DomainEntry { code: 0x000D, mnemonic: "TRUST_LEVEL", value_type: "STRUCT{uuid,level}", unit: "", description: "Trust assessment for agent (0.0-1.0)" },

    // Message Routing (0x0020-0x002F)
    DomainEntry { code: 0x0020, mnemonic: "UNICAST", value_type: "STRUCT{dest_uuid}", unit: "", description: "Directed message to single agent" },
    DomainEntry { code: 0x0021, mnemonic: "MULTICAST", value_type: "STRUCT{dest_list}", unit: "", description: "Directed to set of agents" },
    DomainEntry { code: 0x0022, mnemonic: "BROADCAST", value_type: "NONE", unit: "", description: "Sent to all agents in range" },
    DomainEntry { code: 0x0023, mnemonic: "RELAY_REQUEST", value_type: "STRUCT{dest,via}", unit: "", description: "Request message relay through intermediary" },
    DomainEntry { code: 0x0024, mnemonic: "RELAY_ACK", value_type: "STRUCT{msg_id}", unit: "", description: "Relay node confirms forwarding" },
    DomainEntry { code: 0x0025, mnemonic: "MESH_ROUTE", value_type: "LIST<UINT128>", unit: "", description: "Explicit route through mesh network (UUID list)" },
    DomainEntry { code: 0x0026, mnemonic: "HOP_COUNT", value_type: "UINT8", unit: "", description: "Number of relay hops traversed" },
    DomainEntry { code: 0x0027, mnemonic: "MSG_ID", value_type: "UINT64", unit: "", description: "Unique message identifier for dedup" },
    DomainEntry { code: 0x0028, mnemonic: "REPLY_TO", value_type: "UINT64", unit: "", description: "Message ID this is replying to" },
    DomainEntry { code: 0x0029, mnemonic: "THREAD_ID", value_type: "UINT64", unit: "", description: "Conversation thread identifier" },
    DomainEntry { code: 0x002A, mnemonic: "PRIORITY_OVERRIDE", value_type: "UINT8", unit: "", description: "Override message priority (0-7)" },
    DomainEntry { code: 0x002B, mnemonic: "EXPIRY_TIME", value_type: "TIMESTAMP", unit: "", description: "Message expires after this time" },

    // Channel Management (0x0040-0x004F)
    DomainEntry { code: 0x0040, mnemonic: "CHANNEL_BUSY", value_type: "NONE", unit: "", description: "Carrier sense: channel occupied" },
    DomainEntry { code: 0x0041, mnemonic: "CHANNEL_CLEAR", value_type: "NONE", unit: "", description: "Carrier sense: channel free" },
    DomainEntry { code: 0x0042, mnemonic: "TX_REQUEST", value_type: "STRUCT{duration_ms}", unit: "", description: "Request to transmit for N ms" },
    DomainEntry { code: 0x0043, mnemonic: "TX_GRANT", value_type: "STRUCT{slot_start,duration}", unit: "", description: "Permission to transmit in time slot" },
    DomainEntry { code: 0x0044, mnemonic: "TX_DENY", value_type: "STRUCT{reason}", unit: "", description: "Transmission request denied" },
    DomainEntry { code: 0x0045, mnemonic: "TDMA_SCHEDULE", value_type: "LIST<STRUCT{agent,slot,dur}>", unit: "", description: "Time-division schedule assignment" },
    DomainEntry { code: 0x0046, mnemonic: "INTERFERENCE_REPORT", value_type: "STRUCT{freq,level,direction}", unit: "", description: "Detected RF/acoustic interference" },
    DomainEntry { code: 0x0047, mnemonic: "CHANNEL_SWITCH", value_type: "STRUCT{new_band,time}", unit: "", description: "Request/announce band change" },
    DomainEntry { code: 0x0048, mnemonic: "SILENCE_PERIOD", value_type: "STRUCT{start,duration}", unit: "", description: "Request radio silence period" },
    DomainEntry { code: 0x0049, mnemonic: "ENCRYPTION_MODE", value_type: "UINT8", unit: "", description: "0=none, 1=AES128, 2=AES256, 3=ChaCha20" },
    DomainEntry { code: 0x004A, mnemonic: "KEY_EXCHANGE", value_type: "STRUCT{type,pubkey}", unit: "", description: "Cryptographic key exchange" },
    DomainEntry { code: 0x004B, mnemonic: "SESSION_KEY", value_type: "BYTES", unit: "", description: "Encrypted session key delivery" },

    // Status and Social (0x0060-0x006F)
    DomainEntry { code: 0x0060, mnemonic: "STATUS_UPDATE", value_type: "STRUCT{agent,status,detail}", unit: "", description: "General status broadcast" },
    DomainEntry { code: 0x0061, mnemonic: "HELP_REQUEST", value_type: "STRUCT{type,urgency,pos}", unit: "", description: "Request assistance from peers" },
    DomainEntry { code: 0x0062, mnemonic: "HELP_OFFER", value_type: "STRUCT{to_agent,eta}", unit: "", description: "Offer to assist another agent" },
    DomainEntry { code: 0x0063, mnemonic: "HELP_DECLINE", value_type: "STRUCT{to_agent,reason}", unit: "", description: "Decline assistance offer" },
    DomainEntry { code: 0x0064, mnemonic: "SITUATION_REPORT", value_type: "STRUCT{summary,threats,assets}", unit: "", description: "Comprehensive situation report" },
    DomainEntry { code: 0x0065, mnemonic: "INFORMATION_SHARE", value_type: "STRUCT{topic,data}", unit: "", description: "Proactive information sharing" },
    DomainEntry { code: 0x0066, mnemonic: "ATTENTION_ALERT", value_type: "STRUCT{target,urgency}", unit: "", description: "Request another agent's attention" },
    DomainEntry { code: 0x0067, mnemonic: "THANK", value_type: "STRUCT{to_agent,reason}", unit: "", description: "Social: express gratitude" },
    DomainEntry { code: 0x0068, mnemonic: "APOLOGY", value_type: "STRUCT{to_agent,context}", unit: "", description: "Social: express regret for error" },
    DomainEntry { code: 0x0069, mnemonic: "HUMOR_MARKER", value_type: "NONE", unit: "", description: "Indicates non-literal/playful intent" },
    DomainEntry { code: 0x006A, mnemonic: "SARCASM_MARKER", value_type: "NONE", unit: "", description: "Indicates opposite-meaning intent" },
    DomainEntry { code: 0x006B, mnemonic: "PING", value_type: "STRUCT{dest_uuid}", unit: "", description: "Lightweight liveness check" },
    DomainEntry { code: 0x006C, mnemonic: "PONG", value_type: "STRUCT{src_uuid,latency}", unit: "", description: "Liveness response with measured latency" },

    // Data Synchronization (0x0080-0x008F)
    DomainEntry { code: 0x0080, mnemonic: "SYNC_REQUEST", value_type: "STRUCT{dataset,version}", unit: "", description: "Request data synchronization" },
    DomainEntry { code: 0x0081, mnemonic: "SYNC_OFFER", value_type: "STRUCT{dataset,version,hash}", unit: "", description: "Offer dataset for sync" },
    DomainEntry { code: 0x0082, mnemonic: "SYNC_DIFF", value_type: "STRUCT{dataset,changes}", unit: "", description: "Incremental dataset update" },
    DomainEntry { code: 0x0083, mnemonic: "SYNC_ACK", value_type: "STRUCT{dataset,version}", unit: "", description: "Acknowledge sync complete" },
    DomainEntry { code: 0x0084, mnemonic: "BLACKBOARD_PUT", value_type: "STRUCT{key,value}", unit: "", description: "Write to shared blackboard" },
    DomainEntry { code: 0x0085, mnemonic: "BLACKBOARD_GET", value_type: "STRUCT{key}", unit: "", description: "Read from shared blackboard" },
    DomainEntry { code: 0x0086, mnemonic: "BLACKBOARD_VALUE", value_type: "STRUCT{key,value,ts}", unit: "", description: "Blackboard read response" },
    DomainEntry { code: 0x0087, mnemonic: "BLACKBOARD_SUBSCRIBE", value_type: "STRUCT{key_pattern}", unit: "", description: "Subscribe to blackboard changes" },
    DomainEntry { code: 0x0088, mnemonic: "BLACKBOARD_NOTIFY", value_type: "STRUCT{key,value,ts}", unit: "", description: "Notification of blackboard change" },
    DomainEntry { code: 0x0089, mnemonic: "EVENT_PUBLISH", value_type: "STRUCT{topic,payload}", unit: "", description: "Publish event to topic" },
    DomainEntry { code: 0x008A, mnemonic: "EVENT_SUBSCRIBE", value_type: "STRUCT{topic}", unit: "", description: "Subscribe to event topic" },
    DomainEntry { code: 0x008B, mnemonic: "EVENT_UNSUBSCRIBE", value_type: "STRUCT{topic}", unit: "", description: "Unsubscribe from event topic" },
];
