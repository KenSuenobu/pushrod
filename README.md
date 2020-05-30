# pushrod

[![Build Status](https://travis-ci.org/KenSuenobu/pushrod.svg?branch=master)](https://travis-ci.org/KenSuenobu/pushrod)
[![](https://img.shields.io/crates/d/pushrod.svg)](https://crates.io/crates/pushrod)
[![docs.rs for pushrod](https://docs.rs/pushrod/badge.svg)](https://docs.rs/pushrod)

Pushrod is a UI library written in Rust from the ground up.  It utilizes the 
[Rust SDL2 Library](https://github.com/Rust-SDL2/rust-sdl2) crate for handling system
events and GPU-based drawing.

This library consists of two parts:

- [Pushrod](https://www.github.com/KenSuenobu/pushrod/) main library, containing the controller
- [Pushrod-Widgets](https://www.github.com/KenSuenobu/pushrod-widgets/) widgets library, containing the master widgets

It also includes a builder application:

- [Pushrod-Chassis](https://www.github.com/KenSuenobu/pushrod-chassis/), a GUI-based application layout builder

