# クロージャ（無名関数）

## 基本

```rust
let add = |a, b| a + b;
let result = add(2, 3);  // 5
```

## 型を明示

```rust
let add = |a: i32, b: i32| -> i32 { a + b };
```

## 複数行

```rust
let process = |x| {
    let y = x * 2;
    y + 1
};
```

## イテレータで使う

```rust
let v = vec![1, 2, 3];

// map
v.iter().map(|x| x * 2);

// filter
v.iter().filter(|x| *x > 1);

// fold
v.iter().fold(0, |acc, x| acc + x);
```

## 環境のキャプチャ

```rust
let factor = 2;
let multiply = |x| x * factor;  // factor を参照

multiply(5);  // 10
```
