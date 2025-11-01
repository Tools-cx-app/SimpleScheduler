# **SimpleScheduler**

Just a simple scheduler for Android

## Introduction

> It is a scheduler implementation running in user space

- ### **What is `SimpleScheduler`?**

  - `SimpleScheduler` is a scheduler implementation running in user space. It supports configuring different scheduling modes and switching scheduling for individual applications.

## **Customization (Configuration)**

- ### **Configuration Path: `/data/adb/SimpleScheduler/config.toml`**

- ### **Parameter (`config`) Description:**

  - **`config` Section**
    - `general`: Represents the default mode used by applications.
      - Available values: `powersave`, `balance`, and `performance`.
    - `powersave`: Represents the list of applications using the `powersave` mode. Package names are provided in the array in string format.
      - Example: `powersave = ["com.example.app1", "com.example.app2"]`.
    - The configuration rules for the `balance` and `performance` fields are the same as for the `powersave` field.
  - **`freqs` Section**
    - `general`: This field is meaningless.
    - `powersave`: Uses an array with two elements to represent the maximum and minimum frequencies for the current mode (`powersave`). MHz and kHz are allowed.
      - Example: `powersave = [1000, 1000]`.
    - The configuration rules for the `balance` and `performance` fields are the same as for the `powersave` field.
  - **`governors` Section**
    - Specifies the kernel `governor` mode used for each mode.
    - If not necessary, it is recommended to keep it as "walt".
    - Perhaps the Wind Sprint governor can be enabled by setting this value to `scx`?

Acknowledgements

- [shadow3aaa](https://github.com/shadow3aaa)
- [grz-1](https://github.com/grz-1)
- [AlexLiuDev233](https://github.com/AlexLiuDev233)