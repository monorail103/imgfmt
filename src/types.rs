use serde::Deserialize;

// 画像変換リクエストのパラメータをまとめるstruct
#[derive(Debug, Deserialize)]
pub struct ConversionParams {
    pub format: String, // 変換後の画像フォーマット（例: "png", "jpeg"）
    pub quality: Option<u8>,    // 画像のクオリティ（オプション）
    pub width: Option<u32>,     // 変換後の画像の幅（オプション）
    pub height: Option<u32>,    // 変換後の画像の高さ（オプション）
    
}

// R2内での保存に関する設定をまとめるstruct
#[derive(Debug, Deserialize)]
pub struct R2config {
    pub valid_date: String, // 有効期限（例: "2023-12-31T23:59:59Z"）
    pub delete_password: String, // 削除用パスワード
}

#[derive(Debug, Deserialize)]
pub struct FileRecord {
    pub valid_date: String, // 有効期限
    pub delete_password: String, // 削除用パスワード
    pub is_valid: bool, // ファイルが有効かどうか
}

// 画像処理の結果をまとめるstruct
#[derive(Debug)]
pub struct ImageOutput {
    pub bytes: Vec<u8>,
    pub mime_type: String,
}

// #[derive(Debug, Deserialize)]
// pub struct WaterMark {
//     pub text: String,
//     pub position: (u32, u32), // (x, y) 座標
//     pub font_size: u32,
//     pub color: String, // 色を表す文字列（例: "rgba(255, 255, 255, 0.5)"）
// }