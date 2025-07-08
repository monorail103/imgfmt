// typesモジュールで定義したstructをインポート
use crate::types::{ConversionParams, ImageOutput};
use image::{ImageFormat};
use std::io::Cursor;

/**
 * 画像処理を行う関数
 * この関数は、画像のフォーマット変換やサイズ変更などを行います。
 * @param bytes: 画像データのバイト配列
 * @param params: 変換パラメータ（フォーマット、クオリティ）
 * @return: 変換後の画像データとMIMEタイプを含むImageOutput構造体
 * @throws: 変換に失敗した場合はエラーを返します
 */
pub fn process_image(bytes: &[u8], params: &ConversionParams) -> Result<ImageOutput, Box<dyn std::error::Error>> {
    println!("画像処理を実行中... パラメータ: {:?}", params);

    // バイト配列から画像を読み込み
    let mut img = image::load_from_memory(bytes).map_err(|e| {
        format!("画像の読み込みに失敗しました: {}", e)
    })?;
    
    // w, hの指定がある場合はリサイズ
    if let (Some(width), Some(height)) = (params.width, params.height) {
        println!("画像を{}x{}にリサイズ中...", width, height);
        img = img.resize(width, height, image::imageops::FilterType::Lanczos3);
    }
    
    // フォーマットを文字列から ImageFormat に変換
    let format = match params.format.to_lowercase().as_str() {
        "jpeg" | "jpg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "webp" => ImageFormat::WebP,
        "bmp" => ImageFormat::Bmp,
        "gif" => ImageFormat::Gif,
        _ => return Err("サポートされていないフォーマットです".into()),
    };

    // 出力用のバッファ
    let mut output_buffer = Vec::new();
    let mut cursor = Cursor::new(&mut output_buffer);

    // クオリティ指定別に処理を分岐
    if let Some(quality) = params.quality {
        if quality < 1 || quality > 100 {
            return Err("クオリティは1から100の範囲で指定してください".into());
        }
        println!("クオリティ: {}で変換中...", quality);
        
        match format {
            ImageFormat::Jpeg => {
                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, quality);
                img.write_with_encoder(encoder)?;
            },
            ImageFormat::WebP => {
                let mut cursor = Cursor::new(&mut output_buffer);
                img.write_to(&mut cursor, format).map_err(|e| {
                    format!("WebPフォーマットへの変換に失敗しました: {}", e)
                })?;
            }
            _ => {
                // その他のフォーマットは品質指定をサポートしていない場合がある
                img.write_to(&mut cursor, format)?;
            }
        }
    } else {
        println!("クオリティが指定されていません。デフォルトの処理を行います。");
        img.write_to(&mut cursor, format).map_err(|e| {
            format!("画像の変換に失敗しました: {}", e)
        })?;
    }

    let mime_type = match format {
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Png => "image/png",
        ImageFormat::WebP => "image/webp",
        ImageFormat::Bmp => "image/bmp",
        ImageFormat::Gif => "image/gif",
        _ => "application/octet-stream",
    };

    Ok(ImageOutput {
        bytes: output_buffer,
        mime_type: mime_type.to_string(),
    })
}
