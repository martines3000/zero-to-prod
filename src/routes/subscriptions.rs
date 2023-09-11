use axum::{
    async_trait,
    extract::{rejection::FormRejection, FromRequest},
    response::{IntoResponse, Response},
    Form,
};
use hyper::{Request, StatusCode};
use serde::{de::DeserializeOwned, Deserialize};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SubscribeFormInput {
    #[validate(required, length(min = 1, max = 32))]
    pub name: Option<String>,
    #[validate(required, email)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("{}", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

pub async fn subscribe(
    ValidatedForm(data): ValidatedForm<SubscribeFormInput>,
) -> impl IntoResponse {
    print!("{:?}", data);

    // Return 200 OK
    ""
}
