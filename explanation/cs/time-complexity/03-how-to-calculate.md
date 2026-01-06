# 計算量の求め方

## ルール1: 定数は無視

```rust
for i in 0..n {
    println!("{}", i);     // 1回
    println!("{}", i * 2); // 1回
    println!("{}", i * 3); // 1回
}
```

3n 回だが、O(3n) = **O(n)**（定数は無視）

---

## ルール2: 低次の項は無視

```rust
for i in 0..n {
    for j in 0..n {
        // n² 回
    }
}

for i in 0..n {
    // n 回
}
```

n² + n 回だが、O(n² + n) = **O(n²)**（支配的な項のみ）

---

## ルール3: ループのネストを見る

```rust
// O(n)
for i in 0..n { }

// O(n²)
for i in 0..n {
    for j in 0..n { }
}

// O(n³)
for i in 0..n {
    for j in 0..n {
        for k in 0..n { }
    }
}
```

---

## 次のステップ

- [04-patterns-and-tips.md](./04-patterns-and-tips.md) - よくあるパターンと実践的なヒント
