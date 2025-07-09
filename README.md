# IMG_format_converter
Cloudflare Workers上で動作することを前提とした、画像フォーマット・サイズを変換するツール。

## 技術スタック
- Rust
- Cargo
- Cloudflare Workers
- Cloudflare R2(オブジェクトストレージ)

## 概要
このアプリケーションは、画像編集機能と保存機能の２つに分かれます。以下に概要を記します。
### 画像編集
- サイズ指定で画像リサイズ。
- APIだけでは使いづらいため、簡易的なUI（HTML）を提供。
- 画像フォーマット変換。jpeg, png, webp, bmp, gifなどの画像フォーマットに対応。
- 【JPG限定の機能】画像の品質を指定して変換。

### 保存機能
- Cloudflare R2に保存
- IPアドレスによる、一連の処理についての回数制限
- 指数関数的バックオフ処理によるアップロード処理

画像編集を行うと、そのままR2へ保存されます。

## アクセス
[https://imgfmt.rustexp.workers.dev/](https://imgfmt.rustexp.workers.dev/)

