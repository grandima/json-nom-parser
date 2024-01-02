# handshake_payload json parser
This lib parses the specific json format structure. 
The example format is taken from here:

https://metrics.openmina.com/network/messages/14?node=node1

- Node: `node1`
- Direction: `Incoming`
- Stream Kind: `/noise`
- Message Kind: `handshake_payload`

The example usage is located inside `test_correctness_all_in_one`

### Tests: 

The correctness test is `test_correctness_all_in_one` inside `src/lib.rs`.
The performance test is `bench_parse_json` inside `benchmarks/benches/json.rs`

### The parsing logic: 
- all whitespaces between keys and values are ignored
- Create a `map` where `key` is json key and `value` is the json value's length
- the `key` is read from `"` to `"`
- the `value length` is extracted by the `key` from the `map`
- the `value` is read from `"` to `"` but only `value length` times and it must be alphanumeric
- the parsing result is a vector of tuples `[(key, value), (key, value)...]`

### Implementation note:

This library just demonstrates an ability to parse a *specific* json. 
It **is afraid** of a buffer overflow attack. 

Here are the vulnerabilities for that attack:
 - `all whitespaces are ignored`. 
 - the alleged key is read from `"` to `"`. Thus, it can be *any* length.

The only protection that exists in this implementation is `value length`.  