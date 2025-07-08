use worker::*;

use crate::types::{
    ConversionParams,
    ImageOutput
};

use crate::services;

pub async fn handle_image_conversion(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // ...existing code...
    let url = req.url()?;
    let query_params: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
    
    let params = ConversionParams {
        format: match query_params.get("format") {
            Some(f) => f.clone(),
            None => return Response::error("Query parameter 'format' is required", 400),
        },
        quality: query_params.get("quality").and_then(|q| q.parse().ok()),
        width: query_params.get("width").and_then(|w| w.parse().ok()),
        height: query_params.get("height").and_then(|h| h.parse().ok()),
        watermark: query_params.get("watermark").map(|w| {
            // ウォーターマークのパラメータを解析
            let parts: Vec<&str> = w.split(',').collect();
            if parts.len() != 4 {
                return None; // 無効なウォーターマークパラメータ
            }
            Some(crate::types::WaterMark {
                text: parts[0].to_string(),
                position: (parts[1].parse().unwrap_or(0), parts[2].parse().unwrap_or(0)),
                font_size: parts[3].parse().unwrap_or(12),
                color: "rgba(255, 255, 255, 0.5)".to_string(), // デフォルトの色
            })
        }).flatten(),
    };
    
    let image_bytes = req.bytes().await?;
    if image_bytes.is_empty() {
        return Response::error("Request body with image data is required", 400);
    }
    
    let output: ImageOutput = match services::process_image(&image_bytes, &params) {
        Ok(output) => output,
        Err(e) => return Response::error(&format!("Failed to process image: {}", e), 500),
    };

    let headers = Headers::new();
    headers.set("Content-Type", &output.mime_type)?;

    Ok(Response::from_bytes(output.bytes)?.with_headers(headers))
}