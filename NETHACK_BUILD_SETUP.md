# NetHack ビルドセットアップ（アプローチ A: 既存 Makefile 利用）

## 📦 必要なツール・ライブラリ

### Linux（Ubuntu/Debian）での最小限インストール

```bash
# 基本的なビルドツール
sudo apt-get update
sudo apt-get install -y build-essential gcc make

# NetHack 専用
sudo apt-get install -y bison flex  # yacc/lex 代替（古い build では必要）

# Lua 対応（NetHack 5.0 は Lua で compile する）
sudo apt-get install -y liblua5.4-dev  # または liblua5.3-dev

# オプション：X11 支援
# sudo apt-get install -y libx11-dev libmotif-dev libxaw7-dev

# オプション：curses 支援
# sudo apt-get install -y libncurses-dev
```

### macOS での最小限インストール

```bash
# Xcode Command Line Tools のインストール
xcode-select --install

# Homebrew で Lua (オプション)
brew install lua@5.4
```

---

## 🔨 ビルド手順（Phase 1b）

### Step 1: NetHack hints ファイル確認

```bash
cd /home/oosawak/Workspace/NetHack/sys/unix
ls hints/  # Linux、macOS の hints ファイル一覧
```

**Linux ユーザの場合：**
```bash
cat hints/linux  # または hints/linux-gnu など
```

### Step 2: NetHack をセットアップ＆ビルド

```bash
cd /home/oosawak/Workspace/NetHack
sh sys/unix/setup.sh sys/unix/hints/linux  # Linux の場合

# または
sh sys/unix/setup.sh sys/unix/hints/macosx  # macOS の場合

# Lua ライブラリをダウンロード
make fetch-Lua

# ビルド実行
make all

# ビルドが失敗したら初期化して再試行
# make spotless
# make all
```

### Step 3: ビルド成功確認

```bash
ls -lh src/nethack  # 実行ファイル確認
file src/nethack    # ファイル形式確認

# または

ls -lh src/libnethack.a  # 静的ライブラリ（通常は make の途中で作られる）
```

---

## 🔗 Rust での統合（build.rs）

ビルド成功後、build.rs で以下を実装：

```rust
// crates/nethack-sys/build.rs

use std::path::PathBuf;
use std::process::Command;

fn main() {
    let nethack_root = PathBuf::from(/* ... */);
    let src_dir = nethack_root.join("src");
    
    // Step 1: NetHack がすでにビルド済みか確認
    let libnethack = src_dir.join("libnethack.a");
    
    if !libnethack.exists() {
        println!("cargo:warning=Building NetHack C library...");
        
        // NetHack を make all で再ビルド
        let status = Command::new("make")
            .arg("clean")
            .arg("all")
            .current_dir(&nethack_root)
            .status()
            .expect("Failed to execute make");
        
        if !status.success() {
            panic!("NetHack build failed");
        }
    }
    
    // Step 2: Rust にライブラリをリンク
    println!("cargo:rustc-link-search=native={}", src_dir.display());
    println!("cargo:rustc-link-lib=static=nethack");
    
    // Step 3: include パスを設定
    println!("cargo:rustc-link-search=native={}", nethack_root.join("include").display());
    
    println!("cargo:rerun-if-changed=build.rs");
}
```

---

## 🚨 トラブルシューティング

| エラー | 原因 | 解決 |
|---|---|---|
| `bison: command not found` | bison がない | `sudo apt-get install bison` |
| `make: yacc: No such file` | yacc が無い | Lua ベースなら不要 |
| `lua.h: No such file` | Lua dev ライブラリない | `sudo apt-get install liblua5.4-dev` |
| `libnethack.a: No such file` | ビルド失敗 | `cd NetHack && make spotless && make all` |

---

## 📋 Rust FFI 統合フロー（Phase 1b 完了後）

1. **ビルド成功** → libnethack.a が生成される
2. **Rust リンク** → build.rs で `-lnethack` をリンク
3. **bindgen 実行** → ヘッダから FFI 生成
4. **テスト** → Rust から C 関数呼び出し確認

---

## 次のステップ

ユーザーが以下のいずれかを実行：

```bash
# 1. 必要なツールをインストール
sudo apt-get install -y build-essential gcc make bison flex liblua5.4-dev

# 2. NetHack をビルド
cd /home/oosawak/Workspace/NetHack
sh sys/unix/setup.sh sys/unix/hints/linux
make fetch-Lua
make all

# 3. 成功確認
file src/nethack
```

準備完了したら、Copilot に連絡してください！
