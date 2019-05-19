#[macro_use]
extern crate criterion;

use libcnab::cnab::Bundle;

use criterion::{black_box, Benchmark, Criterion, Throughput};
use std::fs::File;
use std::io::Read;

fn serialize(c: &mut Criterion) {
    let mut json = String::new();
    File::open("./testdata/bundle.json")
        .unwrap()
        .read_to_string(&mut json)
        .unwrap();
    let bundle = Bundle::from_string(&json).unwrap();
    let size = Throughput::Bytes(serde_json::to_string(&bundle).unwrap().len() as u32);

    c.bench(
        "Bundle",
        Benchmark::new("serialize", move |b| {
            b.iter(|| serde_json::to_string(black_box(&bundle)).unwrap())
        })
        .throughput(size),
    );
}

fn deserialize(c: &mut Criterion) {
    let mut json = String::new();
    File::open("./testdata/bundle.json")
        .unwrap()
        .read_to_string(&mut json)
        .unwrap();
    let size = Throughput::Bytes(json.len() as u32);

    c.bench(
        "Bundle",
        Benchmark::new("deserialize", move |b| {
            b.iter(|| Bundle::from_string(black_box(&json)).unwrap())
        })
        .throughput(size),
    );
}

criterion_group!(benches, serialize, deserialize);
criterion_main!(benches);
