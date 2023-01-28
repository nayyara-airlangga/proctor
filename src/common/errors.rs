use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct ApiErrors {
    #[schema(example = json!(["An error occured"]))]
    pub errors: Vec<String>,
}
