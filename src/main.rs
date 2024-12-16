use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures_util::StreamExt; // 仅保留 StreamExt
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tera::{Tera, Context};


#[tokio::main]
async fn main() {
    // 构建路由
    let app = Router::new()
        .route("/", get(index))
        .route("/ws", get(ws_handler));

    // 绑定地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    // 启动服务器
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 处理函数
async fn index() -> Html<String> {
    // 创建Tera实例
    let tera = Tera::new("templates/*.html").unwrap();

    // 创建上下文并插入变量
    let mut context = Context::new();
    context.insert("name", "Axum");

    // 渲染模板
    let rendered = tera.render("index.html", &context).unwrap();
    Html(rendered)
}

// 表单数据结构
#[derive(Deserialize, Serialize)]
struct FormData {
    name: String,
}

// 处理WebSocket连接
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(Message::Text(text))) = socket.next().await {
        if let Ok(data) = serde_json::from_str::<FormData>(&text) {
            let response = serde_json::to_string(&data).unwrap();
            if socket.send(Message::Text(response)).await.is_err() {
                break;
            }
        }
    }
}