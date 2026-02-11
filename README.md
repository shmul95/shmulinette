# Shmulinette

## Overview

Shmulinette is a high-performance automated testing suite inspired by `my_marvin`. Built in **Rust** for speed and safety, it leverages JSON-based configurations to validate projects. By utilizing the **Nix** package manager, Shmulinette ensures consistent dependency management and effortless cross-platform installation, bypassing the common pitfalls of traditional package managers like `apt` or `dnf`.

## Run

If you have cloned the repository, execute the following from the root directory:

```bash
nix run . -- [test_file] [options]
```
Run the latest version directly from the cloud:
```bash
nix run github:shmul95/shmulinette -- [test_file] [options]
```
To install Shmulinette as a global binary:
```bash
nix profile add github:shmul95/shmulinette
shmulinette [test_file] [options]
```

> [!TIP]
> **Keep it updated:** If you have installed the profile, stay up to date by running `nix profile upgrade 0` (or the specific index of the package). To see your installed package indexes, run `nix profile list`.

* `[test_file]`: The name of your test configuration (the `.json` extension is optional).
* If you don't have a `tests.json`, try using one of the repo's test cases, such as `tek2/nanotekspice`.
* `-o, --only [test_names]`: Run only specific tests from the file.
* `-e, --exclude [test_names]`: Skip specific tests during execution.

---

## Pipeline

```text
       CONFIGURATION                     EXECUTION PHASE
      +--------------+              +----------------------------+
      |  Shmulifile  |              |      Read JSON Tests       |
      +--------------+              +--------------+-------------+
             |                                     |
             v                                     v
      +--------------+              +----------------------------+
      | Replace Vars |              |    Serde Syntax Check      |
      |   in JSON    |              |    & Logic Verification    |
      +--------------+              +--------------+-------------+
             |                                     |
             v                                     v
      +--------------+              +----------------------------+
      | Build Binary |              |   Filter: Flags -o / -e    |
      +--------------+              +--------------+-------------+
             |                                     |
             +------------------+------------------+
                                |
                                v
                +--------------------------------+
                |     THREADED TEST EXECUTION    |
                |  (Parallel Workers + Output)   |
                +--------------------------------+

```

---

## Shmulifile

Shmulinette looks for a `Shmulifile` in your current directory to understand how to interact with your project. It supports four parameters:

```ini
BUILD=cargo build
BIN=cargo run
SEPARATOR=true
DATA=./res
```

* **`BUILD`**: The command used to compile your project. If omitted, the build phase is skipped.
* **`BIN`**: The path to the binary or the execution command.
* **`SEPARATOR`**: Set to `true` if your binary requires `--` to pass arguments (e.g., `cargo run -- [args]`). Defaults to `false`.
* **`DATA`**: A path variable used for test resources (e.g., a directory containing input files).

---

## Makefile

For simplicity, you can add a rule to your `Makefile` to trigger tests immediately after compilation. This is especially useful for maintaining a fast development loop.

```makefile
run_tests:
	@nix run github:shmul95/shmulinette -- [test_file]
```

Then, simply run:

```bash
make shmuli
```

---

## JSON Test File

Tests are defined in JSON format. Each test case follows this structure:

```json
{
    "name": "example_test",
    "command": "@BIN -h",
    "result": "Usage: ./my_bin [options]",
    "status": 0
}
```

* **`@BIN`**: Automatically replaced by the `BIN` (and `SEPARATOR`, if applicable) defined in your `Shmulifile`.
* **`status`**: The expected exit code of the process (e.g., `0` for success).
* **`result`**: The expected string output from `stdout`.

#### Using Data Placeholders

```json
{
    "name": "example_with_data",
    "command": "@BIN @DATA/input.txt",
    "result": "Success",
    "status": 0
}
```
* **`@DATA`**: Replaced by the `DATA` path defined in your `Shmulifile`. If `@DATA` is used in a JSON file but not defined in the `Shmulifile`, the test will return an error.
