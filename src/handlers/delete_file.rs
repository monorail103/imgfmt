use worker::*;
use crate::services;
use crate::utils::is_valid_filename;

// APIルート用の、R2ファイル削除ハンドラー
pub async fn handle_delete_file(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // リクエストURLからファイル名を取得
    let url = req.url()?;
    let file_name = url.path().trim_start_matches('/');

    // ファイル名の検証
    if !is_valid_filename(file_name) {
        return Response::error("Invalid file name", 400);
    }

    // リクエストボディから削除用パスワードを取得
    let body = req.text().await?;
    let delete_password = body.trim();

    // R2からファイルを削除
    match services::delete_from_r2(&ctx, file_name, delete_password).await {
        Ok(_) => {
            // レスポンスを返す
            Response::ok(format!("File '{}' deleted successfully.", file_name))
        },
        Err(e) => {
            console_log!("Failed to delete file '{}': {}", file_name, e);
            Response::error("File not found or deletion failed", 404)
        }
    }
}