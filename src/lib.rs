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
        .run(req, env).await
}