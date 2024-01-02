#[macro_use]
extern crate criterion;

use json_nom_parser::parse_json;
use criterion::Criterion;

fn bench_parse_json(c: &mut Criterion) {
    c.bench_function("parse specific json benchmark", |b|{
        b.iter(||{
            let mut map = json_nom_parser::KEY_VALUE_SIZE.clone();
            let input = std::fs::read_to_string("network_data.json").unwrap();
            _ = parse_json(&mut map)(&input);
        });
    });
}
criterion_group!(benches, bench_parse_json);
criterion_main!(benches);
