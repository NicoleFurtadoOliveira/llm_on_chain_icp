use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager}, DefaultMemoryImpl};

use std::cell::RefCell;

mod onnx;
mod storage;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

#[ic_cdk::init]
fn init() {
    // Initialize the WASI memory
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    // Reinitialize the WASI memory after upgrade
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);
}


//////////////////////////////////////////////////////////////////////



const MODEL_FILE: &str = "model.onnx";

/// This is used for incremental chunk uploading of large files.
#[ic_cdk::update]
fn clear_model_bytes() {
    storage::clear_bytes(MODEL_FILE);
}

/// This is used for incremental chunk uploading of large files.
#[ic_cdk::update]
fn append_model_bytes(bytes: Vec<u8>) {
    storage::append_bytes(MODEL_FILE, bytes);
}

/// Returns the length of the model bytes.
#[ic_cdk::query]
fn model_bytes_length() -> usize {
    storage::bytes_length(MODEL_FILE)
}
