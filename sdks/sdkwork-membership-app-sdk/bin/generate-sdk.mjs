#!/usr/bin/env node

import path from "node:path";
import { fileURLToPath } from "node:url";
import { runMembershipSdkGeneration } from "../../../scripts/generate-membership-sdk.mjs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

runMembershipSdkGeneration({
  argv: process.argv.slice(2),
  familyRoot: path.resolve(__dirname, ".."),
  surface: "app",
});
