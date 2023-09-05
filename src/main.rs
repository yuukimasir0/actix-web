use actix_web::{get, web, App, HttpResponse, HttpServer, post};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameter {
    n: u64,
    m: u64,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(get_index)
            .service(post_gcd)
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
}

#[get("/")]
async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title> GCD Calculator </title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
    )
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            n = std::mem::replace(&mut m, n);
        }
        m %= n;
    }
    n
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameter>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring");
    }

    let response = format!(
        "The greatest common divisor of numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );
    HttpResponse::Ok().content_type("text/html").body(response)
}
