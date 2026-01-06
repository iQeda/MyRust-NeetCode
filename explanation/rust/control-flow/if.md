# if 式

## 基本

```rust
let x = 5;

if x > 0 {
    println!("positive");
} else if x < 0 {
    println!("negative");
} else {
    println!("zero");
}
```

## if は式（値を返せる）

```rust
let x = 5;
let sign = if x > 0 { "positive" } else { "negative" };
```

## 条件式

```rust
if x > 0 && x < 10 { }    // AND
if x < 0 || x > 10 { }    // OR
if !flag { }              // NOT
```
