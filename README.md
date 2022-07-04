#log-parser

👋🏼 Hi, during my Rust learning journey I'm trying to publish all my exercises.

`log-parser` is a function that take v8 GC logs as input and parse it.

## Install

Requirements:
- rust
- cargo

```bash
$ cargo build
```

## Run

### Parse logs from a file
```bash
$ cargo run -- --path <path>
```

Example - find `rust` patter in the `README.md` file:
```bash
$ cargo run -- --path ./gc-log.txt
```

Will output: 

```bash
cargo run -- --path gc-log.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/log-parser --path gc-log.txt`
GcStats { phase: Scavenge, timing: Ok("44 ms"), body: "2.3 (3.0) -> 1.9 (4.0) MB, 1.2 / 0.0 ms  (average mu = 1.000, current mu = 1.000) allocation failure" }
GcStats { phase: MarkSweep, timing: Ok("83 ms"), body: "2.3 (3.0) -> 1.9 (4.0) MB, 1.2 / 0.0 ms  (average mu = 1.000, current mu = 1.000) allocation failure" }
GcStats { phase: MarkCompact, timing: Ok("144 ms"), body: "2.3 (3.0) -> 1.9 (4.0) MB, 1.2 / 0.0 ms  (average mu = 1.000, current mu = 1.000) allocation failure" }
```


