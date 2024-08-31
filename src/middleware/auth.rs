use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use ntex::http::header::HeaderName;
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web;
use serde::{Deserialize, Serialize};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtAuth;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    lang: String,
    exp: usize,
}

const SECRET_KEY: &str = "your-very-secret-key";

impl<S> Middleware<S> for JwtAuth {
    type Service = JwtAuthMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        JwtAuthMiddleware { service }
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for JwtAuthMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(
        &self,
        mut req: web::WebRequest<Err>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
                        &Validation::new(Algorithm::HS256),
                    ) {
                        Ok(decoded) => {
                            // Attach claims to request extensions
                            req.extensions_mut().insert(decoded.claims);
                        }
                        Err(_) => {
                            // Invalid token, we might generate a new anonymous token below
                        }
                    }
                }
            }
        }

        if req.extensions().get::<Claims>().is_none() {
            let claims = create_anonymous_jwt();
            let token = encode_jwt(&claims);
            req.extensions_mut().insert(claims);

            // Add the token to the response header
            req.headers_mut().insert(
                HeaderName::from_static("Authorization"),
                format!("Bearer {}", token).parse().unwrap(),
            );
        }

        let res = ctx.call(&self.service, req).await?;

        Ok(res)
    }
}

// Helper functions to create and encode JWTs
fn create_anonymous_jwt() -> Claims {
    Claims {
        sub: "anonymous".to_string(),
        lang: "en".to_string(), // Default language
        exp: (Utc::now() + Duration::days(30)).timestamp() as usize,
    }
}

fn encode_jwt(claims: &Claims) -> String {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )
    .unwrap()
}
