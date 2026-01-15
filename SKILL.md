# SKILLS.md - Claude Code スキル定義

## 概要

このファイルはClaude Codeが使用するスキル（ワークフロー）を定義します。
コンテキスト圧縮後も維持される必須スキルです。

---

## 🔴🔴🔴【絶対必須 - Tier 0】claude-delegator スキル

> ⚠️ **claude-delegator は最優先のAI委任システム**
> ⚠️ **このルールはコンテキスト圧縮後も絶対に維持すること**

### スキル概要

| 項目 | 内容 |
|------|------|
| **スキル名** | claude-delegator |
| **目的** | GPTエキスパートへの委任による高品質な開発 |
| **必須度** | 絶対遵守（例外なし） |
| **優先度** | Tier 0（最優先） |

### 🔴 GPTエキスパート委任（最優先・絶対遵守）

| 場面 | GPTエキスパート | MCPツール | 説明 |
|------|----------------|----------|------|
| **システム設計・アーキテクチャ** | **Architect** | mcp__codex__codex | トレードオフ分析、設計決定 |
| **作業計画のレビュー** | **Plan Reviewer** | mcp__codex__codex | 計画の完全性・明確性検証 |
| **要件が曖昧な時** | **Scope Analyst** | mcp__codex__codex | 要件分析、リスク特定 |
| **コードレビュー・バグ発見** | **Code Reviewer** | mcp__codex__codex | 品質・セキュリティ・パフォーマンス |
| **セキュリティ懸念** | **Security Analyst** | mcp__codex__codex | 脆弱性診断、脅威モデリング |

### 🔴 7セクション委任フォーマット（必須）

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
| **Advisory** | read-only | 分析、推奨、レビュー |
| **Implementation** | workspace-write | 変更実行、修正 |

### 委任フロー

1. タスク開始 → Architect に設計相談【必須】
2. 計画作成後 → Plan Reviewer で検証【必須】
3. 実装完了後 → Code Reviewer でレビュー【必須】
4. セキュリティ懸念時 → Security Analyst で診断【必須】
5. 最終判断 → Claude が全意見を統合して決定

### 🔴 重要：英文での問いかけ

> **GPTエキスパートへの問いかけは必ず英文で行うこと**
> （日本語だとエンコードエラーが発生する場合がある）

---

## 🟢【有効】Perplexity スキル

> **Web検索・調査専用**

### スキル概要

| 項目 | 内容 |
|------|------|
| **スキル名** | perplexity-search |
| **目的** | 事実確認・最新情報の調査 |
| **MCPツール** | ask_perplexity |

### 使用場面

| 場面 | 説明 |
|------|------|
| **事実確認** | 最新の技術情報、ライブラリバージョン等 |
| **調査** | APIドキュメント、ベストプラクティス検索 |

---

<!--
## 🟡【無効化】multi-ai-collaboration スキル

以下のツールは現在無効化されています：

| ツール | 説明 |
|--------|------|
| ask_gemini | 設計・アーキテクチャ相談 |
| ask_zai | コードレビュー |
| ask_grok / grok_brainstorm | 創造的アイデア出し |
| ask_openai | 補助的な意見 |
| ask_all_ais | 全AIに同じ質問 |
| ai_consensus | コンセンサス取得 |
-->

---

## スキル発動条件

| スキル | 発動条件 | 優先度 |
|--------|----------|--------|
| **claude-delegator** | 設計・計画・レビュー・セキュリティ | 🔴 Tier 0（最優先） |
| **perplexity-search** | Web検索・調査 | 🟢 有効 |
| **python-performance** | .py ファイル編集時 | 該当時必須 |
| **go-development** | .go ファイル編集時 | 該当時必須 |
| **language-selection** | 新規プロジェクト作成時 | 該当時必須 |

---

## 更新履歴

| 日付 | 内容 |
|------|------|
| 2026年1月16日 | **multi-AI を Perplexity 以外コメントアウト** |
| 2026年1月15日 | **claude-delegator を Tier 0 最優先に変更** |
