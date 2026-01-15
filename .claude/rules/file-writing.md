---
paths: "**/*"
alwaysApply: true
---

# ファイル書き込みルール（Windows環境・最終版）

## 絶対禁止

| 禁止事項 | 理由 |
|----------|------|
| Read / Write / Edit / Update ツール | パスエスケープ問題でエラー頻発 |
| fw write に直接コンテンツを渡す | ${}が展開される |
| Bashの echo / printf / heredoc | 特殊文字展開問題 |

---

## 必須手順：Base64経由のみ

### Step 1: Pythonでコンテンツを定義

```python
import base64

content = r"""
// ${}があっても問題なし
const url = `${BASE_URL}/search?name=${encodeURIComponent(city)}`;
const price = "$100";
"""

# Base64エンコード
b64 = base64.b64encode(content.encode('utf-8')).decode('ascii')
print(b64)
```

### Step 2: fw b64 で書き込む

```bash
fw b64 "path/to/file.ts" "上でprintされたBase64文字列"
```

---

## ワンライナー版（推奨・最速）

```python
import os, uuid

content = r"""ここにファイル内容"""
path = 'path/to/file.ts'

# 直接Pythonで書き込む（Bashを経由しない）
# 注意: 一時ファイルは同じディレクトリに作成
dir_path = os.path.dirname(path)
temp = os.path.join(dir_path, f".tmp_{uuid.uuid4().hex}")
with open(temp, 'w', encoding='utf-8', newline='
') as f:
    f.write(content)
os.replace(temp, path)
print(f"Done: {path}")
```

---

## fw.js コマンド

| コマンド | 説明 |
|----------|------|
| fw b64 path base64 | Base64からファイル書き込み |
| fw replace path old new | 文字列置換 |
| fw read path | Base64で読み込み出力 |

---

## 重要なルール

1. **${を含むコードは絶対にBashに直接渡さない**
2. **必ずPythonのraw文字列 r"""...""" 内で定義する**
3. **Bashコマンドの引数にコンテンツを渡さない**
4. **ファイル書き込みは必ずPython直接 or fw b64**
5. **一時ファイルは同じドライブに作成する**

---

## トラブルシューティング

### UnicodeEncodeError
- 原因: Windows端末のcp932エンコーディング
- 解決: 絵文字を使わない、ASCII文字のみ使用

### File has been unexpectedly modified
- 原因: Write/Editツールのパス問題
- 解決: Python直接書き込みを使用

### SyntaxError (unterminated string literal)
- 原因: 特殊文字のエスケープ失敗
- 解決: Base64経由で書き込み

### WinError 17 (異なるドライブへの移動)
- 原因: os.replace()はドライブ間移動不可
- 解決: 一時ファイルを同じディレクトリに作成

---

## 優先順位

1. **Python直接書き込み**（最速・最安全）
2. **fw b64**（Base64経由）
3. **fw replace**（部分置換のみ）
