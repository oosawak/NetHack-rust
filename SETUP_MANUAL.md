# NetHack × Rust FFI 統合 - セットアップ手順書

## 🔐 sudo パスワードが必要な部分

以下のコマンドをターミナルで**手動実行**してください（パスワード入力が発生します）：

---

## Phase 1: 環境セットアップ（パスワード必要 ⚠️）

### 1-1. ビルドツール・ライブラリのインストール

**以下をコピーして実行：**

```bash
sudo apt-get update
sudo apt-get install -y build-essential gcc make bison flex pkg-config
sudo apt-get install -y liblua5.4-dev
```

**確認コマンド：**
```bash
gcc --version
make --version
lua54 --version  # または pkg-config --modversion lua
```

---

## Phase 2: NetHack ビルド（パスワード不要）

### 2-1. NetHack セットアップ

```bash
cd /home/oosawak/Workspace/NetHack
sh sys/unix/setup.sh sys/unix/hints/linux
```

### 2-2. Lua ライブラリをダウンロード

```bash
make fetch-Lua
```

### 2-3. NetHack をビルド

```bash
make all
```

このコマンドは 5-15 分かかることがあります。

### 2-4. ビルド成功確認

```bash
file src/nethack
ls -lh src/nethack
```

**成功時の出力例：**
```
src/nethack: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), ...
-rwxr-xr-x 1 oosawak oosawak 6.5M May  6 06:30 src/nethack
```

---

## Phase 3: Rust 側での統合

NetHack ビルド成功後、Copilot に以下を報告：

```
✅ NetHack ビルド完了
✅ src/nethack ファイルが生成された
```

その後、Copilot が以下を実施：

1. build.rs の更新（libnethack.a へのリンク設定）
2. bindgen での FFI 自動生成
3. テストの実行

---

## 🆘 トラブル時

### エラー 1: `make: flex: No such file or directory`

```bash
# 以下を実行
sudo apt-get install -y flex bison
make spotless
make all
```

### エラー 2: `lua.h: No such file`

```bash
# 以下を確認
pkg-config --cflags lua5.4
sudo apt-get install -y liblua5.4-dev
```

### エラー 3: ビルドが進まない

```bash
# リセットして再実行
cd /home/oosawak/Workspace/NetHack
make spotless
make fetch-Lua
make all
```

---

## 📊 次のステップ

1. **上記 Phase 1-2 を実行**
2. **NetHack ビルド成功確認**
3. **以下をコメントで Copilot に報告：**
   ```
   ✅ Phase 1-2 完了
   ✅ src/nethack ファイル確認済み
   ```
4. **Copilot が Phase 3 (Rust 統合) を自動実施**

---

## 📝 実施状況メモ

このドキュメントをこのままコピーして、実施状況をメモしてください：

```
[ ] 1-1. apt-get update
[ ] 1-1. build-essential, gcc, make, bison, flex, pkg-config インストール
[ ] 1-1. liblua5.4-dev インストール
[ ] 2-1. sys/unix/setup.sh 実行
[ ] 2-2. make fetch-Lua 実行
[ ] 2-3. make all 実行（5-15分待機）
[ ] 2-4. file src/nethack で確認
```

完了したら Copilot に報告してください！
