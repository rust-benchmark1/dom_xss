use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use ammonia::clean;
use serde::Deserialize;
#[derive(Deserialize)]
struct ColorParams {
    color: Option<String>,
}

// --- VULNERABLE COLOR ROUTE ---
#[get("/vulnerable_color")]
async fn vulnerable_color(_params: web::Query<ColorParams>) -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Vulnerable Color Example</title>
        </head>
        <body>
            <h1>This color is being applied: <span id="color-name"></span></h1>

            <script>
                const urlParams = new URLSearchParams(window.location.search);
                const color = urlParams.get('color');

                if (color) {
                    // Apply the color to the header (this part is safe)
                    document.querySelector('h1').style.color = color;

                    // DANGEROUS SINK: Display the color name using .innerHTML
                    // .innerHTML parses the string as HTML, allowing script execution
                    // through event handlers like 'onerror'.
                    document.getElementById('color-name').innerHTML = color;
                }
            </script>
        </body>
        </html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

// --- SECURE COLOR ROUTE ---
#[get("/secure_color")]
async fn secure_color() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Secure Color Example</title>
        </head>
        <body>
            <h1>This color is being applied: <span id="color-name"></span></h1>

            <script>
                const urlParams = new URLSearchParams(window.location.search);
                const color = urlParams.get('color');

                if (color) {
                    document.querySelector('h1').style.color = color;

                    // SAFE SINK: .textContent treats the entire string as plain text.
                    // It will be displayed, but never interpreted as HTML or executed.
                    document.getElementById('color-name').textContent = color;
                }
            </script>
        </body>
        </html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

// This handler demonstrates server-side sanitization in Rust.
#[get("/sanitized_color")]
async fn sanitized_color(params: web::Query<ColorParams>) -> impl Responder {
    // Sanitize user-supplied input on the server as a general rule.
    let unsafe_color = params.color.clone().unwrap_or_default();
    let safe_color = clean(&*unsafe_color); // Use ammonia to strip any HTML.

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head><title>Sanitized Page</title></head>
        <body style="color: {};">
            <h1>Color set to '{}' via server-side sanitization.</h1>
        </body>
        </html>
    "#,
        safe_color, safe_color
    );

    HttpResponse::Ok().content_type("text/html").body(html)
}

// This handler demonstrates adding a CSP header in Rust.
#[get("/csp")]
async fn csp() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>CSP Protected Page</title></head>
        <body>
            <h1>This page is protected by a Content Security Policy.</h1>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        // This CSP header is set by the Rust backend to mitigate XSS.
        // This policy disallows inline scripts, a common XSS vector.
        .insert_header((
            "Content-Security-Policy",
            "default-src 'self'; script-src 'self'",
        ))
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:3000");
    println!("- Vulnerable Color Page: http://127.0.0.1:3000/vulnerable_color");
    println!("- Secure Color Page:     http://127.0.0.1:3000/secure_color");
    println!("- Sanitized Color Example: http://127.0.0.1:3000/sanitized_color");
    println!("- CSP Example:           http://127.0.0.1:3000/csp");

    HttpServer::new(|| {
        App::new()
            .service(vulnerable_color)
            .service(secure_color)
            .service(sanitized_color)
            .service(csp)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
