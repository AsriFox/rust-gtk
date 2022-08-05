# rust-gtk

Testing grounds for GTK/Adwaita apps written in Rust.

This is purely a learning experience.

### Structure

The root 'Cargo.toml' is a virtual manifest for managing workspaces (https://doc.rust-lang.org/cargo/reference/workspaces.html).

It references the following workspaces:

* *test-ui*: an Adwaita app built from XML (content/main-window.ui). Used for markup testing purposes.

* *glium-render*: an implementation of Glium (OpenGL wrapper) renderer in a gtk::GLArea. Can be integrated into any GTK window as a Widget.

  * Glium's Context is using a GLSL program to draw to a frame.

### IDE: *VSCodium*

Extensions:

* **rust-analyzer**: installed from VSIX (https://github.com/rust-lang/rust-analyzer/releases)

  * The extension is not up to date in Open VSX Registry; that necessitates the manual installation.
  
* **CodeLLDB**: installed in-app 

  * Open VSX direct link: https://open-vsx.org/extension/vadimcn/vscode-lldb
  
### Status

**Linux** (Manjaro) build is fine.

**Windows** build is untested. However, the last time *libloading* couldn't find *epoxy* libraries.
