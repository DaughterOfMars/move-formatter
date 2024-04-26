use movefmt::GetOptsOptions;
use napi::Status;
use napi_derive::napi;

#[napi]
pub fn format_string(s: String) -> napi::Result<String> {
    movefmt::format_text(
        &s,
        &GetOptsOptions {
            quiet: true,
            ..Default::default()
        },
    )
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
}
