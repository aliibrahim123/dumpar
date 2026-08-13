/* tslint:disable */
/* eslint-disable */

export function main_js(): void;

export function neoview_init_binder(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly main_js: () => void;
    readonly neoview_init_binder: () => void;
    readonly wasm_bindgen_edac182d00970383___convert__closures_____invoke___u32__u32__u32__web_sys_50ec9cc7e5bf384b___features__gen_Event__Event______true_: (a: number, b: number, c: number, d: number, e: number, f: any) => void;
    readonly wasm_bindgen_edac182d00970383___convert__closures_____invoke___wasm_bindgen_edac182d00970383___JsValue__core_7d5f0a2ba6a62c33___result__Result_____wasm_bindgen_edac182d00970383___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_edac182d00970383___convert__closures_____invoke___js_sys_a815e504dd20ad1a___Function__js_sys_a815e504dd20ad1a___Function______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_edac182d00970383___convert__closures_____invoke___bool__true_: (a: number, b: number) => number;
    readonly wasm_bindgen_edac182d00970383___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly wasm_bindgen_edac182d00970383___convert__closures_____invoke_______true__1_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
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
