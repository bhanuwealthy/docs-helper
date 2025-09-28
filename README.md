# cp_docs

`cp_docs` is a small Rust command-line tool.

It recursively finds all `docs/` directories within a specified path and copies their contents to a target directory, preserving the original folder structure.


## Dependencies

- Rust (https://www.rust-lang.org/)
- Cargo (Rust's package manager)


# cp_docs



## Install from crates

```bash
cargo install cp_docs
```


## Install from source

```bash
git clone https://github.com/bhanuwealthy/docs-helper.git
cd docs-helper
cargo build --release
```


## Usage
```bash
$ cp_docs <source> <target>

# OR cargo run
$ cargo r -- ../docs-test ../docs-test-result
```
```bash

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
    │       │   ├── a-b-e.md
    │       │   └── e.md
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

## Docker Usage (untested)
### `Prefer cargo install or cargo run Over docker`

To use `docs-helper` within a Docker container, follow these steps:

1.  **Build the Docker image:**
    Navigate to the root of your `docs-helper` project (where the `Dockerfile` is located) in your terminal and run:
    ```bash
    docker build -t cp_docs .
    ```
    This command builds a Docker image named `cp_docs`.

2.  **Run the Docker container:**
    You'll need to mount your source and target directories from your host machine into the container using volume mounts (`-v`).

    **Example:**
    If your project with `docs/` folders is in `my_project_root` on your host and you want the output in `my_project_root/dist`:

    ```bash
    # From your 'my_project_root' directory
    docker run -v $(pwd):/app/source_code -v $(pwd)/dist:/app/output_docs cp_docs /app/source_code /app/output_docs
    ```

    *   `-v $(pwd):/app/source_code`: Mounts your current host directory (`$(pwd)`) as `/app/source_code` inside the container. This should be the root path where `cp_docs` will start searching for `docs/` directories.

    *   `-v $(pwd)/dist:/app/output_docs`: Mounts your host's `dist` directory as `/app/output_docs` inside the container. This is where the processed documentation will be written.

    *   `cp_docs`: The name of the Docker image you built.

    *   `/app/source_code /app/output_docs`: These are the arguments passed to the `cp_docs` executable inside the container, specifying the root to scan and the target output directory, respectively.
