use oauth2::basic::BasicClient;
use oauth2::reqwest;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use std::io::{self, Write};
use url::Url;

use super::Userinfo;

pub fn get_token(userinfo: Userinfo) -> Result<String, Box<dyn std::error::Error>> {
    let client = BasicClient::new(ClientId::new(userinfo.client_id))
        .set_client_secret(ClientSecret::new(userinfo.client_secret))
        .set_auth_uri(AuthUrl::new(
            "https://www.reddit.com/api/v1/authorize".to_string(),
        )?)
        .set_token_uri(TokenUrl::new(
            "https://www.reddit.com/api/v1/access_token".to_string(),
        )?)
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_token`.

    let http_client = reqwest::blocking::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");
    io::stdout().flush()?;
    let mut auth_code = String::new();
    io::stdin().read_line(&mut auth_code)?;
    let auth_code = auth_code.trim();

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code.to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request(&http_client);
    match token_result {
        Ok(token) => {
            println!("Access Token: {}", token.access_token().secret());
            if let Some(refresh_token) = token.refresh_token() {
                println!("Refresh Token: {}", refresh_token.secret());
            }
            Ok(token.access_token().secret().to_string())
        }
        Err(err) => {
            eprintln!("Error exchanging authorization code: {}", err);
            Err(Box::new(err))
        }
    }

    // Unwrapping token_result will either produce a Token or a RequestTokenError.
}
