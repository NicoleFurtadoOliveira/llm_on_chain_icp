# GPT2 MODEL ICP ON-CHAIN INFERENCE

This project was an adaptation of https://github.com/modclub-app/rust-connect-py-ai-to-ic/tree/main in order to participate on the ICP Hackerhouse on 9-10/11/2024 in the context of AI:CON Crypto Conference in Lisbon.

We can usually infer 7 tokens max before we run out of instructions limit:

Error: Failed update call.
Caused by: The replica returned a rejection error: reject code CanisterError, reject message Error from Canister bkyz2-fmaaa-aaaaa-qaaaq-cai: Canister exceeded the limit of 40000000000 instructions for single message execution..
Try optimizing this method to consume fewer instructions or split the work across multiple messages. See documentation: http://internetcomputer.org/docs/current/references/execution-errors#instruction-limit-exceeded, error code None
 

## Setup

Install Python, Rust and Cargo

Also

cargo install ic-file-uploader

### Rust Setup

First, ensure you have Rust installed. We will then set the default toolchain to stable and add the WebAssembly target.

1. Install Rust and Cargo (if not already installed): Visit [Rust's installation page](https://www.rust-lang.org/tools/install).
2. Set the default toolchain to stable:
   ```bash
   rustup default stable
   ```
3. Add the WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
4. Add Cargo's bin directory to your PATH:
   ```bash
   export PATH="$PATH:~/.cargo/bin"
   ```

### Python Setup

Ensure you have Python installed and then set up PyTorch.

1. Install Python (if not already installed): Visit [Python's download page](https://www.python.org/downloads/).
2. Install PyTorch using pip:
   ```bash
   pip install torch
   pip install transformers
   ```

### Dfinity's DFX Setup

We will be using Dfinity's `dfx` for our development environment.

1. Install Dfinity's `dfx`: Follow the instructions on [Dfinity's SDK documentation](https://sdk.dfinity.org/docs/quickstart/quickstart.html).


### Install WASI SDK 21

1. Download wasi-sdk-21.0 from [WASI SDK Releases](https://github.com/WebAssembly/wasi-sdk/releases/tag/wasi-sdk-21).
2. Export `CC_wasm32_wasi` in your shell such that it points to WASI clang and sysroot. Example:
   ```bash
   export CC_wasm32_wasi="/path/to/wasi-sdk-21.0/bin/clang --sysroot=/path/to/wasi-sdk-21.0/share/wasi-sysroot"
   ```

### Install wasi2ic

1. Clone the repository:
   ```bash
   git clone https://github.com/wasm-forge/wasi2ic
   ```
2. Enter the `wasi2ic` folder.
3. Compile the project with:
   ```bash
   cargo build
   ```
   Alternatively, use:
   ```bash
   cargo install --path .
   ```
4. Ensure the `wasi2ic` binary is in your `$PATH`.

### Partition the GPT-2 Model

- Run the script to partition the GPT-2 model, preparing it for backend use:
  ```bash
  python3 python/GPT2_no_cache.py
  ```

  Move the model file to gpt2/ and rename it to model.onnx


### Additional Setup for wasm-opt

- Install `wasm-opt`:
  ```bash
  cargo install wasm-opt
  ```

### Additional Notes on Using wasi2ic

- To convert a WASI-dependent Wasm module to run on the Internet Computer:
  ```bash
  wasi2ic <input-wasm-file> <output_wasm_file>
  ```
- Add the polyfill dependency to your project:
  ```bash
  cargo add --git https://github.com/wasm-forge/ic-wasi-polyfill
  ```

## Run

dfx start --clean

dfx deploy

ic-file-uploader gpt2_backend append_model_bytes model.onnx

dfx canister call gpt2_backend setup_model


optional check length

dfx canister call gpt2_backend model_bytes_length

Use GPT-2's Byte Pair Encoding (BPE) tokenizer in ChatGPT for example.

Asking

"transformers are more powerful"

dfx canister call gpt2_backend model_inference '(7, vec {26905; 30906; 48451; 37166; 13424; 14305})'

Prints

286 : " the"

6_824 : " first"

284 : " in"

4_589 : " line"

284 : " in"

262 : " a"

976 : " text"

Asking

"who are you?"

dfx canister call gpt2_backend model_inference '(7, vec {750; 389; 345; 30})'

Prints

198: (newline or line break character)

198: (newline or line break character)

40: "I"

1101: " did"

407: " tell"

1654: " you"

13: "!"

