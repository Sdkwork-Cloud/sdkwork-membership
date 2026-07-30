#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const surfaceIndex = process.argv.indexOf("--surface");
const requestedSurface = surfaceIndex >= 0 ? process.argv[surfaceIndex + 1] : "all";

const surfaces = {
  app: {
    source: "apis/app-api/membership/membership-app-api.openapi.json",
    authority:
      "sdks/sdkwork-membership-app-sdk/openapi/sdkwork-membership-app-api.openapi.json",
    derived:
      "sdks/sdkwork-membership-app-sdk/openapi/sdkwork-membership-app-api.sdkgen.json",
    apiAuthority: "sdkwork-membership-app-api",
    apiSurface: "app-api",
    pathPrefix: "/app/v3/api/",
  },
  backend: {
    source: "apis/backend-api/membership/membership-backend-api.openapi.json",
    authority:
      "sdks/sdkwork-membership-backend-sdk/openapi/sdkwork-membership-backend-api.openapi.json",
    derived:
      "sdks/sdkwork-membership-backend-sdk/openapi/sdkwork-membership-backend-api.sdkgen.json",
    apiAuthority: "sdkwork-membership-backend-api",
    apiSurface: "backend-api",
    pathPrefix: "/backend/v3/api/",
  },
};

if (requestedSurface !== "all" && !surfaces[requestedSurface]) {
  throw new Error(`unsupported membership SDK surface: ${requestedSurface}`);
}

const selectedSurfaces =
  requestedSurface === "all"
    ? Object.entries(surfaces)
    : [[requestedSurface, surfaces[requestedSurface]]];

for (const [surface, config] of selectedSurfaces) {
  const sourcePath = path.join(repoRoot, config.source);
  const document = JSON.parse(fs.readFileSync(sourcePath, "utf8"));
  const operations = collectOperations(document);

  if (document.info?.["x-sdkwork-api-authority"] !== config.apiAuthority) {
    throw new Error(`${config.source} has the wrong API authority`);
  }
  if (operations.length === 0) {
    throw new Error(`${config.source} does not declare any operations`);
  }

  for (const operation of operations) {
    if (!operation.path.startsWith(config.pathPrefix)) {
      throw new Error(`${operation.method} ${operation.path} is outside ${config.pathPrefix}`);
    }
    if (operation.contract["x-sdkwork-owner"] !== "sdkwork-membership") {
      throw new Error(`${operation.method} ${operation.path} is not membership-owned`);
    }
    if (operation.contract["x-sdkwork-api-authority"] !== config.apiAuthority) {
      throw new Error(`${operation.method} ${operation.path} has the wrong API authority`);
    }
    if (operation.contract["x-sdkwork-api-surface"] !== config.apiSurface) {
      throw new Error(`${operation.method} ${operation.path} has the wrong API surface`);
    }
  }

  const materialized = `${JSON.stringify(document, null, 2)}\n`;
  writeIfChanged(path.join(repoRoot, config.authority), materialized);
  writeIfChanged(path.join(repoRoot, config.derived), materialized);
  console.log(`materialized ${surface} SDK OpenAPI (${operations.length} operations)`);
}

function collectOperations(document) {
  const operations = [];
  for (const [routePath, pathItem] of Object.entries(document.paths ?? {})) {
    for (const [method, contract] of Object.entries(pathItem ?? {})) {
      if (!contract || typeof contract !== "object" || !contract.operationId) {
        continue;
      }
      operations.push({ path: routePath, method: method.toUpperCase(), contract });
    }
  }
  return operations;
}

function writeIfChanged(targetPath, content) {
  if (fs.existsSync(targetPath) && fs.readFileSync(targetPath, "utf8") === content) {
    return;
  }
  fs.mkdirSync(path.dirname(targetPath), { recursive: true });
  fs.writeFileSync(targetPath, content);
}
