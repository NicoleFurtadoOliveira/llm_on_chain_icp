use std::cell::RefCell;
use prost::Message;
use tract_onnx::prelude::*;
use tract_ndarray::{ArrayD, IxDyn};
use crate::storage;
use crate::MODEL_FILE;
use crate::onnx::tract_data::internal::anyhow;

type Model = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

thread_local! {
    static MODEL: RefCell<Option<Model>> = RefCell::new(None);
}

pub fn setup() -> TractResult<()> {
    ic_cdk::println!("Starting model setup.");
    let bytes = storage::bytes(MODEL_FILE);

    let proto = tract_onnx::pb::ModelProto::decode(bytes).map_err(|e| anyhow!("Failed to decode model proto: {}", e))?;

    let model = tract_onnx::onnx()
        .model_for_proto_model(&proto)
        .and_then(|model| model.into_optimized())
        .and_then(|model| model.into_runnable())
        .map_err(|e| anyhow!("Failed to create runnable model: {}", e))?;

    MODEL.with(|m| *m.borrow_mut() = Some(model));
    ic_cdk::println!("Model setup completed successfully.");
    Ok(())
}

#[ic_cdk::update]
fn setup_model() -> Result<(), String> {
    match setup() {
        Ok(_) => {
            ic_cdk::println!("Model setup was successful.");
            Ok(())
        }
        Err(err) => {
            ic_cdk::println!("Model setup failed: {}", err);
            Err(format!("Failed to setup model: {}", err))
        }
    }
}


#[ic_cdk::update]
fn model_inference(max_tokens: u8, numbers: Vec<i64>) -> Result<Vec<i64>, String> {
    create_tensor_and_run_model(max_tokens, numbers).map_err(|err| err.to_string())
}

pub fn create_tensor_and_run_model(max_tokens: u8, mut input_ids: Vec<i64>) -> Result<Vec<i64>, anyhow::Error> {
    MODEL.with(|model| {
        let model = model.borrow();
        
        // Check if model is initialized; return an error if not
        let model = match model.as_ref() {
            Some(m) => m,
            None => return Err(anyhow::anyhow!("Model has not been initialized. Please call setup_model first.")),
        };

        let mut attention_mask: Vec<i8> = vec![1; input_ids.len()];
        let mut output_ids: Vec<i64> = Vec::new();

        for _ in 0..max_tokens {
            let input_ids_tensor = create_tensor_i64(&input_ids)?;
            let attention_mask_tensor = create_tensor_i8(&attention_mask)?;

            let inputs: TVec<TValue> = tvec!(input_ids_tensor.into(), attention_mask_tensor.into());
            let outputs = model.run(inputs)?;

            let next_token_tensor = outputs[0].to_array_view::<i64>()?;
            let next_token = next_token_tensor[[0, 0]];

            if next_token == 50256_i64 { break; }

            input_ids.push(next_token);
            attention_mask.push(1);
            output_ids.push(next_token);
        }

        Ok(output_ids)
    })
}

fn create_tensor_i64(data: &[i64]) -> TractResult<Tensor> {
    let shape = [1, data.len()];
    let array = ArrayD::from_shape_vec(IxDyn(&shape), data.to_vec())
        .map_err(|_| anyhow::anyhow!("Failed to create tensor from shape and values"))?;
    Ok(array.into_tensor())
}

fn create_tensor_i8(data: &[i8]) -> TractResult<Tensor> {
    let shape = [1, data.len()];
    let array = tract_ndarray::Array::from_shape_vec(shape, data.to_vec())
        .map_err(|_| anyhow::anyhow!("Failed to create tensor from shape and values"))?;
    Ok(array.into_tensor())
}
