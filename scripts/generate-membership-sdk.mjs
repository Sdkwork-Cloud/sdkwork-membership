import { existsSync } from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";

const SURFACES = {
  app: {
    apiPrefix: "/app/v3/api",
    clientName: "SdkworkAppClient",
    defaultBaseUrl: "http://127.0.0.1:18096",
    inputName: "sdkwork-membership-app-api.sdkgen.json",
    packageName: "@sdkwork/membership-app-sdk",
    sdkType: "app",
  },
  backend: {
    apiPrefix: "/backend/v3/api",
    clientName: "SdkworkBackendClient",
    defaultBaseUrl: "http://127.0.0.1:18079",
    inputName: "sdkwork-membership-backend-api.sdkgen.json",
    packageName: "@sdkwork/membership-backend-sdk",
    sdkType: "backend",
  },
};

export function runMembershipSdkGeneration({
  argv = [],
  env = process.env,
  familyRoot,
  surface,
}) {
  const surfaceConfig = SURFACES[surface];
  if (!surfaceConfig) {
    throw new Error(`Unsupported membership SDK surface: ${surface}`);
  }

  const options = parseOptions(argv, env, surfaceConfig.defaultBaseUrl);
  const membershipRoot = path.resolve(familyRoot, "../..");
  const workspaceRoot = path.resolve(membershipRoot, "..");
  const generatorPath = path.join(workspaceRoot, "sdkwork-sdk-generator", "bin", "sdkgen.js");
  const materializerPath = path.join(membershipRoot, "scripts", "materialize-membership-sdk-openapi.mjs");
  const sdkName = path.basename(familyRoot);
  const inputPath = path.join(familyRoot, "openapi", surfaceConfig.inputName);

  requireFile(generatorPath, "Canonical SDK generator");
  requireFile(materializerPath, "Membership SDK OpenAPI materializer");
  runNode([materializerPath, "--surface", surface]);
  requireFile(inputPath, "OpenAPI input");

  for (const language of options.languages) {
    const languageWorkspace = path.join(familyRoot, `${sdkName}-${language}`);
    const outputPath = path.join(languageWorkspace, "generated", "server-openapi");
    assertPathWithin(languageWorkspace, outputPath);

    process.stdout.write(`Generating ${language} SDK at ${outputPath}\n`);
    runNode([
      generatorPath,
      "generate",
      "-i",
      inputPath,
      "-o",
      outputPath,
      "-n",
      sdkName,
      "-t",
      surfaceConfig.sdkType,
      "-l",
      language,
      "--fixed-sdk-version",
      options.sdkVersion,
      "--base-url",
      options.baseUrl,
      "--api-prefix",
      surfaceConfig.apiPrefix,
      "--package-name",
      surfaceConfig.packageName,
      "--client-name",
      surfaceConfig.clientName,
      "--standard-profile",
      "sdkwork-v3",
      "--sdk-root",
      familyRoot,
      "--sdk-name",
      sdkName,
      "--no-sync-published-version",
    ]);
  }
}

function parseOptions(argv, env, defaultBaseUrl) {
  const options = {
    baseUrl: defaultBaseUrl,
    languages: env.LANGUAGES || "typescript",
    sdkVersion: "0.1.0",
  };

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === "--languages") {
      options.languages = requiredValue(argv, ++index, argument);
    } else if (argument === "--base-url") {
      options.baseUrl = requiredValue(argv, ++index, argument);
    } else if (argument === "--sdk-version") {
      options.sdkVersion = requiredValue(argv, ++index, argument);
    } else if (!argument.startsWith("-") && index === 0) {
      options.languages = argument;
    } else {
      throw new Error(`Unsupported SDK generation argument: ${argument}`);
    }
  }

  const languages = String(options.languages)
    .split(",")
    .map((language) => language.trim().toLowerCase())
    .filter(Boolean);
  if (languages.length === 0) {
    throw new Error("At least one SDK language is required");
  }
  return { ...options, languages };
}

function requiredValue(argv, index, argument) {
  const value = argv[index];
  if (!value || value.startsWith("--")) {
    throw new Error(`${argument} requires a value`);
  }
  return value;
}

function requireFile(filePath, label) {
  if (!existsSync(filePath)) {
    throw new Error(`${label} not found: ${filePath}`);
  }
}

function assertPathWithin(root, target) {
  const relative = path.relative(path.resolve(root), path.resolve(target));
  if (relative.startsWith("..") || path.isAbsolute(relative)) {
    throw new Error(`Refusing SDK output outside language workspace: ${target}`);
  }
}

function runNode(args) {
  const result = spawnSync(process.execPath, args, { stdio: "inherit" });
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new Error(`SDK generation command failed with exit code ${result.status ?? 1}`);
  }
}
