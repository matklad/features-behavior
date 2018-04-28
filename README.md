This repository demonstrates what behavior was changed by
https://github.com/rust-lang/cargo/issues/5364#issuecomment-385173824

```
~/tmp/foo master* 10s
λ cargo +stable run -p bar --features xyz
   Compiling bar v0.1.0 (file:///home/matklad/tmp/foo/bar)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35 secs
     Running `target/debug/bar`
xyz disabled

~/tmp/foo master*
λ cargo +nightly run -p bar --features xyz
   Compiling bar v0.1.0 (file:///home/matklad/tmp/foo/bar)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37 secs
     Running `target/debug/bar`
xyz enabled
```
