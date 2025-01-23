import init, { create_employee, read_employees, update_employee, delete_employee } from './pkg/employee_crud_wasm.js';

async function initWasm() {
    // Initialize the WebAssembly instance
    await init();
    console.log('WASM Initialized!');
}

// Initialize the WebAssembly
initWasm();
