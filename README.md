# TutRust

## Basics
### Installation Linux
1. download: `curl https://sh.rustup.rs -sSf | sh` 
2. Rust PATH in zshrc oder bashrc einfügen: `. "$HOME/.cargo/env" `

### Rust package contains
1. rustc: compiler
2. cargo: build/package tool 

### project and cargo 
1. create new project: `cargo new my_project` name should be snake case
2. build and run: `cargo run my_project` 

### VsCode Extension
1. rust-analyzer
2. linkedProjects for rust-analyzer
```
.vscode -> settings.json (folder settings)
{
  "rust-analyzer.linkedProjects": [
    ".pathto/Cargo.toml"
  ]
}
```

## Data
### copy vs reference and owndership
In Rust only data with no heap allocation have a copy mechanism.
For example:
- vector
- deque
etc with heap allocations, there are no copy mechanism.

```
let mut my_vec: Vec<u32> = = vec![1, 2, 3, 4, 5];
let mut my_vec2 = my_vec; // my_vec is moved to my_vec2, ownership is moved to my_vec2
```
for example iteration over a map
```
use std::collections::HashMap;
let mut my_map: HashMap<String, i32> = HashMap::new();
my_map.insert("Daniel".to_string(), 95);

for(key, value) in &my_map { // referenz to my_map, ownership is not changed or moved
}