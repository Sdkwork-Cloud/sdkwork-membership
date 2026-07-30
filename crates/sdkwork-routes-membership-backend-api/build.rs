use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let openapi =
        manifest_dir.join("../../apis/backend-api/membership/membership-backend-api.openapi.json");
    let output =
        PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR")).join("backend_http_routes.rs");

    println!("cargo:rerun-if-changed={}", openapi.display());
    sdkwork_web_build::generate_http_route_manifest(openapi, output, "BACKEND_API_HTTP_ROUTES")
        .expect("generate membership backend HTTP route manifest");
}
