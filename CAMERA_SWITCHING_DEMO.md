# NetHack 3D Camera Switching Demo

## コンセプト：同じ3D世界、複数の視点

```rust
// 1つの3D世界を作成
let mut world = World3D::new(80, 25);
world.add_entity(Entity::new(30.0, 0.0, 20.0, 'd'));  // Monster
world.add_entity(Entity::new(40.0, 0.0, 15.0, '$'));  // Gold

// プレイヤーが移動
world.move_player(1.0, 0.0, 0.0);

// カメラ視点を切り替えるだけで、見え方が変わる
```

## 視点の種類と見え方

### 1. TopDown View（ASCII的）

```
カメラ位置: (40, 50, 12) - 真上
視点角度: 直下を見ている
用途: クラシック NetHack の ASCII画面と同じ

描画結果：
████████████████
█@░░░░░░░░░░░█
█░d░░░░░░░░░░█
█░░░░░░░░░░░░█
█░░░░$░░░░░░░█
████████████████

コード：
let camera = world.get_camera(ViewMode::TopDown);
// camera.position = (40.0, 50.0, 12.0)
// camera.target = (40.0, 0.0, 12.0)
```

### 2. Isometric View（2.5D）

```
カメラ位置: (40 + 17.7, 20, 12 + 17.7) - 45度斜め上
視点角度: 45度傾斜（等角投影）
用途: Diablo, Baldur's Gate 風

描画結果（3D投影）：
   ◇◇◇◇
  ◇d◇◇◇
 ◇◇@◇◇
◇◇◇◇$◇  ← 2.5D らしい奥行き感
◇◇◇◇◇◇

コード：
let camera = world.get_camera(ViewMode::Isometric);
// camera.position = (40 + 17.7, 20.0, 12 + 17.7)
// camera.target = (40.0, 0.0, 12.0)
// fov = 50.0
```

### 3. FirstPerson View（1人称）

```
カメラ位置: (40, 1.7, 12) - プレイヤーの目の高さ
視点角度: 前方を見ている
用途: ダンジョン探索・クラシック風

描画結果（1人称視点）：
  敵が見える！
  
 ▯▯▯▯▯▯▯▯▯
 ▯   d    ▯ ← モンスターが見える
 ▯▯█████▯▯ ← 壁
 ▯▯▯▯▯▯▯▯▯

コード：
let camera = world.get_camera(ViewMode::FirstPerson);
// camera.position = (40.0, 1.7, 12.0)  // Eye height
// camera.target = (40.0, 1.7, 12 + 10)  // Look forward
// fov = 90.0
```

### 4. ThirdPerson View（背後視点）

```
カメラ位置: (40, 2, 12 - 5) - プレイヤー背後・上
視点角度: 背後から見ている
用途: アクション風、プレイヤーキャラを見たい時

描画結果：
        敵が見える
     ▯◇▯
    ◇@◇◇  ← プレイヤーの背を見ている
   ◇◇d◇◇
  ◇◇$◇◇◇

コード：
let camera = world.get_camera(ViewMode::ThirdPerson);
// camera.position = (40.0, 2.0, 12 - 5)
// camera.target = (40.0, 1.5, 12)
// fov = 75.0
```

### 5. Cinematic View（シネマティック）

```
カメラ位置: (40 + 15, 15, 12 + 15) - ドラマティック角度
視点角度: 斜め上から見下ろし
用途: カットシーン、重要なシーン

描画結果（ドラマティック表現）：
            
  ◇    敵 ◇
◇  @      ◇  ← スクリーン等角
◇   d◇$  ◇  ← シネマティック
◇◇◇◇◇◇◇

コード：
let camera = world.get_camera(ViewMode::Cinematic);
// camera.position = (40 + 15, 15, 12 + 15)
// camera.target = (40.0, 1.0, 12.0)
// fov = 45.0  // 狭い視野 = ドラマティック
```

---

## カメラ切り替えのコード例

```rust
// シンプルな実装例

pub struct GameUI {
    world: World3D,
    camera: Camera3D,
    current_view: ViewMode,
}

impl GameUI {
    pub fn new() -> Self {
        let world = World3D::new(80, 25);
        let camera = world.get_camera(ViewMode::TopDown);
        
        Self {
            world,
            camera,
            current_view: ViewMode::TopDown,
        }
    }
    
    /// ユーザーがキーを押した
    pub fn handle_input(&mut self, key: Key) {
        match key {
            Key::ArrowUp => self.world.move_player(0.0, 0.0, -1.0),
            Key::ArrowDown => self.world.move_player(0.0, 0.0, 1.0),
            Key::ArrowLeft => self.world.move_player(-1.0, 0.0, 0.0),
            Key::ArrowRight => self.world.move_player(1.0, 0.0, 0.0),
            
            // 視点切り替え
            Key::Number1 => self.switch_view(ViewMode::TopDown),
            Key::Number2 => self.switch_view(ViewMode::FirstPerson),
            Key::Number3 => self.switch_view(ViewMode::Isometric),
            Key::Number4 => self.switch_view(ViewMode::ThirdPerson),
            Key::Number5 => self.switch_view(ViewMode::Cinematic),
            
            _ => {}
        }
    }
    
    fn switch_view(&mut self, mode: ViewMode) {
        self.current_view = mode;
        self.camera = self.world.get_camera(mode);
        println!("Camera switched to: {:?}", mode);
    }
    
    /// レンダリング用に View-Projection 行列を取得
    pub fn get_view_projection(&self, aspect: f32) -> Mat4 {
        self.camera.view_projection(aspect)
    }
}
```

---

## WASM/Unityでの視点切り替え

### WASM（ブラウザ）

```javascript
// JavaScript側
const gameUI = new GameUI();

// キーボードイベント
document.addEventListener('keydown', (e) => {
    if (e.key === '1') gameUI.switch_view('TopDown');
    if (e.key === '2') gameUI.switch_view('FirstPerson');
    if (e.key === '3') gameUI.switch_view('Isometric');
});
```

### Unity Plugin

```csharp
// C#側（Unity）
using AOT;

public class NetHackGame : MonoBehaviour
{
    [DllImport("nethack_unity")]
    static extern int nethack_get_camera_mode();
    
    [DllImport("nethack_unity")]
    static extern void nethack_set_camera_mode(int mode);
    
    void Update()
    {
        if (Input.GetKeyDown(KeyCode.Alpha1))
            nethack_set_camera_mode(0); // TopDown
        if (Input.GetKeyDown(KeyCode.Alpha2))
            nethack_set_camera_mode(1); // FirstPerson
        if (Input.GetKeyDown(KeyCode.Alpha3))
            nethack_set_camera_mode(2); // Isometric
    }
}
```

---

## まとめ：3D-First × 複数視点

| 視点 | 見え方 | 用途 | FOV |
|---|---|---|---|
| TopDown | ASCII的俯瞰 | クラシック互換性 | 60° |
| Isometric | 2.5D等角 | モダンRPG | 50° |
| FirstPerson | 1人称 | ダンジョン探索 | 90° |
| ThirdPerson | 背後視点 | アクション | 75° |
| Cinematic | ドラマティック | カットシーン | 45° |

**すべて同じ3D世界、同じゲームロジック。**
**カメラ位置を変えるだけで視点が変わる。**

これが、あなたが提案した「3D基盤で、カメラ位置を切り替えて見え方を変える」アプローチの実装です！

