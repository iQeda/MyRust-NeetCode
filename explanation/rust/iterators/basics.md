# イテレータの基本

## イテレータとは

コレクションの要素を順番に処理する仕組み。

```rust
let v = vec![1, 2, 3];

for x in v.iter() {
    println!("{}", x);
}
```

## 3種類のイテレータ

```rust
let v = vec![1, 2, 3];

v.iter()        // &T（参照）
v.iter_mut()    // &mut T（可変参照）
v.into_iter()   // T（所有権を奪う）
```

### 使い分け

```rust
// 読むだけ
for x in v.iter() { }
// v は使える

// 変更する
for x in v.iter_mut() {
    *x *= 2;
}

// 消費する
for x in v.into_iter() { }
// v は使えない
```
