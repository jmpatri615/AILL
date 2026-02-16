/* tslint:disable */
/* eslint-disable */

/**
 * Decode f32 PCM audio samples back into AILL wire-format bytes.
 * Takes a Float32Array of mono PCM samples.
 * If sample_rate is 0, defaults to 48000 Hz.
 */
export function acoustic_decode(samples: Float32Array, sample_rate: number): Uint8Array;

/**
 * Calculate the duration in seconds for encoding a given number of bytes.
 */
export function acoustic_duration(num_bytes: number): number;

/**
 * Encode AILL wire-format bytes into f32 PCM audio samples.
 * Returns a Float32Array of mono PCM samples.
 * If sample_rate is 0, defaults to 48000 Hz.
 */
export function acoustic_encode(wire_bytes: Uint8Array, sample_rate: number): Float32Array;

/**
 * Compute CRC-8 of data. Equivalent to JS `AILL.crc8(data)`.
 */
export function crc8_compute(data: Uint8Array): number;

/**
 * Full AST decode — returns the AST as a JS value (serde-serialized).
 */
export function decode_ast(data: Uint8Array): any;

/**
 * Decode a pragmatic message, returning { act, topic, content, agent } or null.
 */
export function decode_pragmatic_simple(data: Uint8Array): any;

/**
 * Simple decoder that returns {type, content} or null.
 * Matches the demo's `AILL.decode(wire)` behavior.
 */
export function decode_simple(data: Uint8Array): any;

/**
 * Encode arbitrary content as an AILL ASSERT utterance with struct { type, content }.
 * Equivalent to JS `AILL.encodeContent(type, content)`.
 */
export function encode_content(content_type: string, content: string): Uint8Array;

/**
 * Encode a pragmatic act message for song negotiation protocol.
 * act: 0x88=PROPOSE, 0x89=ACCEPT, 0x85=REJECT, 0x81=ASSERT, etc.
 * topic_id: e.g. 0x0100=song_proposal, 0x0101=role_claim, 0x0102=heartbeat
 * content: string payload (e.g. song key, role name, heartbeat data)
 * agent_id: 16-byte UUID of the source agent
 */
export function encode_pragmatic(act: number, topic_id: number, content: string, agent_id: Uint8Array): Uint8Array;

/**
 * Encode a string message as an AILL ASSERT utterance.
 * Equivalent to JS `AILL.encodeString(msg)`.
 */
export function encode_string(msg: string): Uint8Array;

/**
 * Encode a task allocation message (PROPOSE + PLAN-1 ALLOCATE_TASK struct).
 * task_id: numeric task identifier
 * role: role string (e.g. "lead", "harmony", "bass", "descant")
 * agent_id: 16-byte UUID of the source agent
 */
export function encode_task_allocation(task_id: number, role: string, agent_id: Uint8Array): Uint8Array;

/**
 * Encode a URL as an AILL ASSERT utterance with struct { type: "url", content: url }.
 * Equivalent to JS `AILL.encodeURL(url)`.
 */
export function encode_url(url: string): Uint8Array;

/**
 * Get the full mnemonic table as a JS object { code: name, ... }.
 * Equivalent to JS `AILL.MNEMONICS`.
 */
export function get_mnemonics(): any;

/**
 * Generate a hex dump of data with HTML formatting.
 * Equivalent to JS `AILL.hexDump(data, maxBytes)`.
 */
export function hex_dump(data: Uint8Array, max_bytes: number): string;

/**
 * Look up the mnemonic name for a single opcode byte.
 */
export function mnemonic_for(code: number): string;

/**
 * Pretty-print AILL wire-format bytes as a human-readable tree.
 */
export function pretty_print_bytes(data: Uint8Array): string;

/**
 * Validate CRC of wire-format bytes (epoch format).
 */
export function validate_epoch(data: Uint8Array): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly acoustic_decode: (a: number, b: number, c: number) => [number, number, number, number];
    readonly acoustic_duration: (a: number) => number;
    readonly acoustic_encode: (a: number, b: number, c: number) => [number, number, number, number];
    readonly crc8_compute: (a: number, b: number) => number;
    readonly decode_ast: (a: number, b: number) => [number, number, number];
    readonly decode_pragmatic_simple: (a: number, b: number) => any;
    readonly decode_simple: (a: number, b: number) => any;
    readonly encode_content: (a: number, b: number, c: number, d: number) => [number, number];
    readonly encode_pragmatic: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly encode_string: (a: number, b: number) => [number, number];
    readonly encode_task_allocation: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly encode_url: (a: number, b: number) => [number, number];
    readonly get_mnemonics: () => any;
    readonly hex_dump: (a: number, b: number, c: number) => [number, number];
    readonly mnemonic_for: (a: number) => [number, number];
    readonly pretty_print_bytes: (a: number, b: number) => [number, number, number, number];
    readonly validate_epoch: (a: number, b: number) => number;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
