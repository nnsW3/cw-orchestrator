# Osmosis Test Tube

Osmosis Test Tube is a rust-based test framework that allows developers to test for multi-contract interactions without having to dispatch messages, storage and other variables themselves. This environment is even closer to the actual on-chain mechanisms than [Cw Multi Test](./cw-multi-test.md), because it runs tests directly on the actual chain binaries. With cw-orchestrator, most of the `osmosis-test-tube` logic is abstracted away, but you can still <a href="https://docs.rs/crate/osmosis-test-tube/latest" target="_blank">learn more about the framework here</a>.

## Prerequesites

In order to use `osmosis-test-tube`, the library needs to be able to compile and install the chain binaries. For that you will need to install a go compiler as well as the clang library. Run the following commands to install both of those libraries.

### Ubuntu

1. <a href="https://go.dev/doc/install" target="_blank">Install go</a>
2. Install the clang library

```bash
    sudo apt install clang
```

### Arch Linux

1. Install go

```bash
    sudo pacman -Sy go
```

2. Install the clang library

```bash
    sudo pacman -Sy clang
```

## Quick Start

Before starting, here is a few examples utilizing the osmosis-test-tube environment

- <a href="https://github.com/AbstractSDK/cw-orchestrator/blob/main/packages/cw-orch-osmosis-test-tube/examples/basic.rs" target="_blank">Simple test with `osmosis-test-tube`</a>
- <a href="https://github.com/AbstractSDK/cw-orchestrator/blob/main/packages/cw-orch-osmosis-test-tube/examples/complex.rs" target="_blank">More complex example with `token-factory` messages</a>


To be able to use `osmosis-test-tuee` you need to add the <a href="https://crates.io/crates/cw-orch-osmosis-test-tube" target="_blank">`cw-orch-osmosis-test-tube`</a> dependency inside your crate. 

Creating a test environment in cw-orchestrator that leverages `osmosis-test-tube` goes along the lines of:

```rust,ignore
    use cw_orch::prelude::*;
    use cw_orch_osmosis_test_tube::OsmosisTestTube;
    use cosmwasm_std::coins;
{{#include ../../../packages/cw-orch-osmosis-test-tube/examples/basic.rs:osmosis_test_tube_creation}}
```

This snippet will create a new address, provide it with initial balances and create the `osmosis-test-tube` environment.
The addresses are not handled like in the `cw-multi-test` environment or in mock tests and can't be decided upon manually. You will learn more later about [handling addresses in the OsmosisTestTube environement](#additional-tools).

> **_NOTE:_** When using `osmosis-test-tube`, the addresses **are** validated like on a live chain.

> **_NOTE:_** When using `osmosis-test-tube`, gas fees are charged to the sender address. The gas fees don't represent the actual gas fees you will occur when interacting with the actual chain. That's why in the test snippet above, the amount of `uosmo` instantiated with the account is very high.

## Interacting with contracts

You can then use the resulting `OsmosisTestTube` variable to interact with your [contracts](../contracts/index.md):

```rust,ignore
{{#include ../../../packages/cw-orch-osmosis-test-tube/examples/basic.rs:osmosis_test_tube_usage}}
```

When executing contracts in an `osmosis_test_tube` environment, the messages and sub-messages sent along the Response of an endpoint, will be executed as well. This environment mimics the actual on-chain execution by dispatching the messages inside the actual chain binaries.

> This environment uses wasm files compiled from the project. Therefore, you need to compile the WASM artifacts from the project for your osmosis-test-tube integration tests. You will need to have the `wasm` function fom the `Uploadable`trait implemented . [See the dedicated page for more details](../contracts/interfaces.md#creating-an-interface).

## Cloning

When cloning an `osmosis_test_tube` environment, you are not cloning the entire environment, but instead you are creating a new `OsmosisTestTube` typed variable with the same underlying `osmosis_test_tube::App` object reference. This is useful for objects that require to pass the chain as an object rather than by reference.
The underlying `osmosis_test_tube::App` object is however not clonable.

## Additional tools

The `OsmosisTestTube` test environment allows you to change application variables (such as the balance of an account) using wrappers around the underlying `osmosis_test_tube::App` object. Here are some examples of those wrappers in context:


```rust,ignore
{{#include ../../../packages/cw-orch-osmosis-test-tube/examples/basic.rs:osmosis_test_tube_customization}}
```

> **_NOTE:_** With OsmosisTestTube, you can't create accounts with a specific address. Accounts are created by the app directly and you don't have a lot of control over them. As in the example above, use `OsmosisTestTube::init_account` to create a new account.


## Additional customization

As we don't provide wrappers around each and every functionality that `osmosis_test_tube` provides, you can also customize the underlying `osmosis_test_tube::App` object to your specific needs. In the following example, we increase the block time in the test environment:

```rust,ignore
{{#include ../../../packages/cw-orch-osmosis-test-tube/examples/basic.rs:deep_osmosis_test_tube_customization}}
```

