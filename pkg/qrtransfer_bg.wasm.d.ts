/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_decoder_free(a: number): void;
export function decoder_new(): number;
export function decoder_process_chunk(a: number, b: number, c: number): number;
export function decoder_to_base64(a: number, b: number): void;
export function decoder_get_name(a: number, b: number): void;
export function decoder_is_finished(a: number): number;
export function decoder_get_progress(a: number, b: number): void;
export function decoder_scan(a: number, b: number, c: number, d: number, e: number): number;
export function my_set_panic_hook(): void;
export function send(a: number, b: number, c: number): void;
export function BrotliEncoderCreateInstance(a: number, b: number, c: number): number;
export function BrotliEncoderSetParameter(a: number, b: number, c: number): number;
export function BrotliEncoderDestroyInstance(a: number): void;
export function BrotliEncoderIsFinished(a: number): number;
export function BrotliEncoderHasMoreOutput(a: number): number;
export function BrotliEncoderSetCustomDictionary(a: number, b: number, c: number): void;
export function BrotliEncoderTakeOutput(a: number, b: number): number;
export function BrotliEncoderVersion(): number;
export function BrotliEncoderMaxCompressedSize(a: number): number;
export function BrotliEncoderCompress(a: number, b: number, c: number, d: number, e: number, f: number, g: number): number;
export function BrotliEncoderCompressStreaming(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function BrotliEncoderCompressStream(a: number, b: number, c: number, d: number, e: number, f: number, g: number): number;
export function BrotliEncoderMallocU8(a: number, b: number): number;
export function BrotliEncoderFreeU8(a: number, b: number, c: number): void;
export function BrotliEncoderMallocUsize(a: number, b: number): number;
export function BrotliEncoderFreeUsize(a: number, b: number, c: number): void;
export function BrotliEncoderCompressMulti(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number): number;
export function BrotliDecoderCreateInstance(a: number, b: number, c: number): number;
export function BrotliDecoderSetParameter(a: number, b: number, c: number): void;
export function BrotliDecoderDecompress(a: number, b: number, c: number, d: number): number;
export function BrotliDecoderDecompressStream(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function BrotliDecoderDecompressStreaming(a: number, b: number, c: number, d: number, e: number): number;
export function BrotliDecoderDecompressWithReturnInfo(a: number, b: number, c: number, d: number, e: number): void;
export function BrotliDecoderDecompressPrealloc(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number): void;
export function BrotliDecoderMallocU8(a: number, b: number): number;
export function BrotliDecoderFreeU8(a: number, b: number, c: number): void;
export function BrotliDecoderMallocUsize(a: number, b: number): number;
export function BrotliDecoderFreeUsize(a: number, b: number, c: number): void;
export function BrotliDecoderDestroyInstance(a: number): void;
export function BrotliDecoderVersion(): number;
export function CBrotliDecoderErrorString(a: number): number;
export function BrotliDecoderErrorString(a: number): number;
export function CBrotliDecoderHasMoreOutput(a: number): number;
export function BrotliDecoderHasMoreOutput(a: number): number;
export function CBrotliDecoderTakeOutput(a: number, b: number): number;
export function BrotliDecoderTakeOutput(a: number, b: number): number;
export function CBrotliDecoderIsUsed(a: number): number;
export function BrotliDecoderIsUsed(a: number): number;
export function CBrotliDecoderIsFinished(a: number): number;
export function BrotliDecoderIsFinished(a: number): number;
export function CBrotliDecoderGetErrorCode(a: number): number;
export function BrotliDecoderGetErrorCode(a: number): number;
export function CBrotliDecoderGetErrorString(a: number): number;
export function BrotliDecoderGetErrorString(a: number): number;
export function BroccoliCreateInstance(a: number): void;
export function BroccoliCreateInstanceWithWindowSize(a: number, b: number): void;
export function BroccoliDestroyInstance(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number, {: number, |: number, }: number, ~: number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number, : number,  : number, ¡: number, ¢: number, £: number, ¤: number, ¥: number, ¦: number, §: number, ¨: number, ©: number, ª: number, «: number, ¬: number, ­: number, ®: number, ¯: number, °: number, ±: number, ²: number, ³: number, ´: number, µ: number, ¶: number, ·: number, ¸: number, ¹: number, º: number, »: number, ¼: number, ½: number, ¾: number, ¿: number, À: number, Á: number, Â: number, Ã: number, Ä: number, Å: number, Æ: number, Ç: number, È: number, É: number, Ê: number, Ë: number, Ì: number, Í: number, Î: number, Ï: number, Ð: number, Ñ: number, Ò: number, Ó: number, Ô: number, Õ: number, Ö: number, ×: number, Ø: number, Ù: number): void;
export function BroccoliNewBrotliFile(a: number): void;
export function BroccoliConcatStream(a: number, b: number, c: number, d: number, e: number): number;
export function BroccoliConcatStreaming(a: number, b: number, c: number, d: number, e: number): number;
export function BroccoliConcatFinish(a: number, b: number, c: number): number;
export function BroccoliConcatFinished(a: number, b: number, c: number): number;
export function BrotliEncoderMaxCompressedSizeMulti(a: number, b: number): number;
export function BrotliEncoderCreateWorkPool(a: number, b: number, c: number, d: number): number;
export function BrotliEncoderDestroyWorkPool(a: number): void;
export function BrotliEncoderCompressWorkPool(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number): number;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number): void;
export function __wbindgen_exn_store(a: number): void;
