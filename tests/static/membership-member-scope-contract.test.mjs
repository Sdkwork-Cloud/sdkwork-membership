import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = process.cwd();

for (const engine of ["postgres", "sqlite"]) {
  test(`${engine} membership member detail and status update are organization scoped`, () => {
    const source = fs.readFileSync(
      path.join(root, `crates/sdkwork-membership-repository-sqlx/src/${engine}.rs`),
      "utf8",
    );
    const retrieveStart = source.indexOf("async fn load_admin_membership(");
    const retrieveEnd = source.indexOf("async fn ensure_admin_plan_exists", retrieveStart);
    const retrieve = source.slice(retrieveStart, retrieveEnd);
    const updateStart = source.indexOf("async fn update_admin_membership_member_status(");
    const updateEnd = source.indexOf("async fn list_admin_membership_entitlements", updateStart);
    const update = source.slice(updateStart, updateEnd);

    assert.match(retrieve, /organization_id/u);
    assert.match(retrieve, /query\.subject\.organization_id/u);
    assert.match(update, /organization_id/u);
    assert.match(update, /command\.subject\.organization_id/u);
  });
}
