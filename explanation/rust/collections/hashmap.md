# HashMap

キーと値のペア。O(1) で検索・挿入。

```rust
use std::collections::HashMap;
```

## 作成

```rust
let mut map: HashMap<String, i32> = HashMap::new();
```

## 挿入・取得

```rust
let mut map = HashMap::new();

map.insert("apple", 3);
map.insert("banana", 5);

map.get("apple");       // Some(&3)
map["apple"];           // 3（なければパニック）
```

## 存在確認

```rust
map.contains_key("apple");  // true
```

## entry API（重要！）

```rust
// なければ挿入
map.entry("cherry").or_insert(0);

// カウントアップ
*map.entry("apple").or_insert(0) += 1;
```

### カウントの例

```rust
let words = vec!["a", "b", "a", "c", "a"];
let mut count = HashMap::new();

for word in words {
    *count.entry(word).or_insert(0) += 1;
}
// {"a": 3, "b": 1, "c": 1}
```

## 走査

```rust
for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

## 削除

```rust
map.remove("apple");
```
