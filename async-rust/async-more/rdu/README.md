# rdu - disk usage

## Benchmarking

### hyperfine

```
cargo install hyperfine
```

### Max open files

To run the concurrent version of the disk usage calculator,
increase the total number of open files to what is supported.
Also set the same value or lower for the env `MAX_OPEN_FILES`.

```
ulimit -n <max-supported>
```

### Benchmark with hyperfine

Benchmark with linux kernel source code directory.

```
export MAX_OPEN_FILES=409600
cd target/release
hyperfine --warmup 10 -L exe du,./rdu-sync,./rdu-async,./rdu-async-conc '{exe} -hsl ~/SRC/linux'
```
