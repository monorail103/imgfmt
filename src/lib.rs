use worker::*;

mod services;
mod handlers;
mod types;
mod utils;


#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {

    let router = Router::new();

    // ルートを定義
    router
        .get_async("/", handlers::serve_index_ui)  // ルートページでUIを提供
        .post_async("/convert", handlers::handle_image_conversion)
        .get_async("/fileget/:file_name", handlers::handle_get_file) // R2からファイルを取得
        .post_async("delete/:file_name", handlers::handle_delete_file) // R2からファイルを削除
        .run(req, env).await
}

#[event(scheduled)]
pub async fn scheduled(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_log!(
        "cronjob: '{}' at {}",
        event.cron(),
        js_sys::Date::new_0().to_iso_string()
    );

    if let Err(e) = services::cleanup_expired_files(&env).await {
        console_error!("Error during cleanup: {:?}", e);
    }
}