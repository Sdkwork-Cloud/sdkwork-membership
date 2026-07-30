use std::sync::LazyLock;

use sdkwork_web_core::HttpRouteManifest;

include!(concat!(env!("OUT_DIR"), "/app_http_routes.rs"));

pub static APP_API_HTTP_ROUTE_MANIFEST: LazyLock<HttpRouteManifest> =
    LazyLock::new(|| HttpRouteManifest::new(APP_API_HTTP_ROUTES));

pub fn app_route_manifest() -> HttpRouteManifest {
    APP_API_HTTP_ROUTE_MANIFEST.clone()
}
