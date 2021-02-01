import * as wasm from './wasm_chip8_bg.wasm';

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
*/
export function greet() {
    wasm.greet();
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

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

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    };
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*/
export class WasmChip8 {

    static __wrap(ptr) {
        const obj = Object.create(WasmChip8.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_wasmchip8_free(ptr);
    }
    /**
    * @returns {WasmChip8}
    */
    static new() {
        var ret = wasm.wasmchip8_new();
        return WasmChip8.__wrap(ret);
    }
    /**
    * @returns {boolean}
    */
    cycle() {
        var ret = wasm.wasmchip8_cycle(this.ptr);
        return ret !== 0;
    }
    /**
    */
    reset() {
        wasm.wasmchip8_reset(this.ptr);
    }
    /**
    * @param {number} key
    */
    key_down(key) {
        wasm.wasmchip8_key_down(this.ptr, key);
    }
    /**
    * @param {number} key
    */
    key_up(key) {
        wasm.wasmchip8_key_up(this.ptr, key);
    }
    /**
    * @returns {number}
    */
    get_display() {
        var ret = wasm.wasmchip8_get_display(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    get_memory() {
        var ret = wasm.wasmchip8_get_memory(this.ptr);
        return ret;
    }
    /**
    * @param {DataView} data
    */
    load_rom_js(data) {
        wasm.wasmchip8_load_rom_js(this.ptr, addHeapObject(data));
    }
    /**
    * @returns {number}
    */
    pc() {
        var ret = wasm.wasmchip8_pc(this.ptr);
        return ret;
    }
    /**
    * @returns {Array<any>}
    */
    get_registers() {
        var ret = wasm.wasmchip8_get_registers(this.ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    get_other_state() {
        var ret = wasm.wasmchip8_get_other_state(this.ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    disassemble_memory() {
        var ret = wasm.wasmchip8_disassemble_memory(this.ptr);
        return takeObject(ret);
    }
}

export const __wbg_alert_eee8945cd3884570 = function(arg0, arg1) {
    alert(getStringFromWasm0(arg0, arg1));
};

export const __wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

export const __wbg_new_59cb74e423758ede = function() {
    var ret = new Error();
    return addHeapObject(ret);
};

export const __wbg_stack_558ba5917b466edd = function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

export const __wbg_error_9783be44659339ea = function(arg0) {
    console.error(getObject(arg0));
};

export const __wbg_log_206ee8d1389cff8e = function(arg0) {
    console.log(...getObject(arg0));
};

export const __wbg_self_86b4b13392c7af56 = handleError(function() {
    var ret = self.self;
    return addHeapObject(ret);
});

export const __wbg_require_f5521a5b85ad2542 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
};

export const __wbg_crypto_b8c92eaac23d0d80 = function(arg0) {
    var ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};

export const __wbg_msCrypto_9ad6677321a08dd8 = function(arg0) {
    var ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
};

export const __wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};

export const __wbg_getRandomValues_dd27e6b0652b3236 = function(arg0) {
    var ret = getObject(arg0).getRandomValues;
    return addHeapObject(ret);
};

export const __wbg_getRandomValues_e57c9b75ddead065 = function(arg0, arg1) {
    getObject(arg0).getRandomValues(getObject(arg1));
};

export const __wbg_randomFillSync_d2ba53160aec6aba = function(arg0, arg1, arg2) {
    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
};

export const __wbg_static_accessor_MODULE_452b4680e8614c81 = function() {
    var ret = module;
    return addHeapObject(ret);
};

export const __wbg_new_1abc33d4f9ba3e80 = function() {
    var ret = new Array();
    return addHeapObject(ret);
};

export const __wbg_push_44968dcdf4cfbb43 = function(arg0, arg1) {
    var ret = getObject(arg0).push(getObject(arg1));
    return ret;
};

export const __wbg_byteLength_80b4452535587127 = function(arg0) {
    var ret = getObject(arg0).byteLength;
    return ret;
};

export const __wbg_getUint8_266eb0cdb9083210 = function(arg0, arg1) {
    var ret = getObject(arg0).getUint8(arg1 >>> 0);
    return ret;
};

export const __wbg_buffer_bc64154385c04ac4 = function(arg0) {
    var ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};

export const __wbg_new_22a33711cf65b661 = function(arg0) {
    var ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};

export const __wbg_set_b29de3f25280c6ec = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};

export const __wbg_length_e9f6f145de2fede5 = function(arg0) {
    var ret = getObject(arg0).length;
    return ret;
};

export const __wbg_newwithlength_48451d71403bfede = function(arg0) {
    var ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
};

export const __wbg_subarray_6b2dd31c84ee881f = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export const __wbindgen_memory = function() {
    var ret = wasm.memory;
    return addHeapObject(ret);
};

