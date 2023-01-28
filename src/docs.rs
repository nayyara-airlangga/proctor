use utoipa::{
    openapi::{
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        ComponentsBuilder, InfoBuilder,
    },
    Modify, OpenApi,
};

pub use crate::{common, index, user};

#[derive(OpenApi)]
#[openapi(
    paths(
        index::health_check,
        user::handlers::register_user
    ),
    modifiers(&ApiInfo, &ApiBearerAuth),
    components(
        schemas(
            common::errors::ApiErrors,
            user::requests::RegisterUserRequest,
            user::responses::RegisterUserResponse,
        )
    )
)]
pub struct ApiDoc;

pub struct ApiInfo;

impl Modify for ApiInfo {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.info = InfoBuilder::new()
            .title("Proctor API")
            .description(Some("Video proctoring API"))
            .version("0.1.0")
            .build()
    }
}

pub struct ApiBearerAuth;

impl Modify for ApiBearerAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        } else {
            openapi.components = Some(
                ComponentsBuilder::new()
                    .security_scheme(
                        "bearer_auth",
                        SecurityScheme::Http(
                            HttpBuilder::new()
                                .scheme(HttpAuthScheme::Bearer)
                                .bearer_format("JWT")
                                .build(),
                        ),
                    )
                    .build(),
            )
        }
    }
}
