use serde::Deserialize;
use actix_web::{web, App, HttpResponse, HttpServer};
static LISTEN_ADDRESS: &str = "localhost:3000";

#[derive(Deserialize)]
struct GcdParameters {
  n: u64,
  m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    println!("Listening on http://{}", LISTEN_ADDRESS);
    server.bind(LISTEN_ADDRESS).expect("Error binding to port").run().expect("Error running gcd-server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
      r#"
          <title>GCD Calculator</title>
          <form action="/gcd" method="post"/>
            <input type="text", name="n"/>
            <input type="text", name="m"/>
            <button type="submit">Compute GCD</button>
          </form>
      "#,
    )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("html/text").body("Can't compute GCD with zero");
    }
    let response = format!("The GCD of {} and {} is <b>{}</b>", form.n, form.m, gcd(form.n, form.m));
    HttpResponse::Ok().content_type("html/text").body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19), 3 * 11);
}

