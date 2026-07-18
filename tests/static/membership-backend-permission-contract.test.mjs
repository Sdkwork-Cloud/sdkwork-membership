import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = process.cwd();
const reviewContractPath = path.join(
  root,
  "apis/backend-api/membership/membership-backend-api.openapi.json",
);
const sdkAuthorityPath = path.join(
  root,
  "sdks/sdkwork-membership-backend-sdk/openapi/sdkwork-membership-backend-api.openapi.json",
);
const sdkInputPath = path.join(
  root,
  "sdks/sdkwork-membership-backend-sdk/openapi/sdkwork-membership-backend-api.sdkgen.json",
);

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function operationEntries(contract) {
  return Object.values(contract.paths).flatMap((pathItem) =>
    Object.entries(pathItem)
      .filter(([method]) => ["delete", "get", "patch", "post", "put"].includes(method))
      .map(([method, operation]) => ({ method, operation })),
  );
}

function operationMap(contract) {
  return new Map(
    operationEntries(contract).map(({ method, operation }) => [
      operation.operationId,
      { method, operation },
    ]),
  );
}

test("membership backend operator operations declare central commerce permissions", () => {
  const centralCatalog = readJson(
    path.join(root, "../sdkwork-iam/iam/modules/commerce/iam.module.manifest.json"),
  );
  const centralCodes = new Set(
    centralCatalog.permissions.catalog.map((permission) => permission.code),
  );

  for (const contractPath of [reviewContractPath, sdkAuthorityPath, sdkInputPath]) {
    for (const { method, operation } of operationEntries(readJson(contractPath))) {
      if (operation.operationId === "memberships.purchases.fulfillments.create") {
        assert.equal(operation["x-sdkwork-credential-mode"], "service-token");
        assert.equal(operation["x-sdkwork-permission"], undefined);
        continue;
      }

      const expectedPermission = method === "get"
        ? "commerce.memberships.read"
        : "commerce.memberships.manage";
      assert.equal(operation["x-sdkwork-permission"], expectedPermission);
      assert.ok(centralCodes.has(expectedPermission), `${expectedPermission} is not in commerce IMF`);
    }
  }
});

test("membership backend SDK authority excludes service-only fulfillment", () => {
  const reviewOperations = operationMap(readJson(reviewContractPath));
  const authorityOperations = operationMap(readJson(sdkAuthorityPath));
  const inputOperations = operationMap(readJson(sdkInputPath));

  assert.ok(reviewOperations.has("memberships.purchases.fulfillments.create"));
  assert.equal(authorityOperations.has("memberships.purchases.fulfillments.create"), false);
  assert.deepEqual([...inputOperations.keys()].sort(), [...authorityOperations.keys()].sort());
  assert.deepEqual(readJson(sdkInputPath), readJson(sdkAuthorityPath));
});
