use worker::*;

use crate::types::{
    ConversionParams,
    ImageOutput
};

use crate::services;
use uuid::Uuid;
use chrono::{Local};

pub async fn handle_image_conversion(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {

    let url = req.url()?;
    let query_params: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();

    // ipアドレスを取得
    let ip_address: String = match req.headers().get("CF-Connecting-IP") {
        Ok(Some(ip)) => ip.to_string(),
        _ => return Response::error("IP address not found", 400),
    };

    // KVストアにIPアドレスを保存、もしくは制限回数に達していればブロッキング
    if let Err(e) = services::log_ip_address(&ctx, &ip_address).await {
        console_log!("Failed to log IP address {}: {}", ip_address, e);
        return Response::error("Internal server error", 500);
    }
    
    let params = ConversionParams {
        format: match query_params.get("format") {
            Some(f) => f.clone(),
            None => return Response::error("Query parameter 'format' is required", 400),
        },
        quality: query_params.get("quality").and_then(|q| q.parse().ok()),
        width: query_params.get("width").and_then(|w| w.parse().ok()),
        height: query_params.get("height").and_then(|h| h.parse().ok()),
        // watermark: None, // UIの改善をしてから
    };
    
    // リクエストボディから画像データを取得
    let image_bytes = req.bytes().await?;
    if image_bytes.is_empty() {
        return Response::error("Request body with image data is required", 400);
    }
    
    // 画像処理を実行
    let output: ImageOutput = match services::process_image(&image_bytes, &params) {
        Ok(output) => output,
        Err(e) => return Response::error(&format!("Failed to process image: {}", e), 500),
    };

    // R2にアップロード
    let file_id = format!("{}_{}", Uuid::new_v4(), Local::now().timestamp());
    let file_name = format!("{}.{}", file_id, params.format.to_lowercase());
    // R2アップロード: 指数的バックオフのための変数
    let mut attempt = 0;
    let max_attempts = 5; // 最大試行回数
    let mut uploaded = false;
    let mut last_err: Option<worker::Error> = None;

    // アップロードの試行
    console_log!("Starting upload to R2: {}", file_name);
    while attempt < max_attempts && !uploaded {
        match services::upload_to_r2(&ctx, &file_name, &output.bytes).await {
            Ok(_) => {
                uploaded = true;
                console_log!("File uploaded to R2: {}", file_name);
            },
            Err(e) => {
                last_err = Some(e);
                attempt += 1;
                let wait_time = 2u64.pow(attempt as u32) * 1000; // ミリ秒単位
                console_log!("Upload failed, retrying in {} ms... (Attempt {}/{})", wait_time, attempt, max_attempts);
                worker::Delay::from(std::time::Duration::from_millis(wait_time)).await;
            }
        }
    }

    // アップロードが失敗した場合はエラーを返す
    if !uploaded {
        return Response::error(
            &format!("Failed to upload file after {} attempts: {}", max_attempts, last_err.unwrap()),
            500
        );
    }

    let headers = Headers::new();
    headers.set("Content-Type", &output.mime_type)?; // MIMEタイプ
    headers.set("Content-Disposition", &format!("inline; filename=\"{}\"", file_name))?; // ファイル名
    headers.set("X-File-Id", &file_id)?; // 独自ID
    headers.set("Location", &format!("/r2/{}", file_name))?; // アップロード先URL


    Ok(Response::from_bytes(output.bytes)?.with_headers(headers))
}

