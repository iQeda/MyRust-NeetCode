# Vec（ベクタ）

可変長の配列。

## 作成

```rust
let v: Vec<i32> = Vec::new();   // 空
let v = vec![1, 2, 3];          // マクロ
let v = vec![0; 5];             // [0, 0, 0, 0, 0]
```

## 追加・削除

```rust
let mut v = vec![1, 2, 3];

v.push(4);          // 末尾に追加
v.pop();            // 末尾を削除
v.insert(1, 10);    // index 1 に挿入
v.remove(1);        // index 1 を削除
```

## アクセス

```rust
let v = vec![10, 20, 30];

v[0];               // 10（範囲外はパニック）
v.get(0);           // Some(&10)
v.first();          // Some(&10)
v.last();           // Some(&30)
v.len();            // 3
v.is_empty();       // false
```

## スライス

```rust
let v = vec![1, 2, 3, 4, 5];

&v[1..4];           // [2, 3, 4]
&v[..3];            // [1, 2, 3]
&v[2..];            // [3, 4, 5]
```

## ソート

```rust
let mut v = vec![3, 1, 4, 1, 5];

v.sort();                       // 昇順
v.sort_by(|a, b| b.cmp(a));     // 降順
v.reverse();                    // 反転
```

## 検索

```rust
let v = vec![1, 2, 3, 4, 5];

v.contains(&3);                 // true
v.iter().position(|&x| x == 3); // Some(2)
v.binary_search(&3);            // Ok(2)（ソート済み前提）
```
