# rustspace

## For rust 
https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

1 Create a workspace directory
2 cd into that dir
3 Create Cargo.toml
4 Add : 

[workspace]

members = [
    "proj1",
]

5 cargo new proj1
6 For lib cargo new add-one --lib
7. Adding add-one lib as dependency into other proj1
[dependencies]

add-one = { path = "../add-one" }

8. To build in top level run
cargo build

9. To run 
cargo run -p proj1



