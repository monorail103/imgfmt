use worker::*;

use crate::services;

// APIルート用の、R2ファイル取得ハンドラー
pub async fn handle_get_file(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // リクエストURLからファイル名を取得
    let url = req.url()?;
    let file_name = url.path().trim_start_matches('/');

    // R2からファイルを取得
    match services::get_from_r2(&ctx, file_name).await {
        Ok(file_bytes) => {
            // レスポンスを返す
            Response::from_bytes(file_bytes)
                .map(|response| {
                    let headers = Headers::new();
                    headers.set("Content-Type", "application/octet-stream").ok();
                    response.with_headers(headers)
                })
        },
        Err(e) => {
            console_log!("Failed to get file '{}': {}", file_name, e);
            Response::error("File not found", 404)
        }
    }
}