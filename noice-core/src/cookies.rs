use cookie::{Cookie, CookieJar};
use silent::{header, Request, Response, Result, SilentError, StatusCode};

pub fn set_cookie(res: &mut Response, cookies: CookieJar) {
    for cookie in cookies.delta() {
        if let Ok(hv) = cookie.encoded().to_string().parse() {
            res.headers_mut().append(header::SET_COOKIE, hv);
        }
    }
}

pub fn get_cookie(req: &Request) -> Result<CookieJar> {
    let mut jar = CookieJar::new();
    if let Some(cookies) = req.headers().get(header::COOKIE) {
        for cookie_str in cookies.to_str().map_err(|e|
            SilentError::business_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse cookie: {}", e),
            )
        )?.split(';').map(|s| s.trim()) {
            if let Ok(cookie) = Cookie::parse_encoded(cookie_str).map(|c| c.into_owned()) {
                jar.add_original(cookie);
            }
        }
    }
    Ok(jar)
}