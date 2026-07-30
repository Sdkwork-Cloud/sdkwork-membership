use sdkwork_web_core::HttpRouteManifest;

include!(concat!(env!("OUT_DIR"), "/backend_http_routes.rs"));

pub fn backend_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(BACKEND_API_HTTP_ROUTES)
}
