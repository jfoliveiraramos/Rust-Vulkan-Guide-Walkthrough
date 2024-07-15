# Rust Vulkan Guide Walkthrough

This is a personal project following an online guide on Vulkan with Rust. 
The objective is to learn and lay the necessary foundations to later develop an engine for future projects, while also learning Rust.

## Resources

- [Rust Vulkan Tutorial](https://kylemayes.github.io/vulkanalia/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Documentation](https://doc.rust-lang.org/std/)

---

## Ubuntu Installation Requirements

### Rust 

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Vulkan SDK

- `vulkan-tools` - Command line utilities for the Vulkan ecosystem, including `vkcube` and `vulkaninfo`.
- `libvulkan-dev` - Installs the Vulkan loader.
- `vulkan-validationlayers-dev` -  Installs the standard validation layers.

```bash
sudo apt install vulkan-tools libvulkan-dev vulkan-validationlayers-dev
```

Test the Vulkan SDK with:
```bash
vkcube
```

## Run

```bash
cargo run
```

To see debug logs:
```bash
RUST_LOG="debug" cargo run
```

---

# Progress

Current Guide Chaper: [Window Surface](https://kylemayes.github.io/vulkanalia/presentation/window_surface.html)

---

Last Updated: 11-07-2024
