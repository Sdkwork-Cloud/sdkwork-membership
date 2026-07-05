#!/usr/bin/env node
/**
 * Align membership app-api list/search operations with PAGINATION_SPEC and API_SPEC envelopes.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const authorityPath = path.join(
  repoRoot,
  "apis/app-api/membership/membership-app-api.openapi.json",
);
const sdkOpenApiPath = path.join(
  repoRoot,
  "sdks/sdkwork-membership-app-sdk/openapi/sdkwork-membership-app-api.openapi.json",
);
const sdkgenPath = path.join(
  repoRoot,
  "sdks/sdkwork-membership-app-sdk/openapi/sdkwork-membership-app-api.sdkgen.json",
);

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
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      if (!operation || typeof operation !== "object" || !operation.operationId) {
        continue;
      }

      const operationId = String(operation.operationId);
      const successSchema = operation.responses?.["200"]?.content?.["application/json"]?.schema;
      if (!successSchema) {
        continue;
      }

      if (operationId.endsWith(".list")) {
        const parameters = Array.isArray(operation.parameters) ? [...operation.parameters] : [];
        if (!parameters.some((param) => param.name === "page")) {
          operation.parameters = [...parameters, ...PAGE_PARAMS];
        }
        operation.responses["200"].content["application/json"].schema = {
          $ref: "#/components/schemas/SdkWorkListResponse",
        };
        continue;
      }

      if (method === "post") {
        operation.responses["200"].content["application/json"].schema = {
          $ref: "#/components/schemas/SdkWorkCommandResponse",
        };
        continue;
      }

      if (method === "get") {
        operation.responses["200"].content["application/json"].schema = {
          $ref: "#/components/schemas/SdkWorkResourceResponse",
        };
      }
    }
  }

  return document;
}

function writeJson(targetPath, value) {
  fs.writeFileSync(targetPath, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

const aligned = alignDocument(JSON.parse(fs.readFileSync(authorityPath, "utf8")));
writeJson(authorityPath, aligned);
writeJson(sdkOpenApiPath, aligned);
writeJson(sdkgenPath, aligned);
console.log("aligned membership app-api list/search envelopes and pagination params");
