use worker::*;

// トップページのUIを提供する関数
pub async fn serve_index_ui(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let html = include_str!("../templates/index.html");

    Ok(Response::from_html(html)?)
}