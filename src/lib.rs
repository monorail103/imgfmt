use worker::*;

mod services;
mod handlers;
mod types;


#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {

    let router = Router::new();

    // ルートを定義
    router
        .get_async("/", handlers::serve_index_ui)  // ルートページでUIを提供
        .post_async("/convert", handlers::handle_image_conversion)
        .get_async("/fileget/:file_name", handlers::handle_get_file) // R2からファイルを取得
        .run(req, env).await
}