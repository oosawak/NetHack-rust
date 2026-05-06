# WASM Examples Guide

このドキュメントでは、WASM ビルド完成後に `wasm-examples/` フォルダで公開予定のサンプルホルダー構成について説明します。

---

## 📁 ディレクトリ構成

```
wasm-examples/
├── README.md                          このプロジェクトについて
├── package.json                       npm パッケージ設定
├── index.html                         メインページ
│
├── css/
│   ├── style.css                      UI スタイル
│   └── themes.css                     テーマ切り替え
│
├── js/
│   ├── main.js                        WASM 初期化・メインループ
│   ├── renderer.js                    WebGL/Canvas レンダリング
│   ├── input.js                       キーボード入力ハンドリング
│   ├── ui.js                          UI 要素管理
│   └── utils.js                       ユーティリティ関数
│
├── lib/
│   ├── nethack_wasm.js                bindgen 生成 JS バインディング
│   ├── nethack_wasm_bg.wasm           実行可能 WASM バイナリ
│   └── nethack_wasm.d.ts              TypeScript 型定義
│
├── assets/
│   ├── tileset.png                    キャラクター・タイル画像 (256x256)
│   ├── fonts/
│   │   └── consolas.ttf               フォントファイル
│   └── sounds/                        サウンドエフェクト（将来）
│
└── examples/
    ├── basic.html                     最小限のサンプル
    ├── advanced.html                  拡張機能サンプル
    └── README.md                      各サンプル説明
```

---

## 🚀 WASM ビルド手順

### 1. WASM ターゲット追加

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### 2. WASM ビルド

```bash
# wasm-examples フォルダに出力
cd /path/to/NetHack
wasm-pack build \
    crates/nethack-wasm \
    --target web \
    --release \
    --out-dir wasm-examples/lib
```

### 3. 生成ファイル

```
lib/
├── nethack_wasm.js          (Rust 関数をエクスポート)
├── nethack_wasm_bg.wasm    (実際のバイナリ)
├── nethack_wasm_bg.wasm.d.ts
├── package.json
└── README.md
```

---

## 💻 HTML/JS 実装例

### index.html

```html
<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NetHack WASM</title>
    <link rel="stylesheet" href="css/style.css">
</head>
<body>
    <div id="app">
        <header>
            <h1>NetHack</h1>
            <div id="status">Loading...</div>
        </header>
        
        <main>
            <canvas id="game-canvas" width="800" height="600"></canvas>
            <div id="ui-panel">
                <div id="stats">
                    <div>HP: <span id="hp">0</span></div>
                    <div>Level: <span id="level">1</span></div>
                    <div>Floor: <span id="floor">1</span></div>
                </div>
                <div id="inventory">
                    <h3>Inventory</h3>
                    <ul id="items"></ul>
                </div>
            </div>
        </main>
        
        <footer>
            <p>WASM-based NetHack | Press '?' for help</p>
        </footer>
    </div>

    <script type="module">
        import init, { WasmGame } from './lib/nethack_wasm.js';
        
        async function main() {
            // WASM モジュール初期化
            await init();
            
            // ゲーム初期化
            const game = new WasmGame();
            await game.init();
            
            // ゲームループ開始
            gameLoop(game);
        }
        
        main().catch(err => {
            console.error('Failed to initialize:', err);
            document.getElementById('status').textContent = 'Error: ' + err.message;
        });
    </script>
    <script src="js/input.js"></script>
    <script src="js/renderer.js"></script>
    <script src="js/main.js"></script>
</body>
</html>
```

### js/main.js

```javascript
let game;
const canvas = document.getElementById('game-canvas');
const ctx = canvas.getContext('2d');

async function gameLoop(wasmGame) {
    game = wasmGame;
    
    function frame() {
        try {
            // ゲーム状態更新
            game.update();
            
            // 画面クリア
            ctx.fillStyle = '#000';
            ctx.fillRect(0, 0, canvas.width, canvas.height);
            
            // 描画
            renderGame(game, ctx);
            
            // UI 更新
            updateUI(game);
            
            requestAnimationFrame(frame);
        } catch (err) {
            console.error('Game loop error:', err);
        }
    }
    
    frame();
}

function renderGame(game, ctx) {
    // WASM から描画データを取得
    const mapData = game.get_map_data();
    
    // キャラクターを描画
    const tileWidth = 32;
    const tileHeight = 32;
    
    mapData.forEach((tile, index) => {
        const x = (index % 25) * tileWidth;
        const y = Math.floor(index / 25) * tileHeight;
        
        // タイルセットから該当部分を描画
        const srcX = (tile % 16) * 16;
        const srcY = Math.floor(tile / 16) * 16;
        
        ctx.drawImage(
            tileset,
            srcX, srcY, 16, 16,
            x, y, tileWidth, tileHeight
        );
    });
}

function updateUI(game) {
    const status = game.get_status();
    
    document.getElementById('hp').textContent = status.hp;
    document.getElementById('level').textContent = status.level;
    document.getElementById('floor').textContent = status.dungeon_level;
    
    // インベントリ更新
    const items = game.get_inventory();
    const itemsList = document.getElementById('items');
    itemsList.innerHTML = items.map(item => 
        `<li>${item.name} (${item.count})</li>`
    ).join('');
}
```

### js/input.js

```javascript
const keyMap = {
    'ArrowUp': 'k',
    'ArrowDown': 'j',
    'ArrowLeft': 'h',
    'ArrowRight': 'l',
    ' ': ' ',
    'Enter': '\n',
    '?': '?',
};

document.addEventListener('keydown', (e) => {
    if (!game) return;
    
    const command = keyMap[e.key] || e.key;
    
    if (command.length === 1) {
        game.send_command(command.charCodeAt(0));
        e.preventDefault();
    }
});
```

---

## 🎨 CSS スタイル例

### css/style.css

```css
:root {
    --bg-color: #000;
    --fg-color: #0f0;
    --accent-color: #0ff;
    --font-family: 'Courier New', monospace;
}

body {
    background-color: var(--bg-color);
    color: var(--fg-color);
    font-family: var(--font-family);
    margin: 0;
    padding: 20px;
}

#game-canvas {
    border: 2px solid var(--accent-color);
    display: block;
    margin: 20px 0;
    background-color: #111;
}

#ui-panel {
    background-color: #111;
    border: 1px solid var(--fg-color);
    padding: 10px;
    width: 300px;
    float: right;
}

#stats {
    margin-bottom: 20px;
}

#inventory ul {
    list-style: none;
    padding: 0;
}

#inventory li {
    padding: 5px;
    border-bottom: 1px dotted var(--fg-color);
}
```

---

## 📦 package.json 設定

```json
{
  "name": "nethack-wasm-examples",
  "version": "1.0.0",
  "description": "NetHack WASM Examples",
  "type": "module",
  "scripts": {
    "serve": "python3 -m http.server 8000",
    "build": "wasm-pack build ../crates/nethack-wasm --target web --release --out-dir ./lib",
    "dev": "python3 -m http.server 3000"
  },
  "keywords": ["nethack", "wasm", "game"],
  "license": "NGPL"
}
```

### ローカルサーバー実行

```bash
cd wasm-examples

# HTTP サーバー起動
npm run serve
# または
python3 -m http.server 8000

# ブラウザで開く
# http://localhost:8000
```

---

## 🌐 デプロイ（GitHub Pages）

### GitHub Pages で自動公開

```bash
# wasm-examples フォルダを gh-pages ブランチにデプロイ
git checkout gh-pages
git merge master
git push origin gh-pages
```

### URL

```
https://oosawak.github.io/NetHack/wasm-examples/
```

---

## 🧪 サンプル実装（examples/）

### basic.html — 最小限の例

```html
<!DOCTYPE html>
<html>
<head>
    <title>NetHack WASM - Basic</title>
</head>
<body>
    <canvas id="canvas" width="800" height="600"></canvas>
    <script type="module">
        import init, { WasmGame } from '../lib/nethack_wasm.js';
        
        async function run() {
            await init();
            const game = new WasmGame();
            await game.init();
            
            // フレーム更新
            setInterval(() => {
                game.update();
                // 描画処理...
            }, 100);
        }
        
        run().catch(console.error);
    </script>
</body>
</html>
```

### advanced.html — 拡張機能

```html
<!-- より複雑な UI・複数ビューモード切り替えなど -->
```

---

## 📊 ファイルサイズ目安

| ファイル | サイズ | 備考 |
|---------|--------|------|
| nethack_wasm_bg.wasm | 2-5 MB | 圧縮: 500KB-1MB |
| nethack_wasm.js | 10-50 KB | bindgen 生成 |
| tileset.png | 200-500 KB | 256x256 タイルセット |
| **合計** | **3-6 MB** | **圧縮: 1-2 MB** |

### 最適化

```bash
# WASM サイズ削減
wasm-pack build --release --target web

# gzip 圧縮（サーバー側で自動）
# ブラウザのキャッシュで再ダウンロード回避
```

---

## ✅ チェックリスト（公開前）

- [ ] WASM ビルド成功（エラーなし）
- [ ] ローカルテスト（localhost で動作確認）
- [ ] モバイルブラウザテスト（iOS Safari, Android Chrome）
- [ ] リソースファイル確認（タイルセット、フォント）
- [ ] Performance 測定（フレームレート、メモリ使用量）
- [ ] ブラウザコンソール確認（エラーなし）
- [ ] GitHub Pages デプロイ
- [ ] README.md 作成（使い方説明）

---

## 🚀 将来計画

- [ ] セーブデータの IndexedDB 永続化
- [ ] モバイル用 UI（タッチ操作）
- [ ] マルチプレイヤー対応（WebSocket）
- [ ] ハイスコアサーバー連携
- [ ] 音声・BGM 統合（Web Audio API）

---

**最終更新**: 2026-05-06  
**ステータス**: 実装予定（Phase 4）
