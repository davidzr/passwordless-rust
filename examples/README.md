# Passwordless Rust SDK Example

This example is based on the examples provided in the [Python SDK](https://github.com/passwordless/passwordless-python)

Please read the documentation here: https://docs.passwordless.dev

## Getting Started

This example uses *Rocket* and provides UI and REST interfaces to interact with
the [Passwordless Rust SDK][passwordless-Rust-sdk] to the *Passwordless API*.

Install dependencies: `cargo build`

1. Get your own API keys here: https://admin.passwordless.dev/signup
2. Change the value of the `PASSWORDLESS_API_KEY` and `PASSWORDLESS_API_SECRET`
   in [.env file](.env) with your API Key and Secret.
3. Change the value of the `PASSWORDLESS_API_URL`
   with the base url where your *Passwordless API* instance is running.
4. Start the application `cargo run --example server`
5. The application will now listen on port `8000` e.g. http://localhost:8000, where you can *Sign In* and *Register*
   users within your Application.
