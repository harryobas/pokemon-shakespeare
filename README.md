# pokemon-shakespeare

pokemon-shakespeare provides a nifty, restful api endpoint for retriving the shakespearean description of a pokemon. It is written in Rust with the rocket.rs web framework.

## Requirements

1. Rust toolchain

2. rustc 1.52.0-nightly (rocket.rs requires nightly version of the Rust compiler)

## Installation 

1. clone repo and change directory:
    ```
    git clone https://github.com/harryobas/pokemon-shakespeare.git && cd pokemon-shakespeare

    ```
2. set compiler version to nightly:
    ```
    rustup override set nightly

    ```
3. run tests and start app:
    ```
    cargo test && cargo run 

    ```

## API Endpoint

localhost:8000/pokeman/<pokemon-name>




