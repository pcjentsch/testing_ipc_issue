Sample IPC file is `test_ipc, test_julia.ipc`

To reproduce error

```
git clone https://github.com/pcjentsch/testing_ipc.git
cd testing_ipc/test_polars_ipc
cargo run
```

Output should be
```
     Running `target/debug/test_ipc`
Ok((Schema { fields: [Field { name: "a", data_type: Int64, is_nullable: false, metadata: {} }], metadata: {} }, [Chunk { arrays: [Int64[1, 2, 3]] }]))
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /home/workaccount/.cargo/registry/src/github.com-1ecc6299db9ec823/arrow-9.0.2/src/ipc/reader.rs:636:44
note: run with `RUST_BACKTRACE=1` environment variable to display a backtracee
```

# recreate arrow ipc file in julia (needs Arrow.jl)
```
julia make_ipc.jl
```
