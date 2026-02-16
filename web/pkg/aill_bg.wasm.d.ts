/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const acoustic_decode: (a: number, b: number, c: number) => [number, number, number, number];
export const acoustic_duration: (a: number) => number;
export const acoustic_encode: (a: number, b: number, c: number) => [number, number, number, number];
export const crc8_compute: (a: number, b: number) => number;
export const decode_ast: (a: number, b: number) => [number, number, number];
export const decode_pragmatic_simple: (a: number, b: number) => any;
export const decode_simple: (a: number, b: number) => any;
export const encode_content: (a: number, b: number, c: number, d: number) => [number, number];
export const encode_pragmatic: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
export const encode_string: (a: number, b: number) => [number, number];
export const encode_task_allocation: (a: number, b: number, c: number, d: number, e: number) => [number, number];
export const encode_url: (a: number, b: number) => [number, number];
export const get_mnemonics: () => any;
export const hex_dump: (a: number, b: number, c: number) => [number, number];
export const mnemonic_for: (a: number) => [number, number];
export const pretty_print_bytes: (a: number, b: number) => [number, number, number, number];
export const validate_epoch: (a: number, b: number) => number;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __wbindgen_exn_store: (a: number) => void;
export const __externref_table_alloc: () => number;
export const __wbindgen_externrefs: WebAssembly.Table;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_start: () => void;
