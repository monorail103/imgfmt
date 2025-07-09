use worker::*;

pub async fn get_from_r2(ctx: &RouteContext<()>, file_name: &str) -> Result<Vec<u8>> {
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