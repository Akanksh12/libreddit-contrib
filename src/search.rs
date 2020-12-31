// CRATES
use crate::utils::{fetch_posts, param, ErrorTemplate, Post};
use actix_web::{http::StatusCode, HttpRequest, HttpResponse, Result};
use askama::Template;

// STRUCTS
#[derive(Template)]
#[allow(dead_code)]
#[template(path = "search.html", escape = "none")]
struct SearchTemplate {
	posts: Vec<Post>,
	query: String,
	sub: String,
	sort: (String, String),
	ends: (String, String),
}

// SERVICES
pub async fn page(req: HttpRequest) -> Result<HttpResponse> {
	let path = format!("{}.json?{}", req.path(), req.query_string());
	let q = param(&path, "q").await;
	let sort = if param(&path, "sort").await.is_empty() {
		"relevance".to_string()
	} else {
		param(&path, "sort").await
	};
	let sub = req.match_info().get("sub").unwrap_or("").to_string();

	let posts = fetch_posts(path.clone(), String::new()).await;

	if posts.is_err() {
		let s = ErrorTemplate {
			message: posts.err().unwrap().to_string(),
		}
		.render()
		.unwrap();
		Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).content_type("text/html").body(s))
	} else {
		let items = posts.unwrap();

		let s = SearchTemplate {
			posts: items.0,
			query: q,
			sub: sub,
			sort: (sort, param(&path, "t").await),
			ends: (param(&path, "after").await, items.1),
		}
		.render()
		.unwrap();
		Ok(HttpResponse::Ok().content_type("text/html").body(s))
	}
}