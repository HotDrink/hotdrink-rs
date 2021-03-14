/// From https://github.com/chemicstry/wasm_thread/blob/main/src/script_path.js

/// Extracts current script file path from artificially generated stack trace
function script_path() {
    try {
        throw new Error();
    } catch (e) {
        let parts = e.stack.match(/\((\S+):\d+:\d+\)/);
        return parts[1];
    }
}

script_path()