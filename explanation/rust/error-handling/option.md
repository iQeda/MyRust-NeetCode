# Option

「値があるかないか」を表す型。

```rust
enum Option<T> {
    Some(T),   // 値がある
    None,      // 値がない
}
```

## 使用例

```rust
let v = vec![1, 2, 3];

v.get(0);           // Some(&1)
v.get(10);          // None
v.iter().max();     // Some(&3)
```

## 値を取り出す

### unwrap（危険）

```rust
let opt = Some(5);
opt.unwrap();       // 5

let opt: Option<i32> = None;
// opt.unwrap();    // パニック！
```

### unwrap_or

```rust
let opt: Option<i32> = None;
opt.unwrap_or(0);   // 0
```

### unwrap_or_default

```rust
let opt: Option<i32> = None;
opt.unwrap_or_default();  // 0
```

### if let

```rust
if let Some(x) = opt {
    println!("{}", x);
}
```

### match

```rust
match opt {
    Some(x) => println!("{}", x),
    None => println!("none"),
}
```

## メソッド

### is_some / is_none

```rust
opt.is_some();  // true/false
opt.is_none();  // true/false
```

### map

```rust
let opt = Some(5);
opt.map(|x| x * 2);  // Some(10)
```

### and_then

```rust
opt.and_then(|x| Some(x * 2));
```
