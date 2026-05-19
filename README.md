# AUDK32 A31xxxx Series

AUDK32-A31xxxx is an SDK package for ABOV A31xxxx microcontroller series.  
It provides CMSIS-based device support, HAL drivers, product configuration files, example projects, and OpenOCD support files.

## Included Components

- CMSIS core support
- Device startup and system files
- HAL drivers
- Example applications
- Product configuration headers
- SVD files for debugging
- FPA files
- OpenOCD support files
- Terminal macro scripts for Minicom and Tera Term

## Development Environments

This SDK includes project files or build support for:

- ARM/Keil
- IAR Embedded Workbench
- Eclipse
- OpenOCD-based debugging

## Getting Started

1. Clone this repository.
```bash
git clone --recursive https://github.com/abovsemiconductor/AUDK32-A31xxxx.git
```
2. Select the target device family under ProductConfig/.
3. Open an example project from Example/Build/.
4. Build the project using the supported IDE or toolchain.
5. Flash and debug the target using the provided project settings or OpenOCD configuration.

## Configuration

Common SDK configuration headers are located in ProductConfig/Config/.

Main configuration files include:
```text
abov_config.h
abov_example_config.h
abov_module.h
abov_module_config.h
```
Device-specific configuration files are located under each device family directory
```text
ProductConfig/<DeviceFamily>/Config/.
```

## Debug Support

SVD files are provided for supported device families:
```text
ProductConfig/<DeviceFamily>/SVD/
```
OpenOCD-related files are located in:
```text
Tool/OpenOCD/
```
