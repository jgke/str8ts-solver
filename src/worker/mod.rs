use futures::TryStreamExt;
use gloo_worker::{HandlerId, Worker, WorkerScope};
use js_sys::{JsString, Uint8Array};
use serde::{Deserialize, Serialize};
use solver::generator;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_streams::ReadableStream;

pub mod codec;

#[derive(Serialize, Deserialize)]
pub struct HashInput {
    pub size: usize,
    pub blocker_count: usize,
    pub blocker_num_count: usize,
    pub target_difficulty: usize,
    pub symmetric: bool,
}

#[derive(Serialize, Deserialize)]
pub struct HashOutput {
    pub puzzle: String,
}

pub struct HashWorker {}

impl Worker for HashWorker {
    type Input = HashInput;
    type Output = HashOutput;
    type Message = ();

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {}
    }

    fn connected(&mut self, _scope: &WorkerScope<Self>, _id: HandlerId) {}

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        let scope = scope.clone();

        spawn_local(async move {
            let grid = generator::generator(
                msg.size,
                msg.blocker_count + msg.blocker_num_count,
                msg.blocker_num_count,
                msg.target_difficulty,
                msg.symmetric,
            );

            scope.respond(
                id,
                HashOutput {
                    puzzle: grid.to_string(),
                },
            );
        });
    }
}
