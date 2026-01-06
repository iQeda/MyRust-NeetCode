# イテレータメソッド

## 変換系

### map

```rust
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
// [2, 4, 6]
```

### filter

```rust
let v = vec![1, 2, 3, 4, 5];
let even: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();
// [&2, &4]
```

### enumerate

```rust
for (i, val) in v.iter().enumerate() {
    println!("{}: {}", i, val);
}
```

## 集約系

### fold

```rust
let sum = v.iter().fold(0, |acc, x| acc + x);
```

### sum / max / min

```rust
let sum: i32 = v.iter().sum();
let max = v.iter().max();   // Option
let min = v.iter().min();   // Option
```

## 検索系

### find

```rust
let first_even = v.iter().find(|&&x| x % 2 == 0);
// Some(&2)
```

### position

```rust
let idx = v.iter().position(|&x| x == 3);
// Some(2)
```

### any / all

```rust
v.iter().any(|&x| x > 3);   // true
v.iter().all(|&x| x > 0);   // true
```

## 制限系

### take / skip

```rust
v.iter().take(3);    // 最初の3つ
v.iter().skip(2);    // 最初の2つをスキップ
```

## 結合系

### zip

```rust
let a = vec![1, 2, 3];
let b = vec![4, 5, 6];

let pairs: Vec<_> = a.iter().zip(b.iter()).collect();
// [(&1, &4), (&2, &5), (&3, &6)]
```

### chain

```rust
let combined: Vec<_> = a.iter().chain(b.iter()).collect();
// [&1, &2, &3, &4, &5, &6]
```
