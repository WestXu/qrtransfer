/* tslint:disable */
/* eslint-disable */
/**
*/
export function my_set_panic_hook(): void;
/**
* @param {string} file_name
* @param {any} int_array
*/
export function send(file_name: string, int_array: any): void;
/**
*/
export class Decoder {
  free(): void;
/**
* @returns {Decoder}
*/
  static new(): Decoder;
/**
* @param {string} chunk
* @returns {boolean}
*/
  process_chunk(chunk: string): boolean;
/**
* @returns {string}
*/
  to_base64(): string;
/**
* @returns {string}
*/
  get_name(): string;
/**
* @returns {boolean}
*/
  is_finished(): boolean;
/**
* @returns {string}
*/
  get_progress(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly my_set_panic_hook: () => void;
  readonly send: (a: number, b: number, c: number) => void;
  readonly __wbg_decoder_free: (a: number) => void;
  readonly decoder_new: () => number;
  readonly decoder_process_chunk: (a: number, b: number, c: number) => number;
  readonly decoder_to_base64: (a: number, b: number) => void;
  readonly decoder_get_name: (a: number, b: number) => void;
  readonly decoder_is_finished: (a: number) => number;
  readonly decoder_get_progress: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
