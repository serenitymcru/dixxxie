use axum::{extract::{State, Path}, Router, routing::get};
use adjust::{controller::Controller, response::HttpResult};
use crate::{models::hi::MessageDto, service::test::TestService, AppState};

#[derive(Default)]
pub struct TestController;

impl TestController {
  async fn say_hi(
    State(state): State<AppState>,
    Path(name): Path<String>,
  ) -> HttpResult<MessageDto> {
    let mut db = state.postgres.get()?;

    TestService::say_hi(&mut db, name)
  }
}

impl Controller<AppState> for TestController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: Router<AppState>) -> Router<AppState> {
    router
      .nest("/test",
      Router::new()
        .route("/sayHi/{name}", get(Self::say_hi))
    )
  }
}