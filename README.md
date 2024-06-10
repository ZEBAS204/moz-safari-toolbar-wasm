You must manually comment the line that check the pixels length of the `zune-jpg` crate decoder `decode_mcu_ycbcr_baseline()`.
```rust
// mcu.rs
210: assert_eq!(pixels_written, pixels.len());
```

## Open the project in a Dev Container

* Install [Remote Development Extension Pack](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack).
* Start VS Code
* You can either:
  * Run the **Dev Containers: Reopen in Container** command from the Command Palette (`F1`, `Ctrl+Shift+P`).
  * Run the **Dev Containers: Open Workspace in Container...** command.
