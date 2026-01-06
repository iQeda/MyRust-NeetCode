# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RustでNeetCode/LeetCode問題を解くプロジェクト。leetcode-cliを使用して問題取得・提出を行う。

## Environment

- Node.js: v25.2.1 (asdf)
- leetcode-cli: v2.6.2
- Default language: Rust

## Build Commands

```bash
cargo build              # Compile the project
cargo test               # Run all tests
cargo test <test_name>   # Run a specific test
cargo clippy             # Run linter
cargo fmt                # Format code
```

## Skills (Slash Commands)

```
/lc-list [easy|medium|hard]    # 問題一覧
/lc-show <id>                  # 問題詳細とRustテンプレート生成
/lc-test <file>                # テスト実行
/lc-submit <file>              # 提出
```

## leetcode-cli Commands (Direct)

```bash
leetcode list                    # 問題一覧
leetcode list -q eL              # Easy問題のみ (e=Easy, m=Medium, h=Hard, L=Locked, l=Unlocked)
leetcode show <id>               # 問題詳細
leetcode show <id> -g -l rust    # Rustテンプレート生成
leetcode test <file>             # テスト実行
leetcode submit <file>           # 提出
leetcode user                    # ログイン状態確認
```

## leetcode-cli Cookie認証

通常のログイン (`leetcode user -l`) は動作しない。Cookie認証を使用する。

1. ブラウザで https://leetcode.com にログイン
2. DevTools → Application → Cookies → `https://leetcode.com`
3. `LEETCODE_SESSION` と `csrftoken` を取得
4. `~/.lc/leetcode/user.json` を作成:

```json
{
  "login": "USERNAME",
  "loginCSRF": "",
  "sessionCSRF": "CSRFTOKEN_VALUE",
  "sessionId": "LEETCODE_SESSION_VALUE",
  "isSignedIn": true,
  "name": "USERNAME"
}
```

セッション切れの場合は同手順でCookieを再取得して更新する。

## Project Structure

Solutions should be organized by problem category following NeetCode's roadmap structure:
- `src/arrays_hashing/` - Arrays & Hashing problems
- `src/two_pointers/` - Two Pointers problems
- `src/sliding_window/` - Sliding Window problems
- `src/stack/` - Stack problems
- `src/binary_search/` - Binary Search problems
- `src/linked_list/` - Linked List problems
- `src/trees/` - Trees problems
- `src/heap/` - Heap / Priority Queue problems
- `src/backtracking/` - Backtracking problems
- `src/graphs/` - Graphs problems
- `src/dynamic_programming/` - Dynamic Programming problems

Each solution file should include:
1. Problem description as a doc comment
2. Solution implementation
3. Unit tests with example cases from the problem
