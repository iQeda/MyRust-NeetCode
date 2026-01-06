# MyRust-NeetCode

Rust で NeetCode/LeetCode 問題を解くプロジェクト

- https://neetcode.io/roadmap
- https://leetcode.com/u/iQeda/

## セットアップ

### Node.js (asdf)

```bash
asdf install nodejs 25.2.1
asdf set nodejs 25.2.1
```

### leetcode-cli

```bash
npm install -g leetcode-cli
leetcode plugin -i cookie.chrome
leetcode config lang rust
```

### LeetCode 認証 (Cookie 方式)

leetcode-cli の通常ログインは動作しないため、Cookie 認証を使用する。

1. ブラウザで https://leetcode.com にログイン
2. 開発者ツール (F12) → Application → Cookies → `https://leetcode.com`
3. `LEETCODE_SESSION` と `csrftoken` の値をコピー
4. `~/.lc/leetcode/user.json` を作成:

```json
{
  "login": "YOUR_USERNAME",
  "loginCSRF": "",
  "sessionCSRF": "YOUR_CSRFTOKEN",
  "sessionId": "YOUR_LEETCODE_SESSION",
  "isSignedIn": true,
  "name": "YOUR_USERNAME"
}
```

5. 確認: `leetcode user`

## 使い方

### Claude Code Skills

```
/lc-list [easy|medium|hard]    # 問題一覧
/lc-show <id>                  # 問題詳細とRustテンプレート生成
/lc-test <file>                # テスト実行
/lc-submit <file>              # 提出
```

### leetcode-cli (直接実行)

```bash
leetcode list                    # 問題一覧
leetcode list -q eL              # Easy問題のみ
leetcode show <id> -g -l rust    # Rustテンプレート生成
leetcode test <file>             # テスト実行
leetcode submit <file>           # 提出
```

## ビルド

```bash
cargo build
cargo test
cargo clippy
cargo fmt
```
