# collect

イテレータを別のコレクションに変換。

## Vec に変換

```rust
let v: Vec<i32> = (0..5).collect();
// [0, 1, 2, 3, 4]

let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
```

## HashSet に変換

```rust
use std::collections::HashSet;

let set: HashSet<i32> = vec![1, 2, 2, 3].into_iter().collect();
// {1, 2, 3}
```

## HashMap に変換

```rust
use std::collections::HashMap;

let pairs = vec![("a", 1), ("b", 2)];
let map: HashMap<_, _> = pairs.into_iter().collect();
```

## String に変換

```rust
let chars = vec!['h', 'e', 'l', 'l', 'o'];
let s: String = chars.into_iter().collect();
// "hello"
```

## ターボフィッシュ

```rust
// 型推論できない場合
let v = (0..5).collect::<Vec<i32>>();
```
