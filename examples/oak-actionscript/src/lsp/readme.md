# ActionScript Language Support (LSP)

This module provides the core logic for implementing Language Server Protocol (LSP) features for ActionScript.

## 🛠️ Components

### `ActionScriptLanguageService`
The primary implementation of the `LanguageService` trait. It manages the workspace, virtual file system (VFS), and coordinates various language-specific providers.

### `ActionScriptHoverProvider`
Provides contextual information when a user hovers over code elements. It currently supports:
- **Classes**: Descriptions of class blueprints.
- **Interfaces**: Details on interface contracts.
- **Functions**: Explanations of function blocks.

## 🚀 Features

- **Real-time Diagnostics**: Provides immediate feedback on syntax and semantic errors.
- **Context-Aware Hover**: Displays relevant documentation based on the AST node at the cursor.
- **Workspace Management**: Tracks multiple files and their relationships within an ActionScript project.

## 📂 Submodules

- **`formatter`**: Handles source code formatting and beautification.
- **`highlighter`**: Provides semantic and syntax highlighting data.
