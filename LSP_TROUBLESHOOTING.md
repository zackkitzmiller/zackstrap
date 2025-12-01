# 🔧 LSP Troubleshooting Guide

## 🚨 **Issue: Target Specification Error**

### **Error Message**

```
error: error loading target specification: could not find specification for target "target"
```

### **Root Cause**

The error occurs when the LSP (Language Server Protocol) tries to use "target" as a Rust target triple instead of recognizing it as a directory path. This typically happens due to incorrect configuration in `.cargo/config.toml`.

### **Solution**

Change the configuration from:

```toml
[build]
target = "target"  # ❌ Wrong - interpreted as target triple
```

To:

```toml
[build]
target-dir = "target"  # ✅ Correct - specifies directory
```

## 🛠️ **LSP Configuration Best Practices**

### **1. Cargo Configuration**

```toml
# .cargo/config.toml
[build]
# Use target-dir instead of target
target-dir = "target"

# Specify actual target triples
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=native"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "target-cpu=native"]
```

### **2. VS Code Settings**

```json
{
  "rust-analyzer.cargo.target": "x86_64-unknown-linux-gnu",
  "rust-analyzer.cargo.buildScripts.enable": true,
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.cargo.features": "all"
}
```

### **3. LSP Server Configuration**

```json
{
  "rust-analyzer.server.path": "rust-analyzer",
  "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "rust-analyzer.procMacro.server": "rust-analyzer"
}
```

## 🔍 **Common LSP Issues and Solutions**

### **1. Target Specification Errors**

**Symptoms:**

- `error: error loading target specification`
- LSP fails to start
- No code completion or diagnostics

**Solutions:**

1. Fix `.cargo/config.toml` (as shown above)
2. Restart the LSP server
3. Clear LSP cache: `rm -rf target/.rustc_info.json`

### **2. Dependency Resolution Issues**

**Symptoms:**

- "Failed to run `rustc` to learn about target-specific information"
- Missing dependencies in LSP
- Incomplete code completion

**Solutions:**

```bash
# Clean and rebuild
cargo clean
cargo check

# Update dependencies
cargo update

# Rebuild with fresh cache
rm -rf target/
cargo build
```

### **3. LSP Server Crashes**

**Symptoms:**

- LSP server stops responding
- No error messages in LSP output
- Editor shows "LSP not responding"

**Solutions:**

1. **Restart LSP Server:**
   - VS Code: `Ctrl+Shift+P` → "Rust Analyzer: Restart Server"
   - Vim/Neovim: `:LspRestart`

2. **Check LSP Logs:**
   - Look for error messages in LSP output
   - Check for memory issues or infinite loops

3. **Update LSP Server:**
   ```bash
   # Update rust-analyzer
   cargo install --locked --force rust-analyzer
   ```

### **4. Performance Issues**

**Symptoms:**

- Slow code completion
- High CPU usage
- LSP server timeouts

**Solutions:**

1. **Optimize Cargo Configuration:**

   ```toml
   [build]
   target-dir = "target"
   incremental = true

   [profile.dev]
   debug = 1  # Reduce debug info
   ```

2. **Exclude Large Directories:**

   ```json
   {
     "rust-analyzer.files.excludeDirs": ["target", "node_modules", ".git"]
   }
   ```

3. **Limit LSP Features:**
   ```json
   {
     "rust-analyzer.completion.enable": true,
     "rust-analyzer.hover.enable": true,
     "rust-analyzer.diagnostics.enable": true,
     "rust-analyzer.procMacro.enable": false
   }
   ```

## 🔧 **LSP Debugging Commands**

### **Check LSP Status**

```bash
# Check if rust-analyzer is installed
which rust-analyzer

# Check rust-analyzer version
rust-analyzer --version

# Check Rust toolchain
rustc --version
cargo --version
```

### **Test LSP Functionality**

```bash
# Test basic compilation
cargo check

# Test with all features
cargo check --all-features --all-targets

# Test specific target
cargo check --target x86_64-unknown-linux-gnu
```

### **Clear LSP Cache**

```bash
# Clear target directory
cargo clean

# Clear LSP cache
rm -rf target/.rustc_info.json
rm -rf .vscode/settings.json

# Restart LSP server
```

## 📊 **LSP Performance Monitoring**

### **Check LSP Performance**

```bash
# Monitor LSP process
ps aux | grep rust-analyzer

# Check memory usage
top -p $(pgrep rust-analyzer)

# Check LSP logs
tail -f ~/.local/share/rust-analyzer/logs/
```

### **Optimize LSP Settings**

```json
{
  "rust-analyzer.cargo.buildScripts.enable": true,
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.completion.enable": true,
  "rust-analyzer.hover.enable": true,
  "rust-analyzer.diagnostics.enable": true,
  "rust-analyzer.inlayHints.enable": true,
  "rust-analyzer.lens.enable": true
}
```

## 🚀 **LSP Setup for Different Editors**

### **VS Code**

1. Install "rust-analyzer" extension
2. Configure settings in `settings.json`
3. Restart VS Code after configuration changes

### **Vim/Neovim**

1. Install LSP client (e.g., nvim-lspconfig)
2. Configure rust-analyzer
3. Set up keybindings for LSP functions

### **Emacs**

1. Install lsp-mode
2. Configure rust-analyzer
3. Set up company-mode for completion

## 🔍 **Troubleshooting Checklist**

### **Before Reporting Issues**

- [ ] Check `.cargo/config.toml` for incorrect target settings
- [ ] Verify Rust toolchain is properly installed
- [ ] Clear LSP cache and restart server
- [ ] Check LSP logs for error messages
- [ ] Test basic `cargo check` functionality
- [ ] Verify all dependencies are installed

### **Common Fixes**

1. **Fix target specification**: Use `target-dir` instead of `target`
2. **Clear cache**: Run `cargo clean` and restart LSP
3. **Update tools**: Update rust-analyzer and Rust toolchain
4. **Check configuration**: Verify LSP settings are correct
5. **Restart everything**: Restart editor and LSP server

## 📚 **Additional Resources**

- [rust-analyzer Documentation](https://rust-analyzer.github.io/)
- [Cargo Configuration](https://doc.rust-lang.org/cargo/reference/config.html)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [VS Code Rust Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

---

_This guide covers the most common LSP issues and their solutions. For persistent problems, check the rust-analyzer GitHub issues or seek help from the Rust community._
