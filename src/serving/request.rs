extern crate futures;
extern crate futures_cpupool;
extern crate grpcio;
extern crate bytes;

use futures::Future;
use futures::future::join_all;
use self::futures_cpupool::CpuPool;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use super::prediction_service_grpc::PredictionServiceClient;

use bytes::BufMut;
use super::model::ModelSpec;
use super::predict::PredictRequest;
use super::tensor::TensorProto;
use super::tensor_shape::{TensorShapeProto, TensorShapeProto_Dim};
use super::types::DataType;

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

pub fn request(addr: &str) -> Result<HashMap<&str, f32>, String>{

    let x = vec![1;50];

    let pool = CpuPool::new_num_cpus();

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(addr);
    let client = PredictionServiceClient::new(ch);

    let start = Instant::now();

    let tasks = CATEGORY.into_iter().map(|cat| {
        let request = predict_request(&x.as_slice(), cat);
        let predict = client.predict_async(request).map(move |response| {
            let output = response.get_outputs();
            let output = output.get("outputs").unwrap();
            let scores = &output.float_val;
            (*cat, scores[0])
        });
        pool.spawn(predict)
    }).collect::<Vec<_>>();

    let result = match join_all(tasks).wait() {
        Ok(v) => v.into_iter().collect(),
        Err(e) => return Err(e.to_string())
    };

    let elapsed = start.elapsed();
    println!("Elapsed: {} ms",(elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

    Ok(result)
}
