use serde::Deserialize;

// 画像変換リクエストのパラメータをまとめるstruct
#[derive(Debug, Deserialize)]
pub struct ConversionParams {
    pub format: String, // 変換後の画像フォーマット（例: "png", "jpeg"）
    pub quality: Option<u8>,    // 画像のクオリティ（オプション）
    pub width: Option<u32>,     // 変換後の画像の幅（オプション）
    pub height: Option<u32>,    // 変換後の画像の高さ（オプション）
}

// 画像処理の結果をまとめるstruct
#[derive(Debug)]
pub struct ImageOutput {
    pub bytes: Vec<u8>,
    pub mime_type: String,
}