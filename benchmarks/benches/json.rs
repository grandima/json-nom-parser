#[macro_use]
extern crate criterion;

use std::collections::HashMap;
use json_nom_parser::parse_json;
use criterion::Criterion;

fn bench_parse_json(c: &mut Criterion) {
    c.bench_function("parse specific json benchmark", |b|{
        b.iter(||{
            let mut map = HashMap::new();
            map.insert("type".to_string(), 7);
            map.insert("public_key".to_string(), 64);
            map.insert("peer_id".to_string(), 52);
            map.insert("signature".to_string(), 128);
            map.insert("payload_type".to_string(), 0);
            map.insert("payload".to_string(), 38);
            let input = std::fs::read_to_string("network_data.json").unwrap();
            _ = parse_json(&mut map)(&input);
        });
    });
}
criterion_group!(benches, bench_parse_json);
criterion_main!(benches);
