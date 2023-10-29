# Passwordless Rust SDK

Community-driven Rust SDK designed to provide seamless integration with the [Bitwarden Passwordless.dev API](https://passwordless.dev/)

### Dependencies

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://www.rust-lang.org/tools/install)

## Usage


### Registration:

```Rust

    let client = passwordless_rust::PasswordlessClient::new(
        "API_SECRET",
        "https://v4.passwordless.dev"
    );

    let register_options = RegisterRequest {
        user_id: "1",
        username: "test",
        display_name: "Test",
    };
    let token = client.register_token(&register_options).await?;



```


### Verify user

```Rust

    let client = passwordless_rust::PasswordlessClient::new(
        "API_SECRET",
        "https://v4.passwordless.dev"
    );

    let request = SignInVerifyRequest {
        token: "TOKEN",
    };
    let response = client.sign_in(&request).await?;

```


### Examples

See [Examples](examples/) for Rocket Web application.
