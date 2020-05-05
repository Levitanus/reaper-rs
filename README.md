# reaper-rs

[REAPER](https://www.reaper.fm/) bindings for the [Rust](https://www.rust-lang.org/) programming language

**Attention: *reaper-rs* is in alpha stage. While the low-level API should not change much anymore, the medium-level
API is definitely not stable yet and the high-level API is completely in a state of flux!**

---

## Introduction

*reaper-rs* allows programmers to write plug-ins for the [REAPER](https://www.reaper.fm/) DAW 
(digital audio workstation) using the  [Rust](https://www.rust-lang.org/) programming 
language. It does so by providing raw Rust bindings to the 
[REAPER C++ SDK](https://www.reaper.fm/sdk/plugin/plugin.php) and more convenient APIs on top of that.

## Basics

*reaper-rs* consists of three APIs with different abstraction levels:

### 1. Low-level API

Characteristics:
- Makes access of function pointers safe
- Not really a pleasure to work with because it still doesn't use Rust types
- Advantage: Very easy to keep up-to-date because it's completely auto-generated

### 2. Medium-level API

Characteristics:
- Uses low-level API
- Exposes the original REAPER SDK functions almost 1:1, so as closely as possible **but** contains 
  some improvements, like being able to deal with Rust strings instead of C-strings
- Not very opinionated, just some obvious common-sense adjustments probably every Rust programmer
  would do in order to get the API closer to Rust (if you don't think so and want to suggest a
  different form of adjustment for the low-level API, please raise an issue)
   
### 3. High-level API

Example TODO:
```rust
let track = reaper.get_current_project();
```

- Uses medium-level API
- In some ways opinionated because it uses tools like rxRust to deal with events
- It strives to reflects 1:1 the typical hierarchy of a REAPER project
  (e.g. Project → Track → FX)   
- Very fluid
- Integration tests use this

I think that with the right abstractions in place, you can build sophisticated extensions much
easier, faster and with less bugs because there's no need to take care of the same low-level
stuff again and again.
    
## Use

In addition to writing REAPER extension plug-ins, *reaper-rs* can be used for developing VST plug-ins that use REAPER 
functions. No matter what you choose, the possibilities of interacting with REAPER are essentially the same. The
difference between the two is the context in which your plug-in will run.

An extension plug-in is loaded when REAPER starts and remains active until REAPER quits, so it's perfect to add
some functions to REAPER which should be available globally. Popular examples are 
[SWS](https://www.sws-extension.org/) and [ReaPack](https://reapack.com/) (both written in C++).

A REAPER VST plug-in is loaded as track, take or monitoring FX as part of a particular REAPER project, just like 
any instrument or effect plug-in out there. That also means it can be instantiated multiple times. Examples are 
[Playtime](https://www.helgoboss.org/projects/playtime/) (written in C++) and 
[ReaLearn](https://www.helgoboss.org/projects/realearn/) (written in C++ but being ported to Rust).

In both cases you need to make a library crate of type `cdylib`.

### REAPER extension plug-in

There are several ways to create a REAPER extension plug-in using *reaper-rs*.


#### Scenario 1: High-level API, easiest way (recommended)

The fastest way to get going with the high-level API is to use the `reaper_extension!` macro.

`Cargo.toml`:
```toml
[dependencies]
reaper-rs = { version = "0.1.0", features = ["high-level"]} 
reaper-rs-macros = "0.1.0"

[lib]
name = "my_reaper_extension"
crate-type = ["cdylib"]
```

`lib.rs`:
```rust
use reaper_rs_macros::reaper_extension_plugin;
use reaper_rs::high_level::Reaper;
use std::error::Error;
use c_str_macro::c_str;

#[reaper_extension_plugin(email = "info@example.com")]
fn main() -> Result<(), Box<dyn Error>> {
    let reaper = Reaper::get();
    reaper.show_console_msg(c_str!("Hello world"));
    Ok(())
}
```

Let's quickly go through those lines of code.

The macro sets up a `high_level::Reaper` instance for you. In particular, it takes care of:

- Loading all available REAPER functions
- Setting up file-based logging (TODO)
- Installing the default panic hook (which you can still overwrite by calling `std::panic::set_hook()`)

The macro itself doesn't do much more than exposing an `extern "C" ReaperPluginEntry` function which calls
functions `low_level::bootstrap_reaper_plugin` and `high_level::setup_all_with_defaults()`. So if
for some reason you don't want to use macros, have a look into the macro implementation. No magic there.

#### Scenario 2: You want custom configuration (e.g. for logging)

```rust
use reaper_rs::{reaper_plugin};
use reaper_rs::low_level::ReaperPluginContext;
use reaper_rs::high_level::Reaper;
use std::error::Error;
use c_str_macro::c_str;

#[low_level_reaper_plugin]
fn main(context: ReaperPluginContext) -> Result<(), Box<dyn Error>> {
    Reaper::with_all_functions_loaded(context)
        .setup();
    // TODO
    Ok(())
}
```

#### Scenario 3: You want to use just low-level or medium-level API

- [ ] Add an example for loading just some functions

```rust
use reaper_rs::{reaper_plugin};
use reaper_rs::high_level::Reaper;
use reaper_rs::low_level::ReaperPluginContext;
use std::error::Error;
use c_str_macro::c_str;

#[low_level_reaper_plugin]
fn main(context: ReaperPluginContext) -> Result<(), Box<dyn Error>> {
    let low = low_level::Reaper::with_all_functions_loaded(context.function_provider);
    let medium = medium_level::Reaper::new(low);
    Reaper::with_custom_medium(medium)
        .setup();
    // TODO
    Ok(())
}
```

#### Scenario 4: You have an existing REAPER plugin written in Rust
    
TODO


### REAPER VST plug-in

A REAPER VST plug-in is nothing else than a normal VST plug-in which gets access to functions from the REAPER SDK. There
is already a Rust crate for creating normal VST plug-ins: [vst-rs](https://crates.io/crates/vst). So writing a REAPER
VST plug-in is done by writing a VST plug-in using vst-rs and getting access to the REAPER functions by letting
reaper-rs access the `HostCallback` function.  
    
## Develop

### Build

- `bindgen` should be executed on Linux (including Windows WSL)

#### Fresh Ubuntu 18.04.3 LTS
```sh
sudo apt update
sudo apt install curl git build-essential -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # choose 1 (default)
source $HOME/.cargo/env
rustup default nightly # Not necessary if building just low-level oder medium-level API
cd Downloads
git clone --recurse-submodules https://github.com/helgoboss/reaper-rs.git
cd reaper-rs
cargo build
# => target/debug/libreaper_rs_test_extension_plugin.so
# => target/debug/libreaper_rs_test_vst_plugin.so
# Download REAPER and start it at least one time
ln -s $HOME/Downloads/reaper-rs/target/debug/libreaper_rs_test_extension_plugin.so $HOME/.config/REAPER/UserPlugins/reaper_rs_test_extension_plugin.so


```

## Project background

reaper-rs has been born as part of an effort to port the REAPER extension 
[ReaLearn](https://www.helgoboss.org/projects/realearn/) to Rust and publish it as open-source project. The high-level
API is heavily inspired by ReaPlus, a C++ facade for the REAPER SDK which is a basic building block of the original ReaLearn. 

## Project status

reaper-rs is still in alpha stage

- TODO Known issues and points of improvement
- TODO Incomplete (different APIs have different level of "incomplete")
- TODO Mention TODOs in code
- TODO UI
- Future improvements:
    - [ ] Provide an extension of reaper_plugin macro which allows to load just some functions  