# HashSet

重複のないコレクション。O(1) で検索。

```rust
use std::collections::HashSet;
```

## 作成

```rust
let mut set: HashSet<i32> = HashSet::new();
```

## 挿入・確認

```rust
let mut set = HashSet::new();

set.insert(1);
set.insert(2);
set.insert(1);      // 重複は無視

set.len();          // 2
set.contains(&1);   // true
```

## insert の戻り値

```rust
set.insert(1);      // true（新規追加）
set.insert(1);      // false（既に存在）
```

### 重複検出

```rust
let mut set = HashSet::new();

for num in nums {
    if !set.insert(num) {
        println!("duplicate: {}", num);
    }
}
```

## 集合演算

```rust
let a: HashSet<_> = [1, 2, 3].into_iter().collect();
let b: HashSet<_> = [2, 3, 4].into_iter().collect();

a.union(&b);        // {1, 2, 3, 4}
a.intersection(&b); // {2, 3}
a.difference(&b);   // {1}
```

## 削除

```rust
set.remove(&1);
```
