# System Monitor

A simple System Monitor application built with Rust using the eframe and sysinfo crates. This app displays the current CPU and RAM usage percentage and updates it periodically.
## Features

- Monitors CPU and RAM usage and display and update it periodically
- Refreshes the displayed value every second.
- Built using the eframe GUI framework and sysinfo for system information.

## Requirements

- Rust (installed via rustup)
- tokio runtime (used for async tasks)

## Dependencies

This app uses the following crates:

- eframe: A simple and ergonomic framework for building GUIs in Rust.
- sysinfo: A library to access system information like CPU, memory, and more.
- tokio: An asynchronous runtime to manage periodic updates.
