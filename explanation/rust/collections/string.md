# String

## 作成

```rust
let s = String::new();              // 空
let s = String::from("hello");      // &str から
let s = "hello".to_string();        // 同上
```

## 追加

```rust
let mut s = String::from("hello");

s.push(' ');            // 1文字追加
s.push_str("world");    // 文字列追加
// s = "hello world"
```

## 長さ

```rust
let s = "hello";

s.len();                // 5（バイト数）
s.chars().count();      // 5（文字数）
```

## 文字の走査

```rust
for c in s.chars() {
    println!("{}", c);
}
```

## 文字列 ↔ 文字配列

```rust
// String → Vec<char>
let chars: Vec<char> = s.chars().collect();

// Vec<char> → String
let s: String = chars.iter().collect();
```

## 分割

```rust
let s = "a,b,c";
let parts: Vec<&str> = s.split(',').collect();
// ["a", "b", "c"]
```

## 結合

```rust
let parts = vec!["a", "b", "c"];
let s = parts.join(",");
// "a,b,c"
```

## 検索

```rust
let s = "hello world";

s.contains("world");    // true
s.starts_with("hello"); // true
s.ends_with("world");   // true
s.find("world");        // Some(6)
```

## 置換

```rust
let s = "hello world";
let new_s = s.replace("world", "rust");
// "hello rust"
```
