# Rust Storage Script

This script is designed to copy docs from a source directory (checks all subdirectories recursively) to a destination directory, maintaining folder structure.

## Dependencies

- Rust (https://www.rust-lang.org/)
- Cargo (Rust's package manager)

## Installation and run locally

```
git clone https://github.com/KaustubhMishra25/docs-helper.git
cd docs-helper
cargo build
cargo run -- <source path> <destination path>
```

## Run image

#### The destination folder need not be mounted, a folder name can be provided and it will be creadted accordingly.
```
docker build -t docs-helper .
docker run -v <path_to_local_src>:/source -v <path_to_local_dest>:/destination docs-helper /source /destination
```

## Usage
```shell
$ cargo run -- ../docs-test ../docs-test-result 

$ tree ../docs-test ../docs-test-result 

# src dir
../docs-test
└── A
    ├── B
    │   ├── C
    │   │   └── docs
    │   │       ├── a-b-c.md
    │   │       └── c.md
    │   └── docs
    │       ├── E
    │       │   └── docs
    │       │       ├── a-b-e.md
    │       │       └── e.md
    │       └── b.md
    ├── D
    │   └── d.md
    └── docs
        └── A.md


# target dir
../docs-test-result
└── A
    ├── A.md
    └── B
        ├── C
        │   ├── a-b-c.md
        │   └── c.md
        ├── E
        │   ├── a-b-e.md
        │   └── e.md
        └── b.md

8 directories, 6 files
```
