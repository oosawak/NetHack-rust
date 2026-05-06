# NetHack WASM Examples

ブラウザで WASM 版 NetHack をテストするためのサンプルです。

## ⚡ クイックスタート

### 1. WASM をビルド

まず、プロジェクトルートで WASM をビルドします：

```bash
cd /home/oosawak/Workspace/NetHack
wasm-pack build crates/nethack-wasm --target web --release
```

### 2. ローカルサーバーを起動

examples フォルダのスクリプトを実行します：

```bash
./run_server.sh
```

または、手動でサーバーを起動：

```bash
# Python 3 を使う場合
cd /home/oosawak/Workspace/NetHack
python3 -m http.server 8000

# Node.js の http-server を使う場合
cd /home/oosawak/Workspace/NetHack
npx http-server -p 8000
```

### 3. ブラウザで開く

ブラウザで以下にアクセス：

```
http://localhost:8000/examples/wasm.html
```

## 🎮 コントロール

| キー | アクション |
|------|-----------|
| **↑↓←→** | プレイヤーを移動 |
| **1-5** | カメラモードを切り替え |
| **Q** | ゲームを終了 |

## 📊 画面構成

- **左側**: ゲームキャンバス
  - プレイヤー（黄色）
  - ダンジョン床（グレー）
  - グリッド表示

- **右側**: サイドパネル
  - ゲーム状態（プレイヤー位置、FPS）
  - カメラモードボタン
  - コントロール説明
  - ステータス表示

## 🔧 カメラモード

- **TopDown**: 上から俯瞰
- **Isometric**: アイソメトリック視点
- **FirstPerson**: 一人称視点
- **ThirdPerson**: 三人称視点
- **Cinematic**: シネマティック視点

## 📝 注意事項

- WASM 版は Rust のゲームロジックのみを実装しています
- C ライブラリの機能（モンスター、アイテム等）は現在含まれていません
- ゲーム状態はリロード時にリセットされます
- Canvas レンダリングは2Dコンテキストを使用しています

## 🐛 トラブルシューティング

### "読み込み中..." で止まる

1. ブラウザのコンソール（F12）でエラーを確認
2. CORS エラーの場合、HTTP サーバーから実行していることを確認
3. WASM ファイルが生成されているか確認：
   ```bash
   ls -la crates/nethack-wasm/pkg/
   ```

### "Error: Failed to initialize"

1. WASM がビルドされているか確認
2. サーバーのポート番号を確認（デフォルト: 8000）
3. ファイアウォール設定を確認

### カメラが動かない

JavaScript コンソールでエラーを確認してください。

## 📚 参考情報

- [WASM ビルド方法](../README.md#build-for-webassembly-wasm)
- [プロジェクト README](../README.md)
- [Architecture](../ARCHITECTURE.md)
