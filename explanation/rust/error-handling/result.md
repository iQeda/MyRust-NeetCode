# Result

「成功か失敗か」を表す型。

```rust
enum Result<T, E> {
    Ok(T),    // 成功
    Err(E),   // 失敗
}
```

## 使用例

```rust
let n: Result<i32, _> = "42".parse();   // Ok(42)
let n: Result<i32, _> = "abc".parse();  // Err(...)
```

## 値を取り出す

### unwrap / expect

```rust
"42".parse::<i32>().unwrap();           // 42
"42".parse::<i32>().expect("error");    // 42

// エラー時はパニック
```

### unwrap_or

```rust
"abc".parse::<i32>().unwrap_or(0);  // 0
```

### ok()

```rust
// Result → Option に変換
"42".parse::<i32>().ok();   // Some(42)
"abc".parse::<i32>().ok();  // None
```

### match

```rust
match "42".parse::<i32>() {
    Ok(n) => println!("{}", n),
    Err(e) => println!("error: {}", e),
}
```

## ? 演算子

```rust
fn parse_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;  // エラーなら早期リターン
    Ok(n * 2)
}
```

## メソッド

### is_ok / is_err

```rust
result.is_ok();   // true/false
result.is_err();  // true/false
```

### map / map_err

```rust
result.map(|x| x * 2);       // Ok の中身を変換
result.map_err(|e| ...);     // Err の中身を変換
```
