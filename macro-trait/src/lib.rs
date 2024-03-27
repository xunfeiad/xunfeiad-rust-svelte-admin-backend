use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Responder)]
pub fn parse_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl Responder for #name {
        type Body = BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
            let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .status(StatusCode::OK)
                .body(body)
        }
    }
    };
    TokenStream::from(expanded)
}
