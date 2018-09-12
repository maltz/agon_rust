extern crate futures;
extern crate futures_cpupool;
extern crate grpcio;
extern crate bytes;

mod attr_value;
mod classification;
mod example;
mod feature;
mod function;
mod get_model_metadata;
mod graph;
mod inference;
mod input;
mod meta_graph;
mod model;
mod node_def;
mod op_def;
mod predict;
mod prediction_service_grpc;
mod regression;
mod resource_handle;
mod saver;
mod tensor;
mod tensor_shape;
mod types;
mod versions;

use futures::Future;
use futures::future::join_all;
use self::futures_cpupool::CpuPool;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use serving::prediction_service_grpc::PredictionServiceClient;

use bytes::BufMut;
use serving::model::ModelSpec;
use serving::predict::PredictRequest;
use serving::tensor::TensorProto;
use serving::tensor_shape::{TensorShapeProto, TensorShapeProto_Dim};
use serving::types::DataType;

use std::time::Instant;
use std::collections::HashMap;

const CATEGORY: &[&str] = &[
        "adult",
        "accident",
        "death",
        "violence",
        "gossip",
        "religion",
        "discrimination",
        "international_criticism",
        "politics",
        "drug",
        "illegal_downloading",
        "dating",
        "legal_consulting",
        "cigarette",
        "alcohol",
        "gambling"
    ];


fn predict_request<'a>(image: &'a [i32], cat: &str) -> PredictRequest {
    let mut model_spec = ModelSpec::new();
    model_spec.set_name(cat.into());
    model_spec.set_signature_name("serving_default".into());

    let mut request = PredictRequest::new();
    request.set_model_spec(model_spec);
    request.inputs.insert("inputs".into(), to_proto(image));

    request
}

fn encode<'a>(input: &'a [i32]) -> Vec<u8> {
    let mut buf = vec![];
    for i in input {
        buf.put_i32_le(*i);
    }
    buf
}

fn to_proto<'a>(image: &'a [i32]) -> TensorProto {
    let mut image_proto = TensorProto::new();
    image_proto.set_dtype(DataType::DT_INT32);

    // Set shape
    let mut shape = TensorShapeProto::new();
    let mut dim = TensorShapeProto_Dim::new();
    dim.set_size(1);
    shape.dim.insert(0, dim);

    let mut dim = TensorShapeProto_Dim::new();
    dim.set_size(50);
    shape.dim.insert(1, dim);
    image_proto.set_tensor_shape(shape);

    // Set content
    image_proto.set_tensor_content(encode(&image));

    image_proto
}

pub fn request<'a>(addr: &str, text: &str) -> Result<HashMap<&'a str, f32>, String>{

    let x = vec![1;50];

    let pool = CpuPool::new_num_cpus();

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(addr);
    let client = PredictionServiceClient::new(ch);

    // let start = Instant::now();

    let tasks = CATEGORY.into_iter().map(|cat| {
        let request = predict_request(&x.as_slice(), cat);
        let predict = client.predict_async(request).map(move |response| {
            let output = response.get_outputs();
            let scores = match output.get("outputs") {
                Some(val) => &val.float_val,
                None      => return ("", 0.0)
            };
            (*cat, scores[0])
        });
        pool.spawn(predict)
    }).collect::<Vec<_>>();

    let result = match join_all(tasks).wait() {
        Ok(v) => if v.iter().filter(|x| **x != ("", 0.0)).collect::<Vec<_>>().len() == CATEGORY.len() {
                    v.into_iter().collect()
                }else{
                    return Err("".to_string())
                },
        Err(e) => return Err(e.to_string())
    };

    // let elapsed = start.elapsed();
    // println!("Elapsed: {} ms",(elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

    Ok(result)
}
