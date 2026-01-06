# ループ

## for ループ

### 範囲

```rust
for i in 0..5 { }      // 0, 1, 2, 3, 4
for i in 0..=5 { }     // 0, 1, 2, 3, 4, 5（含む）
for i in (0..5).rev() { }  // 4, 3, 2, 1, 0（逆順）
```

### 配列の走査

```rust
let nums = vec![10, 20, 30];

for num in nums.iter() { }           // 参照
for (i, num) in nums.iter().enumerate() { }  // インデックス付き
for num in nums.iter_mut() { *num *= 2; }    // 変更
```

## while ループ

```rust
let mut count = 0;
while count < 5 {
    count += 1;
}
```

## loop（無限ループ）

```rust
loop {
    if condition {
        break;
    }
}
```

### loop は値を返せる

```rust
let result = loop {
    if done {
        break 42;  // 値を返す
    }
};
```

## break と continue

```rust
for i in 0..10 {
    if i == 3 { continue; }  // 次へスキップ
    if i == 7 { break; }     // ループ終了
}
```
