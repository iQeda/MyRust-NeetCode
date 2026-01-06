# よくある計算量

## O(1) - 定数時間

入力サイズに関係なく、常に同じ時間。

```rust
fn get_first(arr: &[i32]) -> i32 {
    arr[0]  // 配列が100個でも100万個でも同じ
}
```

---

## O(n) - 線形時間

入力サイズに比例して増加。

```rust
fn sum(arr: &[i32]) -> i32 {
    let mut total = 0;
    for x in arr {      // n 回ループ
        total += x;
    }
    total
}
```

- n = 100 → 100回の処理
- n = 1000 → 1000回の処理

---

## O(n²) - 二乗時間

入力サイズの二乗に比例。2重ループでよく発生。

```rust
fn has_pair_with_sum(arr: &[i32], target: i32) -> bool {
    for i in 0..arr.len() {         // n 回
        for j in (i+1)..arr.len() { // n 回
            if arr[i] + arr[j] == target {
                return true;
            }
        }
    }
    false
}
```

- n = 100 → 約 5,000 回の処理
- n = 1000 → 約 500,000 回の処理

### なぜ n² ではなく n²/2 なのか？

内側のループが `j = (i+1)..n` で始まるため、**三角形の面積** になる:

```
i=0: j は 1,2,3,...,99 → 99回
i=1: j は 2,3,4,...,99 → 98回
i=2: j は 3,4,5,...,99 → 97回
...
i=98: j は 99         → 1回
i=99: j は (なし)      → 0回
```

合計 = 99 + 98 + 97 + ... + 1 + 0 = **n(n-1)/2**

```
n = 100  → 100 × 99 / 2 = 4,950  ≈ 5,000
n = 1000 → 1000 × 999 / 2 = 499,500 ≈ 500,000
```

### 等差数列の和の公式

1 + 2 + 3 + ... + (n-1) = **(n-1) × n / 2**

これは「1 から n-1 までの和」で、ガウスの公式とも呼ばれる。

### それでも O(n²)

n(n-1)/2 = (n² - n) / 2 = **n²/2 - n/2**

Big O では定数と低次の項を無視するので:

O(n²/2 - n/2) = O(n²/2) = **O(n²)**

---

## O(log n) - 対数時間

毎回半分に減る処理。二分探索が代表例。

```rust
fn binary_search(arr: &[i32], target: i32) -> bool {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return true;
        } else if arr[mid] < target {
            left = mid + 1;   // 半分に絞る
        } else {
            right = mid;      // 半分に絞る
        }
    }
    false
}
```

- n = 1000 → 約 10 回の処理 (log₂1000 ≈ 10)
- n = 1,000,000 → 約 20 回の処理

---

## O(n log n) - 線形対数時間

**O(n) × O(log n)** の組み合わせ。ソートアルゴリズムの代表的な計算量。

### なぜソートは O(n log n) か？

マージソートを例に説明:

```
[8, 3, 5, 1, 9, 2, 7, 4]  ← n個の要素
         ↓ 分割
[8, 3, 5, 1] [9, 2, 7, 4]
    ↓            ↓
[8, 3] [5, 1] [9, 2] [7, 4]
  ↓      ↓      ↓      ↓
[8][3] [5][1] [9][2] [7][4]  ← log n 回分割
         ↓ マージ
[3, 8] [1, 5] [2, 9] [4, 7]  ← 各レベルで n 個の要素を処理
    ↓            ↓
[1, 3, 5, 8] [2, 4, 7, 9]
         ↓
[1, 2, 3, 4, 5, 6, 7, 8, 9]
```

- **分割の深さ**: log n 回（毎回半分にするから）
- **各レベルの処理**: n 個の要素をマージ
- **合計**: n × log n

### コードで見る O(n log n)

```rust
// パターン1: ソート
fn sort_example(mut arr: Vec<i32>) -> Vec<i32> {
    arr.sort();  // O(n log n)
    arr
}

// パターン2: 各要素に対して二分探索
fn binary_search_each(arr: &[i32], targets: &[i32]) -> Vec<bool> {
    let mut sorted = arr.to_vec();
    sorted.sort();  // O(n log n)

    targets.iter()
        .map(|t| sorted.binary_search(t).is_ok())  // n 回 × O(log n)
        .collect()
}

// パターン3: 分割統治法
fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);   // 左半分を再帰
    merge_sort(&mut arr[mid..]);   // 右半分を再帰
    merge(arr, mid);               // O(n) でマージ
}
```

### 計算例

```
n = 1,000
  → 1,000 × log₂(1,000)
  → 1,000 × 10
  → 約 10,000 回

n = 1,000,000
  → 1,000,000 × log₂(1,000,000)
  → 1,000,000 × 20
  → 約 20,000,000 回
```

### O(n²) との比較

n = 10,000 の場合:

| 計算量 | 処理回数 |
|--------|----------|
| O(n log n) | 10,000 × 13 = **130,000** |
| O(n²) | 10,000 × 10,000 = **100,000,000** |

O(n log n) は O(n²) より **約770倍速い**

### よくある O(n log n) のパターン

1. **ソート**: `sort()`, マージソート, クイックソート
2. **ヒープ操作 × n回**: n 個の要素をヒープに追加/削除
3. **分割統治法**: 問題を半分に分割して再帰
4. **TreeMap/TreeSet 操作 × n回**: 各操作が O(log n)

---

## 次のステップ

- [03-how-to-calculate.md](./03-how-to-calculate.md) - 計算量の求め方
