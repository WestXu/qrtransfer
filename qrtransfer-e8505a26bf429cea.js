import { Interpreter } from './snippets/dioxus-interpreter-js-459fb15b86d869f7/src/interpreter.js';

let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_24(arg0, arg1) {
    wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2a4b99979e898253(arg0, arg1);
}

function __wbg_adapter_27(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h52afe8b23cece680(arg0, arg1, addHeapObject(arg2));
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

let stack_pointer = 128;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_30(arg0, arg1, arg2) {
    try {
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc6331c357a202396(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_33(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hfc61b691b3e81768(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_36(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h55096d1d937f1548(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_39(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8831694b0426b094(arg0, arg1);
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
/**
*/
export class Decoder {

    static __wrap(ptr) {
        const obj = Object.create(Decoder.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_decoder_free(ptr);
    }
    /**
    * @returns {Decoder}
    */
    static new() {
        const ret = wasm.decoder_new();
        return Decoder.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    get_progress() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.decoder_get_progress(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getCachedStringFromWasm0(r0, r1);
        if (r0 !== 0) { wasm.__wbindgen_free(r0, r1); }
        return v0;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
/**
* @returns {boolean}
*/
is_finished() {
    const ret = wasm.decoder_is_finished(this.ptr);
    return ret !== 0;
}
/**
* @param {string} chunk
* @returns {boolean}
*/
process_chunk(chunk) {
    const ptr0 = passStringToWasm0(chunk, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.decoder_process_chunk(this.ptr, ptr0, len0);
    return ret !== 0;
}
/**
* @param {number} width
* @param {number} height
* @param {Uint8Array} data
* @returns {number}
*/
scan(width, height, data) {
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.decoder_scan(this.ptr, width, height, ptr0, len0);
    return ret >>> 0;
}
/**
* @returns {Finished}
*/
get_finished() {
    const ptr = this.__destroy_into_raw();
    const ret = wasm.decoder_get_finished(ptr);
    return Finished.__wrap(ret);
}
}
/**
*/
export class Finished {

    static __wrap(ptr) {
        const obj = Object.create(Finished.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_finished_free(ptr);
    }
    /**
    * @returns {string}
    */
    to_base64() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.finished_to_base64(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getCachedStringFromWasm0(r0, r1);
        if (r0 !== 0) { wasm.__wbindgen_free(r0, r1); }
        return v0;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
/**
* @returns {string}
*/
get_name() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.finished_get_name(retptr, this.ptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v0 = getCachedStringFromWasm0(r0, r1);
    if (r0 !== 0) { wasm.__wbindgen_free(r0, r1); }
    return v0;
} finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
}
}
}
/**
*/
export class QrTransfer {

    static __wrap(ptr) {
        const obj = Object.create(QrTransfer.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_qrtransfer_free(ptr);
    }
    /**
    * @returns {Decoder}
    */
    new_decoder() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.qrtransfer_new_decoder(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Decoder.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_qrtransfer_new = function(arg0) {
        const ret = QrTransfer.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
    if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1); }
    console.error(v0);
};
imports.wbg.__wbindgen_number_new = function(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
};
imports.wbg.__wbg_clearTimeout_76877dbc010e786d = function(arg0) {
    const ret = clearTimeout(takeObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_setTimeout_75cb9b6991a4031d = function() { return handleError(function (arg0, arg1) {
    const ret = setTimeout(getObject(arg0), arg1);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_new_67f219e1431f9bfb = function(arg0) {
    const ret = new Interpreter(takeObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_SetNode_e348a2373658c0e5 = function(arg0, arg1, arg2) {
    getObject(arg0).SetNode(arg1 >>> 0, takeObject(arg2));
};
imports.wbg.__wbg_PushRoot_d77bf2f07d78ee04 = function(arg0, arg1) {
    getObject(arg0).PushRoot(BigInt.asUintN(64, arg1));
};
imports.wbg.__wbg_PopRoot_e5d3ec5dea882a1d = function(arg0) {
    getObject(arg0).PopRoot();
};
imports.wbg.__wbg_AppendChildren_6d62bb4fe693e279 = function(arg0, arg1) {
    getObject(arg0).AppendChildren(arg1 >>> 0);
};
imports.wbg.__wbg_ReplaceWith_8ebcac8be1fb1ca8 = function(arg0, arg1, arg2) {
    getObject(arg0).ReplaceWith(BigInt.asUintN(64, arg1), arg2 >>> 0);
};
imports.wbg.__wbg_InsertAfter_f22ec9a3aadcd11d = function(arg0, arg1, arg2) {
    getObject(arg0).InsertAfter(BigInt.asUintN(64, arg1), arg2 >>> 0);
};
imports.wbg.__wbg_InsertBefore_c5a8dca846bdfe92 = function(arg0, arg1, arg2) {
    getObject(arg0).InsertBefore(BigInt.asUintN(64, arg1), arg2 >>> 0);
};
imports.wbg.__wbg_Remove_49f795846a239f69 = function(arg0, arg1) {
    getObject(arg0).Remove(BigInt.asUintN(64, arg1));
};
imports.wbg.__wbg_CreateTextNode_96506678bbad22ec = function(arg0, arg1, arg2) {
    getObject(arg0).CreateTextNode(takeObject(arg1), BigInt.asUintN(64, arg2));
};
imports.wbg.__wbg_CreateElement_3def0fedd162d196 = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).CreateElement(v0, BigInt.asUintN(64, arg3));
};
imports.wbg.__wbg_CreateElementNs_8bf15ccda6dbcc6d = function(arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg4, arg5);
    getObject(arg0).CreateElementNs(v0, BigInt.asUintN(64, arg3), v1);
};
imports.wbg.__wbg_CreatePlaceholder_0d13c892ea9189a7 = function(arg0, arg1) {
    getObject(arg0).CreatePlaceholder(BigInt.asUintN(64, arg1));
};
imports.wbg.__wbg_NewEventListener_1191f0ffe7726998 = function(arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).NewEventListener(v0, BigInt.asUintN(64, arg3), getObject(arg4));
};
imports.wbg.__wbg_RemoveEventListener_b89963320b7e7fec = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    getObject(arg0).RemoveEventListener(BigInt.asUintN(64, arg1), v0);
};
imports.wbg.__wbg_SetText_805dc255ca48ee91 = function(arg0, arg1, arg2) {
    getObject(arg0).SetText(BigInt.asUintN(64, arg1), takeObject(arg2));
};
imports.wbg.__wbg_SetAttribute_eb74ba21b8a1196f = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var v1 = getCachedStringFromWasm0(arg5, arg6);
    getObject(arg0).SetAttribute(BigInt.asUintN(64, arg1), v0, takeObject(arg4), v1);
};
imports.wbg.__wbg_RemoveAttribute_ff4184d97eafc3f8 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var v1 = getCachedStringFromWasm0(arg4, arg5);
    getObject(arg0).RemoveAttribute(BigInt.asUintN(64, arg1), v0, v1);
};
imports.wbg.__wbg_instanceof_Window_e266f02eee43b570 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Window;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_document_950215a728589a2d = function(arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_innerHeight_3ef25a30618357e0 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).innerHeight;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_scrollY_e70715f189810ad0 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).scrollY;
    return ret;
}, arguments) };
imports.wbg.__wbg_requestAnimationFrame_afe426b568f84138 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
    return ret;
}, arguments) };
imports.wbg.__wbg_requestIdleCallback_93900f7911ff34fa = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).requestIdleCallback(getObject(arg1));
    return ret;
}, arguments) };
imports.wbg.__wbg_scrollBy_da0a667565d9f07c = function(arg0, arg1) {
    getObject(arg0).scrollBy(getObject(arg1));
};
imports.wbg.__wbg_scrollTo_881c41fdbf5d1cdd = function(arg0, arg1, arg2) {
    getObject(arg0).scrollTo(arg1, arg2);
};
imports.wbg.__wbg_clearInterval_3f324662ce039e9a = function(arg0, arg1) {
    getObject(arg0).clearInterval(arg1);
};
imports.wbg.__wbg_setInterval_9df1d5cc0a15b20b = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).setInterval(getObject(arg1), arg2);
    return ret;
}, arguments) };
imports.wbg.__wbg_body_be46234bb33edd63 = function(arg0) {
    const ret = getObject(arg0).body;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_createElement_e2a0e21263eb5416 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createElement(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_getElementById_eb93a47327bb5585 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).getElementById(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_CompositionEvent_57e2f1c0e7330ddc = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof CompositionEvent;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_data_70b505c651722930 = function(arg0, arg1) {
    const ret = getObject(arg1).data;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_get_460ba3644fab1c42 = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_IdleDeadline_8b362801b25ca450 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof IdleDeadline;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_timeRemaining_312e0ca1b3ff923d = function(arg0) {
    const ret = getObject(arg0).timeRemaining();
    return ret;
};
imports.wbg.__wbg_pointerId_d2caae4465ba386f = function(arg0) {
    const ret = getObject(arg0).pointerId;
    return ret;
};
imports.wbg.__wbg_width_a1d5efe9db3fb17a = function(arg0) {
    const ret = getObject(arg0).width;
    return ret;
};
imports.wbg.__wbg_height_7a964b61a7a42a7d = function(arg0) {
    const ret = getObject(arg0).height;
    return ret;
};
imports.wbg.__wbg_pressure_352c13794490720b = function(arg0) {
    const ret = getObject(arg0).pressure;
    return ret;
};
imports.wbg.__wbg_tangentialPressure_1dfc978ca995fe64 = function(arg0) {
    const ret = getObject(arg0).tangentialPressure;
    return ret;
};
imports.wbg.__wbg_tiltX_5e065070f9907bab = function(arg0) {
    const ret = getObject(arg0).tiltX;
    return ret;
};
imports.wbg.__wbg_tiltY_88a95794fa24061b = function(arg0) {
    const ret = getObject(arg0).tiltY;
    return ret;
};
imports.wbg.__wbg_twist_1ac0d3bf085b9fa5 = function(arg0) {
    const ret = getObject(arg0).twist;
    return ret;
};
imports.wbg.__wbg_pointerType_df759fa0bd6634ed = function(arg0, arg1) {
    const ret = getObject(arg1).pointerType;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_isPrimary_31079f1bab7f6665 = function(arg0) {
    const ret = getObject(arg0).isPrimary;
    return ret;
};
imports.wbg.__wbg_deltaX_b7d127c94d6265c0 = function(arg0) {
    const ret = getObject(arg0).deltaX;
    return ret;
};
imports.wbg.__wbg_deltaY_b32fa858e16edcc0 = function(arg0) {
    const ret = getObject(arg0).deltaY;
    return ret;
};
imports.wbg.__wbg_deltaZ_f74840dd94a2e4c0 = function(arg0) {
    const ret = getObject(arg0).deltaZ;
    return ret;
};
imports.wbg.__wbg_deltaMode_11f7b19e64d9a515 = function(arg0) {
    const ret = getObject(arg0).deltaMode;
    return ret;
};
imports.wbg.__wbg_result_4c6690478b5532e4 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).result;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_setonloadend_664a6399dd863d96 = function(arg0, arg1) {
    getObject(arg0).onloadend = getObject(arg1);
};
imports.wbg.__wbg_new_8eef8a8754c6aae7 = function() { return handleError(function () {
    const ret = new FileReader();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_readAsArrayBuffer_bc9f4aff6d3e1bb1 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).readAsArrayBuffer(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_screenX_efa7b61dc7fa2efd = function(arg0) {
    const ret = getObject(arg0).screenX;
    return ret;
};
imports.wbg.__wbg_screenY_c5e6449919709a69 = function(arg0) {
    const ret = getObject(arg0).screenY;
    return ret;
};
imports.wbg.__wbg_clientX_35f23f953e04ec0e = function(arg0) {
    const ret = getObject(arg0).clientX;
    return ret;
};
imports.wbg.__wbg_clientY_8104e462abc0b3ec = function(arg0) {
    const ret = getObject(arg0).clientY;
    return ret;
};
imports.wbg.__wbg_ctrlKey_e1b8f1de1eb24d5d = function(arg0) {
    const ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_fdd99b6df96e25c5 = function(arg0) {
    const ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_altKey_d531a4d3704557cb = function(arg0) {
    const ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_metaKey_934772989e28020c = function(arg0) {
    const ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_button_a1c470d5e4c997f2 = function(arg0) {
    const ret = getObject(arg0).button;
    return ret;
};
imports.wbg.__wbg_buttons_42a7b7de33d8e572 = function(arg0) {
    const ret = getObject(arg0).buttons;
    return ret;
};
imports.wbg.__wbg_propertyName_e41c6239ffb77f2a = function(arg0, arg1) {
    const ret = getObject(arg1).propertyName;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_elapsedTime_6bbd8686761d767f = function(arg0) {
    const ret = getObject(arg0).elapsedTime;
    return ret;
};
imports.wbg.__wbg_pseudoElement_589db5e026c64890 = function(arg0, arg1) {
    const ret = getObject(arg1).pseudoElement;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_animationName_8b08b6b55e2871c3 = function(arg0, arg1) {
    const ret = getObject(arg1).animationName;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_elapsedTime_b9b2848f0ce69c4d = function(arg0) {
    const ret = getObject(arg0).elapsedTime;
    return ret;
};
imports.wbg.__wbg_pseudoElement_489395cbb841b9a8 = function(arg0, arg1) {
    const ret = getObject(arg1).pseudoElement;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_length_5ce97df09b61b986 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_item_2ab86c1e3cb70ed3 = function(arg0, arg1) {
    const ret = getObject(arg0).item(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_HtmlSelectElement_d22585b1943c6b08 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLSelectElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_value_1b15c45090422f7f = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_charCode_504e79c3e550d1bb = function(arg0) {
    const ret = getObject(arg0).charCode;
    return ret;
};
imports.wbg.__wbg_keyCode_b33194be2ceec53b = function(arg0) {
    const ret = getObject(arg0).keyCode;
    return ret;
};
imports.wbg.__wbg_altKey_dff2a075455ac01b = function(arg0) {
    const ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_ctrlKey_993b558f853d64ce = function(arg0) {
    const ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_31e62e9d172b26f0 = function(arg0) {
    const ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_metaKey_9f0f19692d0498bd = function(arg0) {
    const ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_location_e0f6bf3e5e23180a = function(arg0) {
    const ret = getObject(arg0).location;
    return ret;
};
imports.wbg.__wbg_repeat_7ef48474c78e74bd = function(arg0) {
    const ret = getObject(arg0).repeat;
    return ret;
};
imports.wbg.__wbg_key_f0decac219aa904b = function(arg0, arg1) {
    const ret = getObject(arg1).key;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_altKey_088afaf4c4a6fe7c = function(arg0) {
    const ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_metaKey_69f3ca388ab2c6b2 = function(arg0) {
    const ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_ctrlKey_011662ff21669501 = function(arg0) {
    const ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_5472f6ec6b0da088 = function(arg0) {
    const ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_name_ccf3024ae4e3ac54 = function(arg0, arg1) {
    const ret = getObject(arg1).name;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_Node_abf5312af68f179e = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Node;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_parentElement_0e8c9afce5cb9d6e = function(arg0) {
    const ret = getObject(arg0).parentElement;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_childNodes_40585508577e98df = function(arg0) {
    const ret = getObject(arg0).childNodes;
    return addHeapObject(ret);
};
imports.wbg.__wbg_textContent_dff59ad5e030bb86 = function(arg0, arg1) {
    const ret = getObject(arg1).textContent;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_settextContent_19dc6a6146112f16 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).textContent = v0;
};
imports.wbg.__wbg_instanceof_Text_1a0eedfe37c49263 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Text;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_type_1dff9d19be2750ce = function(arg0, arg1) {
    const ret = getObject(arg1).type;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_target_b629c177f9bee3da = function(arg0) {
    const ret = getObject(arg0).target;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_preventDefault_16b2170b12f56317 = function(arg0) {
    getObject(arg0).preventDefault();
};
imports.wbg.__wbg_pageX_fa6e927410d8f9a8 = function(arg0) {
    const ret = getObject(arg0).pageX;
    return ret;
};
imports.wbg.__wbg_pageY_fcdd161e44399498 = function(arg0) {
    const ret = getObject(arg0).pageY;
    return ret;
};
imports.wbg.__wbg_which_0dd05aa408002d08 = function(arg0) {
    const ret = getObject(arg0).which;
    return ret;
};
imports.wbg.__wbg_instanceof_Element_cb847a3fc7b1b1a4 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Element;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_setinnerHTML_76167cda24d9b96b = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).innerHTML = v0;
};
imports.wbg.__wbg_getAttribute_2c20e00a5cd314af = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = getObject(arg1).getAttribute(v0);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbg_setAttribute_79c9562d32d05e66 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setAttribute(v0, v1);
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlElement_9e442d53bb553421 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_scrollHeight_8f2c60d7490e1346 = function(arg0) {
    const ret = getObject(arg0).scrollHeight;
    return ret;
};
imports.wbg.__wbg_log_7bb108d119bafbc1 = function(arg0) {
    console.log(getObject(arg0));
};
imports.wbg.__wbg_instanceof_HtmlFormElement_04e7484e36bd99d6 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLFormElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_elements_ce4445b6cda59af6 = function(arg0) {
    const ret = getObject(arg0).elements;
    return addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_HtmlTextAreaElement_4bc39f9d861a6832 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLTextAreaElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_value_00fb0fdc46959169 = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_get_8187c9b59091f3ad = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_HtmlInputElement_5c9d54338207f061 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLInputElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_checked_44c09d0c819e33ad = function(arg0) {
    const ret = getObject(arg0).checked;
    return ret;
};
imports.wbg.__wbg_files_e7db01553b30ef33 = function(arg0) {
    const ret = getObject(arg0).files;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_type_6bd11acdabd07813 = function(arg0, arg1) {
    const ret = getObject(arg1).type;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_value_1f2c9e357d18d3ea = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_newnoargs_2b8b6bd7753c76ba = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Function(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_call_95d1ea488d03e4e8 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_new_f9876326328f45ed = function() {
    const ret = new Object();
    return addHeapObject(ret);
};
imports.wbg.__wbg_self_e7c1f827057f6584 = function() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_window_a09ec664e14b1b81 = function() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_globalThis_87cbb8506fecf3a9 = function() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_global_c85a9259e621f3db = function() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};
imports.wbg.__wbg_instanceof_Object_f5a826c4da0d4a94 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Object;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_hasOwnProperty_ea09f8ec7275047a = function(arg0, arg1) {
    const ret = getObject(arg0).hasOwnProperty(getObject(arg1));
    return ret;
};
imports.wbg.__wbg_resolve_fd40f858d9db1a04 = function(arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_ec5db6d509eb475f = function(arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_buffer_cf65c07de34b9a08 = function(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};
imports.wbg.__wbg_new_537b7341ce90bb31 = function(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_set_17499e8aa4003ebd = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};
imports.wbg.__wbg_length_27a2afe8ab42b09f = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_set_6aa458a4ebdb65cb = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    return ret;
}, arguments) };
imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper291 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 60, __wbg_adapter_24);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper293 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 60, __wbg_adapter_27);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1379 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 216, __wbg_adapter_30);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1381 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 216, __wbg_adapter_33);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1540 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 314, __wbg_adapter_36);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1582 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 334, __wbg_adapter_39);
    return addHeapObject(ret);
};

return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedFloat64Memory0 = null;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('qrtransfer-e8505a26bf429cea_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
