let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


function getFromExternrefTable0(idx) { return wasm.__wbindgen_export_0.get(idx); }

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getFromExternrefTable0(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_0.set(idx, obj);
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

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

const cachedTextEncoder = new lTextEncoder('utf-8');

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
    if (builtInMatches && builtInMatches.length > 1) {
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

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_0.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(
state => {
    wasm.__wbindgen_export_7.get(state.dtor)(state.a, state.b);
}
);

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
                wasm.__wbindgen_export_7.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

let cachedUint16ArrayMemory0 = null;

function getUint16ArrayMemory0() {
    if (cachedUint16ArrayMemory0 === null || cachedUint16ArrayMemory0.byteLength === 0) {
        cachedUint16ArrayMemory0 = new Uint16Array(wasm.memory.buffer);
    }
    return cachedUint16ArrayMemory0;
}

function getArrayU16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
}
function __wbg_adapter_6(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__hd0670797aa4f822b(arg0, arg1);
}

function __wbg_adapter_13(arg0, arg1, arg2) {
    wasm.closure458_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_1120(arg0, arg1, arg2, arg3) {
    wasm.closure479_externref_shim(arg0, arg1, arg2, arg3);
}

/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10}
 */
export const LibraryType = Object.freeze({
    AntibodyCapture: 0, "0": "AntibodyCapture",
    AntigenCapture: 1, "1": "AntigenCapture",
    ChromatinAccessibility: 2, "2": "ChromatinAccessibility",
    CrisprGuideCapture: 3, "3": "CrisprGuideCapture",
    Custom: 4, "4": "Custom",
    GeneExpression: 5, "5": "GeneExpression",
    MultiplexingCapture: 6, "6": "MultiplexingCapture",
    Vdj: 7, "7": "Vdj",
    VdjB: 8, "8": "VdjB",
    VdjT: 9, "9": "VdjT",
    VdjTGd: 10, "10": "VdjTGd",
});
/**
 * @enum {0 | 1 | 2 | 3 | 4}
 */
export const SampleMultiplexing = Object.freeze({
    Cellplex: 0, "0": "Cellplex",
    FlexBarcode: 1, "1": "FlexBarcode",
    Hashtag: 2, "2": "Hashtag",
    OnChipMultiplexing: 3, "3": "OnChipMultiplexing",
    Singleplex: 4, "4": "Singleplex",
});
/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8}
 */
export const Species = Object.freeze({
    AmbystomaMexicanum: 0, "0": "AmbystomaMexicanum",
    CanisFamiliaris: 1, "1": "CanisFamiliaris",
    CallithrixJacchus: 2, "2": "CallithrixJacchus",
    DrosophilaMelanogaster: 3, "3": "DrosophilaMelanogaster",
    GasterosteusAculeatus: 4, "4": "GasterosteusAculeatus",
    HomoSapiens: 5, "5": "HomoSapiens",
    MusMusculus: 6, "6": "MusMusculus",
    RattusNorvegicus: 7, "7": "RattusNorvegicus",
    SminthopsisCrassicaudata: 8, "8": "SminthopsisCrassicaudata",
});
/**
 * @enum {0 | 1 | 2}
 */
export const SpecimenType = Object.freeze({
    Block: 0, "0": "Block",
    Suspension: 1, "1": "Suspension",
    Tissue: 2, "2": "Tissue",
});
/**
 * @enum {0 | 1 | 2}
 */
export const UserRole = Object.freeze({
    AppAdmin: 0, "0": "AppAdmin",
    BiologyStaff: 1, "1": "BiologyStaff",
    ComputationalStaff: 2, "2": "ComputationalStaff",
});

const __wbindgen_enum_RequestCache = ["default", "no-store", "reload", "no-cache", "force-cache", "only-if-cached"];

const __wbindgen_enum_RequestCredentials = ["omit", "same-origin", "include"];

const __wbindgen_enum_RequestMode = ["same-origin", "no-cors", "cors", "navigate"];

const CdnaFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cdna_free(ptr >>> 0, 1));

export class Cdna {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Cdna.prototype);
        obj.__wbg_ptr = ptr;
        CdnaFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CdnaFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cdna_free(ptr, 0);
    }
    /**
     * @returns {CdnaSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_cdna_summary(this.__wbg_ptr);
        return CdnaSummary.__wrap(ret);
    }
    /**
     * @param {CdnaSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, CdnaSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cdna_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string[]}
     */
    get prepared_by() {
        const ret = wasm.__wbg_get_cdna_prepared_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set prepared_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdna_prepared_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {CdnaMeasurement[]}
     */
    get measurements() {
        const ret = wasm.__wbg_get_cdna_measurements(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {CdnaMeasurement[]} arg0
     */
    set measurements(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdna_measurements(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.cdna_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.cdna_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.cdna_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {Cdna}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdna_from_json_bytes(ptr0, len0);
        return Cdna.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {Cdna}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdna_from_json_string(ptr0, len0);
        return Cdna.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {Cdna}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdna_from_base64_json(ptr0, len0);
        return Cdna.__wrap(ret);
    }
}

const CdnaGemsErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cdnagemserror_free(ptr >>> 0, 1));

export class CdnaGemsError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CdnaGemsError.prototype);
        obj.__wbg_ptr = ptr;
        CdnaGemsErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CdnaGemsErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cdnagemserror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        const ret = wasm.__wbg_get_cdnagemserror_message(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.cdnagemserror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.cdnagemserror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.cdnagemserror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {CdnaGemsError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnagemserror_from_json_bytes(ptr0, len0);
        return CdnaGemsError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {CdnaGemsError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnagemserror_from_json_string(ptr0, len0);
        return CdnaGemsError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {CdnaGemsError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnagemserror_from_base64_json(ptr0, len0);
        return CdnaGemsError.__wrap(ret);
    }
}

const CdnaLibraryTypeErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cdnalibrarytypeerror_free(ptr >>> 0, 1));

export class CdnaLibraryTypeError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CdnaLibraryTypeError.prototype);
        obj.__wbg_ptr = ptr;
        CdnaLibraryTypeErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CdnaLibraryTypeErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cdnalibrarytypeerror_free(ptr, 0);
    }
    /**
     * @returns {LibraryTypeSpecification[]}
     */
    get expected_specifications() {
        const ret = wasm.__wbg_get_cdnalibrarytypeerror_expected_specifications(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {LibraryTypeSpecification[]} arg0
     */
    set expected_specifications(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnalibrarytypeerror_expected_specifications(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.cdnalibrarytypeerror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.cdnalibrarytypeerror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.cdnalibrarytypeerror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {CdnaLibraryTypeError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnalibrarytypeerror_from_json_bytes(ptr0, len0);
        return CdnaLibraryTypeError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {CdnaLibraryTypeError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnalibrarytypeerror_from_json_string(ptr0, len0);
        return CdnaLibraryTypeError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {CdnaLibraryTypeError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnalibrarytypeerror_from_base64_json(ptr0, len0);
        return CdnaLibraryTypeError.__wrap(ret);
    }
}

const CdnaMeasurementFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cdnameasurement_free(ptr >>> 0, 1));

export class CdnaMeasurement {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CdnaMeasurement.prototype);
        obj.__wbg_ptr = ptr;
        CdnaMeasurementFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof CdnaMeasurement)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CdnaMeasurementFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cdnameasurement_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_cdnameasurement_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnameasurement_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get cdna_id() {
        const ret = wasm.__wbg_get_cdnameasurement_cdna_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set cdna_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnameasurement_cdna_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get measured_by() {
        const ret = wasm.__wbg_get_cdnameasurement_measured_by(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set measured_by(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnameasurement_measured_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {ElectrophoreticMeasurementData}
     */
    get data() {
        const ret = wasm.__wbg_get_cdnameasurement_data(this.__wbg_ptr);
        return ElectrophoreticMeasurementData.__wrap(ret);
    }
    /**
     * @param {ElectrophoreticMeasurementData} arg0
     */
    set data(arg0) {
        _assertClass(arg0, ElectrophoreticMeasurementData);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cdnameasurement_data(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.cdnameasurement_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.cdnameasurement_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.cdnameasurement_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {CdnaMeasurement}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnameasurement_from_json_bytes(ptr0, len0);
        return CdnaMeasurement.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {CdnaMeasurement}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnameasurement_from_json_string(ptr0, len0);
        return CdnaMeasurement.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {CdnaMeasurement}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnameasurement_from_base64_json(ptr0, len0);
        return CdnaMeasurement.__wrap(ret);
    }
}

const CdnaQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cdnaquery_free(ptr >>> 0, 1));

export class CdnaQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CdnaQuery.prototype);
        obj.__wbg_ptr = ptr;
        CdnaQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CdnaQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cdnaquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_cdnaquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnaquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_cdnaquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnaquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_cdnaquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cdnaquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.cdnaquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.cdnaquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.cdnaquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {CdnaQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnaquery_from_json_bytes(ptr0, len0);
        return CdnaQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {CdnaQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnaquery_from_json_string(ptr0, len0);
        return CdnaQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {CdnaQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnaquery_from_base64_json(ptr0, len0);
        return CdnaQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.cdnaquery_new();
        this.__wbg_ptr = ret >>> 0;
        CdnaQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const CdnaSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cdnasummary_free(ptr >>> 0, 1));

export class CdnaSummary {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CdnaSummary.prototype);
        obj.__wbg_ptr = ptr;
        CdnaSummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CdnaSummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cdnasummary_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_cdnasummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnasummary_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_cdnasummary_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {LibraryType}
     */
    get library_type() {
        const ret = wasm.__wbg_get_cdnasummary_library_type(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {LibraryType} arg0
     */
    set library_type(arg0) {
        wasm.__wbg_set_cdnasummary_library_type(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get readable_id() {
        const ret = wasm.__wbg_get_cdnasummary_readable_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set readable_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnasummary_readable_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date}
     */
    get prepared_at() {
        const ret = wasm.__wbg_get_cdnasummary_prepared_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set prepared_at(arg0) {
        wasm.__wbg_set_cdnasummary_prepared_at(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get n_amplification_cycles() {
        const ret = wasm.__wbg_get_cdnasummary_n_amplification_cycles(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set n_amplification_cycles(arg0) {
        wasm.__wbg_set_cdnasummary_n_amplification_cycles(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get storage_location() {
        const ret = wasm.__wbg_get_cdnasummary_storage_location(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set storage_location(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnasummary_storage_location(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get notes() {
        const ret = wasm.__wbg_get_cdnasummary_notes(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set notes(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnasummary_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.cdnasummary_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.cdnasummary_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.cdnasummary_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {CdnaSummary}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnasummary_from_json_bytes(ptr0, len0);
        return CdnaSummary.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {CdnaSummary}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnasummary_from_json_string(ptr0, len0);
        return CdnaSummary.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {CdnaSummary}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cdnasummary_from_base64_json(ptr0, len0);
        return CdnaSummary.__wrap(ret);
    }
}

const ChromiumDatasetFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chromiumdataset_free(ptr >>> 0, 1));

export class ChromiumDataset {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChromiumDataset.prototype);
        obj.__wbg_ptr = ptr;
        ChromiumDatasetFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChromiumDatasetFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chromiumdataset_free(ptr, 0);
    }
    /**
     * @returns {ChromiumDatasetSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_chromiumdataset_summary(this.__wbg_ptr);
        return ChromiumDatasetSummary.__wrap(ret);
    }
    /**
     * @param {ChromiumDatasetSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, ChromiumDatasetSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumdataset_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string[]}
     */
    get library_ids() {
        const ret = wasm.__wbg_get_chromiumdataset_library_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set library_ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumdataset_library_ids(this.__wbg_ptr, ptr0, len0);
    }
}

const ChromiumDatasetErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chromiumdataseterror_free(ptr >>> 0, 1));

export class ChromiumDatasetError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChromiumDatasetError.prototype);
        obj.__wbg_ptr = ptr;
        ChromiumDatasetErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChromiumDatasetErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chromiumdataseterror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        const ret = wasm.__wbg_get_chromiumdataseterror_message(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.chromiumdataseterror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.chromiumdataseterror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.chromiumdataseterror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {ChromiumDatasetError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumdataseterror_from_json_bytes(ptr0, len0);
        return ChromiumDatasetError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {ChromiumDatasetError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumdataseterror_from_json_string(ptr0, len0);
        return ChromiumDatasetError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {ChromiumDatasetError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumdataseterror_from_base64_json(ptr0, len0);
        return ChromiumDatasetError.__wrap(ret);
    }
}

const ChromiumDatasetQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chromiumdatasetquery_free(ptr >>> 0, 1));

export class ChromiumDatasetQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChromiumDatasetQuery.prototype);
        obj.__wbg_ptr = ptr;
        ChromiumDatasetQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChromiumDatasetQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chromiumdatasetquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_chromiumdatasetquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumdatasetquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get names() {
        const ret = wasm.__wbg_get_chromiumdatasetquery_names(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set names(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumdatasetquery_names(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get lab_ids() {
        const ret = wasm.__wbg_get_chromiumdatasetquery_lab_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set lab_ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumdataset_library_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date | undefined}
     */
    get delivered_before() {
        const ret = wasm.__wbg_get_chromiumdatasetquery_delivered_before(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date | null} [arg0]
     */
    set delivered_before(arg0) {
        wasm.__wbg_set_chromiumdatasetquery_delivered_before(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {Date | undefined}
     */
    get delivered_after() {
        const ret = wasm.__wbg_get_chromiumdatasetquery_delivered_after(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date | null} [arg0]
     */
    set delivered_after(arg0) {
        wasm.__wbg_set_chromiumdatasetquery_delivered_after(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {TenxAssayQuery | undefined}
     */
    get tenx_assay() {
        const ret = wasm.__wbg_get_chromiumdatasetquery_tenx_assay(this.__wbg_ptr);
        return ret === 0 ? undefined : TenxAssayQuery.__wrap(ret);
    }
    /**
     * @param {TenxAssayQuery | null} [arg0]
     */
    set tenx_assay(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, TenxAssayQuery);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_chromiumdatasetquery_tenx_assay(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {SpecimenQuery | undefined}
     */
    get specimen() {
        const ret = wasm.__wbg_get_chromiumdatasetquery_specimen(this.__wbg_ptr);
        return ret === 0 ? undefined : SpecimenQuery.__wrap(ret);
    }
    /**
     * @param {SpecimenQuery | null} [arg0]
     */
    set specimen(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, SpecimenQuery);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_chromiumdatasetquery_specimen(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_chromiumdatasetquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumdatasetquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_chromiumdatasetquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumdatasetquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.chromiumdatasetquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.chromiumdatasetquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.chromiumdatasetquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {ChromiumDatasetQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumdatasetquery_from_json_bytes(ptr0, len0);
        return ChromiumDatasetQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {ChromiumDatasetQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumdatasetquery_from_json_string(ptr0, len0);
        return ChromiumDatasetQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {ChromiumDatasetQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumdatasetquery_from_base64_json(ptr0, len0);
        return ChromiumDatasetQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.chromiumdatasetquery_new();
        this.__wbg_ptr = ret >>> 0;
        ChromiumDatasetQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const ChromiumDatasetSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chromiumdatasetsummary_free(ptr >>> 0, 1));

export class ChromiumDatasetSummary {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChromiumDatasetSummary.prototype);
        obj.__wbg_ptr = ptr;
        ChromiumDatasetSummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChromiumDatasetSummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chromiumdatasetsummary_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_chromiumdatasetsummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumdatasetsummary_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_chromiumdatasetsummary_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_chromiumdatasetsummary_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumdatasetsummary_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {LabSummary}
     */
    get lab() {
        const ret = wasm.__wbg_get_chromiumdatasetsummary_lab(this.__wbg_ptr);
        return LabSummary.__wrap(ret);
    }
    /**
     * @param {LabSummary} arg0
     */
    set lab(arg0) {
        _assertClass(arg0, LabSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumdatasetsummary_lab(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string}
     */
    get data_path() {
        const ret = wasm.__wbg_get_chromiumdatasetsummary_data_path(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set data_path(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumdatasetsummary_data_path(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date}
     */
    get delivered_at() {
        const ret = wasm.__wbg_get_chromiumdatasetsummary_delivered_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set delivered_at(arg0) {
        wasm.__wbg_set_chromiumdatasetsummary_delivered_at(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {TenxAssay}
     */
    get tenx_assay() {
        const ret = wasm.__wbg_get_chromiumdatasetsummary_tenx_assay(this.__wbg_ptr);
        return TenxAssay.__wrap(ret);
    }
    /**
     * @param {TenxAssay} arg0
     */
    set tenx_assay(arg0) {
        _assertClass(arg0, TenxAssay);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumdatasetsummary_tenx_assay(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string}
     */
    get web_summary() {
        const ret = wasm.__wbg_get_chromiumdatasetsummary_web_summary(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set web_summary(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumdatasetsummary_web_summary(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.chromiumdatasetsummary_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.chromiumdatasetsummary_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.chromiumdatasetsummary_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {ChromiumDatasetSummary}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumdatasetsummary_from_json_bytes(ptr0, len0);
        return ChromiumDatasetSummary.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {ChromiumDatasetSummary}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumdatasetsummary_from_json_string(ptr0, len0);
        return ChromiumDatasetSummary.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {ChromiumDatasetSummary}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumdatasetsummary_from_base64_json(ptr0, len0);
        return ChromiumDatasetSummary.__wrap(ret);
    }
}

const ChromiumRunFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chromiumrun_free(ptr >>> 0, 1));

export class ChromiumRun {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChromiumRun.prototype);
        obj.__wbg_ptr = ptr;
        ChromiumRunFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChromiumRunFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chromiumrun_free(ptr, 0);
    }
    /**
     * @returns {ChromiumRunSummaryWithParents}
     */
    get info() {
        const ret = wasm.__wbg_get_chromiumrun_info(this.__wbg_ptr);
        return ChromiumRunSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {ChromiumRunSummaryWithParents} arg0
     */
    set info(arg0) {
        _assertClass(arg0, ChromiumRunSummaryWithParents);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumrun_info(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Gems[]}
     */
    get gems() {
        const ret = wasm.__wbg_get_chromiumrun_gems(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Gems[]} arg0
     */
    set gems(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrun_gems(this.__wbg_ptr, ptr0, len0);
    }
}

const ChromiumRunQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chromiumrunquery_free(ptr >>> 0, 1));

export class ChromiumRunQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChromiumRunQuery.prototype);
        obj.__wbg_ptr = ptr;
        ChromiumRunQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChromiumRunQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chromiumrunquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_chromiumrunquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get readable_ids() {
        const ret = wasm.__wbg_get_chromiumrunquery_readable_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set readable_ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunquery_readable_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {TenxAssayQuery | undefined}
     */
    get assay() {
        const ret = wasm.__wbg_get_chromiumrunquery_assay(this.__wbg_ptr);
        return ret === 0 ? undefined : TenxAssayQuery.__wrap(ret);
    }
    /**
     * @param {TenxAssayQuery | null} [arg0]
     */
    set assay(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, TenxAssayQuery);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_chromiumrunquery_assay(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Date | undefined}
     */
    get run_before() {
        const ret = wasm.__wbg_get_chromiumrunquery_run_before(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date | null} [arg0]
     */
    set run_before(arg0) {
        wasm.__wbg_set_chromiumrunquery_run_before(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {Date | undefined}
     */
    get run_after() {
        const ret = wasm.__wbg_get_chromiumrunquery_run_after(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date | null} [arg0]
     */
    set run_after(arg0) {
        wasm.__wbg_set_chromiumrunquery_run_after(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {boolean | undefined}
     */
    get succeeded() {
        const ret = wasm.__wbg_get_chromiumrunquery_succeeded(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
     * @param {boolean | null} [arg0]
     */
    set succeeded(arg0) {
        wasm.__wbg_set_chromiumrunquery_succeeded(this.__wbg_ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0 ? 1 : 0);
    }
    /**
     * @returns {string[]}
     */
    get notes() {
        const ret = wasm.__wbg_get_chromiumrunquery_notes(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set notes(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunquery_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_chromiumrunquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_chromiumrunquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumrunquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.chromiumrunquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.chromiumrunquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.chromiumrunquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {ChromiumRunQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumrunquery_from_json_bytes(ptr0, len0);
        return ChromiumRunQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {ChromiumRunQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumrunquery_from_json_string(ptr0, len0);
        return ChromiumRunQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {ChromiumRunQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumrunquery_from_base64_json(ptr0, len0);
        return ChromiumRunQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.chromiumrunquery_new();
        this.__wbg_ptr = ret >>> 0;
        ChromiumRunQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const ChromiumRunSummaryWithParentsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chromiumrunsummarywithparents_free(ptr >>> 0, 1));

export class ChromiumRunSummaryWithParents {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChromiumRunSummaryWithParents.prototype);
        obj.__wbg_ptr = ptr;
        ChromiumRunSummaryWithParentsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChromiumRunSummaryWithParentsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chromiumrunsummarywithparents_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_chromiumrunsummarywithparents_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunsummarywithparents_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get readable_id() {
        const ret = wasm.__wbg_get_chromiumrunsummarywithparents_readable_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set readable_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunsummarywithparents_readable_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date}
     */
    get run_at() {
        const ret = wasm.__wbg_get_chromiumrunsummarywithparents_run_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set run_at(arg0) {
        wasm.__wbg_set_chromiumrunsummarywithparents_run_at(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get run_by() {
        const ret = wasm.__wbg_get_chromiumrunsummarywithparents_run_by(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set run_by(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunsummarywithparents_run_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {boolean}
     */
    get succeeded() {
        const ret = wasm.__wbg_get_chromiumrunsummarywithparents_succeeded(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set succeeded(arg0) {
        wasm.__wbg_set_chromiumrunsummarywithparents_succeeded(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get notes() {
        const ret = wasm.__wbg_get_chromiumrunsummarywithparents_notes(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set notes(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunsummarywithparents_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {TenxAssay}
     */
    get assay() {
        const ret = wasm.__wbg_get_chromiumrunsummarywithparents_assay(this.__wbg_ptr);
        return TenxAssay.__wrap(ret);
    }
    /**
     * @param {TenxAssay} arg0
     */
    set assay(arg0) {
        _assertClass(arg0, TenxAssay);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumrunsummarywithparents_assay(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.chromiumrunsummarywithparents_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.chromiumrunsummarywithparents_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.chromiumrunsummarywithparents_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {ChromiumRunSummaryWithParents}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumrunsummarywithparents_from_json_bytes(ptr0, len0);
        return ChromiumRunSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {ChromiumRunSummaryWithParents}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumrunsummarywithparents_from_json_string(ptr0, len0);
        return ChromiumRunSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {ChromiumRunSummaryWithParents}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.chromiumrunsummarywithparents_from_base64_json(ptr0, len0);
        return ChromiumRunSummaryWithParents.__wrap(ret);
    }
}

const ClientErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_clienterror_free(ptr >>> 0, 1));

export class ClientError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ClientError.prototype);
        obj.__wbg_ptr = ptr;
        ClientErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ClientErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_clienterror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        const ret = wasm.__wbg_get_clienterror_message(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.clienterror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.clienterror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.clienterror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {ClientError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.clienterror_from_json_bytes(ptr0, len0);
        return ClientError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {ClientError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.clienterror_from_json_string(ptr0, len0);
        return ClientError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {ClientError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.clienterror_from_base64_json(ptr0, len0);
        return ClientError.__wrap(ret);
    }
}

const CommitteeApprovalFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_committeeapproval_free(ptr >>> 0, 1));

export class CommitteeApproval {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CommitteeApproval.prototype);
        obj.__wbg_ptr = ptr;
        CommitteeApprovalFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CommitteeApprovalFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_committeeapproval_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get institution_id() {
        const ret = wasm.__wbg_get_committeeapproval_institution_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set institution_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_committeeapproval_institution_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get specimen_id() {
        const ret = wasm.__wbg_get_committeeapproval_specimen_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set specimen_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_committeeapproval_specimen_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Institution}
     */
    get institution() {
        const ret = wasm.__wbg_get_committeeapproval_institution(this.__wbg_ptr);
        return Institution.__wrap(ret);
    }
    /**
     * @param {Institution} arg0
     */
    set institution(arg0) {
        _assertClass(arg0, Institution);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_committeeapproval_institution(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string}
     */
    get committee_type() {
        const ret = wasm.__wbg_get_committeeapproval_committee_type(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set committee_type(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_committeeapproval_committee_type(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get compliance_identifier() {
        const ret = wasm.__wbg_get_committeeapproval_compliance_identifier(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set compliance_identifier(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_committeeapproval_compliance_identifier(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.committeeapproval_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.committeeapproval_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.committeeapproval_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {CommitteeApproval}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.committeeapproval_from_json_bytes(ptr0, len0);
        return CommitteeApproval.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {CommitteeApproval}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.committeeapproval_from_json_string(ptr0, len0);
        return CommitteeApproval.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {CommitteeApproval}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.committeeapproval_from_base64_json(ptr0, len0);
        return CommitteeApproval.__wrap(ret);
    }
}

const ConcentrationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_concentration_free(ptr >>> 0, 1));

export class Concentration {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Concentration.prototype);
        obj.__wbg_ptr = ptr;
        ConcentrationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ConcentrationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_concentration_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get value() {
        const ret = wasm.__wbg_get_concentration_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set value(arg0) {
        wasm.__wbg_set_concentration_value(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get unit() {
        const ret = wasm.concentration_unit(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
}

const CreatedUserFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_createduser_free(ptr >>> 0, 1));

export class CreatedUser {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CreatedUser.prototype);
        obj.__wbg_ptr = ptr;
        CreatedUserFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CreatedUserFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_createduser_free(ptr, 0);
    }
    /**
     * @returns {Person}
     */
    get person() {
        const ret = wasm.__wbg_get_createduser_person(this.__wbg_ptr);
        return Person.__wrap(ret);
    }
    /**
     * @param {Person} arg0
     */
    set person(arg0) {
        _assertClass(arg0, Person);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_createduser_person(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string}
     */
    get api_key() {
        const ret = wasm.__wbg_get_createduser_api_key(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set api_key(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_createduser_api_key(this.__wbg_ptr, ptr0, len0);
    }
}

const DatasetMetricsFileParseErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_datasetmetricsfileparseerror_free(ptr >>> 0, 1));

export class DatasetMetricsFileParseError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DatasetMetricsFileParseError.prototype);
        obj.__wbg_ptr = ptr;
        DatasetMetricsFileParseErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DatasetMetricsFileParseErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_datasetmetricsfileparseerror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        const ret = wasm.__wbg_get_datasetmetricsfileparseerror_message(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.datasetmetricsfileparseerror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.datasetmetricsfileparseerror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.datasetmetricsfileparseerror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {DatasetMetricsFileParseError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datasetmetricsfileparseerror_from_json_bytes(ptr0, len0);
        return DatasetMetricsFileParseError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {DatasetMetricsFileParseError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datasetmetricsfileparseerror_from_json_string(ptr0, len0);
        return DatasetMetricsFileParseError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {DatasetMetricsFileParseError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datasetmetricsfileparseerror_from_base64_json(ptr0, len0);
        return DatasetMetricsFileParseError.__wrap(ret);
    }
}

const DatasetNMetricsFilesErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_datasetnmetricsfileserror_free(ptr >>> 0, 1));

export class DatasetNMetricsFilesError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DatasetNMetricsFilesError.prototype);
        obj.__wbg_ptr = ptr;
        DatasetNMetricsFilesErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DatasetNMetricsFilesErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_datasetnmetricsfileserror_free(ptr, 0);
    }
    /**
     * @returns {bigint}
     */
    get expected_n_metrics_files() {
        const ret = wasm.__wbg_get_datasetnmetricsfileserror_expected_n_metrics_files(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @param {bigint} arg0
     */
    set expected_n_metrics_files(arg0) {
        wasm.__wbg_set_datasetnmetricsfileserror_expected_n_metrics_files(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {bigint}
     */
    get found_n_metrics_files() {
        const ret = wasm.__wbg_get_datasetnmetricsfileserror_found_n_metrics_files(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @param {bigint} arg0
     */
    set found_n_metrics_files(arg0) {
        wasm.__wbg_set_datasetnmetricsfileserror_found_n_metrics_files(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.datasetnmetricsfileserror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.datasetnmetricsfileserror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.datasetnmetricsfileserror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {DatasetNMetricsFilesError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datasetnmetricsfileserror_from_json_bytes(ptr0, len0);
        return DatasetNMetricsFilesError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {DatasetNMetricsFilesError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datasetnmetricsfileserror_from_json_string(ptr0, len0);
        return DatasetNMetricsFilesError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {DatasetNMetricsFilesError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datasetnmetricsfileserror_from_base64_json(ptr0, len0);
        return DatasetNMetricsFilesError.__wrap(ret);
    }
}

const DuplicateResourceErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_duplicateresourceerror_free(ptr >>> 0, 1));

export class DuplicateResourceError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DuplicateResourceError.prototype);
        obj.__wbg_ptr = ptr;
        DuplicateResourceErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DuplicateResourceErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_duplicateresourceerror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get entity() {
        const ret = wasm.__wbg_get_duplicateresourceerror_entity(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set entity(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get fields() {
        const ret = wasm.__wbg_get_duplicateresourceerror_fields(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set fields(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_duplicateresourceerror_fields(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get values() {
        const ret = wasm.__wbg_get_duplicateresourceerror_values(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set values(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_duplicateresourceerror_values(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.duplicateresourceerror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.duplicateresourceerror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.duplicateresourceerror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {DuplicateResourceError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.duplicateresourceerror_from_json_bytes(ptr0, len0);
        return DuplicateResourceError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {DuplicateResourceError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.duplicateresourceerror_from_json_string(ptr0, len0);
        return DuplicateResourceError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {DuplicateResourceError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.duplicateresourceerror_from_base64_json(ptr0, len0);
        return DuplicateResourceError.__wrap(ret);
    }
}

const ElectrophoreticMeasurementDataFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_electrophoreticmeasurementdata_free(ptr >>> 0, 1));

export class ElectrophoreticMeasurementData {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ElectrophoreticMeasurementData.prototype);
        obj.__wbg_ptr = ptr;
        ElectrophoreticMeasurementDataFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ElectrophoreticMeasurementDataFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_electrophoreticmeasurementdata_free(ptr, 0);
    }
    /**
     * @returns {Date}
     */
    get measured_at() {
        const ret = wasm.__wbg_get_electrophoreticmeasurementdata_measured_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set measured_at(arg0) {
        wasm.__wbg_set_electrophoreticmeasurementdata_measured_at(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get instrument_name() {
        const ret = wasm.__wbg_get_electrophoreticmeasurementdata_instrument_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set instrument_name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_electrophoreticmeasurementdata_instrument_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number | undefined}
     */
    get mean_size_bp() {
        const ret = wasm.__wbg_get_electrophoreticmeasurementdata_mean_size_bp(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set mean_size_bp(arg0) {
        wasm.__wbg_set_electrophoreticmeasurementdata_mean_size_bp(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : Math.fround(arg0));
    }
    /**
     * @returns {Concentration}
     */
    get concentration() {
        const ret = wasm.__wbg_get_electrophoreticmeasurementdata_concentration(this.__wbg_ptr);
        return Concentration.__wrap(ret);
    }
    /**
     * @param {Concentration} arg0
     */
    set concentration(arg0) {
        _assertClass(arg0, Concentration);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_electrophoreticmeasurementdata_concentration(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint16Array}
     */
    get sizing_range() {
        const ret = wasm.electrophoreticmeasurementdata_sizing_range(this.__wbg_ptr);
        var v1 = getArrayU16FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 2, 2);
        return v1;
    }
}

const EmptyStringErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_emptystringerror_free(ptr >>> 0, 1));

export class EmptyStringError {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        EmptyStringErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_emptystringerror_free(ptr, 0);
    }
}

const GemsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gems_free(ptr >>> 0, 1));

export class Gems {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Gems.prototype);
        obj.__wbg_ptr = ptr;
        GemsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof Gems)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GemsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gems_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_gems_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_gems_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get readable_id() {
        const ret = wasm.__wbg_get_gems_readable_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set readable_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_gems_readable_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get chromium_run_id() {
        const ret = wasm.__wbg_get_gems_chromium_run_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set chromium_run_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_gems_chromium_run_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.gems_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.gems_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.gems_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {Gems}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.gems_from_json_bytes(ptr0, len0);
        return Gems.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {Gems}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.gems_from_json_string(ptr0, len0);
        return Gems.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {Gems}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.gems_from_base64_json(ptr0, len0);
        return Gems.__wrap(ret);
    }
}

const InstitutionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_institution_free(ptr >>> 0, 1));

export class Institution {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Institution.prototype);
        obj.__wbg_ptr = ptr;
        InstitutionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InstitutionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_institution_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_institution_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_institution_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_institution_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_institution_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_institution_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.institution_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.institution_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.institution_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {Institution}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.institution_from_json_bytes(ptr0, len0);
        return Institution.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {Institution}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.institution_from_json_string(ptr0, len0);
        return Institution.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {Institution}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.institution_from_base64_json(ptr0, len0);
        return Institution.__wrap(ret);
    }
}

const InstitutionQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_institutionquery_free(ptr >>> 0, 1));

export class InstitutionQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InstitutionQuery.prototype);
        obj.__wbg_ptr = ptr;
        InstitutionQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InstitutionQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_institutionquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_institutionquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_institutionquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get names() {
        const ret = wasm.__wbg_get_institutionquery_names(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set names(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_institutionquery_names(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_institutionquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_institutionquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_institutionquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_institutionquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.institutionquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.institutionquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.institutionquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {InstitutionQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.institutionquery_from_json_bytes(ptr0, len0);
        return InstitutionQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {InstitutionQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.institutionquery_from_json_string(ptr0, len0);
        return InstitutionQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {InstitutionQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.institutionquery_from_base64_json(ptr0, len0);
        return InstitutionQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.institutionquery_new();
        this.__wbg_ptr = ret >>> 0;
        InstitutionQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const InvalidDataErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_invaliddataerror_free(ptr >>> 0, 1));

export class InvalidDataError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InvalidDataError.prototype);
        obj.__wbg_ptr = ptr;
        InvalidDataErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InvalidDataErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_invaliddataerror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        const ret = wasm.__wbg_get_invaliddataerror_message(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.invaliddataerror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.invaliddataerror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.invaliddataerror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {InvalidDataError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.invaliddataerror_from_json_bytes(ptr0, len0);
        return InvalidDataError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {InvalidDataError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.invaliddataerror_from_json_string(ptr0, len0);
        return InvalidDataError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {InvalidDataError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.invaliddataerror_from_base64_json(ptr0, len0);
        return InvalidDataError.__wrap(ret);
    }
}

const InvalidMeasurementErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_invalidmeasurementerror_free(ptr >>> 0, 1));

export class InvalidMeasurementError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InvalidMeasurementError.prototype);
        obj.__wbg_ptr = ptr;
        InvalidMeasurementErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InvalidMeasurementErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_invalidmeasurementerror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        const ret = wasm.__wbg_get_invalidmeasurementerror_message(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.invalidmeasurementerror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.invalidmeasurementerror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.invalidmeasurementerror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {InvalidMeasurementError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.invalidmeasurementerror_from_json_bytes(ptr0, len0);
        return InvalidMeasurementError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {InvalidMeasurementError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.invalidmeasurementerror_from_json_string(ptr0, len0);
        return InvalidMeasurementError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {InvalidMeasurementError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.invalidmeasurementerror_from_base64_json(ptr0, len0);
        return InvalidMeasurementError.__wrap(ret);
    }
}

const InvalidReferenceErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_invalidreferenceerror_free(ptr >>> 0, 1));

export class InvalidReferenceError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InvalidReferenceError.prototype);
        obj.__wbg_ptr = ptr;
        InvalidReferenceErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InvalidReferenceErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_invalidreferenceerror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get entity() {
        const ret = wasm.__wbg_get_invalidreferenceerror_entity(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set entity(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get referenced_entity() {
        const ret = wasm.__wbg_get_invalidreferenceerror_referenced_entity(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set referenced_entity(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_invalidreferenceerror_referenced_entity(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get value() {
        const ret = wasm.__wbg_get_invalidreferenceerror_value(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set value(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_invalidreferenceerror_value(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.invalidreferenceerror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.invalidreferenceerror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.invalidreferenceerror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {InvalidReferenceError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.invalidreferenceerror_from_json_bytes(ptr0, len0);
        return InvalidReferenceError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {InvalidReferenceError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.invalidreferenceerror_from_json_string(ptr0, len0);
        return InvalidReferenceError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {InvalidReferenceError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.invalidreferenceerror_from_base64_json(ptr0, len0);
        return InvalidReferenceError.__wrap(ret);
    }
}

const LabFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_lab_free(ptr >>> 0, 1));

export class Lab {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Lab.prototype);
        obj.__wbg_ptr = ptr;
        LabFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LabFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lab_free(ptr, 0);
    }
    /**
     * @returns {LabSummaryWithParents}
     */
    get info() {
        const ret = wasm.__wbg_get_lab_info(this.__wbg_ptr);
        return LabSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {LabSummaryWithParents} arg0
     */
    set info(arg0) {
        _assertClass(arg0, LabSummaryWithParents);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_lab_info(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {PersonSummary[]}
     */
    get members() {
        const ret = wasm.__wbg_get_lab_members(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {PersonSummary[]} arg0
     */
    set members(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_lab_members(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.lab_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.lab_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.lab_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {Lab}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.lab_from_json_bytes(ptr0, len0);
        return Lab.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {Lab}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.lab_from_json_string(ptr0, len0);
        return Lab.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {Lab}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.lab_from_base64_json(ptr0, len0);
        return Lab.__wrap(ret);
    }
}

const LabQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_labquery_free(ptr >>> 0, 1));

export class LabQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LabQuery.prototype);
        obj.__wbg_ptr = ptr;
        LabQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LabQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_labquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_labquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get names() {
        const ret = wasm.__wbg_get_labquery_names(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set names(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labquery_names(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_labquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_labquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_labquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.labquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.labquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.labquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {LabQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.labquery_from_json_bytes(ptr0, len0);
        return LabQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {LabQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.labquery_from_json_string(ptr0, len0);
        return LabQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {LabQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.labquery_from_base64_json(ptr0, len0);
        return LabQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.labquery_new();
        this.__wbg_ptr = ret >>> 0;
        LabQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const LabSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_labsummary_free(ptr >>> 0, 1));

export class LabSummary {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LabSummary.prototype);
        obj.__wbg_ptr = ptr;
        LabSummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LabSummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_labsummary_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_labsummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labsummary_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_labsummary_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_labsummary_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labsummary_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get delivery_dir() {
        const ret = wasm.__wbg_get_labsummary_delivery_dir(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set delivery_dir(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labsummary_delivery_dir(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.labsummary_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.labsummary_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.labsummary_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {LabSummary}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.labsummary_from_json_bytes(ptr0, len0);
        return LabSummary.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {LabSummary}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.labsummary_from_json_string(ptr0, len0);
        return LabSummary.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {LabSummary}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.labsummary_from_base64_json(ptr0, len0);
        return LabSummary.__wrap(ret);
    }
}

const LabSummaryWithParentsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_labsummarywithparents_free(ptr >>> 0, 1));

export class LabSummaryWithParents {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LabSummaryWithParents.prototype);
        obj.__wbg_ptr = ptr;
        LabSummaryWithParentsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LabSummaryWithParentsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_labsummarywithparents_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id_() {
        const ret = wasm.__wbg_get_labsummarywithparents_id_(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id_(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labsummarywithparents_id_(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {LabSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_labsummarywithparents_summary(this.__wbg_ptr);
        return LabSummary.__wrap(ret);
    }
    /**
     * @param {LabSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, LabSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_labsummarywithparents_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {PersonSummary}
     */
    get pi() {
        const ret = wasm.__wbg_get_labsummarywithparents_pi(this.__wbg_ptr);
        return PersonSummary.__wrap(ret);
    }
    /**
     * @param {PersonSummary} arg0
     */
    set pi(arg0) {
        _assertClass(arg0, PersonSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_labsummarywithparents_pi(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.labsummarywithparents_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.labsummarywithparents_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.labsummarywithparents_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {LabSummaryWithParents}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.labsummarywithparents_from_json_bytes(ptr0, len0);
        return LabSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {LabSummaryWithParents}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.labsummarywithparents_from_json_string(ptr0, len0);
        return LabSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {LabSummaryWithParents}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.labsummarywithparents_from_base64_json(ptr0, len0);
        return LabSummaryWithParents.__wrap(ret);
    }
}

const LabUpdateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_labupdate_free(ptr >>> 0, 1));

export class LabUpdate {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LabUpdateFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_labupdate_free(ptr, 0);
    }
    /**
     * @returns {LabUpdateFields}
     */
    get fields() {
        const ret = wasm.__wbg_get_labupdate_fields(this.__wbg_ptr);
        return LabUpdateFields.__wrap(ret);
    }
    /**
     * @param {LabUpdateFields} arg0
     */
    set fields(arg0) {
        _assertClass(arg0, LabUpdateFields);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_labupdate_fields(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string[]}
     */
    get add_members() {
        const ret = wasm.__wbg_get_labupdate_add_members(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set add_members(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labupdate_add_members(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get remove_members() {
        const ret = wasm.__wbg_get_labupdate_remove_members(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set remove_members(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labupdate_remove_members(this.__wbg_ptr, ptr0, len0);
    }
}

const LabUpdateFieldsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_labupdatefields_free(ptr >>> 0, 1));

export class LabUpdateFields {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LabUpdateFields.prototype);
        obj.__wbg_ptr = ptr;
        LabUpdateFieldsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LabUpdateFieldsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_labupdatefields_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_labupdatefields_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labupdatefields_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_labupdatefields_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set name(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labupdatefields_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get pi_id() {
        const ret = wasm.__wbg_get_labupdatefields_pi_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set pi_id(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labupdatefields_pi_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get delivery_dir() {
        const ret = wasm.__wbg_get_labupdatefields_delivery_dir(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set delivery_dir(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labupdatefields_delivery_dir(this.__wbg_ptr, ptr0, len0);
    }
}

const LibraryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_library_free(ptr >>> 0, 1));

export class Library {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Library.prototype);
        obj.__wbg_ptr = ptr;
        LibraryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LibraryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_library_free(ptr, 0);
    }
    /**
     * @returns {LibrarySummaryWithParents}
     */
    get info() {
        const ret = wasm.__wbg_get_library_info(this.__wbg_ptr);
        return LibrarySummaryWithParents.__wrap(ret);
    }
    /**
     * @param {LibrarySummaryWithParents} arg0
     */
    set info(arg0) {
        _assertClass(arg0, LibrarySummaryWithParents);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_library_info(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string[]}
     */
    get prepared_by() {
        const ret = wasm.__wbg_get_library_prepared_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set prepared_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_library_prepared_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {LibraryMeasurement[]}
     */
    get measurements() {
        const ret = wasm.__wbg_get_library_measurements(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {LibraryMeasurement[]} arg0
     */
    set measurements(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_library_measurements(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.library_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.library_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.library_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {Library}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.library_from_json_bytes(ptr0, len0);
        return Library.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {Library}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.library_from_json_string(ptr0, len0);
        return Library.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {Library}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.library_from_base64_json(ptr0, len0);
        return Library.__wrap(ret);
    }
}

const LibraryIndexSetErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_libraryindexseterror_free(ptr >>> 0, 1));

export class LibraryIndexSetError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LibraryIndexSetError.prototype);
        obj.__wbg_ptr = ptr;
        LibraryIndexSetErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LibraryIndexSetErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_libraryindexseterror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        const ret = wasm.__wbg_get_libraryindexseterror_message(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.libraryindexseterror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.libraryindexseterror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.libraryindexseterror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {LibraryIndexSetError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.libraryindexseterror_from_json_bytes(ptr0, len0);
        return LibraryIndexSetError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {LibraryIndexSetError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.libraryindexseterror_from_json_string(ptr0, len0);
        return LibraryIndexSetError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {LibraryIndexSetError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.libraryindexseterror_from_base64_json(ptr0, len0);
        return LibraryIndexSetError.__wrap(ret);
    }
}

const LibraryMeasurementFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_librarymeasurement_free(ptr >>> 0, 1));

export class LibraryMeasurement {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LibraryMeasurement.prototype);
        obj.__wbg_ptr = ptr;
        LibraryMeasurementFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof LibraryMeasurement)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LibraryMeasurementFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_librarymeasurement_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_librarymeasurement_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_librarymeasurement_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get library_id() {
        const ret = wasm.__wbg_get_librarymeasurement_library_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set library_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_librarymeasurement_library_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get measured_by() {
        const ret = wasm.__wbg_get_librarymeasurement_measured_by(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set measured_by(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_librarymeasurement_measured_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.librarymeasurement_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.librarymeasurement_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.librarymeasurement_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {LibraryMeasurement}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.librarymeasurement_from_json_bytes(ptr0, len0);
        return LibraryMeasurement.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {LibraryMeasurement}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.librarymeasurement_from_json_string(ptr0, len0);
        return LibraryMeasurement.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {LibraryMeasurement}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.librarymeasurement_from_base64_json(ptr0, len0);
        return LibraryMeasurement.__wrap(ret);
    }
}

const LibraryQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_libraryquery_free(ptr >>> 0, 1));

export class LibraryQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LibraryQuery.prototype);
        obj.__wbg_ptr = ptr;
        LibraryQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LibraryQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_libraryquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_libraryquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_institutionquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {any[]}
     */
    get library_types() {
        const ret = wasm.__wbg_get_libraryquery_library_types(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {any[]} arg0
     */
    set library_types(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_libraryquery_library_types(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_institutionquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_institutionquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_libraryquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_libraryquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.libraryquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.libraryquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.libraryquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {LibraryQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.libraryquery_from_json_bytes(ptr0, len0);
        return LibraryQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {LibraryQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.libraryquery_from_json_string(ptr0, len0);
        return LibraryQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {LibraryQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.libraryquery_from_base64_json(ptr0, len0);
        return LibraryQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.libraryquery_new();
        this.__wbg_ptr = ret >>> 0;
        LibraryQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const LibrarySummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_librarysummary_free(ptr >>> 0, 1));

export class LibrarySummary {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LibrarySummary.prototype);
        obj.__wbg_ptr = ptr;
        LibrarySummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LibrarySummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_librarysummary_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_librarysummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_institution_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_librarysummary_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get readable_id() {
        const ret = wasm.__wbg_get_librarysummary_readable_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set readable_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_librarysummary_readable_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get single_index_set_name() {
        const ret = wasm.__wbg_get_librarysummary_single_index_set_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set single_index_set_name(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_librarysummary_single_index_set_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get dual_index_set_name() {
        const ret = wasm.__wbg_get_librarysummary_dual_index_set_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set dual_index_set_name(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_librarysummary_dual_index_set_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number}
     */
    get number_of_sample_index_pcr_cycles() {
        const ret = wasm.__wbg_get_librarysummary_number_of_sample_index_pcr_cycles(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set number_of_sample_index_pcr_cycles(arg0) {
        wasm.__wbg_set_librarysummary_number_of_sample_index_pcr_cycles(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get target_reads_per_cell() {
        const ret = wasm.__wbg_get_librarysummary_target_reads_per_cell(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set target_reads_per_cell(arg0) {
        wasm.__wbg_set_librarysummary_target_reads_per_cell(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Date}
     */
    get prepared_at() {
        const ret = wasm.__wbg_get_librarysummary_prepared_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set prepared_at(arg0) {
        wasm.__wbg_set_librarysummary_prepared_at(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get notes() {
        const ret = wasm.__wbg_get_librarysummary_notes(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set notes(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_librarysummary_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.librarysummary_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.librarysummary_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.librarysummary_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {LibrarySummary}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.librarysummary_from_json_bytes(ptr0, len0);
        return LibrarySummary.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {LibrarySummary}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.librarysummary_from_json_string(ptr0, len0);
        return LibrarySummary.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {LibrarySummary}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.librarysummary_from_base64_json(ptr0, len0);
        return LibrarySummary.__wrap(ret);
    }
}

const LibrarySummaryWithParentsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_librarysummarywithparents_free(ptr >>> 0, 1));

export class LibrarySummaryWithParents {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LibrarySummaryWithParents.prototype);
        obj.__wbg_ptr = ptr;
        LibrarySummaryWithParentsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LibrarySummaryWithParentsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_librarysummarywithparents_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id_() {
        const ret = wasm.__wbg_get_librarysummarywithparents_id_(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id_(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_librarysummarywithparents_id_(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {LibrarySummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_librarysummarywithparents_summary(this.__wbg_ptr);
        return LibrarySummary.__wrap(ret);
    }
    /**
     * @param {LibrarySummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, LibrarySummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_librarysummarywithparents_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {CdnaSummary}
     */
    get cdna() {
        const ret = wasm.__wbg_get_librarysummarywithparents_cdna(this.__wbg_ptr);
        return CdnaSummary.__wrap(ret);
    }
    /**
     * @param {CdnaSummary} arg0
     */
    set cdna(arg0) {
        _assertClass(arg0, CdnaSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_librarysummarywithparents_cdna(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.librarysummarywithparents_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.librarysummarywithparents_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.librarysummarywithparents_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {LibrarySummaryWithParents}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.librarysummarywithparents_from_json_bytes(ptr0, len0);
        return LibrarySummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {LibrarySummaryWithParents}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.librarysummarywithparents_from_json_string(ptr0, len0);
        return LibrarySummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {LibrarySummaryWithParents}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.librarysummarywithparents_from_base64_json(ptr0, len0);
        return LibrarySummaryWithParents.__wrap(ret);
    }
}

const LibraryTypeSpecificationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_librarytypespecification_free(ptr >>> 0, 1));

export class LibraryTypeSpecification {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LibraryTypeSpecification.prototype);
        obj.__wbg_ptr = ptr;
        LibraryTypeSpecificationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof LibraryTypeSpecification)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LibraryTypeSpecificationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_librarytypespecification_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get assay_id() {
        const ret = wasm.__wbg_get_librarytypespecification_assay_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set assay_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_librarytypespecification_assay_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {LibraryType}
     */
    get library_type() {
        const ret = wasm.__wbg_get_librarytypespecification_library_type(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {LibraryType} arg0
     */
    set library_type(arg0) {
        wasm.__wbg_set_librarytypespecification_library_type(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get index_kit() {
        const ret = wasm.__wbg_get_librarytypespecification_index_kit(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set index_kit(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_librarytypespecification_index_kit(this.__wbg_ptr, ptr0, len0);
    }
}

const MalformedRequestErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_malformedrequesterror_free(ptr >>> 0, 1));

export class MalformedRequestError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MalformedRequestError.prototype);
        obj.__wbg_ptr = ptr;
        MalformedRequestErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MalformedRequestErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_malformedrequesterror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        const ret = wasm.__wbg_get_malformedrequesterror_message(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.malformedrequesterror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.malformedrequesterror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.malformedrequesterror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {MalformedRequestError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.malformedrequesterror_from_json_bytes(ptr0, len0);
        return MalformedRequestError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {MalformedRequestError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.malformedrequesterror_from_json_string(ptr0, len0);
        return MalformedRequestError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {MalformedRequestError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.malformedrequesterror_from_base64_json(ptr0, len0);
        return MalformedRequestError.__wrap(ret);
    }
}

const MultiplexingTagFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_multiplexingtag_free(ptr >>> 0, 1));

export class MultiplexingTag {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MultiplexingTag.prototype);
        obj.__wbg_ptr = ptr;
        MultiplexingTagFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MultiplexingTagFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_multiplexingtag_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_multiplexingtag_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_multiplexingtag_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get tag_id() {
        const ret = wasm.__wbg_get_multiplexingtag_tag_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set tag_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_multiplexingtag_tag_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get type_() {
        const ret = wasm.__wbg_get_multiplexingtag_type_(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set type_(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_multiplexingtag_type_(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.multiplexingtag_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.multiplexingtag_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.multiplexingtag_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {MultiplexingTag}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.multiplexingtag_from_json_bytes(ptr0, len0);
        return MultiplexingTag.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {MultiplexingTag}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.multiplexingtag_from_json_string(ptr0, len0);
        return MultiplexingTag.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {MultiplexingTag}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.multiplexingtag_from_base64_json(ptr0, len0);
        return MultiplexingTag.__wrap(ret);
    }
}

const NewLabFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_newlab_free(ptr >>> 0, 1));

export class NewLab {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NewLabFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_newlab_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_newlab_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_newlab_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get pi_id() {
        const ret = wasm.__wbg_get_newlab_pi_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set pi_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_newlab_pi_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get delivery_dir() {
        const ret = wasm.__wbg_get_newlab_delivery_dir(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set delivery_dir(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_newlab_delivery_dir(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get member_ids() {
        const ret = wasm.__wbg_get_newlab_member_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set member_ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_newlab_member_ids(this.__wbg_ptr, ptr0, len0);
    }
}

const NewPersonFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_newperson_free(ptr >>> 0, 1));

export class NewPerson {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NewPersonFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_newperson_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_newperson_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_newlab_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get email() {
        const ret = wasm.__wbg_get_newperson_email(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set email(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_newperson_email(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get institution_id() {
        const ret = wasm.__wbg_get_newperson_institution_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set institution_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_newperson_institution_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get ms_user_id() {
        const ret = wasm.__wbg_get_newperson_ms_user_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set ms_user_id(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_newperson_ms_user_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} ms_user_id
     */
    constructor(ms_user_id) {
        const ptr0 = passStringToWasm0(ms_user_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.newperson_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        NewPersonFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const OrderByFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_orderby_free(ptr >>> 0, 1));

export class OrderBy {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(OrderBy.prototype);
        obj.__wbg_ptr = ptr;
        OrderByFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof OrderBy)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        OrderByFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_orderby_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get field() {
        const ret = wasm.__wbg_get_orderby_field(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set field(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_orderby_field(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {boolean}
     */
    get descending() {
        const ret = wasm.__wbg_get_orderby_descending(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set descending(arg0) {
        wasm.__wbg_set_orderby_descending(this.__wbg_ptr, arg0);
    }
    /**
     * @param {string} field
     * @param {boolean} descending
     */
    constructor(field, descending) {
        const ptr0 = passStringToWasm0(field, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.orderby_js_new(ptr0, len0, descending);
        this.__wbg_ptr = ret >>> 0;
        OrderByFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const PaginationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_pagination_free(ptr >>> 0, 1));

export class Pagination {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Pagination.prototype);
        obj.__wbg_ptr = ptr;
        PaginationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PaginationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pagination_free(ptr, 0);
    }
    /**
     * @returns {bigint}
     */
    get limit() {
        const ret = wasm.__wbg_get_pagination_limit(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {bigint} arg0
     */
    set limit(arg0) {
        wasm.__wbg_set_pagination_limit(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {bigint}
     */
    get offset() {
        const ret = wasm.__wbg_get_pagination_offset(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {bigint} arg0
     */
    set offset(arg0) {
        wasm.__wbg_set_pagination_offset(this.__wbg_ptr, arg0);
    }
}

const PermissionDeniedErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_permissiondeniederror_free(ptr >>> 0, 1));

export class PermissionDeniedError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PermissionDeniedError.prototype);
        obj.__wbg_ptr = ptr;
        PermissionDeniedErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PermissionDeniedErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_permissiondeniederror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        const ret = wasm.__wbg_get_permissiondeniederror_message(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.permissiondeniederror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.permissiondeniederror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.permissiondeniederror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {PermissionDeniedError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.permissiondeniederror_from_json_bytes(ptr0, len0);
        return PermissionDeniedError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {PermissionDeniedError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.permissiondeniederror_from_json_string(ptr0, len0);
        return PermissionDeniedError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {PermissionDeniedError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.permissiondeniederror_from_base64_json(ptr0, len0);
        return PermissionDeniedError.__wrap(ret);
    }
}

const PersonFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_person_free(ptr >>> 0, 1));

export class Person {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Person.prototype);
        obj.__wbg_ptr = ptr;
        PersonFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PersonFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_person_free(ptr, 0);
    }
    /**
     * @returns {PersonSummaryWithParents}
     */
    get info() {
        const ret = wasm.__wbg_get_person_info(this.__wbg_ptr);
        return PersonSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {PersonSummaryWithParents} arg0
     */
    set info(arg0) {
        _assertClass(arg0, PersonSummaryWithParents);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_person_info(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {any[]}
     */
    get roles() {
        const ret = wasm.__wbg_get_person_roles(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {any[]} arg0
     */
    set roles(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_person_roles(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.person_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.person_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.person_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {Person}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.person_from_json_bytes(ptr0, len0);
        return Person.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {Person}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.person_from_json_string(ptr0, len0);
        return Person.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {Person}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.person_from_base64_json(ptr0, len0);
        return Person.__wrap(ret);
    }
}

const PersonQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_personquery_free(ptr >>> 0, 1));

export class PersonQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PersonQuery.prototype);
        obj.__wbg_ptr = ptr;
        PersonQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PersonQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_personquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_personquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get names() {
        const ret = wasm.__wbg_get_personquery_names(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set names(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labquery_names(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get emails() {
        const ret = wasm.__wbg_get_personquery_emails(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set emails(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_personquery_emails(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get orcids() {
        const ret = wasm.__wbg_get_personquery_orcids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set orcids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_personquery_orcids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get ms_user_ids() {
        const ret = wasm.__wbg_get_personquery_ms_user_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ms_user_ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_personquery_ms_user_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_personquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_personquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_labquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_labquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.personquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.personquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.personquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {PersonQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personquery_from_json_bytes(ptr0, len0);
        return PersonQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {PersonQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personquery_from_json_string(ptr0, len0);
        return PersonQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {PersonQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personquery_from_base64_json(ptr0, len0);
        return PersonQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.personquery_new();
        this.__wbg_ptr = ret >>> 0;
        PersonQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const PersonSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_personsummary_free(ptr >>> 0, 1));

export class PersonSummary {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PersonSummary.prototype);
        obj.__wbg_ptr = ptr;
        PersonSummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof PersonSummary)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PersonSummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_personsummary_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_personsummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labsummary_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_personsummary_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_personsummary_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_personsummary_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get email() {
        const ret = wasm.__wbg_get_personsummary_email(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set email(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_personsummary_email(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get orcid() {
        const ret = wasm.__wbg_get_personsummary_orcid(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set orcid(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_personsummary_orcid(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.personsummary_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.personsummary_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.personsummary_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {PersonSummary}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personsummary_from_json_bytes(ptr0, len0);
        return PersonSummary.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {PersonSummary}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personsummary_from_json_string(ptr0, len0);
        return PersonSummary.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {PersonSummary}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personsummary_from_base64_json(ptr0, len0);
        return PersonSummary.__wrap(ret);
    }
}

const PersonSummaryWithParentsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_personsummarywithparents_free(ptr >>> 0, 1));

export class PersonSummaryWithParents {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PersonSummaryWithParents.prototype);
        obj.__wbg_ptr = ptr;
        PersonSummaryWithParentsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PersonSummaryWithParentsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_personsummarywithparents_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id_() {
        const ret = wasm.__wbg_get_personsummarywithparents_id_(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id_(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labsummarywithparents_id_(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {PersonSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_labsummarywithparents_pi(this.__wbg_ptr);
        return PersonSummary.__wrap(ret);
    }
    /**
     * @param {PersonSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, PersonSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_labsummarywithparents_pi(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Institution}
     */
    get institution() {
        const ret = wasm.__wbg_get_personsummarywithparents_institution(this.__wbg_ptr);
        return Institution.__wrap(ret);
    }
    /**
     * @param {Institution} arg0
     */
    set institution(arg0) {
        _assertClass(arg0, Institution);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_personsummarywithparents_institution(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.personsummarywithparents_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.personsummarywithparents_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.personsummarywithparents_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {PersonSummaryWithParents}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personsummarywithparents_from_json_bytes(ptr0, len0);
        return PersonSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {PersonSummaryWithParents}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personsummarywithparents_from_json_string(ptr0, len0);
        return PersonSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {PersonSummaryWithParents}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personsummarywithparents_from_base64_json(ptr0, len0);
        return PersonSummaryWithParents.__wrap(ret);
    }
}

const ResourceNotFoundErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_resourcenotfounderror_free(ptr >>> 0, 1));

export class ResourceNotFoundError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ResourceNotFoundError.prototype);
        obj.__wbg_ptr = ptr;
        ResourceNotFoundErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ResourceNotFoundErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_resourcenotfounderror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get requested_resource_id() {
        const ret = wasm.__wbg_get_resourcenotfounderror_requested_resource_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set requested_resource_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_resourcenotfounderror_requested_resource_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.resourcenotfounderror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.resourcenotfounderror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.resourcenotfounderror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {ResourceNotFoundError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.resourcenotfounderror_from_json_bytes(ptr0, len0);
        return ResourceNotFoundError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {ResourceNotFoundError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.resourcenotfounderror_from_json_string(ptr0, len0);
        return ResourceNotFoundError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {ResourceNotFoundError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.resourcenotfounderror_from_base64_json(ptr0, len0);
        return ResourceNotFoundError.__wrap(ret);
    }
}

const ScamplersClientFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_scamplersclient_free(ptr >>> 0, 1));

export class ScamplersClient {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ScamplersClientFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_scamplersclient_free(ptr, 0);
    }
    /**
     * @param {string} api_base_url
     * @param {string | null} [frontend_token]
     * @param {string | null} [api_key]
     */
    constructor(api_base_url, frontend_token, api_key) {
        const ptr0 = passStringToWasm0(api_base_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(frontend_token) ? 0 : passStringToWasm0(frontend_token, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(api_key) ? 0 : passStringToWasm0(api_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_js_new(ptr0, len0, ptr1, len1, ptr2, len2);
        this.__wbg_ptr = ret >>> 0;
        ScamplersClientFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {InstitutionQuery} data
     * @returns {Promise<Institution[]>}
     */
    list_institutions(data) {
        _assertClass(data, InstitutionQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_institutions(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {NewPerson} data
     * @returns {Promise<CreatedUser>}
     */
    ms_login(data) {
        _assertClass(data, NewPerson);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_ms_login(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {PersonQuery} data
     * @returns {Promise<Person[]>}
     */
    list_people(data) {
        _assertClass(data, PersonQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_people(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {NewLab} data
     * @returns {Promise<Lab>}
     */
    create_lab(data) {
        _assertClass(data, NewLab);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_create_lab(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {LabQuery} data
     * @returns {Promise<Lab[]>}
     */
    list_labs(data) {
        _assertClass(data, LabQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_labs(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {LabUpdate} data
     * @returns {Promise<Lab>}
     */
    update_lab(data) {
        _assertClass(data, LabUpdate);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_update_lab(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {SpecimenQuery} data
     * @returns {Promise<Specimen[]>}
     */
    list_specimens(data) {
        _assertClass(data, SpecimenQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_specimens(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {SequencingRunQuery} data
     * @returns {Promise<SequencingRun[]>}
     */
    list_sequencing_runs(data) {
        _assertClass(data, SequencingRunQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_sequencing_runs(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {SuspensionQuery} data
     * @returns {Promise<Suspension[]>}
     */
    list_suspensions(data) {
        _assertClass(data, SuspensionQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_suspensions(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {SuspensionPoolQuery} data
     * @returns {Promise<SuspensionPool[]>}
     */
    list_suspension_pools(data) {
        _assertClass(data, SuspensionPoolQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_suspension_pools(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {ChromiumRunQuery} data
     * @returns {Promise<ChromiumRun[]>}
     */
    list_chromium_runs(data) {
        _assertClass(data, ChromiumRunQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_chromium_runs(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {CdnaQuery} data
     * @returns {Promise<Cdna[]>}
     */
    list_cdna(data) {
        _assertClass(data, CdnaQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_cdna(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {LibraryQuery} data
     * @returns {Promise<Library[]>}
     */
    list_libraries(data) {
        _assertClass(data, LibraryQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_libraries(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {ChromiumDatasetQuery} data
     * @returns {Promise<ChromiumDataset[]>}
     */
    list_chromium_datasets(data) {
        _assertClass(data, ChromiumDatasetQuery);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_chromium_datasets(this.__wbg_ptr, ptr0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<Institution>}
     */
    fetch_institution(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_institution(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<Person>}
     */
    fetch_person(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_person(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<Lab>}
     */
    fetch_lab(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_lab(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<Specimen>}
     */
    fetch_specimen(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_specimen(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<SequencingRun>}
     */
    fetch_sequencing_run(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_sequencing_run(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<Suspension>}
     */
    fetch_suspension(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_suspension(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<SuspensionPool>}
     */
    fetch_suspension_pool(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_suspension_pool(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<ChromiumRun>}
     */
    fetch_chromium_run(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_chromium_run(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<Cdna>}
     */
    fetch_cdna(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_cdna(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<Library>}
     */
    fetch_library(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_library(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} data
     * @returns {Promise<ChromiumDataset>}
     */
    fetch_chromium_dataset(data) {
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplersclient_fetch_chromium_dataset(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} id
     * @param {SpecimenQuery} query
     * @returns {Promise<Specimen[]>}
     */
    list_person_specimens(id, query) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(query, SpecimenQuery);
        var ptr1 = query.__destroy_into_raw();
        const ret = wasm.scamplersclient_list_person_specimens(this.__wbg_ptr, ptr0, len0, ptr1);
        return ret;
    }
}

const ScamplersErrorResponseFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_scamplerserrorresponse_free(ptr >>> 0, 1));

export class ScamplersErrorResponse {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ScamplersErrorResponse.prototype);
        obj.__wbg_ptr = ptr;
        ScamplersErrorResponseFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ScamplersErrorResponseFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_scamplerserrorresponse_free(ptr, 0);
    }
    /**
     * @returns {number | undefined}
     */
    get status() {
        const ret = wasm.__wbg_get_scamplerserrorresponse_status(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set status(arg0) {
        wasm.__wbg_set_scamplerserrorresponse_status(this.__wbg_ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0);
    }
    /**
     * @returns {any}
     */
    get error() {
        const ret = wasm.__wbg_get_scamplerserrorresponse_error(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.scamplerserrorresponse_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.scamplerserrorresponse_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.scamplerserrorresponse_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {ScamplersErrorResponse}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplerserrorresponse_from_json_bytes(ptr0, len0);
        return ScamplersErrorResponse.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {ScamplersErrorResponse}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplerserrorresponse_from_json_string(ptr0, len0);
        return ScamplersErrorResponse.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {ScamplersErrorResponse}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scamplerserrorresponse_from_base64_json(ptr0, len0);
        return ScamplersErrorResponse.__wrap(ret);
    }
}

const SequencingRunFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sequencingrun_free(ptr >>> 0, 1));

export class SequencingRun {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SequencingRun.prototype);
        obj.__wbg_ptr = ptr;
        SequencingRunFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SequencingRunFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sequencingrun_free(ptr, 0);
    }
    /**
     * @returns {SequencingRunSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_sequencingrun_summary(this.__wbg_ptr);
        return SequencingRunSummary.__wrap(ret);
    }
    /**
     * @param {SequencingRunSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, SequencingRunSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_sequencingrun_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string[]}
     */
    get libraries() {
        const ret = wasm.__wbg_get_sequencingrun_libraries(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set libraries(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_sequencingrun_libraries(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.sequencingrun_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.sequencingrun_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.sequencingrun_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SequencingRun}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sequencingrun_from_json_bytes(ptr0, len0);
        return SequencingRun.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SequencingRun}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sequencingrun_from_json_string(ptr0, len0);
        return SequencingRun.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SequencingRun}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sequencingrun_from_base64_json(ptr0, len0);
        return SequencingRun.__wrap(ret);
    }
}

const SequencingRunQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sequencingrunquery_free(ptr >>> 0, 1));

export class SequencingRunQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SequencingRunQuery.prototype);
        obj.__wbg_ptr = ptr;
        SequencingRunQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SequencingRunQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sequencingrunquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_sequencingrunquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_sequencingrunquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_sequencingrunquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_sequencingrunquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_sequencingrunquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_sequencingrunquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.sequencingrunquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.sequencingrunquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.sequencingrunquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SequencingRunQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sequencingrunquery_from_json_bytes(ptr0, len0);
        return SequencingRunQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SequencingRunQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sequencingrunquery_from_json_string(ptr0, len0);
        return SequencingRunQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SequencingRunQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sequencingrunquery_from_base64_json(ptr0, len0);
        return SequencingRunQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.sequencingrunquery_new();
        this.__wbg_ptr = ret >>> 0;
        SequencingRunQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const SequencingRunSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sequencingrunsummary_free(ptr >>> 0, 1));

export class SequencingRunSummary {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SequencingRunSummary.prototype);
        obj.__wbg_ptr = ptr;
        SequencingRunSummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SequencingRunSummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sequencingrunsummary_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_sequencingrunsummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_sequencingrunsummary_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_sequencingrunsummary_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get readable_id() {
        const ret = wasm.__wbg_get_sequencingrunsummary_readable_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set readable_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_sequencingrunsummary_readable_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date}
     */
    get begun_at() {
        const ret = wasm.__wbg_get_sequencingrunsummary_begun_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set begun_at(arg0) {
        wasm.__wbg_set_sequencingrunsummary_begun_at(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Date | undefined}
     */
    get finished_at() {
        const ret = wasm.__wbg_get_sequencingrunsummary_finished_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date | null} [arg0]
     */
    set finished_at(arg0) {
        wasm.__wbg_set_sequencingrunsummary_finished_at(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {string}
     */
    get notes() {
        const ret = wasm.__wbg_get_sequencingrunsummary_notes(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set notes(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_sequencingrunsummary_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.sequencingrunsummary_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.sequencingrunsummary_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.sequencingrunsummary_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SequencingRunSummary}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sequencingrunsummary_from_json_bytes(ptr0, len0);
        return SequencingRunSummary.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SequencingRunSummary}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sequencingrunsummary_from_json_string(ptr0, len0);
        return SequencingRunSummary.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SequencingRunSummary}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sequencingrunsummary_from_base64_json(ptr0, len0);
        return SequencingRunSummary.__wrap(ret);
    }
}

const ServerErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_servererror_free(ptr >>> 0, 1));

export class ServerError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ServerError.prototype);
        obj.__wbg_ptr = ptr;
        ServerErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ServerErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_servererror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        const ret = wasm.__wbg_get_servererror_message(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnagemserror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get raw_response_body() {
        const ret = wasm.__wbg_get_servererror_raw_response_body(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set raw_response_body(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_invalidreferenceerror_referenced_entity(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.servererror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.servererror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.servererror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {ServerError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.servererror_from_json_bytes(ptr0, len0);
        return ServerError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {ServerError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.servererror_from_json_string(ptr0, len0);
        return ServerError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {ServerError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.servererror_from_base64_json(ptr0, len0);
        return ServerError.__wrap(ret);
    }
}

const SpecimenFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_specimen_free(ptr >>> 0, 1));

export class Specimen {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Specimen.prototype);
        obj.__wbg_ptr = ptr;
        SpecimenFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SpecimenFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_specimen_free(ptr, 0);
    }
    /**
     * @returns {SpecimenSummaryWithParents}
     */
    get info() {
        const ret = wasm.__wbg_get_specimen_info(this.__wbg_ptr);
        return SpecimenSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {SpecimenSummaryWithParents} arg0
     */
    set info(arg0) {
        _assertClass(arg0, SpecimenSummaryWithParents);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimen_info(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {SpecimenMeasurement[]}
     */
    get measurements() {
        const ret = wasm.__wbg_get_specimen_measurements(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {SpecimenMeasurement[]} arg0
     */
    set measurements(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimen_measurements(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.specimen_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.specimen_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.specimen_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {Specimen}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimen_from_json_bytes(ptr0, len0);
        return Specimen.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {Specimen}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimen_from_json_string(ptr0, len0);
        return Specimen.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {Specimen}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimen_from_base64_json(ptr0, len0);
        return Specimen.__wrap(ret);
    }
}

const SpecimenMeasurementFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_specimenmeasurement_free(ptr >>> 0, 1));

export class SpecimenMeasurement {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SpecimenMeasurement.prototype);
        obj.__wbg_ptr = ptr;
        SpecimenMeasurementFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof SpecimenMeasurement)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SpecimenMeasurementFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_specimenmeasurement_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_specimenmeasurement_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenmeasurement_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get specimen_id() {
        const ret = wasm.__wbg_get_specimenmeasurement_specimen_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set specimen_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenmeasurement_specimen_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get measured_by() {
        const ret = wasm.__wbg_get_specimenmeasurement_measured_by(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set measured_by(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenmeasurement_measured_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.specimenmeasurement_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.specimenmeasurement_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.specimenmeasurement_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SpecimenMeasurement}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimenmeasurement_from_json_bytes(ptr0, len0);
        return SpecimenMeasurement.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SpecimenMeasurement}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimenmeasurement_from_json_string(ptr0, len0);
        return SpecimenMeasurement.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SpecimenMeasurement}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimenmeasurement_from_base64_json(ptr0, len0);
        return SpecimenMeasurement.__wrap(ret);
    }
}

const SpecimenQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_specimenquery_free(ptr >>> 0, 1));

export class SpecimenQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SpecimenQuery.prototype);
        obj.__wbg_ptr = ptr;
        SpecimenQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SpecimenQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_specimenquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_specimenquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get names() {
        const ret = wasm.__wbg_get_specimenquery_names(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set names(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenquery_names(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get submitters() {
        const ret = wasm.__wbg_get_specimenquery_submitters(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set submitters(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenquery_submitters(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get labs() {
        const ret = wasm.__wbg_get_specimenquery_labs(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set labs(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenquery_labs(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date | undefined}
     */
    get received_before() {
        const ret = wasm.__wbg_get_specimenquery_received_before(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date | null} [arg0]
     */
    set received_before(arg0) {
        wasm.__wbg_set_chromiumrunquery_run_before(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {Date | undefined}
     */
    get received_after() {
        const ret = wasm.__wbg_get_specimenquery_received_after(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date | null} [arg0]
     */
    set received_after(arg0) {
        wasm.__wbg_set_chromiumrunquery_run_after(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {any[]}
     */
    get species() {
        const ret = wasm.__wbg_get_specimenquery_species(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {any[]} arg0
     */
    set species(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenquery_species(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get notes() {
        const ret = wasm.__wbg_get_specimenquery_notes(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set notes(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenquery_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {any[]}
     */
    get types() {
        const ret = wasm.__wbg_get_specimenquery_types(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {any[]} arg0
     */
    set types(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenquery_types(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get storage_buffers() {
        const ret = wasm.__wbg_get_specimenquery_storage_buffers(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set storage_buffers(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenquery_storage_buffers(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {boolean | undefined}
     */
    get frozen() {
        const ret = wasm.__wbg_get_specimenquery_frozen(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
     * @param {boolean | null} [arg0]
     */
    set frozen(arg0) {
        wasm.__wbg_set_specimenquery_frozen(this.__wbg_ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0 ? 1 : 0);
    }
    /**
     * @returns {boolean | undefined}
     */
    get cryopreserved() {
        const ret = wasm.__wbg_get_specimenquery_cryopreserved(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
     * @param {boolean | null} [arg0]
     */
    set cryopreserved(arg0) {
        wasm.__wbg_set_specimenquery_cryopreserved(this.__wbg_ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0 ? 1 : 0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_specimenquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimenquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_chromiumrunquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumrunquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.specimenquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.specimenquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.specimenquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SpecimenQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimenquery_from_json_bytes(ptr0, len0);
        return SpecimenQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SpecimenQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimenquery_from_json_string(ptr0, len0);
        return SpecimenQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SpecimenQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimenquery_from_base64_json(ptr0, len0);
        return SpecimenQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.specimenquery_new();
        this.__wbg_ptr = ret >>> 0;
        SpecimenQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const SpecimenSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_specimensummary_free(ptr >>> 0, 1));

export class SpecimenSummary {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SpecimenSummary.prototype);
        obj.__wbg_ptr = ptr;
        SpecimenSummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SpecimenSummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_specimensummary_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_specimensummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunsummarywithparents_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_specimensummary_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get readable_id() {
        const ret = wasm.__wbg_get_specimensummary_readable_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set readable_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimensummary_readable_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_specimensummary_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimensummary_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date}
     */
    get received_at() {
        const ret = wasm.__wbg_get_specimensummary_received_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set received_at(arg0) {
        wasm.__wbg_set_specimensummary_received_at(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get notes() {
        const ret = wasm.__wbg_get_specimensummary_notes(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set notes(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimensummary_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date | undefined}
     */
    get returned_at() {
        const ret = wasm.__wbg_get_specimensummary_returned_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date | null} [arg0]
     */
    set returned_at(arg0) {
        wasm.__wbg_set_specimensummary_returned_at(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {SpecimenType}
     */
    get type_() {
        const ret = wasm.__wbg_get_specimensummary_type_(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {SpecimenType} arg0
     */
    set type_(arg0) {
        wasm.__wbg_set_specimensummary_type_(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get embedded_in() {
        const ret = wasm.__wbg_get_specimensummary_embedded_in(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set embedded_in(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimensummary_embedded_in(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get fixative() {
        const ret = wasm.__wbg_get_specimensummary_fixative(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set fixative(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimensummary_fixative(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {boolean}
     */
    get frozen() {
        const ret = wasm.__wbg_get_specimensummary_frozen(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set frozen(arg0) {
        wasm.__wbg_set_specimensummary_frozen(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get cryopreserved() {
        const ret = wasm.__wbg_get_specimensummary_cryopreserved(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set cryopreserved(arg0) {
        wasm.__wbg_set_specimensummary_cryopreserved(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get storage_buffer() {
        const ret = wasm.__wbg_get_specimensummary_storage_buffer(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set storage_buffer(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimensummary_storage_buffer(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.specimensummary_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.specimensummary_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.specimensummary_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SpecimenSummary}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimensummary_from_json_bytes(ptr0, len0);
        return SpecimenSummary.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SpecimenSummary}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimensummary_from_json_string(ptr0, len0);
        return SpecimenSummary.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SpecimenSummary}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimensummary_from_base64_json(ptr0, len0);
        return SpecimenSummary.__wrap(ret);
    }
    /**
     * @returns {string[]}
     */
    get species() {
        const ret = wasm.specimensummary_species(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
}

const SpecimenSummaryWithParentsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_specimensummarywithparents_free(ptr >>> 0, 1));

export class SpecimenSummaryWithParents {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SpecimenSummaryWithParents.prototype);
        obj.__wbg_ptr = ptr;
        SpecimenSummaryWithParentsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SpecimenSummaryWithParentsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_specimensummarywithparents_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id_() {
        const ret = wasm.__wbg_get_specimensummarywithparents_id_(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id_(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_specimensummarywithparents_id_(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {SpecimenSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_specimensummarywithparents_summary(this.__wbg_ptr);
        return SpecimenSummary.__wrap(ret);
    }
    /**
     * @param {SpecimenSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, SpecimenSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimensummarywithparents_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {LabSummary}
     */
    get lab() {
        const ret = wasm.__wbg_get_specimensummarywithparents_lab(this.__wbg_ptr);
        return LabSummary.__wrap(ret);
    }
    /**
     * @param {LabSummary} arg0
     */
    set lab(arg0) {
        _assertClass(arg0, LabSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimensummarywithparents_lab(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {PersonSummary}
     */
    get submitted_by() {
        const ret = wasm.__wbg_get_specimensummarywithparents_submitted_by(this.__wbg_ptr);
        return PersonSummary.__wrap(ret);
    }
    /**
     * @param {PersonSummary} arg0
     */
    set submitted_by(arg0) {
        _assertClass(arg0, PersonSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimensummarywithparents_submitted_by(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.specimensummarywithparents_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.specimensummarywithparents_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.specimensummarywithparents_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SpecimenSummaryWithParents}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimensummarywithparents_from_json_bytes(ptr0, len0);
        return SpecimenSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SpecimenSummaryWithParents}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimensummarywithparents_from_json_string(ptr0, len0);
        return SpecimenSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SpecimenSummaryWithParents}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.specimensummarywithparents_from_base64_json(ptr0, len0);
        return SpecimenSummaryWithParents.__wrap(ret);
    }
}

const SuspensionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspension_free(ptr >>> 0, 1));

export class Suspension {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Suspension.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspension_free(ptr, 0);
    }
    /**
     * @returns {SuspensionSummaryWithParents}
     */
    get info() {
        const ret = wasm.__wbg_get_suspension_info(this.__wbg_ptr);
        return SuspensionSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {SuspensionSummaryWithParents} arg0
     */
    set info(arg0) {
        _assertClass(arg0, SuspensionSummaryWithParents);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspension_info(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string[]}
     */
    get prepared_by() {
        const ret = wasm.__wbg_get_suspension_prepared_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set prepared_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspension_prepared_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {SuspensionMeasurement[]}
     */
    get measurements() {
        const ret = wasm.__wbg_get_suspension_measurements(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {SuspensionMeasurement[]} arg0
     */
    set measurements(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspension_measurements(this.__wbg_ptr, ptr0, len0);
    }
}

const SuspensionMeasurementFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionmeasurement_free(ptr >>> 0, 1));

export class SuspensionMeasurement {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionMeasurement.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionMeasurementFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof SuspensionMeasurement)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionMeasurementFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensionmeasurement_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_suspensionmeasurement_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionmeasurement_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get measured_by() {
        const ret = wasm.__wbg_get_suspensionmeasurement_measured_by(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set measured_by(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionmeasurement_measured_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get suspension_id() {
        const ret = wasm.__wbg_get_suspensionmeasurement_suspension_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set suspension_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionmeasurement_suspension_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.suspensionmeasurement_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.suspensionmeasurement_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.suspensionmeasurement_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SuspensionMeasurement}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionmeasurement_from_json_bytes(ptr0, len0);
        return SuspensionMeasurement.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SuspensionMeasurement}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionmeasurement_from_json_string(ptr0, len0);
        return SuspensionMeasurement.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SuspensionMeasurement}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionmeasurement_from_base64_json(ptr0, len0);
        return SuspensionMeasurement.__wrap(ret);
    }
}

const SuspensionPoolFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionpool_free(ptr >>> 0, 1));

export class SuspensionPool {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionPool.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionPoolFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionPoolFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensionpool_free(ptr, 0);
    }
    /**
     * @returns {SuspensionPoolSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_suspensionpool_summary(this.__wbg_ptr);
        return SuspensionPoolSummary.__wrap(ret);
    }
    /**
     * @param {SuspensionPoolSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, SuspensionPoolSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspensionpool_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {SuspensionSummary[]}
     */
    get suspensions() {
        const ret = wasm.__wbg_get_suspensionpool_suspensions(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {SuspensionSummary[]} arg0
     */
    set suspensions(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpool_suspensions(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get preparers() {
        const ret = wasm.__wbg_get_suspensionpool_preparers(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set preparers(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpool_preparers(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {SuspensionPoolMeasurement[]}
     */
    get measurements() {
        const ret = wasm.__wbg_get_suspensionpool_measurements(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {SuspensionPoolMeasurement[]} arg0
     */
    set measurements(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpool_measurements(this.__wbg_ptr, ptr0, len0);
    }
}

const SuspensionPoolMeasurementFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionpoolmeasurement_free(ptr >>> 0, 1));

export class SuspensionPoolMeasurement {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionPoolMeasurement.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionPoolMeasurementFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof SuspensionPoolMeasurement)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionPoolMeasurementFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensionpoolmeasurement_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_suspensionpoolmeasurement_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpoolmeasurement_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get pool_id() {
        const ret = wasm.__wbg_get_suspensionpoolmeasurement_pool_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set pool_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpoolmeasurement_pool_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get measured_by() {
        const ret = wasm.__wbg_get_suspensionpoolmeasurement_measured_by(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set measured_by(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpoolmeasurement_measured_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.suspensionpoolmeasurement_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.suspensionpoolmeasurement_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.suspensionpoolmeasurement_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SuspensionPoolMeasurement}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionpoolmeasurement_from_json_bytes(ptr0, len0);
        return SuspensionPoolMeasurement.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SuspensionPoolMeasurement}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionpoolmeasurement_from_json_string(ptr0, len0);
        return SuspensionPoolMeasurement.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SuspensionPoolMeasurement}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionpoolmeasurement_from_base64_json(ptr0, len0);
        return SuspensionPoolMeasurement.__wrap(ret);
    }
}

const SuspensionPoolQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionpoolquery_free(ptr >>> 0, 1));

export class SuspensionPoolQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionPoolQuery.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionPoolQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionPoolQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensionpoolquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_suspensionpoolquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpoolquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_suspensionpoolquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpoolquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_suspensionpoolquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspensionpoolquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.suspensionpoolquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.suspensionpoolquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.suspensionpoolquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SuspensionPoolQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionpoolquery_from_json_bytes(ptr0, len0);
        return SuspensionPoolQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SuspensionPoolQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionpoolquery_from_json_string(ptr0, len0);
        return SuspensionPoolQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SuspensionPoolQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionpoolquery_from_base64_json(ptr0, len0);
        return SuspensionPoolQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.suspensionpoolquery_new();
        this.__wbg_ptr = ret >>> 0;
        SuspensionPoolQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const SuspensionPoolSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionpoolsummary_free(ptr >>> 0, 1));

export class SuspensionPoolSummary {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionPoolSummary.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionPoolSummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionPoolSummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensionpoolsummary_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_suspensionpoolsummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpoolsummary_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_suspensionpoolsummary_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get readable_id() {
        const ret = wasm.__wbg_get_suspensionpoolsummary_readable_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set readable_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpoolsummary_readable_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_suspensionpoolsummary_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpoolsummary_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date}
     */
    get pooled_at() {
        const ret = wasm.__wbg_get_suspensionpoolsummary_pooled_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set pooled_at(arg0) {
        wasm.__wbg_set_suspensionpoolsummary_pooled_at(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get notes() {
        const ret = wasm.__wbg_get_suspensionpoolsummary_notes(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set notes(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionpoolsummary_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.suspensionpoolsummary_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.suspensionpoolsummary_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.suspensionpoolsummary_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SuspensionPoolSummary}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionpoolsummary_from_json_bytes(ptr0, len0);
        return SuspensionPoolSummary.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SuspensionPoolSummary}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionpoolsummary_from_json_string(ptr0, len0);
        return SuspensionPoolSummary.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SuspensionPoolSummary}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionpoolsummary_from_base64_json(ptr0, len0);
        return SuspensionPoolSummary.__wrap(ret);
    }
}

const SuspensionQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionquery_free(ptr >>> 0, 1));

export class SuspensionQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionQuery.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensionquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_suspensionquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {SpecimenQuery | undefined}
     */
    get parent_specimen() {
        const ret = wasm.__wbg_get_suspensionquery_parent_specimen(this.__wbg_ptr);
        return ret === 0 ? undefined : SpecimenQuery.__wrap(ret);
    }
    /**
     * @param {SpecimenQuery | null} [arg0]
     */
    set parent_specimen(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, SpecimenQuery);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_suspensionquery_parent_specimen(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_suspensionquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_sequencingrunquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_sequencingrunquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.suspensionquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.suspensionquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.suspensionquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SuspensionQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionquery_from_json_bytes(ptr0, len0);
        return SuspensionQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SuspensionQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionquery_from_json_string(ptr0, len0);
        return SuspensionQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SuspensionQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionquery_from_base64_json(ptr0, len0);
        return SuspensionQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.suspensionquery_new();
        this.__wbg_ptr = ret >>> 0;
        SuspensionQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const SuspensionSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionsummary_free(ptr >>> 0, 1));

export class SuspensionSummary {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionSummary.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionSummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof SuspensionSummary)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionSummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensionsummary_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_suspensionsummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionsummary_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_suspensionsummary_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get readable_id() {
        const ret = wasm.__wbg_get_suspensionsummary_readable_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set readable_id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionsummary_readable_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get biological_material() {
        const ret = wasm.__wbg_get_suspensionsummary_biological_material(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set biological_material(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionsummary_biological_material(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get pooled_into() {
        const ret = wasm.__wbg_get_suspensionsummary_pooled_into(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set pooled_into(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionsummary_pooled_into(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date | undefined}
     */
    get created_at() {
        const ret = wasm.__wbg_get_suspensionsummary_created_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date | null} [arg0]
     */
    set created_at(arg0) {
        wasm.__wbg_set_suspensionsummary_created_at(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {number | undefined}
     */
    get lysis_duration_minutes() {
        const ret = wasm.__wbg_get_suspensionsummary_lysis_duration_minutes(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set lysis_duration_minutes(arg0) {
        wasm.__wbg_set_suspensionsummary_lysis_duration_minutes(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : Math.fround(arg0));
    }
    /**
     * @returns {number}
     */
    get target_cell_recovery() {
        const ret = wasm.__wbg_get_suspensionsummary_target_cell_recovery(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set target_cell_recovery(arg0) {
        wasm.__wbg_set_suspensionsummary_target_cell_recovery(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get notes() {
        const ret = wasm.__wbg_get_suspensionsummary_notes(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set notes(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionsummary_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.suspensionsummary_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.suspensionsummary_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.suspensionsummary_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SuspensionSummary}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionsummary_from_json_bytes(ptr0, len0);
        return SuspensionSummary.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SuspensionSummary}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionsummary_from_json_string(ptr0, len0);
        return SuspensionSummary.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SuspensionSummary}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionsummary_from_base64_json(ptr0, len0);
        return SuspensionSummary.__wrap(ret);
    }
}

const SuspensionSummaryWithParentsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionsummarywithparents_free(ptr >>> 0, 1));

export class SuspensionSummaryWithParents {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionSummaryWithParents.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionSummaryWithParentsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionSummaryWithParentsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensionsummarywithparents_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id_() {
        const ret = wasm.__wbg_get_suspensionsummarywithparents_id_(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id_(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_suspensionsummarywithparents_id_(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {SuspensionSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_suspensionsummarywithparents_summary(this.__wbg_ptr);
        return SuspensionSummary.__wrap(ret);
    }
    /**
     * @param {SuspensionSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, SuspensionSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspensionsummarywithparents_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {SpecimenSummary}
     */
    get parent_specimen() {
        const ret = wasm.__wbg_get_suspensionsummarywithparents_parent_specimen(this.__wbg_ptr);
        return SpecimenSummary.__wrap(ret);
    }
    /**
     * @param {SpecimenSummary} arg0
     */
    set parent_specimen(arg0) {
        _assertClass(arg0, SpecimenSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspensionsummarywithparents_parent_specimen(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {MultiplexingTag | undefined}
     */
    get multiplexing_tag() {
        const ret = wasm.__wbg_get_suspensionsummarywithparents_multiplexing_tag(this.__wbg_ptr);
        return ret === 0 ? undefined : MultiplexingTag.__wrap(ret);
    }
    /**
     * @param {MultiplexingTag | null} [arg0]
     */
    set multiplexing_tag(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, MultiplexingTag);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_suspensionsummarywithparents_multiplexing_tag(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.suspensionsummarywithparents_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.suspensionsummarywithparents_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.suspensionsummarywithparents_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SuspensionSummaryWithParents}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionsummarywithparents_from_json_bytes(ptr0, len0);
        return SuspensionSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SuspensionSummaryWithParents}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionsummarywithparents_from_json_string(ptr0, len0);
        return SuspensionSummaryWithParents.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SuspensionSummaryWithParents}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensionsummarywithparents_from_base64_json(ptr0, len0);
        return SuspensionSummaryWithParents.__wrap(ret);
    }
}

const TenxAssayFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tenxassay_free(ptr >>> 0, 1));

export class TenxAssay {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TenxAssay.prototype);
        obj.__wbg_ptr = ptr;
        TenxAssayFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TenxAssayFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tenxassay_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_tenxassay_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labsummary_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get links() {
        const ret = wasm.__wbg_get_tenxassay_links(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_tenxassay_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_tenxassay_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {SampleMultiplexing | undefined}
     */
    get sample_multiplexing() {
        const ret = wasm.__wbg_get_tenxassay_sample_multiplexing(this.__wbg_ptr);
        return ret === 5 ? undefined : ret;
    }
    /**
     * @param {SampleMultiplexing | null} [arg0]
     */
    set sample_multiplexing(arg0) {
        wasm.__wbg_set_tenxassay_sample_multiplexing(this.__wbg_ptr, isLikeNone(arg0) ? 5 : arg0);
    }
    /**
     * @returns {string}
     */
    get chemistry_version() {
        const ret = wasm.__wbg_get_tenxassay_chemistry_version(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set chemistry_version(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_tenxassay_chemistry_version(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get protocol_url() {
        const ret = wasm.__wbg_get_tenxassay_protocol_url(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set protocol_url(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_tenxassay_protocol_url(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get chromium_chip() {
        const ret = wasm.__wbg_get_tenxassay_chromium_chip(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set chromium_chip(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_personsummary_orcid(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.tenxassay_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.tenxassay_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.tenxassay_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {TenxAssay}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.tenxassay_from_json_bytes(ptr0, len0);
        return TenxAssay.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {TenxAssay}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.tenxassay_from_json_string(ptr0, len0);
        return TenxAssay.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {TenxAssay}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.tenxassay_from_base64_json(ptr0, len0);
        return TenxAssay.__wrap(ret);
    }
}

const TenxAssayQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tenxassayquery_free(ptr >>> 0, 1));

export class TenxAssayQuery {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TenxAssayQuery.prototype);
        obj.__wbg_ptr = ptr;
        TenxAssayQueryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TenxAssayQueryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tenxassayquery_free(ptr, 0);
    }
    /**
     * @returns {string[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_tenxassayquery_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get names() {
        const ret = wasm.__wbg_get_tenxassayquery_names(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set names(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labquery_names(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {any[]}
     */
    get sample_multiplexing() {
        const ret = wasm.__wbg_get_tenxassayquery_sample_multiplexing(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {any[]} arg0
     */
    set sample_multiplexing(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_tenxassayquery_sample_multiplexing(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get chemistry_versions() {
        const ret = wasm.__wbg_get_tenxassayquery_chemistry_versions(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set chemistry_versions(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_tenxassayquery_chemistry_versions(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get chromium_chips() {
        const ret = wasm.__wbg_get_tenxassayquery_chromium_chips(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set chromium_chips(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_tenxassayquery_chromium_chips(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {OrderBy[]}
     */
    get order_by() {
        const ret = wasm.__wbg_get_tenxassayquery_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} arg0
     */
    set order_by(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_tenxassayquery_order_by(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_labquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_labquery_pagination(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.tenxassayquery_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.tenxassayquery_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.tenxassayquery_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {TenxAssayQuery}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.tenxassayquery_from_json_bytes(ptr0, len0);
        return TenxAssayQuery.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {TenxAssayQuery}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.tenxassayquery_from_json_string(ptr0, len0);
        return TenxAssayQuery.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {TenxAssayQuery}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.tenxassayquery_from_base64_json(ptr0, len0);
        return TenxAssayQuery.__wrap(ret);
    }
    constructor() {
        const ret = wasm.tenxassayquery_new();
        this.__wbg_ptr = ret >>> 0;
        TenxAssayQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

export function __wbg_abort_6665281623826052(arg0) {
    arg0.abort();
};

export function __wbg_abort_c11a5d245a242912(arg0, arg1) {
    arg0.abort(arg1);
};

export function __wbg_append_3e86b0cd6215edd8() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    arg0.append(v0, v1);
}, arguments) };

export function __wbg_arrayBuffer_55e4a430671abfd8() { return handleError(function (arg0) {
    const ret = arg0.arrayBuffer();
    return ret;
}, arguments) };

export function __wbg_call_2f8d426a20a307fe() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments) };

export function __wbg_call_f53f0647ceb9c567() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.call(arg1, arg2);
    return ret;
}, arguments) };

export function __wbg_cdna_new(arg0) {
    const ret = Cdna.__wrap(arg0);
    return ret;
};

export function __wbg_cdnameasurement_new(arg0) {
    const ret = CdnaMeasurement.__wrap(arg0);
    return ret;
};

export function __wbg_cdnameasurement_unwrap(arg0) {
    const ret = CdnaMeasurement.__unwrap(arg0);
    return ret;
};

export function __wbg_chromiumdataset_new(arg0) {
    const ret = ChromiumDataset.__wrap(arg0);
    return ret;
};

export function __wbg_chromiumrun_new(arg0) {
    const ret = ChromiumRun.__wrap(arg0);
    return ret;
};

export function __wbg_clearTimeout_6222fede17abcb1a(arg0) {
    const ret = clearTimeout(arg0);
    return ret;
};

export function __wbg_createduser_new(arg0) {
    const ret = CreatedUser.__wrap(arg0);
    return ret;
};

export function __wbg_done_4a7743b6f942c9f3(arg0) {
    const ret = arg0.done;
    return ret;
};

export function __wbg_fetch_9885d2e26ad251bb(arg0, arg1) {
    const ret = arg0.fetch(arg1);
    return ret;
};

export function __wbg_fetch_f156d10be9a5c88a(arg0) {
    const ret = fetch(arg0);
    return ret;
};

export function __wbg_gems_new(arg0) {
    const ret = Gems.__wrap(arg0);
    return ret;
};

export function __wbg_gems_unwrap(arg0) {
    const ret = Gems.__unwrap(arg0);
    return ret;
};

export function __wbg_getTime_5b1dd03bb6d4b784(arg0) {
    const ret = arg0.getTime();
    return ret;
};

export function __wbg_get_27b4bcbec57323ca() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(arg0, arg1);
    return ret;
}, arguments) };

export function __wbg_has_85abdd8aeb8edebf() { return handleError(function (arg0, arg1) {
    const ret = Reflect.has(arg0, arg1);
    return ret;
}, arguments) };

export function __wbg_headers_177bc880a5823968(arg0) {
    const ret = arg0.headers;
    return ret;
};

export function __wbg_instanceof_Response_0ab386c6818f788a(arg0) {
    let result;
    try {
        result = arg0 instanceof Response;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_institution_new(arg0) {
    const ret = Institution.__wrap(arg0);
    return ret;
};

export function __wbg_iterator_96378c3c9a17347c() {
    const ret = Symbol.iterator;
    return ret;
};

export function __wbg_lab_new(arg0) {
    const ret = Lab.__wrap(arg0);
    return ret;
};

export function __wbg_length_904c0910ed998bf3(arg0) {
    const ret = arg0.length;
    return ret;
};

export function __wbg_library_new(arg0) {
    const ret = Library.__wrap(arg0);
    return ret;
};

export function __wbg_librarymeasurement_new(arg0) {
    const ret = LibraryMeasurement.__wrap(arg0);
    return ret;
};

export function __wbg_librarymeasurement_unwrap(arg0) {
    const ret = LibraryMeasurement.__unwrap(arg0);
    return ret;
};

export function __wbg_librarytypespecification_new(arg0) {
    const ret = LibraryTypeSpecification.__wrap(arg0);
    return ret;
};

export function __wbg_librarytypespecification_unwrap(arg0) {
    const ret = LibraryTypeSpecification.__unwrap(arg0);
    return ret;
};

export function __wbg_new_12588505388d0897() { return handleError(function () {
    const ret = new Headers();
    return ret;
}, arguments) };

export function __wbg_new_1930cbb8d9ffc31b() {
    const ret = new Object();
    return ret;
};

export function __wbg_new_56407f99198feff7() {
    const ret = new Map();
    return ret;
};

export function __wbg_new_6a8b180049d9484e() { return handleError(function () {
    const ret = new AbortController();
    return ret;
}, arguments) };

export function __wbg_new_9190433fb67ed635(arg0) {
    const ret = new Uint8Array(arg0);
    return ret;
};

export function __wbg_new_d5e3800b120e37e1(arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_adapter_1120(a, state0.b, arg0, arg1);
            } finally {
                state0.a = a;
            }
        };
        const ret = new Promise(cb0);
        return ret;
    } finally {
        state0.a = state0.b = 0;
    }
};

export function __wbg_new_eb6fa6c1e9ac9fb7(arg0) {
    const ret = new Date(arg0);
    return ret;
};

export function __wbg_newfromslice_d0d56929c6d9c842(arg0, arg1) {
    const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
    return ret;
};

export function __wbg_newnoargs_a81330f6e05d8aca(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Function(v0);
    return ret;
};

export function __wbg_newwithstrandinit_e8e22e9851f3c2fe() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Request(v0, arg2);
    return ret;
}, arguments) };

export function __wbg_next_2e6b37020ac5fe58() { return handleError(function (arg0) {
    const ret = arg0.next();
    return ret;
}, arguments) };

export function __wbg_next_3de8f2669431a3ff(arg0) {
    const ret = arg0.next;
    return ret;
};

export function __wbg_orderby_new(arg0) {
    const ret = OrderBy.__wrap(arg0);
    return ret;
};

export function __wbg_orderby_unwrap(arg0) {
    const ret = OrderBy.__unwrap(arg0);
    return ret;
};

export function __wbg_person_new(arg0) {
    const ret = Person.__wrap(arg0);
    return ret;
};

export function __wbg_personsummary_new(arg0) {
    const ret = PersonSummary.__wrap(arg0);
    return ret;
};

export function __wbg_personsummary_unwrap(arg0) {
    const ret = PersonSummary.__unwrap(arg0);
    return ret;
};

export function __wbg_prototypesetcall_c5f74efd31aea86b(arg0, arg1, arg2) {
    Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
};

export function __wbg_queueMicrotask_bcc6e26d899696db(arg0) {
    const ret = arg0.queueMicrotask;
    return ret;
};

export function __wbg_queueMicrotask_f24a794d09c42640(arg0) {
    queueMicrotask(arg0);
};

export function __wbg_resolve_5775c0ef9222f556(arg0) {
    const ret = Promise.resolve(arg0);
    return ret;
};

export function __wbg_scamplerserrorresponse_new(arg0) {
    const ret = ScamplersErrorResponse.__wrap(arg0);
    return ret;
};

export function __wbg_sequencingrun_new(arg0) {
    const ret = SequencingRun.__wrap(arg0);
    return ret;
};

export function __wbg_setTimeout_2b339866a2aa3789(arg0, arg1) {
    const ret = setTimeout(arg0, arg1);
    return ret;
};

export function __wbg_set_31197016f65a6a19(arg0, arg1, arg2) {
    const ret = arg0.set(arg1, arg2);
    return ret;
};

export function __wbg_setbody_e324371c31597f2a(arg0, arg1) {
    arg0.body = arg1;
};

export function __wbg_setcache_7c95e3469a5bfb76(arg0, arg1) {
    arg0.cache = __wbindgen_enum_RequestCache[arg1];
};

export function __wbg_setcredentials_55a9317ed2777533(arg0, arg1) {
    arg0.credentials = __wbindgen_enum_RequestCredentials[arg1];
};

export function __wbg_setheaders_ac0b1e4890a949cd(arg0, arg1) {
    arg0.headers = arg1;
};

export function __wbg_setmethod_9ce6e95af1ae0eaf(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.method = v0;
};

export function __wbg_setmode_b89d1784e7e7f118(arg0, arg1) {
    arg0.mode = __wbindgen_enum_RequestMode[arg1];
};

export function __wbg_setsignal_e663c6d962763cd5(arg0, arg1) {
    arg0.signal = arg1;
};

export function __wbg_signal_bdb003fe19e53a13(arg0) {
    const ret = arg0.signal;
    return ret;
};

export function __wbg_specimen_new(arg0) {
    const ret = Specimen.__wrap(arg0);
    return ret;
};

export function __wbg_specimenmeasurement_new(arg0) {
    const ret = SpecimenMeasurement.__wrap(arg0);
    return ret;
};

export function __wbg_specimenmeasurement_unwrap(arg0) {
    const ret = SpecimenMeasurement.__unwrap(arg0);
    return ret;
};

export function __wbg_static_accessor_GLOBAL_1f13249cc3acc96d() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_GLOBAL_THIS_df7ae94b1e0ed6a3() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_SELF_6265471db3b3c228() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_WINDOW_16fb482f8ec52863() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_status_31874648c8651949(arg0) {
    const ret = arg0.status;
    return ret;
};

export function __wbg_stringify_1f41b6198e0932e0() { return handleError(function (arg0) {
    const ret = JSON.stringify(arg0);
    return ret;
}, arguments) };

export function __wbg_suspension_new(arg0) {
    const ret = Suspension.__wrap(arg0);
    return ret;
};

export function __wbg_suspensionmeasurement_new(arg0) {
    const ret = SuspensionMeasurement.__wrap(arg0);
    return ret;
};

export function __wbg_suspensionmeasurement_unwrap(arg0) {
    const ret = SuspensionMeasurement.__unwrap(arg0);
    return ret;
};

export function __wbg_suspensionpool_new(arg0) {
    const ret = SuspensionPool.__wrap(arg0);
    return ret;
};

export function __wbg_suspensionpoolmeasurement_new(arg0) {
    const ret = SuspensionPoolMeasurement.__wrap(arg0);
    return ret;
};

export function __wbg_suspensionpoolmeasurement_unwrap(arg0) {
    const ret = SuspensionPoolMeasurement.__unwrap(arg0);
    return ret;
};

export function __wbg_suspensionsummary_new(arg0) {
    const ret = SuspensionSummary.__wrap(arg0);
    return ret;
};

export function __wbg_suspensionsummary_unwrap(arg0) {
    const ret = SuspensionSummary.__unwrap(arg0);
    return ret;
};

export function __wbg_then_8d2fcccde5380a03(arg0, arg1, arg2) {
    const ret = arg0.then(arg1, arg2);
    return ret;
};

export function __wbg_then_9cc266be2bf537b6(arg0, arg1) {
    const ret = arg0.then(arg1);
    return ret;
};

export function __wbg_url_d5273b9e10503471(arg0, arg1) {
    const ret = arg1.url;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_value_09d0b4eaab48b91d(arg0) {
    const ret = arg0.value;
    return ret;
};

export function __wbg_wbindgencbdrop_a85ed476c6a370b9(arg0) {
    const obj = arg0.original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};

export function __wbg_wbindgendebugstring_bb652b1bc2061b6d(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_wbindgenisfunction_ea72b9d66a0e1705(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
};

export function __wbg_wbindgenisobject_dfe064a121d87553(arg0) {
    const val = arg0;
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};

export function __wbg_wbindgenisundefined_71f08a6ade4354e7(arg0) {
    const ret = arg0 === undefined;
    return ret;
};

export function __wbg_wbindgennumberget_d855f947247a3fbc(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
};

export function __wbg_wbindgenstringget_43fe05afe34b0cb1(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_wbindgenthrow_4c11a24fca429ccf(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    throw new Error(v0);
};

export function __wbg_wbindgentryintonumber_c9e75d941453b6d7(arg0) {
    let result;
    try { result = +arg0 } catch (e) { result = e }
    const ret = result;
    return ret;
};

export function __wbindgen_cast_0f8f63afe78cd418(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("Specimen")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_1b6984ddff73948d(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("Lab")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_375610b97d04cca6(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("Suspension")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_3a1b5e92d563e471(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("Library")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_3e1ff392f7dfba6c(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("Cdna")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_45c41c2f89ce6d9f(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("SuspensionPool")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_7e9c58eeb11b0a6f(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    // Cast intrinsic for `Ref(CachedString) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_830d2e277bf97812(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 457, function: Function { arguments: [Externref], shim_idx: 458, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, 457, __wbg_adapter_13);
    return ret;
};

export function __wbindgen_cast_9ed9d84624444210(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("ChromiumDataset")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_b13b63fe807b76b9(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("ChromiumRun")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_c15a5e554ce1a329(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("SequencingRun")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_d40d1de55fe59430(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 439, function: Function { arguments: [], shim_idx: 440, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, 439, __wbg_adapter_6);
    return ret;
};

export function __wbindgen_cast_d6cd19b81560fd6e(arg0) {
    // Cast intrinsic for `F64 -> Externref`.
    const ret = arg0;
    return ret;
};

export function __wbindgen_cast_e47be5220b4874d6(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("Institution")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_cast_e90871800fe8ca73(arg0, arg1) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // Cast intrinsic for `Vector(NamedExternref("Person")) -> Externref`.
    const ret = v0;
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

