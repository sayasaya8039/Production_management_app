# Production Manager

シンプルな制作物管理アプリ（Windows用）

## 概要

制作中のプロジェクトをカテゴリ別に管理できるカンバン形式のデスクトップアプリケーションです。

## 機能

- **3カテゴリ管理**: 拡張機能、Webアプリ、Windowsアプリ
- **アイテム管理**: タイトルとコメントを登録
- **ソート機能**: A-Z、Z-A、日付順
- **Markdownエクスポート**: カテゴリごとにエクスポート可能
- **自動保存**: 変更は即座に保存

## スクリーンショット

![Production Manager](docs/screenshot.png)

## インストール

### 実行ファイルから起動

1. `ProductionManager` フォルダをダウンロード
2. `production-manager.exe` を実行

### 必要要件

- Windows 10/11
- フォント: Noto Sans JP（同梱）またはシステムの日本語フォント

## ビルド方法

```bash
# リポジトリをクローン
git clone https://github.com/sayasaya8039/Production_management_app.git
cd Production_management_app

# ビルド
cargo build --release

# 実行ファイルは target/release/production-manager.exe に生成
```

## 技術スタック

| 項目 | 技術 |
|------|------|
| 言語 | Rust |
| GUI | eframe / egui 0.29 |
| データ形式 | JSON |
| フォント | Noto Sans JP |

## データ保存場所

```
%LOCALAPPDATA%/ProductionManager/data.json
```

## ライセンス

MIT License

## バージョン履歴

| バージョン | 内容 |
|------------|------|
| v0.19.0 | レイアウト修正、フォントサイズ拡大 |
| v0.16.0 | ui.columns()によるレイアウト改善 |
| v0.14.0 | CentralPanelレイアウト修正 |
| v0.3.0 | 縦型カラムレイアウト |
| v0.2.0 | 日本語・絵文字対応 |
| v0.1.0 | 初回リリース |

## 作者

[@sayasaya8039](https://github.com/sayasaya8039)
