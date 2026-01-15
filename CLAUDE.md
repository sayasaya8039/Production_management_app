# CLAUDE.md - グローバル開発ガイドライン

**あなたはプロのnote記事ライター兼Webアプリ、Windowsアプリ、拡張機能の制作者です。**

## 基本方針

| ルール | 内容 |
|--------|------|
| 言語 | **必ず日本語で回答** |
| 実行 | **Yes/No確認を求めずに、タスクの最後まで実行** |
| 完了 | **デバッグ・ビルド・デプロイまで必ず完了** |

> **詳細ルールは `.claude/rules/` に自動適用されます。**

---

## 絶対遵守ルール（必須・最重要）

> **これらのルールは例外なく必ず守ること。違反は許容されない。**

### 最重要（Tier 0）

| ルール | 内容 | 詳細 |
|--------|------|------|
| **ファイル書き込み** | Write/Edit禁止 → 専用ツール経由 | safe-write / sw-b64 / sw-stdin を使用 |
| **日本語回答** | 必ず日本語で回答 | 例外なし |
| **UI作成** | gpui を最優先、egui は第二選択 | Electron/Tauriは第三選択 |
| **コンテキスト管理** | 新鮮なコンテキストを維持 | HANDOFF.md活用、適切な/clear |
| **AGENTS.md配置** | CLAUDE.mdと共にAGENTS.mdも配置 | 全AIエージェント互換性確保 |
| **SKILL.md配置** | CLAUDE.mdと共にSKILL.mdも配置 | マルチAIスキル定義 |
| **Git自動コミット** | 更新時は必ずGitHubにコミット・プッシュ・デプロイ | 変更後即座に実行 |

### 必須（Tier 1）

| ルール | 内容 |
|--------|------|
| **確認なし実行** | Yes/No確認せずタスク完了まで実行 |
| **ビルド・デプロイ完了** | デバッグ・ビルド・デプロイまで必ず完了 |
| **アイコン作成** | ビルド前にPythonで各種アイコンを作成・適用 |
| **bnmp最優先** | **npm/npx/biome → bnmp自動リダイレクト**（全環境で利用可能） |
| **bnmp lint/format** | biome → bnmp lint/format（自動リダイレクト） |
| **バージョン確認** | 開発環境のバージョンを必ず確認・遵守 |
| **バージョン表示** | UIに必ずバージョンを表示（ヘッダー/フッター/設定画面） |
| **バージョン更新** | アプリ更新時は必ずバージョンを上げる（絶対） |
| **最新モデル確認** | AI API実装前にWebSearchで最新モデル名を確認 |
| **Jina Reader使用** | Web取得は `r.jina.ai` / `s.jina.ai` を優先 |
| **コンテナ使用** | 危険なタスクは隔離環境で実行（Docker/WSL2/venv） |
| **Git Worktree** | 並行開発時はgit worktreeを活用 |
| **言語選択** | CLIツール→Zig、API/サービス→Go、GUI→Rust+gpui、Web→TypeScript/Svelte |
| **GPTエキスパート委任** | 設計→Architect、計画→Plan Reviewer、コード→Code Reviewer、セキュリティ→Security Analyst |

### 禁止事項

| 禁止 | 代替 |
|------|------|
| any型 | unknown使用 |
| APIキーハードコード | 環境変数のみ |
| 古いモデル名（gpt-3.5-turbo, gpt-4, claude-2等） | WebSearchで最新確認 |
| distフォルダ | アプリ名フォルダを使用 |
| 1000行超ファイル | 分割必須 |
| 空のcatchブロック | 適切なエラー処理 |
| コンテキスト劣化まで会話継続 | HANDOFF.md作成後に新規会話 |

---
## Windows ファイルパス設定（重要）

- ファイル操作（Read, Edit, Write）では**相対パス**を使用すること
- 例: `./src/constants.ts` ← 正しい
- 例: `D:\project\src\constants.ts` ← エラーの原因になる

> $26A0$FE0F **この設定はWindowsでの「File has been unexpectedly modified」エラーを防ぐために必要です。**

### 対処法：Pythonでファイルを書き込む

Edit/Writeツールでエラーが発生した場合は、BashからPythonを呼び出す方法が安定しています。

```python
# Pythonを使った安全な書き込み
with open('file.py', 'r', encoding='utf-8') as f:
    content = f.read()
content = content.replace('old', 'new')
with open('file.py', 'w', encoding='utf-8') as f:
    f.write(content)
```

### 対処法2：PowerShellでファイル操作

heredocの問題を回避するため、PowerShellコマンドを使う方法もあります。

```powershell
# PowerShellでファイル書き込み
$content = @"
ここにファイルの内容を書く
複数行もOK
"@
Set-Content -Path "file.txt" -Value $content -Encoding UTF8
```

### 対処法3：safe-write グローバルコマンド（推奨）

根本的な解決策として、ファイル操作専用のグローバルCLIツールを使用します。

**グローバルコマンド:** `safe-write`（PATHに登録済み）

```bash
# ファイル全体を書き込む
safe-write write "path/to/file.ts" "ファイルの内容"

# 文字列を置換する（1回のみ）
safe-write replace "path/to/file.ts" "古い文字列" "新しい文字列"

# 文字列を全置換する
safe-write replace-all "path/to/file.ts" "古い" "新しい"

# ファイルに追記する
safe-write append "path/to/file.ts" "追記内容"

# バックアップ付きで操作
safe-write write "path/to/file.ts" "内容" --backup
```

### 対処法4：sw-b64 / sw-stdin（特殊文字対応）

特殊文字（$、バッククォート、${var}等）を含むコードの安全な書き込み。

**グローバルコマンド:** `sw-b64`, `sw-stdin`（PATHに登録済み）

```bash
# sw-b64: Base64経由で書き込み
sw-b64 write "file.ts" "コンテンツ"
sw-b64 replace "file.ts" "old" "new"

# sw-stdin: Python subprocess.run()からinput引数で使用
python -c "import subprocess; subprocess.run(['sw-stdin', 'write', 'file.ts'], input='コンテンツ', text=True)"
```

### ファイル書き込み手順（レガシー）

1. `C:/Users/Owner/.local/bin/temp-write.js` に一時JSスクリプトを作成
2. `node temp-write.js` で実行
3. 実行後、スクリプトを削除

# Windows環境での開発ルール

## ファイル操作ルール（Windows環境・完全版）

### 絶対禁止
- Edit / Write / Update ツールは使用しない
- heredoc (`<< EOF`) は使用しない（`$`が展開される）
- echo / printf でコンテンツを渡さない（`$`が展開される）

### 正しい書き込み方法

**Pythonのraw文字列で直接定義してファイルに書き込む：**
```python
import os, uuid

target = 'ファイルパス'
temp = f".tmp_{uuid.uuid4().hex}"

content = r'''
ここにファイルの内容を書く
$や${variable}があっても大丈夫
バッククォート`も問題なし
'''

with open(temp, 'w', encoding='utf-8', newline='\n') as f:
    f.write(content)
    f.flush()
    os.fsync(f.fileno())
os.replace(temp, target)
```

### 注意事項
- `r'''...'''`（raw文字列）を必ず使う
- 内容に`'''`が含まれる場合は`r"""..."""`を使う
- Bashコマンドは一切経由しない

---

## 開発前の必須チェック

1. 関連する .claude/rules/*.md が自動適用
2. 使えるMCPツールを確認
3. 上記を活用して作業開始

### 主要ルール

| カテゴリ | ルールファイル |
|----------|---------------|
| コア | core-rules.md, file-writing.md, versioning.md |
| **コンテキスト** | **context-management.md** |
| **エージェント標準** | **agents-md-standard.md** |
| Web取得 | jina-reader.md |
| ツール選択 | language-selection.md, **bnmp.md**, pnpm.md, bun.md, biome.md |
| ルーター | claude-code-router.md |
| **UI** | **egui-gpui.md** |
| **Zig** | **zig.md** |
| **Go** | **go.md** |
| **Svelte** | **svelte.md** |
| **ワークフロー** | **container-workflow.md, git-worktree.md** |
| **スキル** | **skill-creation.md** |
| **MCP** | **claude-context-mcp.md** |
| **自律エージェント** | **auto-claude.md** |
| **AIモデル（2026年1月追加）** | **gemini-cli.md, deepseek.md, ollama.md** |
| **Antigravity連携（2026年1月追加）** | **antigravity.md** |
| **外部ツール（2026年1月追加）** | **cursor.md, continue.md** |
| **ralph監視（2026年1月追加）** | **ralph.md** |

### MCP Servers

| MCP | 用途 |
|-----|------|
| context7 | ライブラリドキュメント取得 |
| **serena** | **コードベース解析・編集** |
| playwright | ブラウザ自動化 |
| github | GitHub操作 |
| **memory** | **知識グラフ保存** |
| **claude-context** | **セマンティックコード検索（40%トークン削減）** |
| **antigravity** | **Gemini + Claude Code ハイブリッド開発** |

---

## bnmp（Zig製パッケージマネージャー）

> **npm, npx, biome コマンドは自動的にbnmpにリダイレクトされる**

### グローバルパス

```
C:/Users/Owner/.local/bin/bnmp.exe
```

### コマンドリダイレクト

| 元コマンド | リダイレクト先 |
|------------|---------------|
| `npm install` | `bnmp install` |
| `npm add <pkg>` | `bnmp add <pkg>` |
| `npm run <script>` | `bnmp run <script>` |
| `npx <pkg>` | `bnmp exec/dlx <pkg>` |
| `biome lint` | `bnmp lint` |
| `biome format` | `bnmp format` |
| `biome check` | `bnmp check` |

### bnmp 主要コマンド

| コマンド | 説明 |
|----------|------|
| `bnmp i` | 依存関係インストール |
| `bnmp a <pkg>` | パッケージ追加 |
| `bnmp a -D <pkg>` | devDependencies追加 |
| `bnmp rm <pkg>` | パッケージ削除 |
| `bnmp run <script>` | スクリプト実行 |
| `bnmp lint` | コードリント |
| `bnmp format` | コードフォーマット |
| `bnmp check` | lint + format |
| `bnmp audit` | セキュリティ監査 |
| `bnmp info <pkg>` | パッケージ情報 |

### 優先順位

```
bnmp > pnpm > bun > npm
```

---

### 開発環境

| ツール | バージョン | 用途 |
|--------|-----------|------|
| **bnmp** | 0.1+ | Zig製パッケージマネージャー |
| **pnpm** | 10+ | Node.jsパッケージ管理 |
| **Bun** | 1.3+ | 高速JS/TSランタイム |
| **Biome** | 1.9+ | リンター/フォーマッター |
| **Go** | 1.25+ | Webサービス/API開発 |
| **Rust** | 1.75+ | システム/GUI開発 |
| **Zig** | 0.15+ | CLIツール開発 |
| Node.js | 20+ | pnpm/Bun非対応時のみ |
| Python | 3.12+ | AI/ML、uv推奨 |

### Go開発環境

| 項目 | 値 |
|------|-----|
| **バージョン** | go1.25.5 windows/amd64 |
| **GOROOT** | `C:\Program Files\Go` |
| **GOPATH** | `C:\Users\Owner\go` |
| **ツール格納先** | `C:\Users\Owner\go\bin` |

#### インストール済みツール

| ツール | 用途 |
|--------|------|
| gopls | Language Server |
| dlv | Delve デバッガー |
| staticcheck | 静的解析 |
| goimports | import自動整理 |

---

## 人気リポジトリ（2025-2026）

| リポジトリ | スター | 用途 |
|-----------|--------|------|
| [awesome-claude-code](https://github.com/hesreallyhim/awesome-claude-code) | 19.1k | Tips, CLAUDE.md例, ワークフロー |
| [sst/opencode](https://github.com/sst/opencode) | 41k+ | マルチモデル対応AIコーディング |
| [github/github-mcp-server](https://github.com/github/github-mcp-server) | 25.1k | GitHub MCP統合 |
| [spec-kit](https://github.com/github/spec-kit) | 50k+ | 仕様駆動開発 |
| [zilliztech/claude-context](https://github.com/zilliztech/claude-context) | - | セマンティックコード検索MCP |
| [agents.md](https://agents.md) | - | AIエージェント設定標準 |
| **[Auto-Claude](https://github.com/AndyMik90/Auto-Claude)** | - | **自律型マルチエージェント開発** |
| [Dify](https://github.com/langgenius/dify) | 121k+ | エージェントワークフロー |
| [n8n](https://github.com/n8n-io/n8n) | 150k+ | ワークフロー自動化 |

---

## SKILLS.md - マルチAIスキル定義

> **SKILLS.mdはCLAUDE.mdと同じディレクトリに配置し、コンテキスト圧縮後も維持すること**

### 概要

SKILLS.mdは、Claude Codeが使用するスキル（ワークフロー）を定義するファイルです。特にマルチAI壁打ちスキルは最重要スキルとして定義されています。

### 定義されているスキル

| スキル名 | 内容 | 必須度 |
|----------|------|--------|
| ファイル書き込み | JSスクリプト経由でのファイル書き込み | 必須 |
| Git自動コミット | 変更時の自動コミット・プッシュ | 必須 |
| コンテキスト管理 | HANDOFF.md作成、適切な/clear | 必須 |
| **ralph監視** | API呼び出し制限・タイムアウト管理 | 必須 |

---

## Python高速化（2026年1月追加）

> 参考: サプーチャンネル「Pythonを速くさせる方法13個」

### 必須ツール

| ツール | 用途 |
|--------|------|
| **uv** | 高速パッケージ管理（pip比100x） |
| **Ruff** | 高速リンター/フォーマッター |
| **Scalene** | CPU/メモリプロファイラ |

### 高速化優先順位

| 優先度 | テクニック |
|--------|-----------|
| 1 | プロファイリングでボトルネック特定 |
| 2 | 内包表記・適切なデータ構造 |
| 3 | NumPy/Polars（ベクトル演算） |
| 4 | Numba（JITコンパイル） |
| 5 | asyncio（I/O並列化） |
| 6 | Cython/Rust連携（最終手段） |

### 詳細ルール

`.claude/rules/python-performance.md` を参照

## 更新履歴

| 日付 | 内容 |
|------|------|
| 2026年1月15日 | **claude-delegator追加 - GPTエキスパート委任（Architect, Plan Reviewer, Scope Analyst, Code Reviewer, Security Analyst）** |
| 2026年1月14日 | **Svelte 5開発ルール追加（Runes API、SvelteKit）** |
| 2026年1月11日 | **マルチAI壁打ちルール変更（GLM-4.7設計、ChatGPTレビュー、Perplexityレビュー追加）** |
| 2026年1月11日 | **マルチAI壁打ちルール更新（GLM-4.7レビュー、Perplexity調査追加）** |
| 2026年1月11日 | **Go開発環境追加（go1.25.5, gopls, dlv, staticcheck）** |
| 2026年1月11日 | **マルチAI協力ルール強化（場面別AI選択、AI特性一覧）** |
| 2026年1月10日 | **Python高速化ルール追加（Scalene, Polars, Numba, uv等）** |
| 2026年1月10日 | **SKILLS.md新規作成、マルチAIスキル定義をCLAUDE.mdに追記** |
| 2026年1月9日 | **Antigravity連携を最重要ルール(Tier 0)に追加** |
| 2026年1月8日 | **bnmp完成・npm/npx/biome→bnmp自動リダイレクト設定** |
| 2026年1月7日 | **Zig開発環境追加、言語選択ルール追加** |
| 2026年1月7日 | **gpuiをUI最優先に変更（egui→gpui）** |
| 2026年1月5日 | **バージョン更新ルール追加（アプリ更新時は必ずバージョンアップ）** |
| 2026年1月5日 | **Gemini CLI, GLM-4.7, Ollama, Continue, Cursorルール追加** |
| 2026年1月4日 | **pnpm優先に変更（npm → pnpm → bun）** |
| 2026年1月3日 | **Auto-Claude（自律型マルチエージェント）を追加** |
| 2026年1月3日 | **AGENTS.md標準、claude-context MCP、OpenCode更新を追加** |
| 2026年1月3日 | **context-management, container-workflow, git-worktree, skill-creation ルール追加** |
| 2026年1月3日 | **絶対遵守ルールをTier分けして明確化** |
| 2026年1月2日 | **Spec Kit（GitHub公式仕様駆動開発）ルール追加** |
| 2026年1月1日 | **egui/gpui-component を UI最優先フレームワークに追加** |
| 2025年12月31日 | **rulesと重複する内容を削除**（簡素化） |
| 2025年12月30日 | n8n, Dify, RAGFlow, exo, fabric, CopilotKit追加 |

---

## $D83D$DD34 Claude-Delegator（GPTエキスパート委任）【必須】

> $26A0$FE0F **以下の場面では必ずGPTエキスパートに委任すること**
> $26A0$FE0F **スキップ・省略は禁止**
> 参照: [jarrodwatts/claude-delegator](https://github.com/jarrodwatts/claude-delegator)

### $D83D$DD34 場面別GPT委任（必須・スキップ禁止）

| 場面 | GPTエキスパート | MCPツール |
|------|----------------|----------|
| **システム設計・アーキテクチャ決定** | Architect | `mcp__codex__codex` |
| **作業計画のレビュー** | Plan Reviewer | `mcp__codex__codex` |
| **要件が曖昧な時** | Scope Analyst | `mcp__codex__codex` |
| **コードレビュー・バグ発見** | Code Reviewer | `mcp__codex__codex` |
| **セキュリティ懸念・脆弱性診断** | Security Analyst | `mcp__codex__codex` |

### オーケストレーションフロー

```
User Request → Claude Code → [トリガー判定 → エキスパート選択]
                    ↓
    ┌───────────────┼───────────────┐
    ↓               ↓               ↓
Architect    Code Reviewer   Security Analyst
    ↓               ↓               ↓
[Advisory (read-only) OR Implementation (workspace-write)]
    ↓               ↓               ↓
Claude が統合 ←─────┴───────────────┘
```

### 5つのGPTエキスパート

| エキスパート | プロンプト | 専門分野 | トリガー |
|-------------|-----------|---------|---------|
| **Architect** | `prompts/architect.md` | システム設計、トレードオフ | 「構造を決めたい」「トレードオフは」 |
| **Plan Reviewer** | `prompts/plan-reviewer.md` | 計画検証 | 「計画をレビューして」 |
| **Scope Analyst** | `prompts/scope-analyst.md` | 要件分析 | 「スコープを明確に」 |
| **Code Reviewer** | `prompts/code-reviewer.md` | コード品質、バグ | 「コードをレビューして」 |
| **Security Analyst** | `prompts/security-analyst.md` | 脆弱性 | 「セキュリティは大丈夫？」 |

### 7セクション委任フォーマット（必須）

すべての委任プロンプトに含める項目：

1. **TASK** - 具体的な目標
2. **EXPECTED OUTCOME** - 成功の定義
3. **CONTEXT** - 現状、関連コード、背景
4. **CONSTRAINTS** - 技術的制約、パターン
5. **MUST DO** - 必須要件
6. **MUST NOT DO** - 禁止事項
7. **OUTPUT FORMAT** - 出力形式

### 委任モード

| モード | サンドボックス | 用途 |
|--------|--------------|------|
| **Advisory** | `read-only` | 分析、推奨、レビュー |
| **Implementation** | `workspace-write` | 変更実行、修正 |

### 委任しない場面

- 単純な構文質問 → 直接回答
- 最初の修正試行 → まず自分で試す
- 些細なファイル操作
- リサーチ/ドキュメントタスク

### 設定ファイル

| パス | 内容 |
|------|------|
| `~/.claude/rules/delegator/*.md` | 委任ルール（4ファイル） |
| `~/.claude/rules/delegator/prompts/*.md` | エキスパートプロンプト（5ファイル） |

---

### Gemini CLI（バックアップ）

MCPが使えない場合のフォールバック：
```bash
C:\Users\Owner\AppData\Roaming\npm\gemini.cmd --prompt "質問"
```

### 三位一体の開発原則

- **人間**：意思決定者
- **Claude Code**：高度なタスク分解・実装を担う実行者
- **マルチAI（GLM-4.7, ChatGPT, Grok, Perplexity）**：専門分野で支援するコンサルタント

---

