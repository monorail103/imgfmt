use::worker::*;
use crate::types::FileRecord;

pub async fn delete_from_r2(ctx: &RouteContext<()>, file_name: &str, delete_password: &str) -> Result<()> {
    // d1からファイル名で検索
    let d1 = ctx.env.d1("DB")?;
    let stmt = d1.prepare("SELECT valid_date, is_valid FROM r2_files WHERE file_name = ?1");

    // `first(None)` を使って最初の1件
    let result: Option<FileRecord> = stmt.bind(&[file_name.into()])?.first(None).await?;

    // レコードが見つかった場合
    match result {
        Some(file_info) => {
            console_log!("ファイル '{}' の情報を取得しました: {:?}", file_name, file_info);
            
            // 有効期限のチェック
            let current_time = chrono::Utc::now();
            if file_info.valid_date < current_time.to_rfc3339() {
                console_log!("ファイル '{}' は有効期限切れです。", file_name);
                return Err(worker::Error::from("File expired"));
            }

            // 削除されているかどうかのチェック
            if !file_info.is_valid {
                console_log!("ファイル '{}' は既に削除されています。", file_name);
                return Err(worker::Error::from("File already deleted"));
            }

            // パスワードの検証
            if !bcrypt::verify(delete_password, &file_info.delete_password).unwrap_or(false) {
                console_log!("パスワードが間違っています。");
                return Err(worker::Error::from("Invalid password"));
            }

            // ファイルを削除する
            let bucket = ctx.env.bucket("IMG_BUCKET")?;
            bucket.delete(file_name).await?;
            console_log!("ファイル '{}' の削除が完了しました。", file_name);

            // D1のレコードを更新して無効化
            let update_stmt = d1.prepare("UPDATE r2_files SET is_valid = FALSE WHERE file_name = ?1");
            update_stmt.bind(&[file_name.into()])?.run().await?;
            Ok(())
        }
        None => {
            // レコードが見つからなかった場合
            console_log!("ファイル '{}' が見つかりませんでした。", file_name);
            return Err(worker::Error::from("File not found"));
        }
    }
}