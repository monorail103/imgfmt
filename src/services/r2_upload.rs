// Cloudflare R2に、変換した画像をアップロードする
use worker::*;


// Cloudflare R2に、変換した画像をアップロードする
pub async fn upload_to_r2(ctx: &RouteContext<()>, file_name: &str, bytes: &[u8]) -> Result<()> {
    // wrangler.tomlで設定したbinding名でバケットを取得
    let bucket = ctx.env.bucket("IMG_BUCKET")?;
    console_log!("R2バケットを取得しました。アップロードを開始します...");

    // putメソッドでファイルをアップロード
    bucket.put(file_name, bytes.to_vec()).execute().await?;
    console_log!("ファイル '{}' のアップロードが完了しました。", file_name);

    Ok(())
}