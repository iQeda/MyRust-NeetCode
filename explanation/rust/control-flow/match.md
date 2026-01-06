# match（パターンマッチング）

## 基本

```rust
let x = 2;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 | 4 => println!("three or four"),  // 複数
    5..=10 => println!("five to ten"),   // 範囲
    _ => println!("other"),              // デフォルト
}
```

## match は式

```rust
let msg = match x {
    1 => "one",
    2 => "two",
    _ => "other",
};
```

## Option のマッチング

```rust
let opt: Option<i32> = Some(5);

match opt {
    Some(x) => println!("value: {}", x),
    None => println!("no value"),
}
```

## if let（簡略化）

```rust
// Some の場合だけ処理したい
if let Some(x) = opt {
    println!("value: {}", x);
}

// else も書ける
if let Some(x) = opt {
    println!("{}", x);
} else {
    println!("none");
}
```

## while let

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("{}", top);
}
// 出力: 3, 2, 1
```
