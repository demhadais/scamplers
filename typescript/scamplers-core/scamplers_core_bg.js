let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


function getFromExternrefTable0(idx) { return wasm.__wbindgen_export_0.get(idx); }

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

function isLikeNone(x) {
    return x === undefined || x === null;
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

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_5.get(state.dtor)(state.a, state.b)
});

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
                wasm.__wbindgen_export_5.get(state.dtor)(a, state.b);
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

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
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

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_0.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}
function __wbg_adapter_40(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hba324d8185144b2c(arg0, arg1);
}

function __wbg_adapter_43(arg0, arg1, arg2) {
    wasm.closure116_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_430(arg0, arg1, arg2, arg3) {
    wasm.closure151_externref_shim(arg0, arg1, arg2, arg3);
}

/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7}
 */
export const Species = Object.freeze({
    AmbystomaMexicanum: 0, "0": "AmbystomaMexicanum",
    CanisFamiliaris: 1, "1": "CanisFamiliaris",
    DrosophilaMelanogaster: 2, "2": "DrosophilaMelanogaster",
    GasterosteusAculeatus: 3, "3": "GasterosteusAculeatus",
    HomoSapiens: 4, "4": "HomoSapiens",
    MusMusculus: 5, "5": "MusMusculus",
    RattusNorvegicus: 6, "6": "RattusNorvegicus",
    SminthopsisCrassicaudata: 7, "7": "SminthopsisCrassicaudata",
});
/**
 * @enum {0 | 1 | 2}
 */
export const UserRole = Object.freeze({
    AppAdmin: 0, "0": "AppAdmin",
    ComputationalStaff: 1, "1": "ComputationalStaff",
    BiologyStaff: 2, "2": "BiologyStaff",
});

const __wbindgen_enum_RequestCredentials = ["omit", "same-origin", "include"];

const __wbindgen_enum_RequestMode = ["same-origin", "no-cors", "cors", "navigate"];

const CdnaHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cdnahandle_free(ptr >>> 0, 1));

export class CdnaHandle {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CdnaHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cdnahandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_cdnahandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_cdnahandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_cdnahandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_cdnahandle_link(this.__wbg_ptr, ptr0, len0);
    }
}

const ChromiumRunFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chromiumrun_free(ptr >>> 0, 1));

export class ChromiumRun {

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
     * @returns {ChromiumRunSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_chromiumrun_summary(this.__wbg_ptr);
        return ChromiumRunSummary.__wrap(ret);
    }
    /**
     * @param {ChromiumRunSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, ChromiumRunSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumrun_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {GemsHandle[]}
     */
    get gems() {
        const ret = wasm.__wbg_get_chromiumrun_gems(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {GemsHandle[]} arg0
     */
    set gems(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrun_gems(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.chromiumrun_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
}

const ChromiumRunHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chromiumrunhandle_free(ptr >>> 0, 1));

export class ChromiumRunHandle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChromiumRunHandle.prototype);
        obj.__wbg_ptr = ptr;
        ChromiumRunHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChromiumRunHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chromiumrunhandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_chromiumrunhandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_chromiumrunhandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_chromiumrunhandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunhandle_link(this.__wbg_ptr, ptr0, len0);
    }
}

const ChromiumRunSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chromiumrunsummary_free(ptr >>> 0, 1));

export class ChromiumRunSummary {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChromiumRunSummary.prototype);
        obj.__wbg_ptr = ptr;
        ChromiumRunSummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChromiumRunSummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chromiumrunsummary_free(ptr, 0);
    }
    /**
     * @returns {ChromiumRunHandle}
     */
    get handle() {
        const ret = wasm.__wbg_get_chromiumrunsummary_handle(this.__wbg_ptr);
        return ChromiumRunHandle.__wrap(ret);
    }
    /**
     * @param {ChromiumRunHandle} arg0
     */
    set handle(arg0) {
        _assertClass(arg0, ChromiumRunHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumrunsummary_handle(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string}
     */
    get readable_id() {
        const ret = wasm.__wbg_get_chromiumrunsummary_readable_id(this.__wbg_ptr);
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
        wasm.__wbg_set_chromiumrunsummary_readable_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get chip() {
        const ret = wasm.__wbg_get_chromiumrunsummary_chip(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set chip(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunsummary_chip(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date}
     */
    get run_at() {
        const ret = wasm.__wbg_get_chromiumrunsummary_run_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set run_at(arg0) {
        wasm.__wbg_set_chromiumrunsummary_run_at(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get succeeded() {
        const ret = wasm.__wbg_get_chromiumrunsummary_succeeded(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set succeeded(arg0) {
        wasm.__wbg_set_chromiumrunsummary_succeeded(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get notes() {
        const ret = wasm.__wbg_get_chromiumrunsummary_notes(this.__wbg_ptr);
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
        wasm.__wbg_set_chromiumrunsummary_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.chromiumrunsummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
}

const ClientFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_client_free(ptr >>> 0, 1));

export class Client {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ClientFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_client_free(ptr, 0);
    }
    /**
     * @param {string} backend_base_url
     * @param {string | null} [frontend_token]
     * @param {string | null} [api_key]
     */
    constructor(backend_base_url, frontend_token, api_key) {
        const ptr0 = passStringToWasm0(backend_base_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(frontend_token) ? 0 : passStringToWasm0(frontend_token, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(api_key) ? 0 : passStringToWasm0(api_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.client_new(ptr0, len0, ptr1, len1, ptr2, len2);
        this.__wbg_ptr = ret >>> 0;
        ClientFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {NewMsLogin} data
     * @returns {Promise<CreatedUser>}
     */
    ms_login(data) {
        _assertClass(data, NewMsLogin);
        var ptr0 = data.__destroy_into_raw();
        const ret = wasm.client_ms_login(this.__wbg_ptr, ptr0);
        return ret;
    }
}

const CommitteeApprovalFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_committeeapproval_free(ptr >>> 0, 1));

export class CommitteeApproval {

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
     * @returns {InstitutionHandle}
     */
    get institution() {
        const ret = wasm.__wbg_get_committeeapproval_institution(this.__wbg_ptr);
        return InstitutionHandle.__wrap(ret);
    }
    /**
     * @param {InstitutionHandle} arg0
     */
    set institution(arg0) {
        _assertClass(arg0, InstitutionHandle);
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
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.createduser_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.createduser_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.createduser_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get email() {
        const ret = wasm.createduser_email(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get orcid() {
        const ret = wasm.createduser_orcid(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {any[]}
     */
    get roles() {
        const ret = wasm.createduser_roles(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {Institution}
     */
    get institution() {
        const ret = wasm.createduser_institution(this.__wbg_ptr);
        return Institution.__wrap(ret);
    }
}

const DatasetHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_datasethandle_free(ptr >>> 0, 1));

export class DatasetHandle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DatasetHandle.prototype);
        obj.__wbg_ptr = ptr;
        DatasetHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DatasetHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_datasethandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_datasethandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_datasethandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_datasethandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_datasethandle_link(this.__wbg_ptr, ptr0, len0);
    }
}

const DatasetSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_datasetsummary_free(ptr >>> 0, 1));

export class DatasetSummary {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DatasetSummaryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_datasetsummary_free(ptr, 0);
    }
    /**
     * @returns {DatasetHandle}
     */
    get handle() {
        const ret = wasm.__wbg_get_datasetsummary_handle(this.__wbg_ptr);
        return DatasetHandle.__wrap(ret);
    }
    /**
     * @param {DatasetHandle} arg0
     */
    set handle(arg0) {
        _assertClass(arg0, DatasetHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_datasetsummary_handle(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {string}
     */
    get data_path() {
        const ret = wasm.__wbg_get_datasetsummary_data_path(this.__wbg_ptr);
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
        wasm.__wbg_set_datasetsummary_data_path(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Date}
     */
    get delivered_at() {
        const ret = wasm.__wbg_get_datasetsummary_delivered_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set delivered_at(arg0) {
        wasm.__wbg_set_datasetsummary_delivered_at(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get web_summary() {
        const ret = wasm.__wbg_get_datasetsummary_web_summary(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set web_summary(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_datasetsummary_web_summary(this.__wbg_ptr, ptr0, len0);
    }
}

const EmptyStringErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_emptystringerror_free(ptr >>> 0, 1));

export class EmptyStringError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(EmptyStringError.prototype);
        obj.__wbg_ptr = ptr;
        EmptyStringErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

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

const GemsHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gemshandle_free(ptr >>> 0, 1));

export class GemsHandle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GemsHandle.prototype);
        obj.__wbg_ptr = ptr;
        GemsHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof GemsHandle)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GemsHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gemshandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_gemshandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_chromiumrunhandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_gemshandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunhandle_link(this.__wbg_ptr, ptr0, len0);
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
     * @returns {InstitutionHandle}
     */
    get handle() {
        const ret = wasm.__wbg_get_institution_handle(this.__wbg_ptr);
        return InstitutionHandle.__wrap(ret);
    }
    /**
     * @param {InstitutionHandle} arg0
     */
    set handle(arg0) {
        _assertClass(arg0, InstitutionHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_institution_handle(this.__wbg_ptr, ptr0);
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
     * @returns {string}
     */
    get id() {
        const ret = wasm.institution_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.institution_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
}

const InstitutionHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_institutionhandle_free(ptr >>> 0, 1));

export class InstitutionHandle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InstitutionHandle.prototype);
        obj.__wbg_ptr = ptr;
        InstitutionHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InstitutionHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_institutionhandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_institutionhandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_institutionhandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_institutionhandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_institutionhandle_link(this.__wbg_ptr, ptr0, len0);
    }
}

const InstitutionQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_institutionquery_free(ptr >>> 0, 1));

export class InstitutionQuery {

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
    constructor() {
        const ret = wasm.institutionquery_new();
        this.__wbg_ptr = ret >>> 0;
        InstitutionQueryFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const LabFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_lab_free(ptr >>> 0, 1));

export class Lab {

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
     * @returns {LabCore}
     */
    get core() {
        const ret = wasm.__wbg_get_lab_core(this.__wbg_ptr);
        return LabCore.__wrap(ret);
    }
    /**
     * @param {LabCore} arg0
     */
    set core(arg0) {
        _assertClass(arg0, LabCore);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_lab_core(this.__wbg_ptr, ptr0);
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
     * @returns {string}
     */
    get id() {
        const ret = wasm.lab_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.lab_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.lab_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get delivery_dir() {
        const ret = wasm.lab_delivery_dir(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
}

const LabCoreFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_labcore_free(ptr >>> 0, 1));

export class LabCore {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LabCore.prototype);
        obj.__wbg_ptr = ptr;
        LabCoreFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LabCoreFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_labcore_free(ptr, 0);
    }
    /**
     * @returns {LabSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_labcore_summary(this.__wbg_ptr);
        return LabSummary.__wrap(ret);
    }
    /**
     * @param {LabSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, LabSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_labcore_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {PersonSummary}
     */
    get pi() {
        const ret = wasm.__wbg_get_labcore_pi(this.__wbg_ptr);
        return PersonSummary.__wrap(ret);
    }
    /**
     * @param {PersonSummary} arg0
     */
    set pi(arg0) {
        _assertClass(arg0, PersonSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_labcore_pi(this.__wbg_ptr, ptr0);
    }
}

const LabHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_labhandle_free(ptr >>> 0, 1));

export class LabHandle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LabHandle.prototype);
        obj.__wbg_ptr = ptr;
        LabHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LabHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_labhandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_labhandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_cdnahandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_labhandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labhandle_link(this.__wbg_ptr, ptr0, len0);
    }
}

const LabQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_labquery_free(ptr >>> 0, 1));

export class LabQuery {

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
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_labquery_name(this.__wbg_ptr);
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
        wasm.__wbg_set_labquery_name(this.__wbg_ptr, ptr0, len0);
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
     * @returns {LabHandle}
     */
    get handle() {
        const ret = wasm.__wbg_get_labsummary_handle(this.__wbg_ptr);
        return LabHandle.__wrap(ret);
    }
    /**
     * @param {LabHandle} arg0
     */
    set handle(arg0) {
        _assertClass(arg0, LabHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_labsummary_handle(this.__wbg_ptr, ptr0);
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
     * @returns {string}
     */
    get id() {
        const ret = wasm.labsummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    link() {
        const ret = wasm.labsummary_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
}

const LibraryHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_libraryhandle_free(ptr >>> 0, 1));

export class LibraryHandle {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LibraryHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_libraryhandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_libraryhandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_institutionhandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_libraryhandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_institutionhandle_link(this.__wbg_ptr, ptr0, len0);
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
}

const NewMsLoginFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_newmslogin_free(ptr >>> 0, 1));

export class NewMsLogin {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(NewMsLogin.prototype);
        obj.__wbg_ptr = ptr;
        NewMsLoginFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NewMsLoginFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_newmslogin_free(ptr, 0);
    }
    /**
     * @returns {NewPersonEmpty}
     */
    static new() {
        const ret = wasm.newmslogin_new();
        return NewPersonEmpty.__wrap(ret);
    }
}

const NewPersonEmailFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_newpersonemail_free(ptr >>> 0, 1));

export class NewPersonEmail {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(NewPersonEmail.prototype);
        obj.__wbg_ptr = ptr;
        NewPersonEmailFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NewPersonEmailFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_newpersonemail_free(ptr, 0);
    }
    /**
     * @param {string} ms_user_id
     * @returns {NewPersonMsUserId}
     */
    ms_user_id(ms_user_id) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(ms_user_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.newpersonemail_ms_user_id(ptr, ptr0, len0);
        return NewPersonMsUserId.__wrap(ret);
    }
}

const NewPersonEmptyFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_newpersonempty_free(ptr >>> 0, 1));

export class NewPersonEmpty {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(NewPersonEmpty.prototype);
        obj.__wbg_ptr = ptr;
        NewPersonEmptyFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NewPersonEmptyFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_newpersonempty_free(ptr, 0);
    }
    /**
     * @param {string} name
     * @returns {NewPersonName}
     */
    name(name) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.newpersonempty_name(ptr, ptr0, len0);
        return NewPersonName.__wrap(ret);
    }
}

const NewPersonInstitutionIdFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_newpersoninstitutionid_free(ptr >>> 0, 1));

export class NewPersonInstitutionId {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(NewPersonInstitutionId.prototype);
        obj.__wbg_ptr = ptr;
        NewPersonInstitutionIdFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NewPersonInstitutionIdFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_newpersoninstitutionid_free(ptr, 0);
    }
    /**
     * @returns {NewMsLogin}
     */
    build() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.newpersoninstitutionid_build(ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return NewMsLogin.__wrap(ret[0]);
    }
}

const NewPersonMsUserIdFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_newpersonmsuserid_free(ptr >>> 0, 1));

export class NewPersonMsUserId {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(NewPersonMsUserId.prototype);
        obj.__wbg_ptr = ptr;
        NewPersonMsUserIdFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NewPersonMsUserIdFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_newpersonmsuserid_free(ptr, 0);
    }
    /**
     * @param {string} institution_id
     * @returns {NewPersonInstitutionId}
     */
    institution_id(institution_id) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(institution_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.newpersonmsuserid_institution_id(ptr, ptr0, len0);
        return NewPersonInstitutionId.__wrap(ret);
    }
}

const NewPersonNameFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_newpersonname_free(ptr >>> 0, 1));

export class NewPersonName {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(NewPersonName.prototype);
        obj.__wbg_ptr = ptr;
        NewPersonNameFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NewPersonNameFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_newpersonname_free(ptr, 0);
    }
    /**
     * @param {string} email
     * @returns {NewPersonEmail}
     */
    email(email) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(email, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.newpersonname_email(ptr, ptr0, len0);
        return NewPersonEmail.__wrap(ret);
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
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.person_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.person_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get name() {
        const ret = wasm.person_name(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get email() {
        const ret = wasm.person_email(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get orcid() {
        const ret = wasm.person_orcid(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {Institution}
     */
    get institution() {
        const ret = wasm.person_institution(this.__wbg_ptr);
        return Institution.__wrap(ret);
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
}

const PersonHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_personhandle_free(ptr >>> 0, 1));

export class PersonHandle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PersonHandle.prototype);
        obj.__wbg_ptr = ptr;
        PersonHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PersonHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_personhandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_personhandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_personhandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_personhandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_personhandle_link(this.__wbg_ptr, ptr0, len0);
    }
}

const PersonQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_personquery_free(ptr >>> 0, 1));

export class PersonQuery {

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
     * @returns {PersonHandle}
     */
    get handle() {
        const ret = wasm.__wbg_get_personsummary_handle(this.__wbg_ptr);
        return PersonHandle.__wrap(ret);
    }
    /**
     * @param {PersonHandle} arg0
     */
    set handle(arg0) {
        _assertClass(arg0, PersonHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_personsummary_handle(this.__wbg_ptr, ptr0);
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
     * @returns {string}
     */
    get id() {
        const ret = wasm.personsummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.personsummary_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
}

const SequencingRunHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sequencingrunhandle_free(ptr >>> 0, 1));

export class SequencingRunHandle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SequencingRunHandle.prototype);
        obj.__wbg_ptr = ptr;
        SequencingRunHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SequencingRunHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sequencingrunhandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_sequencingrunhandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_sequencingrunhandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_sequencingrunhandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_sequencingrunhandle_link(this.__wbg_ptr, ptr0, len0);
    }
}

const SequencingRunSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sequencingrunsummary_free(ptr >>> 0, 1));

export class SequencingRunSummary {

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
     * @returns {SequencingRunHandle}
     */
    get handle() {
        const ret = wasm.__wbg_get_sequencingrunsummary_handle(this.__wbg_ptr);
        return SequencingRunHandle.__wrap(ret);
    }
    /**
     * @param {SequencingRunHandle} arg0
     */
    set handle(arg0) {
        _assertClass(arg0, SequencingRunHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_sequencingrunsummary_handle(this.__wbg_ptr, ptr0);
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
     * @returns {Date}
     */
    get finished_at() {
        const ret = wasm.__wbg_get_sequencingrunsummary_finished_at(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Date} arg0
     */
    set finished_at(arg0) {
        wasm.__wbg_set_sequencingrunsummary_finished_at(this.__wbg_ptr, arg0);
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
     * @returns {string}
     */
    get id() {
        const ret = wasm.sequencingrunsummary_id(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
}

const SpecimenFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_specimen_free(ptr >>> 0, 1));

export class Specimen {

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
     * @returns {SpecimenCore}
     */
    get core() {
        const ret = wasm.__wbg_get_specimen_core(this.__wbg_ptr);
        return SpecimenCore.__wrap(ret);
    }
    /**
     * @param {SpecimenCore} arg0
     */
    set core(arg0) {
        _assertClass(arg0, SpecimenCore);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimen_core(this.__wbg_ptr, ptr0);
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
}

const SpecimenCoreFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_specimencore_free(ptr >>> 0, 1));

export class SpecimenCore {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SpecimenCore.prototype);
        obj.__wbg_ptr = ptr;
        SpecimenCoreFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SpecimenCoreFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_specimencore_free(ptr, 0);
    }
    /**
     * @returns {SpecimenSummary}
     */
    get summary() {
        const ret = wasm.__wbg_get_specimencore_summary(this.__wbg_ptr);
        return SpecimenSummary.__wrap(ret);
    }
    /**
     * @param {SpecimenSummary} arg0
     */
    set summary(arg0) {
        _assertClass(arg0, SpecimenSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimencore_summary(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {LabSummary}
     */
    get lab() {
        const ret = wasm.__wbg_get_specimencore_lab(this.__wbg_ptr);
        return LabSummary.__wrap(ret);
    }
    /**
     * @param {LabSummary} arg0
     */
    set lab(arg0) {
        _assertClass(arg0, LabSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimencore_lab(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {PersonSummary}
     */
    get submitted_by() {
        const ret = wasm.__wbg_get_specimencore_submitted_by(this.__wbg_ptr);
        return PersonSummary.__wrap(ret);
    }
    /**
     * @param {PersonSummary} arg0
     */
    set submitted_by(arg0) {
        _assertClass(arg0, PersonSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimencore_submitted_by(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {PersonSummary}
     */
    get returned_by() {
        const ret = wasm.__wbg_get_specimencore_returned_by(this.__wbg_ptr);
        return PersonSummary.__wrap(ret);
    }
    /**
     * @param {PersonSummary} arg0
     */
    set returned_by(arg0) {
        _assertClass(arg0, PersonSummary);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimencore_returned_by(this.__wbg_ptr, ptr0);
    }
}

const SpecimenHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_specimenhandle_free(ptr >>> 0, 1));

export class SpecimenHandle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SpecimenHandle.prototype);
        obj.__wbg_ptr = ptr;
        SpecimenHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SpecimenHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_specimenhandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_specimenhandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_chromiumrunhandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_specimenhandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_chromiumrunhandle_link(this.__wbg_ptr, ptr0, len0);
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
     * @returns {PersonHandle}
     */
    get measured_by() {
        const ret = wasm.__wbg_get_specimenmeasurement_measured_by(this.__wbg_ptr);
        return PersonHandle.__wrap(ret);
    }
    /**
     * @param {PersonHandle} arg0
     */
    set measured_by(arg0) {
        _assertClass(arg0, PersonHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimenmeasurement_measured_by(this.__wbg_ptr, ptr0);
    }
}

const SpecimenQueryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_specimenquery_free(ptr >>> 0, 1));

export class SpecimenQuery {

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
     * @returns {string}
     */
    get name() {
        const ret = wasm.__wbg_get_specimenquery_name(this.__wbg_ptr);
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
        wasm.__wbg_set_specimenquery_name(this.__wbg_ptr, ptr0, len0);
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
        wasm.__wbg_set_specimenquery_received_before(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
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
        wasm.__wbg_set_specimenquery_received_after(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
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
     * @returns {string}
     */
    get notes() {
        const ret = wasm.__wbg_get_specimenquery_notes(this.__wbg_ptr);
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
        wasm.__wbg_set_specimenquery_notes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get storage_buffer() {
        const ret = wasm.__wbg_get_specimenquery_storage_buffer(this.__wbg_ptr);
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
        wasm.__wbg_set_specimenquery_storage_buffer(this.__wbg_ptr, ptr0, len0);
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
     * @returns {Pagination}
     */
    get pagination() {
        const ret = wasm.__wbg_get_specimenquery_pagination(this.__wbg_ptr);
        return Pagination.__wrap(ret);
    }
    /**
     * @param {Pagination} arg0
     */
    set pagination(arg0) {
        _assertClass(arg0, Pagination);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_specimenquery_pagination(this.__wbg_ptr, ptr0);
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
     * @returns {SpecimenHandle}
     */
    get handle() {
        const ret = wasm.__wbg_get_chromiumrunsummary_handle(this.__wbg_ptr);
        return SpecimenHandle.__wrap(ret);
    }
    /**
     * @param {SpecimenHandle} arg0
     */
    set handle(arg0) {
        _assertClass(arg0, SpecimenHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_chromiumrunsummary_handle(this.__wbg_ptr, ptr0);
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
        wasm.__wbg_set_chromiumrunsummary_readable_id(this.__wbg_ptr, ptr0, len0);
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
        wasm.__wbg_set_chromiumrunsummary_chip(this.__wbg_ptr, ptr0, len0);
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
}

const SuspensionCoreFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensioncore_free(ptr >>> 0, 1));

export class SuspensionCore {

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
}

const SuspensionHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionhandle_free(ptr >>> 0, 1));

export class SuspensionHandle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionHandle.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensionhandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_suspensionhandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_suspensionhandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_suspensionhandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_multiplexingtag_tag_id(this.__wbg_ptr, ptr0, len0);
    }
}

const SuspensionMeasurementFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionmeasurement_free(ptr >>> 0, 1));

export class SuspensionMeasurement {

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
     * @returns {PersonHandle}
     */
    get measured_by() {
        const ret = wasm.__wbg_get_suspensionmeasurement_measured_by(this.__wbg_ptr);
        return PersonHandle.__wrap(ret);
    }
    /**
     * @param {PersonHandle} arg0
     */
    set measured_by(arg0) {
        _assertClass(arg0, PersonHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspensionmeasurement_measured_by(this.__wbg_ptr, ptr0);
    }
}

const SuspensionPoolHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionpoolhandle_free(ptr >>> 0, 1));

export class SuspensionPoolHandle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SuspensionPoolHandle.prototype);
        obj.__wbg_ptr = ptr;
        SuspensionPoolHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuspensionPoolHandleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_suspensionpoolhandle_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        const ret = wasm.__wbg_get_suspensionpoolhandle_id(this.__wbg_ptr);
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
        wasm.__wbg_set_sequencingrunhandle_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get link() {
        const ret = wasm.__wbg_get_suspensionpoolhandle_link(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
        if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
        return v1;
    }
    /**
     * @param {string} arg0
     */
    set link(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_sequencingrunhandle_link(this.__wbg_ptr, ptr0, len0);
    }
}

const SuspensionPoolSummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_suspensionpoolsummary_free(ptr >>> 0, 1));

export class SuspensionPoolSummary {

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
     * @returns {SuspensionPoolHandle}
     */
    get handle() {
        const ret = wasm.__wbg_get_sequencingrunsummary_handle(this.__wbg_ptr);
        return SuspensionPoolHandle.__wrap(ret);
    }
    /**
     * @param {SuspensionPoolHandle} arg0
     */
    set handle(arg0) {
        _assertClass(arg0, SuspensionPoolHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_sequencingrunsummary_handle(this.__wbg_ptr, ptr0);
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
        wasm.__wbg_set_sequencingrunsummary_readable_id(this.__wbg_ptr, ptr0, len0);
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
     * @returns {SuspensionHandle}
     */
    get handle() {
        const ret = wasm.__wbg_get_suspensionsummary_handle(this.__wbg_ptr);
        return SuspensionHandle.__wrap(ret);
    }
    /**
     * @param {SuspensionHandle} arg0
     */
    set handle(arg0) {
        _assertClass(arg0, SuspensionHandle);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_suspensionsummary_handle(this.__wbg_ptr, ptr0);
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
}

export function __wbg_abort_410ec47a64ac6117(arg0, arg1) {
    arg0.abort(arg1);
};

export function __wbg_abort_775ef1d17fc65868(arg0) {
    arg0.abort();
};

export function __wbg_append_8c7dd8d641a5f01b() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    arg0.append(v0, v1);
}, arguments) };

export function __wbg_arrayBuffer_d1b44c4390db422f() { return handleError(function (arg0) {
    const ret = arg0.arrayBuffer();
    return ret;
}, arguments) };

export function __wbg_buffer_609cc3eee51ed158(arg0) {
    const ret = arg0.buffer;
    return ret;
};

export function __wbg_call_672a4d21634d4a24() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments) };

export function __wbg_call_7cccdd69e0791ae2() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.call(arg1, arg2);
    return ret;
}, arguments) };

export function __wbg_clearTimeout_b1115618e821c3b2(arg0) {
    const ret = clearTimeout(arg0);
    return ret;
};

export function __wbg_createduser_new(arg0) {
    const ret = CreatedUser.__wrap(arg0);
    return ret;
};

export function __wbg_done_769e5ede4b31c67b(arg0) {
    const ret = arg0.done;
    return ret;
};

export function __wbg_emptystringerror_new(arg0) {
    const ret = EmptyStringError.__wrap(arg0);
    return ret;
};

export function __wbg_fetch_3afbdcc7ddbf16fe(arg0) {
    const ret = fetch(arg0);
    return ret;
};

export function __wbg_fetch_509096533071c657(arg0, arg1) {
    const ret = arg0.fetch(arg1);
    return ret;
};

export function __wbg_gemshandle_new(arg0) {
    const ret = GemsHandle.__wrap(arg0);
    return ret;
};

export function __wbg_gemshandle_unwrap(arg0) {
    const ret = GemsHandle.__unwrap(arg0);
    return ret;
};

export function __wbg_getTime_46267b1c24877e30(arg0) {
    const ret = arg0.getTime();
    return ret;
};

export function __wbg_get_67b2ba62fc30de12() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(arg0, arg1);
    return ret;
}, arguments) };

export function __wbg_has_a5ea9117f258a0ec() { return handleError(function (arg0, arg1) {
    const ret = Reflect.has(arg0, arg1);
    return ret;
}, arguments) };

export function __wbg_headers_9cb51cfd2ac780a4(arg0) {
    const ret = arg0.headers;
    return ret;
};

export function __wbg_instanceof_Response_f2cc20d9f7dfd644(arg0) {
    let result;
    try {
        result = arg0 instanceof Response;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_iterator_9a24c88df860dc65() {
    const ret = Symbol.iterator;
    return ret;
};

export function __wbg_length_a446193dc22c12f8(arg0) {
    const ret = arg0.length;
    return ret;
};

export function __wbg_new_018dcc2d6c8c2f6a() { return handleError(function () {
    const ret = new Headers();
    return ret;
}, arguments) };

export function __wbg_new_23a2665fac83c611(arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_adapter_430(a, state0.b, arg0, arg1);
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

export function __wbg_new_31a97dac4f10fab7(arg0) {
    const ret = new Date(arg0);
    return ret;
};

export function __wbg_new_405e22f390576ce2() {
    const ret = new Object();
    return ret;
};

export function __wbg_new_5e0be73521bc8c17() {
    const ret = new Map();
    return ret;
};

export function __wbg_new_78feb108b6472713() {
    const ret = new Array();
    return ret;
};

export function __wbg_new_a12002a7f91c75be(arg0) {
    const ret = new Uint8Array(arg0);
    return ret;
};

export function __wbg_new_e25e5aab09ff45db() { return handleError(function () {
    const ret = new AbortController();
    return ret;
}, arguments) };

export function __wbg_newnoargs_105ed471475aaf50(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Function(v0);
    return ret;
};

export function __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a(arg0, arg1, arg2) {
    const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
    return ret;
};

export function __wbg_newwithstrandinit_06c535e0a867c635() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Request(v0, arg2);
    return ret;
}, arguments) };

export function __wbg_next_25feadfc0913fea9(arg0) {
    const ret = arg0.next;
    return ret;
};

export function __wbg_next_6574e1a8a62d1055() { return handleError(function (arg0) {
    const ret = arg0.next();
    return ret;
}, arguments) };

export function __wbg_personsummary_new(arg0) {
    const ret = PersonSummary.__wrap(arg0);
    return ret;
};

export function __wbg_personsummary_unwrap(arg0) {
    const ret = PersonSummary.__unwrap(arg0);
    return ret;
};

export function __wbg_queueMicrotask_97d92b4fcc8a61c5(arg0) {
    queueMicrotask(arg0);
};

export function __wbg_queueMicrotask_d3219def82552485(arg0) {
    const ret = arg0.queueMicrotask;
    return ret;
};

export function __wbg_resolve_4851785c9c5f573d(arg0) {
    const ret = Promise.resolve(arg0);
    return ret;
};

export function __wbg_setTimeout_ca12ead8b48245e2(arg0, arg1) {
    const ret = setTimeout(arg0, arg1);
    return ret;
};

export function __wbg_set_37837023f3d740e8(arg0, arg1, arg2) {
    arg0[arg1 >>> 0] = arg2;
};

export function __wbg_set_3f1d0b984ed272ed(arg0, arg1, arg2) {
    arg0[arg1] = arg2;
};

export function __wbg_set_65595bdd868b3009(arg0, arg1, arg2) {
    arg0.set(arg1, arg2 >>> 0);
};

export function __wbg_set_8fc6bf8a5b1071d1(arg0, arg1, arg2) {
    const ret = arg0.set(arg1, arg2);
    return ret;
};

export function __wbg_setbody_5923b78a95eedf29(arg0, arg1) {
    arg0.body = arg1;
};

export function __wbg_setcredentials_c3a22f1cd105a2c6(arg0, arg1) {
    arg0.credentials = __wbindgen_enum_RequestCredentials[arg1];
};

export function __wbg_setheaders_834c0bdb6a8949ad(arg0, arg1) {
    arg0.headers = arg1;
};

export function __wbg_setmethod_3c5280fe5d890842(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.method = v0;
};

export function __wbg_setmode_5dc300b865044b65(arg0, arg1) {
    arg0.mode = __wbindgen_enum_RequestMode[arg1];
};

export function __wbg_setsignal_75b21ef3a81de905(arg0, arg1) {
    arg0.signal = arg1;
};

export function __wbg_signal_aaf9ad74119f20a4(arg0) {
    const ret = arg0.signal;
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

export function __wbg_static_accessor_GLOBAL_88a902d13a557d07() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_SELF_37c5d418e4bf5819() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_WINDOW_5de37043a91a9c40() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_status_f6360336ca686bf0(arg0) {
    const ret = arg0.status;
    return ret;
};

export function __wbg_stringify_f7ed6987935b4a24() { return handleError(function (arg0) {
    const ret = JSON.stringify(arg0);
    return ret;
}, arguments) };

export function __wbg_then_44b73946d2fb3e7d(arg0, arg1) {
    const ret = arg0.then(arg1);
    return ret;
};

export function __wbg_then_48b406749878a531(arg0, arg1, arg2) {
    const ret = arg0.then(arg1, arg2);
    return ret;
};

export function __wbg_url_ae10c34ca209681d(arg0, arg1) {
    const ret = arg1.url;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_value_cd1ffa7b1ab794f1(arg0) {
    const ret = arg0.value;
    return ret;
};

export function __wbindgen_bigint_from_i64(arg0) {
    const ret = arg0;
    return ret;
};

export function __wbindgen_bigint_from_u64(arg0) {
    const ret = BigInt.asUintN(64, arg0);
    return ret;
};

export function __wbindgen_cb_drop(arg0) {
    const obj = arg0.original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};

export function __wbindgen_closure_wrapper805(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 97, __wbg_adapter_40);
    return ret;
};

export function __wbindgen_closure_wrapper861(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 117, __wbg_adapter_43);
    return ret;
};

export function __wbindgen_debug_string(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbindgen_error_new(arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
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

export function __wbindgen_is_function(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
};

export function __wbindgen_is_object(arg0) {
    const val = arg0;
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};

export function __wbindgen_is_string(arg0) {
    const ret = typeof(arg0) === 'string';
    return ret;
};

export function __wbindgen_is_undefined(arg0) {
    const ret = arg0 === undefined;
    return ret;
};

export function __wbindgen_memory() {
    const ret = wasm.memory;
    return ret;
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

