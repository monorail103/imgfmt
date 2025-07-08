use serde::Deserialize;

// 画像変換リクエストのパラメータをまとめるstruct
#[derive(Debug, Deserialize)]
pub struct ConversionParams {
    pub format: String, // 変換後の画像フォーマット（例: "png", "jpeg"）
    pub quality: Option<u8>,    // 画像のクオリティ（オプション）
    pub width: Option<u32>,     // 変換後の画像の幅（オプション）
    pub height: Option<u32>,    // 変換後の画像の高さ（オプション）
    pub watermark: Option<WaterMark>, // ウォーターマークの情報（オプション）
}

// 画像処理の結果をまとめるstruct
#[derive(Debug)]
pub struct ImageOutput {
    pub bytes: Vec<u8>,
    pub mime_type: String,
}

#[derive(Debug, Deserialize)]
pub struct WaterMark {
    pub text: String,
    pub position: (u32, u32), // (x, y) 座標
    pub font_size: u32,
    pub color: String, // 色を表す文字列（例: "rgba(255, 255, 255, 0.5)"）
}