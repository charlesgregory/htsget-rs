use std::collections::HashMap;

use actix_web::{
  web::{Data, Path, Query},
  Responder,
};
use tracing::info;
use tracing::instrument;

use htsget_http_core::{get_response_for_get_request, Endpoint};
use htsget_search::htsget::HtsGet;

use crate::AppState;

use super::handle_response;

/// GET request reads endpoint
#[instrument(skip(app_state))]
pub async fn reads<H: HtsGet + Send + Sync + 'static>(
  request: Query<HashMap<String, String>>,
  path: Path<String>,
  app_state: Data<AppState<H>>,
) -> impl Responder {
  let mut query_information = request.into_inner();
  query_information.insert("id".to_string(), path.into_inner());
  info!(query = ?query_information, "reads endpoint GET request");

  handle_response(
    get_response_for_get_request(
      app_state.get_ref().htsget.clone(),
      query_information,
      Endpoint::Reads,
    )
    .await,
  )
}

/// GET request variants endpoint
#[instrument(skip(app_state))]
pub async fn variants<H: HtsGet + Send + Sync + 'static>(
  request: Query<HashMap<String, String>>,
  path: Path<String>,
  app_state: Data<AppState<H>>,
) -> impl Responder {
  let mut query_information = request.into_inner();
  query_information.insert("id".to_string(), path.into_inner());
  info!(query = ?query_information, "variants endpoint GET request");

  handle_response(
    get_response_for_get_request(
      app_state.get_ref().htsget.clone(),
      query_information,
      Endpoint::Variants,
    )
    .await,
  )
}
