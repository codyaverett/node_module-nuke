# 🚀 node_module-nuke

*The nuclear option for those pesky `node_modules` directories*

## 💥 What is this?

Are you tired of `node_modules` directories multiplying like rabbits and eating your disk space? Do you have more `node_modules` folders than actual projects? Has your SSD started crying every time you run `npm install`?

**Fear no more!** `node_module-nuke` is here to save the day (and your storage).

This blazingly fast Rust-powered tool will hunt down every single `node_modules` directory on your system and give you the power to **DELETE THEM ALL** with the satisfaction of watching your free disk space counter go *brrrrr*.

## 🎯 Why node_modules directories are the worst

- 📦 They're basically black holes that suck up disk space
- 🐌 They make your backups slower than dial-up internet
- 🔍 They hide in every corner of your filesystem like digital dust bunnies
- 💸 They cost you money in cloud storage fees
- 🤡 They contain 47,000 packages just to center a div
- 🏠 Each one could house a small family of dependencies
- 📚 They have more files than the Library of Congress
- 🌕 They're probably visible from space at this point

## ✨ Features

- ⚡ **Blazingly fast** - Written in Rust because life's too short for slow tools
- 🎯 **Surgical precision** - Only targets `node_modules` directories (your code is safe!)
- 📊 **Progress bars** - Watch your disk space come back to life in real-time
- 🏃‍♂️ **Parallel processing** - Uses all your CPU cores to maximize the carnage
- 🧪 **Dry run mode** - Preview the destruction before committing to it
- 🎨 **Pretty output** - Because even nuclear warfare should look good
- 🛡️ **Exclude paths** - Protect your production builds (if you must)

## 🚀 Installation

### From crates.io (once published):
```bash
cargo install node_module-nuke
```

### From source (local development):
```bash
# Clone the repository
git clone <repository-url>
cd node_module-nuke

# Install locally
cargo install --path .
```

### Just run it without installing:
```bash
cargo run
```

## 📦 Publishing to crates.io

For maintainers who want to publish this tool:

1. **Prepare your package:**
   ```bash
   # Make sure everything builds
   cargo build --release
   cargo test
   ```

2. **Update Cargo.toml metadata:**
   ```toml
   [package]
   name = "node_module-nuke"
   version = "0.1.0"
   edition = "2021"
   description = "Efficiently delete node_modules directories with nuclear precision"
   license = "MIT"
   repository = "https://github.com/yourusername/node_module-nuke"
   homepage = "https://github.com/yourusername/node_module-nuke"
   documentation = "https://docs.rs/node_module-nuke"
   keywords = ["cli", "node_modules", "cleanup", "disk-space", "nodejs"]
   categories = ["command-line-utilities", "filesystem"]
   ```

3. **Login to crates.io:**
   ```bash
   cargo login
   # Enter your API token from https://crates.io/me
   ```

4. **Publish:**
   ```bash
   # Dry run first to check everything
   cargo publish --dry-run
   
   # Actually publish
   cargo publish
   ```

5. **After publishing, users can install with:**
   ```bash
   cargo install node_module-nuke
   ```

## 🎮 Usage

### Basic nuclear strike (current directory):
```bash
node_module-nuke
```

### Target a specific directory:
```bash
node_module-nuke ~/Projects
```

### See what would be destroyed (dry run):
```bash
node_module-nuke --dry-run
```

### Maximum verbosity (watch the world burn):
```bash
node_module-nuke --verbose
```

### Limit the depth of destruction:
```bash
node_module-nuke --depth 3
```

### Spare some directories from the apocalypse:
```bash
node_module-nuke --exclude ./important-project/node_modules,./another-project/node_modules
```

### Get help (you'll need it):
```bash
node_module-nuke --help
```

## 🎪 Example Output

```
Scan complete in 0.42s:
- Folders found: 47
- Total size: 12.34 GB
- Estimated savings: 12.34 GB

Proceed with deletion? (yes/no): yes

Deleting... ████████████████████ 47/47 [00:03] Freed: 12.34 GB

Deletion complete in 3.14s:
- Folders deleted: 47
- Space freed: 12.34 GB
```

*Your SSD just sighed with relief.*

## ⚠️ Warning

This tool is nuclear. It will delete `node_modules` directories. Forever. Gone. Poof. 💨

Make sure you can recreate them with `npm install` or `yarn install` before proceeding. 

(But let's be honest, you probably should have done this ages ago.)

## 🤝 Contributing

Found a bug? Want to make it even more destructive? PRs welcome!

Just remember: with great power comes great responsibility. Use this tool wisely.

## 📜 License

MIT - Because even nuclear weapons should be free and open source.

---

*"I used to have 50GB of node_modules. Now I have 50GB of free space and inner peace."* - Happy User

*"This tool gave me back my weekend. And my sanity."* - Another Happy User

*"node_modules directories hate this one simple trick!"* - Definitely Not Clickbait