# 型

## 整数型

```rust
let a: i32 = -42;    // 32ビット符号付き（デフォルト）
let b: i64 = 100;    // 64ビット符号付き
let c: u32 = 42;     // 32ビット符号なし（0以上）
let d: usize = 10;   // 配列インデックス用
```

| 型 | 範囲 | 用途 |
|----|------|------|
| `i32` | -2³¹ ~ 2³¹-1 | 一般的な整数 |
| `i64` | -2⁶³ ~ 2⁶³-1 | 大きな整数 |
| `u32` | 0 ~ 2³²-1 | 正の整数のみ |
| `usize` | 環境依存 | 配列インデックス |

## 浮動小数点・真偽値

```rust
let x: f64 = 3.14;   // 64ビット（デフォルト）
let t: bool = true;
let f: bool = false;
```

## 文字・文字列

```rust
let c: char = 'あ';              // 単一文字
let s: &str = "hello";           // 文字列スライス（固定）
let s: String = String::from("hello");  // 可変文字列
```

### `&` とは？（参照）

`&str` の `&` は **参照（reference）** を表す。データそのものではなく、データへの「ポインタ」。

```rust
let s: String = String::from("hello");  // String はデータを所有
let r: &String = &s;                    // &String は s への参照
let slice: &str = &s;                   // &str は文字列データへの参照
```

| 型 | 意味 | 所有権 |
|----|------|--------|
| `String` | 文字列データそのもの | 所有する |
| `&String` | String への参照 | 借りている |
| `&str` | 文字列スライスへの参照 | 借りている |

**なぜ `&str` が使われるか：**
- `"hello"` のようなリテラルはバイナリに埋め込まれる
- そのデータへの参照として `&str` を使う
- コピーせずにデータを参照できるので効率的

```rust
fn greet(name: &str) {       // 参照を受け取る（コピーしない）
    println!("Hello, {}", name);
}

let s = String::from("World");
greet(&s);      // String を &str として渡せる
greet("World"); // リテラルはそのまま &str
```

## 型推論

```rust
let x = 5;           // i32 と推論
let v = vec![1, 2];  // Vec<i32> と推論
```

## 型変換

```rust
let x: i32 = 10;
let y: i64 = x as i64;          // キャスト

let n: i32 = "42".parse().unwrap();  // 文字列 → 数値
let s: String = 42.to_string();      // 数値 → 文字列
```
