use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, response::Html, routing::get, routing::post};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

use crate::bus::bus::EventBus;
use crate::bus::event::{Event, MotorCommand, MotorDirection};

pub async fn run(bus: EventBus) {
    let bus = Arc::new(bus);
    let static_files = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", static_files)
        .nest_service("/camera/frame.jpg", ServeFile::new("/tmp/frame.jpg"))
        .route("/time", get(time))
        .route("/partials/camera", get(partial_camera))
        .route("/partials/sensors", get(partial_sensors))
        .route("/api/motor/forward", post(motor_forward_handler))
        .route("/api/motor/stop", post(motor_stop_handler))
        .layer(CorsLayer::permissive())
        .with_state(bus.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Robot UI running at http://0.0.0.0:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(bus))
        .await
        .unwrap();
}

async fn shutdown_signal(bus: Arc<EventBus>) {
    let mut rx = bus.subscribe();

    loop {
        if let Ok(Event::Shutdown) = rx.recv().await {
            println!("Web node shutting down");
            break;
        }
    }
}

async fn index() -> Html<String> {
    let html =
        std::fs::read_to_string("templates/index.html").expect("missing templates/index.html");

    Html(html)
}

async fn time() -> impl IntoResponse {
    chrono::Utc::now().format("%H:%M:%S").to_string()
}

async fn partial_sensors() -> Html<String> {
    let ldr_left = 0;
    let ldr_middle = 0;
    let ldr_right = 0;
    let ultrasound = 0;
    let neopixel_r = 0;
    let neopixel_g = 0;
    let neopixel_b = 0;

    let html = format!(
        r#"
        <ul>
            <li><strong>LDR Left:</strong> {}</li>
            <li><strong>LDR Middle:</strong> {}</li>
            <li><strong>LDR Right:</strong> {}</li>
            <li><strong>Ultrasound:</strong> {} cm</li>
            <li><strong>Neopixel RGB:</strong> {}, {}, {}</li>
        </ul>
        "#,
        ldr_left, ldr_middle, ldr_right, ultrasound, neopixel_r, neopixel_g, neopixel_b,
    );

    Html(html)
}

async fn partial_camera() -> impl IntoResponse {
    let ts = chrono::Utc::now().timestamp_millis();
    format!("/camera/frame.jpg?ts={}", ts)
}

async fn motor_forward_handler(State(bus): State<Arc<EventBus>>) {
    println!("Received forward command");

    let cmd = MotorCommand {
        direction: MotorDirection::Forward,
        speed: 100,
    };

    bus.publish(Event::MotorCommand(cmd));
}

async fn motor_stop_handler(State(bus): State<Arc<EventBus>>) {
    println!("Received stop command");

    let cmd = MotorCommand {
        direction: MotorDirection::Stop,
        speed: 0,
    };

    bus.publish(Event::MotorCommand(cmd));
}
