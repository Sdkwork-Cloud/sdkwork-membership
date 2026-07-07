#!/usr/bin/env node
/**
 * Ensure membership app-api list operations document standard page/page_size params.
 * Does NOT rewrite success response schemas (preserves SdkWorkApiResponse allOf envelopes).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const targets = [
  path.join(repoRoot, "apis/app-api/membership/membership-app-api.openapi.json"),
  path.join(
    repoRoot,
    "sdks/sdkwork-membership-app-sdk/openapi/sdkwork-membership-app-api.openapi.json",
  ),
  path.join(
    repoRoot,
    "sdks/sdkwork-membership-app-sdk/openapi/sdkwork-membership-app-api.sdkgen.json",
  ),
];

const PAGE_PARAMS = [
  {
    name: "page",
    in: "query",
    required: false,
    schema: { type: "integer", minimum: 1, default: 1 },
  },
  {
    name: "page_size",
    in: "query",
    required: false,
    schema: { type: "integer", minimum: 1, maximum: 200, default: 20 },
  },
];

function alignDocument(document) {
  for (const pathItem of Object.values(document.paths ?? {})) {
    for (const operation of Object.values(pathItem ?? {})) {
      if (!operation || typeof operation !== "object" || !operation.operationId) {
        continue;
      }
      if (!String(operation.operationId).endsWith(".list")) {
        continue;
      }
      let parameters = Array.isArray(operation.parameters) ? [...operation.parameters] : [];
      for (const param of PAGE_PARAMS) {
        if (!parameters.some((entry) => entry.name === param.name)) {
          parameters.push(param);
        }
      }
      operation.parameters = parameters;
    }
  }
  return document;
}

for (const targetPath of targets) {
  if (!fs.existsSync(targetPath)) {
    console.warn(`skip missing ${targetPath}`);
    continue;
  }
  const document = alignDocument(JSON.parse(fs.readFileSync(targetPath, "utf8")));
  fs.writeFileSync(targetPath, `${JSON.stringify(document, null, 2)}\n`);
  console.log(`aligned pagination params in ${path.relative(repoRoot, targetPath)}`);
}
