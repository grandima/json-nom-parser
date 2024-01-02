# handshake_payload JSON Parser

This library parses a specific JSON format, exemplified by the handshake payload data used in [OpenMina](https://metrics.openmina.com/network/messages/14?node=node1) for nodes like `node1` under the stream kind `/noise` and message kind `handshake_payload`.

### Example Usage

See `test_correctness_all_in_one` in `src/lib.rs` for example usage.

### Tests

- **Correctness Test**: `test_correctness_all_in_one` ensures the parser accurately reads keys and values according to the specified format. 
- **Performance Test**: `bench_parse_json` in `benchmarks/benches/json.rs` evaluates the parser's performance. To run: open terminal from `benchmarks` directory and run `cargo bench`. 

### Parsing Logic

- Whitespaces between keys and values are ignored.
- A map is created with keys as JSON keys and values as the lengths of the corresponding JSON values.
- Keys are read from one quotation mark to another.
- Value lengths are determined based on the keys from the map.
- Values are alphanumeric characters, read from one quotation mark to another. They must have the specified length.
- The parsing result is a vector of tuples of slices to the input `[(key1, value1), (key2, value2)...]`.


### Implementation Notes

- This library is designed for parsing a *specific* JSON structure.
- The parser ignores whitespaces **and does not** validate the length of keys, which could lead to buffer overflow if keys are excessively long or there are a lot of whitespaces.
