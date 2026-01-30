# Shmulinette

## Overview

Shmulinette is a high-performance automated testing suite inspired bypassing
my_marvin. Built in Rust for speed and safety, it leverages JSON-based test 
configurations to validate projects. By utilizing the Nix package manager, 
Shmulinette ensures consistent dependency management and effortless 
cross-platform installation, bypassing the common pitfalls of traditional 
package managers like apt or dnf

## Run

If you have cloned the repository, execute the following from the root directory:
    
```bash
nix run . -- [test_file] [options]
```
Run the latest version directly from the cloud:
```bash
nix run github:shmul95/shmulinette -- [test_file] [options]
```
To install Shmulinette as a global binary on your system:
```bash
nix profile add github:shmul95/shmulinette
shmulinette [test_file] [options]
```

> [!TIP]
> **Keep it updated:** If you have installed the profile, stay up to date with the latest features by running:
> `nix profile upgrade 0` (or the specific index of the package).
> to see indexes run `nix profile list`

* `[test_file]`: The name of your test configuration (the `.json` extension is optional).
* If you have no tests.json try to use some of the repo's test cases, like `tek2/nanotekspice`
* `-o, --only [test_names]`: Run only specific tests from the file.
* `-e, --exclude [test_names]`: Skip specific tests during execution.

## Pipeline

```
       CONFIGURATION                       EXECUTION PHASE
      +--------------+              +----------------------------+
      |  Shmulifile  |              |      Read JSON Tests       |
      +--------------+              +--------------+-------------+
             |                                     |
             v                                     v
      +--------------+              +----------------------------+
      | Replace Bin  |              |    Serde Syntax Check      |
      |  in JSON     |              |    & Logic Verification    |
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

## Shmulifile

Shmulinette looks for a `Shmulifile` in your current directory to understand how to interact with your project. It currently supports 3 parameters:

```ini
BUILD=cargo build
BIN=cargo run
SEPARATOR=true

```

* **`BUILD`**: The command used to compile your project. If omitted, the build phase is skipped.
* **`BIN`**: The path to the binary or the execution command.
* **`SEPARATOR`**: Set to `true` if your binary requires a `--` to pass arguments (e.g., `cargo run -- [args]`). Defaults to `false`.

## Json Test File

Tests are defined in JSON format. Each test case follows this structure:

```json
{
    "name": "example_test",
    "command": "@BIN -h",
    "result": "Usage: ./my_bin [options]",
    "status": 0
}

```

* **`@BIN`**: This placeholder is automatically replaced by the `BIN` and `SEPARATOR` defined in your `Shmulifile`.
* **`status`**: The expected exit code of the process (e.g., `0` for success).

