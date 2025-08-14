let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
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

let WASM_VECTOR_LEN = 0;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

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

function isLikeNone(x) {
    return x === undefined || x === null;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getFromExternrefTable0(idx) { return wasm.__wbindgen_export_2.get(idx); }

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getFromExternrefTable0(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_2.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
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

let cachedFloat32ArrayMemory0 = null;

function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
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
 * @enum {0 | 1 | 2}
 */
export const UserRole = Object.freeze({
    AppAdmin: 0, "0": "AppAdmin",
    BiologyStaff: 1, "1": "BiologyStaff",
    ComputationalStaff: 2, "2": "ComputationalStaff",
});

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
     * @returns {any[]}
     */
    get expected_library_types() {
        const ret = wasm.__wbg_get_cdnalibrarytypeerror_expected_library_types(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {any[]} arg0
     */
    set expected_library_types(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnalibrarytypeerror_expected_library_types(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {any[]}
     */
    get found_library_types() {
        const ret = wasm.__wbg_get_cdnalibrarytypeerror_found_library_types(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {any[]} arg0
     */
    set found_library_types(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnalibrarytypeerror_found_library_types(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Float32Array}
     */
    get expected_volumes() {
        const ret = wasm.__wbg_get_cdnalibrarytypeerror_expected_volumes(this.__wbg_ptr);
        var v1 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Float32Array} arg0
     */
    set expected_volumes(arg0) {
        const ptr0 = passArrayF32ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnalibrarytypeerror_expected_volumes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Float32Array}
     */
    get found_volumes() {
        const ret = wasm.__wbg_get_cdnalibrarytypeerror_found_volumes(this.__wbg_ptr);
        var v1 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Float32Array} arg0
     */
    set found_volumes(arg0) {
        const ptr0 = passArrayF32ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnalibrarytypeerror_found_volumes(this.__wbg_ptr, ptr0, len0);
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

const CreatedUserFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_createduser_free(ptr >>> 0, 1));

export class CreatedUser {

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

const DatasetCmdlineErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_datasetcmdlineerror_free(ptr >>> 0, 1));

export class DatasetCmdlineError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DatasetCmdlineError.prototype);
        obj.__wbg_ptr = ptr;
        DatasetCmdlineErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DatasetCmdlineErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_datasetcmdlineerror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get chemistry() {
        const ret = wasm.__wbg_get_datasetcmdlineerror_chemistry(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set chemistry(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_datasetcmdlineerror_chemistry(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string[]}
     */
    get expected_cmdlines() {
        const ret = wasm.__wbg_get_datasetcmdlineerror_expected_cmdlines(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string[]} arg0
     */
    set expected_cmdlines(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_datasetcmdlineerror_expected_cmdlines(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get found_cmdline() {
        const ret = wasm.__wbg_get_datasetcmdlineerror_found_cmdline(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set found_cmdline(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_datasetcmdlineerror_found_cmdline(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.datasetcmdlineerror_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.datasetcmdlineerror_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.datasetcmdlineerror_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {DatasetCmdlineError}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datasetcmdlineerror_from_json_bytes(ptr0, len0);
        return DatasetCmdlineError.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {DatasetCmdlineError}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datasetcmdlineerror_from_json_string(ptr0, len0);
        return DatasetCmdlineError.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {DatasetCmdlineError}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.datasetcmdlineerror_from_base64_json(ptr0, len0);
        return DatasetCmdlineError.__wrap(ret);
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
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_institutionquery_name(this.__wbg_ptr);
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
        wasm.__wbg_set_institutionquery_name(this.__wbg_ptr, ptr0, len0);
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
    /**
     * @returns {OrderBy[]}
     */
    get get_order_by() {
        const ret = wasm.institutionquery_get_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} orderings
     */
    set order_by(orderings) {
        const ptr0 = passArrayJsValueToWasm0(orderings, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.institutionquery_set_order_by(this.__wbg_ptr, ptr0, len0);
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
        wasm.__wbg_set_datasetcmdlineerror_found_cmdline(this.__wbg_ptr, ptr0, len0);
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
        wasm.__wbg_set_datasetcmdlineerror_chemistry(this.__wbg_ptr, ptr0, len0);
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
        wasm.__wbg_set_newperson_name(this.__wbg_ptr, ptr0, len0);
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
     * @returns {PersonCore}
     */
    get core() {
        const ret = wasm.__wbg_get_person_core(this.__wbg_ptr);
        return PersonCore.__wrap(ret);
    }
    /**
     * @param {PersonCore} arg0
     */
    set core(arg0) {
        _assertClass(arg0, PersonCore);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_person_core(this.__wbg_ptr, ptr0);
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
}

const PersonCoreFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_personcore_free(ptr >>> 0, 1));

export class PersonCore {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PersonCore.prototype);
        obj.__wbg_ptr = ptr;
        PersonCoreFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PersonCoreFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_personcore_free(ptr, 0);
    }
    /**
     * @returns {PersonSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_personcore_summary(this.__wbg_ptr);
        return PersonSummary.__wrap(ret);
    }
    /**
     * @param {PersonSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, PersonSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_personcore_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Institution}
     */
    get institution() {
        const ret = wasm.__wbg_get_personcore_institution(this.__wbg_ptr);
        return Institution.__wrap(ret);
    }
    /**
     * @param {Institution} arg0
     */
    set institution(arg0) {
        _assertClass(arg0, Institution);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_personcore_institution(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.personcore_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.personcore_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.personcore_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {PersonCore}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personcore_from_json_bytes(ptr0, len0);
        return PersonCore.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {PersonCore}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personcore_from_json_string(ptr0, len0);
        return PersonCore.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {PersonCore}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.personcore_from_base64_json(ptr0, len0);
        return PersonCore.__wrap(ret);
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
        wasm.__wbg_set_personquery_ids(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_personquery_name(this.__wbg_ptr);
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
        wasm.__wbg_set_personquery_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get email() {
        const ret = wasm.__wbg_get_personquery_email(this.__wbg_ptr);
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
        wasm.__wbg_set_personquery_email(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get orcid() {
        const ret = wasm.__wbg_get_personquery_orcid(this.__wbg_ptr);
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
        wasm.__wbg_set_personquery_orcid(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get ms_user_id() {
        const ret = wasm.__wbg_get_personquery_ms_user_id(this.__wbg_ptr);
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
        wasm.__wbg_set_personquery_ms_user_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_personquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_personquery_pagination(this.__wbg_ptr, ptr0);
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
    /**
     * @returns {OrderBy[]}
     */
    get get_order_by() {
        const ret = wasm.personquery_get_order_by(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {OrderBy[]} orderings
     */
    set order_by(orderings) {
        const ptr0 = passArrayJsValueToWasm0(orderings, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.personquery_set_order_by(this.__wbg_ptr, ptr0, len0);
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
        wasm.__wbg_set_personsummary_id(this.__wbg_ptr, ptr0, len0);
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
        wasm.__wbg_set_datasetcmdlineerror_found_cmdline(this.__wbg_ptr, ptr0, len0);
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
        wasm.__wbg_set_specimensummary_id(this.__wbg_ptr, ptr0, len0);
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
     * @returns {string}
     */
    get type_() {
        const ret = wasm.__wbg_get_specimensummary_type_(this.__wbg_ptr);
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
        wasm.__wbg_set_specimensummary_type_(this.__wbg_ptr, ptr0, len0);
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
}

const SuspensionCoreFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensioncore_free(ptr >>> 0, 1));

export class SuspensionCore {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionCore.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionCoreFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionCoreFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensioncore_free(ptr, 0);
    }
    /**
     * @returns {SuspensionSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_suspensioncore_summary(this.__wbg_ptr);
        return SuspensionSummary.__wrap(ret);
    }
    /**
     * @param {SuspensionSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, SuspensionSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspensioncore_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {SpecimenSummary}
     */
    get parent_specimen() {
        const ret = wasm.__wbg_get_suspensioncore_parent_specimen(this.__wbg_ptr);
        return SpecimenSummary.__wrap(ret);
    }
    /**
     * @param {SpecimenSummary} arg0
     */
    set parent_specimen(arg0) {
        _assertClass(arg0, SpecimenSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspensioncore_parent_specimen(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {MultiplexingTag}
     */
    get multiplexing_tag() {
        const ret = wasm.__wbg_get_suspensioncore_multiplexing_tag(this.__wbg_ptr);
        return MultiplexingTag.__wrap(ret);
    }
    /**
     * @param {MultiplexingTag} arg0
     */
    set multiplexing_tag(arg0) {
        _assertClass(arg0, MultiplexingTag);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspensioncore_multiplexing_tag(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Uint8Array}
     */
    to_json_bytes() {
        const ret = wasm.suspensioncore_to_json_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    to_json_string() {
        const ret = wasm.suspensioncore_to_json_string(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    to_base64_json() {
        const ret = wasm.suspensioncore_to_base64_json(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {Uint8Array} json_bytes
     * @returns {SuspensionCore}
     */
    static from_json_bytes(json_bytes) {
        const ptr0 = passArray8ToWasm0(json_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensioncore_from_json_bytes(ptr0, len0);
        return SuspensionCore.__wrap(ret);
    }
    /**
     * @param {string} json_str
     * @returns {SuspensionCore}
     */
    static from_json_string(json_str) {
        const ptr0 = passStringToWasm0(json_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensioncore_from_json_string(ptr0, len0);
        return SuspensionCore.__wrap(ret);
    }
    /**
     * @param {string} base64_json_bytes
     * @returns {SuspensionCore}
     */
    static from_base64_json(base64_json_bytes) {
        const ptr0 = passStringToWasm0(base64_json_bytes, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.suspensioncore_from_base64_json(ptr0, len0);
        return SuspensionCore.__wrap(ret);
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
     * @returns {PersonSummary}
     */
    get measured_by() {
        const ret = wasm.__wbg_get_suspensionmeasurement_measured_by(this.__wbg_ptr);
        return PersonSummary.__wrap(ret);
    }
    /**
     * @param {PersonSummary} arg0
     */
    set measured_by(arg0) {
        _assertClass(arg0, PersonSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspensionmeasurement_measured_by(this.__wbg_ptr, ptr0);
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
     * @returns {number}
     */
    get target_reads_per_cell() {
        const ret = wasm.__wbg_get_suspensionsummary_target_reads_per_cell(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set target_reads_per_cell(arg0) {
        wasm.__wbg_set_suspensionsummary_target_reads_per_cell(this.__wbg_ptr, arg0);
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

export function __wbg_getTime_46267b1c24877e30(arg0) {
    const ret = arg0.getTime();
    return ret;
};

export function __wbg_new_31a97dac4f10fab7(arg0) {
    const ret = new Date(arg0);
    return ret;
};

export function __wbg_new_5e0be73521bc8c17() {
    const ret = new Map();
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

export function __wbg_set_8fc6bf8a5b1071d1(arg0, arg1, arg2) {
    const ret = arg0.set(arg1, arg2);
    return ret;
};

export function __wbindgen_debug_string(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_2;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

export function __wbindgen_number_get(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
};

export function __wbindgen_number_new(arg0) {
    const ret = arg0;
    return ret;
};

export function __wbindgen_string_get(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_try_into_number(arg0) {
    let result;
    try { result = +arg0 } catch (e) { result = e }
    const ret = result;
    return ret;
};

