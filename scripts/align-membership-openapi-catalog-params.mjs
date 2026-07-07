#!/usr/bin/env node
/**
 * Align membership app-api catalog/list query parameters with router handlers.
 * Preserves SdkWorkApiResponse envelope schemas (does not regress to legacy wrappers).
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

const CURSOR_PARAM = {
  name: "cursor",
  in: "query",
  required: false,
  schema: { type: "string" },
  description: "Opaque cursor for keyset pagination continuation.",
};

const PLAN_ID_PARAM = {
  name: "plan_id",
  in: "query",
  required: false,
  schema: { type: "integer", format: "int64" },
  description:
    "Filter catalog results by membership plan rank (matches AppMembershipPlanItem.id).",
};

const RECOMMENDED_ONLY_PARAM = {
  name: "recommended_only",
  in: "query",
  required: false,
  schema: { type: "boolean", default: false },
  description: "When true, return only recommended packages or groups with recommended packages.",
};

const PACKAGE_GROUP_ID_PARAM = {
  name: "package_group_id",
  in: "query",
  required: false,
  schema: { type: "integer", format: "int64" },
  description: "Filter packages by package group external id.",
};

const OPERATION_PARAMS = {
  "memberships.benefits.list": [PLAN_ID_PARAM, CURSOR_PARAM],
  "memberships.plans.list": [CURSOR_PARAM],
  "memberships.packageGroups.list": [PLAN_ID_PARAM, RECOMMENDED_ONLY_PARAM, CURSOR_PARAM],
  "memberships.packageGroups.packages.list": [PLAN_ID_PARAM, RECOMMENDED_ONLY_PARAM, CURSOR_PARAM],
  "memberships.packages.list": [PACKAGE_GROUP_ID_PARAM, PLAN_ID_PARAM, CURSOR_PARAM],
  "memberships.points.history.list": [CURSOR_PARAM],
};

function upsertParameter(parameters, param) {
  const next = Array.isArray(parameters) ? [...parameters] : [];
  const index = next.findIndex((entry) => entry.name === param.name);
  if (index >= 0) {
    next[index] = { ...next[index], ...param };
    return next;
  }
  return [...next, param];
}

function alignDocument(document) {
  for (const pathItem of Object.values(document.paths ?? {})) {
    for (const operation of Object.values(pathItem ?? {})) {
      if (!operation || typeof operation !== "object" || !operation.operationId) {
        continue;
      }
      const extras = OPERATION_PARAMS[operation.operationId];
      if (!extras) {
        continue;
      }
      let parameters = operation.parameters ?? [];
      for (const param of extras) {
        parameters = upsertParameter(parameters, param);
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
  console.log(`aligned ${path.relative(repoRoot, targetPath)}`);
}
