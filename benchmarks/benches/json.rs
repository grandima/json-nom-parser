#[macro_use]
extern crate criterion;

use std::time::Duration;
use json_nom_parser::*;
use criterion::Criterion;
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
static NETWORK_DATA: &str = include_str!("../network_data.json");
fn bench_parse_json_whitespace(c: &mut Criterion) {
    c.bench_function("parse no key checking json benchmark", |b|{
        b.iter(||{
            let mut map = json_nom_parser::KEY_VALUE_SIZE.clone();
            _ = parse_json(&mut map)(NETWORK_DATA);
        });
    });
}

fn bench_parse_json_key_length(c: &mut Criterion) {
    c.bench_function("parse key checking json benchmark", |b|{
        b.iter(||{
            let mut map = json_nom_parser::LEN_KV_SIZE.clone();
            _ = parse_sized_json(&mut map)(NETWORK_DATA);
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = bench_parse_json_key_length, bench_parse_json_whitespace
}
criterion_main!(benches);
