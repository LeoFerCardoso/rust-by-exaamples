# Hello World
 This is the source code of the traditional Hello World program.

 println! is a macro that prints text to the console.

A binary can be generated using the Rust compiler: rustc.

```
$ rustc hello.rs
```
rustc will produce a hello binary that can be executed.

```
$ ./hello
Hello World!
```
But the best way to compile and run your code, specially for real world projects, is using Cargo.

```
$ cargo run
```

This command will compile and run your program at once.

```
$ cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.07s
Running `target\debug\hello-world.exe`
Hello World!
```