#[macro_use]
extern crate criterion;

use json_nom_parser::parse_json;
use criterion::Criterion;
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
static NETWORK_DATA: &str = include_str!("../network_data.json");
fn bench_parse_json(c: &mut Criterion) {
    c.bench_function("parse specific json benchmark", |b|{
        b.iter(||{
            let mut map = json_nom_parser::KEY_VALUE_SIZE.clone();
            _ = parse_json(&mut map)(NETWORK_DATA);
        });
    });
}
criterion_group!(benches, bench_parse_json);
criterion_main!(benches);
