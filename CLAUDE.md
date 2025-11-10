# gitp - Git複数リポジトリ管理ツール

## 概要

**gitp** は、複数のGitリポジトリを一括で管理するためのツールです。YAML設定ファイルに基づいて、複数のリポジトリに対してclone、pull、pushなどの操作を並列または直列で実行できます。

## プロジェクト状況

このプロジェクトは、元々Bashスクリプトとして実装されていた`gitp.sh`をRustで再実装しようとするものです。

### gitp.sh (完全版)

**場所**: `~/repos/gitp.sh`

**機能**:
- ✅ `clone` - 複数リポジトリの一括クローン
- ✅ `pull` - 複数リポジトリの一括プル
- ✅ `push` - 複数リポジトリの一括プッシュ（add, commit, pushを自動実行）
- ✅ 並列実行（デフォルト）
- ✅ シリアル実行（`serial`オプション）
- ✅ Git設定の自動適用（user.name, user.email）

**使用方法**:
```bash
# 並列でクローン
bash ~/repos/gitp.sh clone

# シリアルでプル
bash ~/repos/gitp.sh pull serial

# 並列でプッシュ
bash ~/repos/gitp.sh push

# ※自動的にuser.name/user.emailが各リポジトリに設定されます
```

### gitp (Rust版)

**実装済み機能**:
- ✅ YAML設定ファイルの読み込み (`src/setting_util.rs`)
- ✅ `git clone` の実行 (`src/git_controller.rs:45`)
- ✅ `git pull` の実行 (`src/git_controller.rs:51`)
- ✅ `git push` の実行 (`src/git_controller.rs:57`)
- ✅ `git status` の実行 (`src/git_controller.rs:39`)
- ✅ **任意のgit config設定** - YAMLで定義、全リポジトリに一括適用
- ✅ `config` コマンド - YAML全体を適用
- ✅ `config user` コマンド - user/emailのみ適用（ショートカット）
- ✅ Windows/Linux対応（文字コード処理）
- ✅ **インタラクティブモード** (`rustyline`) - Tab補完、履歴、ヒント
- ✅ コマンドショートカット（clo, pu, conf など）
- ✅ **Git設定の自動適用** - clone直後、pull/push前に自動実行
- ✅ リポジトリごとのenabledフラグの処理
- ✅ グループディレクトリの作成と移動
- ✅ 複数リポジトリの並列実行（マルチスレッド）
- ✅ **ratatui フルスクリーンTUI** (`src/tui.rs`)
- ✅ **btop風の美しい表示** - 枠線、プログレスバー、アイコン
- ✅ **リアルタイム進捗更新** - 各リポジトリの状態を0-100%で表示
- ✅ **カラー出力** - ステータスに応じた色分け表示

**gitp.shとの比較**:

| 機能 | gitp.sh (Bash) | gitp (Rust) |
|------|----------------|-------------|
| 基本操作 | clone/pull/push | clone/pull/push/config |
| 自動config | ✅ | ✅ **clone直後、各操作前** |
| config user | ❌ | ✅ **専用コマンド** |
| 並列実行 | ✅ | ✅ マルチスレッド |
| 表示 | テキスト出力 | **ratatui フルスクリーンTUI** |
| 進捗表示 | ❌ | ✅ **プログレスバー（0-100%）** |
| ステータス | テキストのみ | ✅ **アイコン + カラー** |
| リアルタイム更新 | ❌ | ✅ |
| 統計情報 | ❌ | ✅ **Total/Success/Failed** |
| 美しさ | ⚠️ シンプル | ✅ **btop風** |

## 設定ファイル

### gitp.sh用: gitp_config.yml

```yaml
user:
  name: "your-name"
  email: "your-email@example.com"
comments:
  default: "update."
repos:
  - enabled: true
    name: "repo-name"
    remote: "git@github.com:user/repo.git"
    branch: "main"
    group: "group-name"
```

### Rust版用: gitp_setting.yaml

```yaml
user:
  name: kako-jun
  email: 3541096+kako-jun@users.noreply.github.com
comments:
  default: update.
config:
  core.editor: vim
  pull.rebase: "true"
  commit.gpgsign: "false"
  init.defaultBranch: main
repos:
  - enabled: true
    remote: git@github.com:kako-jun/gitp.git
    branch: main
    group: "2024"
```

**注**: `.yml` と `.yaml` の両方の拡張子をサポートしています（`.yaml` 優先）。

### git configの一括設定

**自動設定（clone/pull/push時）**:
- `user.name`と`user.email`は、clone直後、pull/push前に自動設定されます

**手動設定コマンド**:
```bash
gitp config         # YAMLのconfig全体 + user を全リポジトリに適用
gitp config user    # user.name/user.emailのみ適用（ショートカット）
```

**YAMLの`config`セクション**:
- 任意のgit configを追加できます
- `gitp config`実行時に全リポジトリに一括適用されます
- 例: `core.editor`, `pull.rebase`, `commit.gpgsign`, `init.defaultBranch`など

### 動作の制限

**pull時のコンフリクト**:
- コンフリクトが発生した場合、自動解決は行いません
- エラーとしてマークされ、ユーザーが手動で解決する必要があります

**pushのコミットメッセージ**:
- YAMLの`comments.default`を使用（固定）
- 個別のコミットメッセージは指定できません
- 例: `"update."`

このツールは**一括操作に特化**しており、細かい制御が必要な場合は個別にgitコマンドを実行してください。

## アーキテクチャ

### プロジェクト構成

```
gitp/                       # リポジトリルート
├── Cargo.toml              # 依存関係設定
├── gitp_setting.yaml       # 設定ファイル（.ymlも可）
├── CLAUDE.md               # 開発ドキュメント
├── README.md
├── LICENSE
└── src/
    ├── main.rs             # エントリーポイント、ワーカースレッド管理
    ├── interactive.rs      # インタラクティブモード (rustyline)
    ├── git_controller.rs   # Git操作の実行
    ├── setting_util.rs     # 設定ファイルの読み込み
    └── tui.rs              # ratatui TUI実装
```

### 主要モジュール

#### `git_controller.rs`
- `GitController::new()` - コントローラーの初期化（OS別文字コード設定）
- `git_status()` - git statusの実行 (gitp/src/git_controller.rs:39)
- `git_clone(repo_name, branch)` - git cloneの実行 (gitp/src/git_controller.rs:45)
- `git_pull()` - git pullの実行 (gitp/src/git_controller.rs:51)
- `git_push(commit_message)` - git push（add, commit, push） (gitp/src/git_controller.rs:57)
- `git_config(name, email)` - user.name, user.emailの設定 (gitp/src/git_controller.rs:65)
- `exec_command(cmd, args)` - 任意のコマンドの実行（stdout/stderrキャプチャ）

#### `setting_util.rs`
- `GitpSetting` - 設定データ構造（User, Repos, Config）
- `load()` - gitp_setting.yaml（または.yml）の読み込み（src/setting_util.rs:42）

#### `main.rs`
- コマンドライン引数のパース
- `spawn_clone_workers()` - cloneワーカースレッドの起動（clone直後にgit config自動実行）
- `spawn_pull_workers()` - pullワーカースレッドの起動（pull前にgit config自動実行）
- `spawn_push_workers()` - pushワーカースレッドの起動（push前にgit config自動実行）
- `spawn_config_user_workers()` - config userワーカースレッドの起動（全リポジトリに一括設定）
- `extract_repo_name()` - リモートURLからリモジトリ名を抽出
- TUIアプリケーションのセットアップと実行

#### `tui.rs`
- `TuiApp` - ratatuiを使ったTUIアプリケーション
- `RepoProgress` - リポジトリの進捗状態（名前、ステータス、メッセージ、進捗率）
- `RepoStatus` - ステータス列挙型（Pending, Running, Success, Failed）
- `update_repo_status()` - ワーカースレッドから状態を更新するヘルパー関数
- `render_repos()` - リポジトリリストとプログレスバーのレンダリング
- フルスクリーンTUI、枠線付きパネル、リアルタイム更新

## TUI機能

Rust版では、**ratatui** と **crossterm** を使ったbtop風の美しいフルスクリーンTUIを実装しています。

### 表示イメージ

```
┌─────────────────────────────────────────────────────────────┐
│ gitp - Git Multiple Repository Manager                     │
└─────────────────────────────────────────────────────────────┘
┌─ Repositories ──────────────────────────────────────────────┐
│                                                             │
│ ⚙ gitp                                    Cloning...       │
│ [████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 40%   │
│                                                             │
│ ✓ my-project                              Done             │
│ [██████████████████████████████████████████████████] 100%  │
│                                                             │
│ ⏸ another-repo                            Waiting...       │
│ [░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 0%    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│ Total: 3 | Completed: 1 | Success: 1 | Failed: 0 | Press 'q'│
└─────────────────────────────────────────────────────────────┘
```

### TUIの特徴

**リアルタイム更新**:
- 各リポジトリの状態がリアルタイムで更新される
- プログレスバーで進捗を視覚化（0-100%）
- ステータスアイコンで一目で状態がわかる
  - ⏸ グレー: 待機中 (Pending)
  - ⚙ 黄色: 実行中 (Running)
  - ✓ 緑: 成功 (Success)
  - ✗ 赤: 失敗 (Failed)

**美しいレイアウト**:
- 枠線で区切られたパネル表示
- ヘッダー: アプリケーション名
- メインパネル: リポジトリ一覧とプログレスバー
- フッター: 統計情報（合計、完了数、成功数、失敗数）

**カラースキーム**:
- リポジトリ名: シアン (強調)
- ステータスメッセージ: 白
- プログレスバー: 状態に応じた色（緑/黄/赤/グレー）
- 背景: 黒

**操作**:
- 自動終了: すべてのリポジトリの処理完了後、1秒表示してから終了
- 手動終了: `q`キーでいつでも強制終了可能

### 並列/シリアル実行

- **並列実行** (デフォルト): すべてのリポジトリを同時に処理、TUIで進捗を表示
- **シリアル実行** (`serial`オプション): 順次処理、TUIで進捗を表示

## 依存関係

```toml
[dependencies]
encoding_rs = "0.8.34"     # 文字コード変換（Windows対応）
serde = { version = "1.0.200", features = ["derive"] }  # シリアライゼーション
serde_yaml = "0.9.34"      # YAML解析
ratatui = "0.28"           # TUIフレームワーク (btop風の美しい表示)
crossterm = "0.28"         # ターミナル制御
rustyline = "14.0"         # インタラクティブモード (Tab補完、履歴)
dirs = "5.0"               # ホームディレクトリ取得（履歴ファイル用）
```

## ビルドと実行

```bash
# ビルド
cargo build --release

# インストール
cargo install --path .

# 使用方法

## インタラクティブモード（推奨）
gitp                     # 対話モード開始
gitp> clone              # クローン実行
gitp> pull serial        # シリアルでプル
gitp> config             # 全git config設定を適用
gitp> exit               # 終了

# Tab補完、コマンド履歴、ヒント表示が使えます

## ワンショットモード
gitp clone               # 並列クローン（自動でuser/email設定）
gitp pull serial         # シリアルでプル（自動でuser/email設定）
gitp push                # 並列プッシュ（自動でuser/email設定）
gitp config              # YAMLのconfig全体を全リポジトリに適用
gitp config user         # user.name/user.emailのみ適用

# ショートカット
gitp clo                 # clone
gitp pu                  # pull
gitp conf u              # config user
```

## ライセンス

MITライセンス（LICENSE ファイル参照）

## 参考リンク

- 元のBashスクリプト: `~/repos/gitp.sh`
- 設定例: `gitp_setting.yaml`
