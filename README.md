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

