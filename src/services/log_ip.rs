use worker::*;

/**
 * IPアドレスをログに記録し、画像の一連処理について制限回数を超えないように管理します。
 * @param ctx: RouteContext
 * @param ip_address: ログに記録するIPアドレス
 * @return: 成功時はOk、制限回数に達した場合はErrを返します
 */
pub async fn log_ip_address(ctx: &RouteContext<()>, ip_address: &str) -> Result<()> {
    let kv = ctx.kv("IP_LOGS")?;
    let key = format!("ip:{}", ip_address);

    let maybe_value = kv.get(&key).text().await?;

    if let Some(existing_value) = maybe_value {
        // 既に存在する場合はカウントを増やす
        let count: u32 = existing_value.parse().unwrap_or(0);
        if count >= 10 { // 制限回数
            return Err(worker::Error::from("IP address limit reached"));
        }
        // 書き込みを実行する
        kv.put(&key, (count + 1).to_string())?
          .execute()
          .await?;
    } else {
        // 新規の場合はカウントを1に設定
        kv.put(&key, "1")?
          .execute()
          .await?;
    }

    Ok(())
}