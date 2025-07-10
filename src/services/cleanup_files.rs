use worker::*;
use serde_json;

pub async fn cleanup_expired_files(env: &Env) -> Result<()> {
    let d1 = env.d1("DB")?;
    let bucket = env.bucket("IMG_BUCKET")?;

    // 現在時刻(UTC)を取得
    let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();

    // 有効期限切れのファイルを取得
    let statement = d1.prepare("SELECT file_name FROM r2_files WHERE valid_date < ?1");
    let results = statement.bind(&[now.into()])?.all().await?;

    for row in results.results::<serde_json::Value>()? {
        if let Some(file_name_value) = row.get("file_name") {
            if let Some(file_name) = file_name_value.as_str() {
                console_log!("Deleting expired file: {}", file_name);

                // R2からファイルを削除
                bucket.delete(file_name).await?;
                
                // D1からレコードを削除
                let delete_stmt = d1.prepare("DELETE FROM r2_files WHERE file_name = ?1");
                delete_stmt.bind(&[file_name.into()])?.run().await?;
            }
        }
    }
    Ok(())
}