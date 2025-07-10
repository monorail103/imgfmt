// Cloudflare R2に、変換した画像をアップロードする
use worker::*;
use bcrypt::{hash, DEFAULT_COST};
use crate::types::R2config;

// Cloudflare R2に、変換した画像をアップロードする
pub async fn upload_to_r2(ctx: &RouteContext<()>, file_name: &str, bytes: &[u8], config: &R2config) -> Result<()> {

    // 有効期限と削除用パスワードを取得
    let valid_date = &config.valid_date;
    // パスワードをハッシュ化
    let hashed_password = hash(&config.delete_password, DEFAULT_COST)
        .map_err(|e| worker::Error::from(format!("Password hashing failed: {}", e)))?;

    // D1に有効期限と削除用パスワードを保存する
    let d1 = ctx.env.d1("DB")?;
    let statement = d1.prepare("INSERT INTO r2_files (file_name, valid_date, delete_password) VALUES (?, ?, ?)");
    let query = statement.bind(&[file_name.into(), valid_date.into(), hashed_password.into()])?;

    // クエリを実行
    query.run().await?;

    // wrangler.tomlで設定したbinding名でバケットを取得
    let bucket = ctx.env.bucket("IMG_BUCKET")?;
    console_log!("R2バケットを取得しました。アップロードを開始します...");

    // putメソッドでファイルをアップロード
    bucket.put(file_name, bytes.to_vec()).execute().await?;
    console_log!("ファイル '{}' のアップロードが完了しました。", file_name);

    Ok(())
}