use worker::*;
use crate::types::FileRecord;

pub async fn get_from_r2(ctx: &RouteContext<()>, file_name: &str) -> Result<Vec<u8>> {
    // d1からファイル名で検索
    let d1 = ctx.env.d1("DB")?;
    let stmt = d1.prepare("SELECT valid_date, is_valid FROM r2_files WHERE file_name = ?1");

    // `first(None)` を使って最初の1件
    let result: Option<FileRecord> = stmt.bind(&[file_name.into()])?.first(None).await?;

    match result {
        Some(file_info) => {
            // レコードが見つかった場合
            console_log!("ファイル '{}' の情報を取得しました: {:?}", file_name, file_info);
            
            // 有効期限のチェック
            let current_time = chrono::Utc::now();
            if file_info.valid_date < current_time.to_rfc3339() {
                console_log!("ファイル '{}' は有効期限切れです。", file_name);
                return Err(worker::Error::from("File expired"));
            }

            // 削除されているかどうかのチェック
            if !file_info.is_valid {
                console_log!("ファイル '{}' は削除されています。", file_name);
                return Err(worker::Error::from("File deleted"));
            }
        }
        None => {
            // レコードが見つからなかった場合
            console_log!("ファイル '{}' が見つかりませんでした。", file_name);
            return Err(worker::Error::from("File not found"));
        }
    }

    // wrangler.tomlで設定したbinding名でバケットを取得
    let bucket = ctx.env.bucket("IMG_BUCKET")?;
    console_log!("R2バケットを取得しました。ファイル '{}' のダウンロードを開始します...", file_name);

    // getメソッドでファイルをダウンロード
    let object = bucket.get(file_name).execute().await?;
    
    if let Some(object) = object {
        console_log!("ファイル '{}' のダウンロードに成功しました。", file_name);
        // オブジェクトのデータをVec<u8>として取得
        let bytes = object.body().unwrap().bytes().await?;
        Ok(bytes)
    } else {
        console_log!("ファイル '{}' が見つかりませんでした。", file_name);
        Err(worker::Error::from("File not found"))
    }
}

