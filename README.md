# IMG_format_converter
Cloudflare Workers上で動作することを前提とした、画像フォーマット・サイズを変換するツール。

## 概要
- サイズ指定で画像リサイズ。
- 画像フォーマット変換。jpeg, png, webp, bmp, gifなどの画像フォーマットに対応。
- 【JPG限定の機能】画像の品質を指定して変換。

## 技術スタック
- Rust
- Cloudflare Workers

## アクセス
- [https://imgfmt.rustexp.workers.dev/](Cloudflare Workers上でホスト)しています。