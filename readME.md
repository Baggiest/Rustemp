# CPU Temperature Monitor

A simple Rust utility for monitoring CPU core temperatures in real-time.

## Overview

This lightweight application continuously reads temperature data from thermal sensors on Linux systems and displays the information with live updates in your terminal. The temperature values update in place, providing a clean, non-scrolling interface.

## Features

- Real-time monitoring of CPU core temperatures
- Clean single-line display that updates in place
- Low system resource usage
- 500ms refresh rate (configurable)

## Requirements

- Rust programming environment
- Linux-based operating system with accessible thermal sensors
- Thermal zone files available at `/sys/class/thermal/thermal_zone*/temp`

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/baggiest/rustemp.git
   cd rustemp
   ```

2. Build the application:
   ```
   cargo build --release
   ```

3. Run the program:
   ```
   ./target/release/rustemp-cpu
   ```

## Usage

Simply run the application, and it will display the current temperature of your CPU cores:

```
Monitoring CPU temperatures (Press Ctrl+C to exit)...
CPU 0: 45.2° C | CPU 1: 46.8° C
```

Press `Ctrl+C` to exit the program.

## Customization

You can modify the following parameters in the source code:

- Update interval: Change the `interval` value (currently set to 500ms)
- Display format: Modify the print format string to suit your preferences
- Thermal zones: Adjust the file paths if your system uses different thermal zone files

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
