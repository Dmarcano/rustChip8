let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_3.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
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
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

export function greet() {
    wasm.greet();
}

const WasmChip8Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmchip8_free(ptr >>> 0, 1));

export class WasmChip8 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmChip8.prototype);
        obj.__wbg_ptr = ptr;
        WasmChip8Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmChip8Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmchip8_free(ptr, 0);
    }
    /**
     * @returns {WasmChip8}
     */
    static new() {
        const ret = wasm.wasmchip8_new();
        return WasmChip8.__wrap(ret);
    }
    /**
     * @returns {boolean}
     */
    cycle() {
        const ret = wasm.wasmchip8_cycle(this.__wbg_ptr);
        return ret !== 0;
    }
    reset() {
        wasm.wasmchip8_reset(this.__wbg_ptr);
    }
    /**
     * @param {number} key
     */
    key_down(key) {
        wasm.wasmchip8_key_down(this.__wbg_ptr, key);
    }
    /**
     * @param {number} key
     */
    key_up(key) {
        wasm.wasmchip8_key_up(this.__wbg_ptr, key);
    }
    /**
     * @returns {number}
     */
    get_display() {
        const ret = wasm.wasmchip8_get_display(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get_memory() {
        const ret = wasm.wasmchip8_get_memory(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {DataView} data
     */
    load_rom_js(data) {
        wasm.wasmchip8_load_rom_js(this.__wbg_ptr, data);
    }
    /**
     * @returns {number}
     */
    pc() {
        const ret = wasm.wasmchip8_pc(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get_registers() {
        const ret = wasm.wasmchip8_get_registers(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get_other_state() {
        const ret = wasm.wasmchip8_get_other_state(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    disassemble_memory() {
        const ret = wasm.wasmchip8_disassemble_memory(this.__wbg_ptr);
        return ret;
    }
}

export function __wbg_alert_a0d36ba65789b052(arg0, arg1) {
    alert(getStringFromWasm0(arg0, arg1));
};

export function __wbg_buffer_609cc3eee51ed158(arg0) {
    const ret = arg0.buffer;
    return ret;
};

export function __wbg_byteLength_1bdb96d98ab0d871(arg0) {
    const ret = arg0.byteLength;
    return ret;
};

export function __wbg_crypto_038798f665f985e2(arg0) {
    const ret = arg0.crypto;
    return ret;
};

export function __wbg_error_524f506f44df1645(arg0) {
    console.error(arg0);
};

export function __wbg_error_7534b8e9a36f1ab4(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

export function __wbg_getRandomValues_371e7ade8bd92088(arg0, arg1) {
    arg0.getRandomValues(arg1);
};

export function __wbg_getRandomValues_7dfe5bd1b67c9ca1(arg0) {
    const ret = arg0.getRandomValues;
    return ret;
};

export function __wbg_getUint8_749a77380c219f58(arg0, arg1) {
    const ret = arg0.getUint8(arg1 >>> 0);
    return ret;
};

export function __wbg_length_a446193dc22c12f8(arg0) {
    const ret = arg0.length;
    return ret;
};

export function __wbg_log_245868b4b99cdf20(arg0) {
    console.log(...arg0);
};

export function __wbg_msCrypto_ff35fce085fab2a3(arg0) {
    const ret = arg0.msCrypto;
    return ret;
};

export function __wbg_new_78feb108b6472713() {
    const ret = new Array();
    return ret;
};

export function __wbg_new_8a6f238a6ece86ea() {
    const ret = new Error();
    return ret;
};

export function __wbg_new_a12002a7f91c75be(arg0) {
    const ret = new Uint8Array(arg0);
    return ret;
};

export function __wbg_newwithlength_a381634e90c276d4(arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return ret;
};

export function __wbg_push_737cfc8c1432c2c6(arg0, arg1) {
    const ret = arg0.push(arg1);
    return ret;
};

export function __wbg_randomFillSync_994ac6d9ade7a695(arg0, arg1, arg2) {
    arg0.randomFillSync(getArrayU8FromWasm0(arg1, arg2));
};

export function __wbg_require_0d6aeaec3c042c88(arg0, arg1, arg2) {
    const ret = arg0.require(getStringFromWasm0(arg1, arg2));
    return ret;
};

export function __wbg_self_25aabeb5a7b41685() { return handleError(function () {
    const ret = self.self;
    return ret;
}, arguments) };

export function __wbg_set_65595bdd868b3009(arg0, arg1, arg2) {
    arg0.set(arg1, arg2 >>> 0);
};

export function __wbg_stack_0ed75d68575b0f3c(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_static_accessor_MODULE_ef3aa2eb251158a5() {
    const ret = module;
    return ret;
};

export function __wbg_subarray_aa9065fa9dc5df96(arg0, arg1, arg2) {
    const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_3;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

export function __wbindgen_is_undefined(arg0) {
    const ret = arg0 === undefined;
    return ret;
};

export function __wbindgen_memory() {
    const ret = wasm.memory;
    return ret;
};

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

