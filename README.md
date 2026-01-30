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
> to see indexes run `nix progile list`

* `[test_file]`: The name of your test configuration (the `.json` extension is optional).
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
