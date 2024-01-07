# handshake_payload JSON Parser

This library parses a specific JSON format, exemplified by the handshake payload data used in [OpenMina](https://metrics.openmina.com/network/messages/14?node=node1) for nodes like `node1` under the stream kind `/noise` and message kind `handshake_payload`.

### Example Usage

See `test_key_length_correctness_all_in_one` in `tests/key_length.rs` for example usage.

### Tests

- **Correctness Test**: `test_key_length_correctness_all_in_one` ensures the parser accurately reads keys and values according to the specified format. 
- **Performance Test**: `bench_parse_json_key_length` in `benchmarks/benches/json.rs` evaluates the parser's performance. To run: open terminal from `benchmarks` directory and run `cargo bench`. 

### Parsing Logic

#### There are two implementations. 

##### With whitespaces:
- Whitespaces between keys and values are ignored.
- A map is created with keys as JSON keys and values as the lengths of the corresponding JSON values.
- Keys are read from one quotation mark to another.
- Value lengths are determined based on the keys from the map.
- Values are alphanumeric characters, read from one quotation mark to another. They must have the specified length.
- The parsing result is a vector of tuples of slices to the input `[(key1, value1), (key2, value2)...]`.
- The parser **ignores** whitespaces **and does not** validate the length of keys. It will lead to **buffer overflow** if keys are excessively long or there are a lot of whitespaces.

##### Key length checking:

- A BTreeMap<Key_legnth, HashMap<Key, Value_length>> is created where Key is a length of the key that must exist in a JSON. The Value is a HashMap where the Key is the key that must exist in a JSON and the Value is a length of the value that must correspond to a key in a JSON.
- Keys are read from one quotation mark to another **and** in a range from the lowest length to the highest length from a BTreeMap.
- Get the HashMap by the length of the read key from BTreeMap.
- Get and remove the length of the value from the HashMap by the read key. If the HashMap's length is 0, remove the entry from BTreeMap.
- Values are alphanumeric characters, read from one quotation mark to another. They must have the read length.
- The parsing result is a vector of tuples of slices to the input `[(key1, value1), (key2, value2)...]`.
- The parser **does not** allow whitespaces or any other characters. It **does validate** the key length as well as the value's length. As a result, this solution is more efficient and **is claimed to protect** from buffer overflow attacks.