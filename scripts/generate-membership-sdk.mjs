import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";

// Generated TypeScript SDK packages resolve @sdkwork/sdk-common through the
// pnpm workspace protocol and pin toolchain devDependencies for reproducible
// builds. Aligned with cloudrouter_strict_sdk_generate.mjs constants.
const SDK_COMMON_SPEC = "@sdkwork/sdk-common@workspace:*";
const SDK_TYPES_NODE_VERSION = "24.13.3";
const SDK_TYPESCRIPT_VERSION = "5.9.3";
const SDK_ROLLUP_VERSION = "4.62.4";
const STANDARDIZE_PUBLISH_CORE_HELPER = `function hasTypeScriptSdkDependencies(projectDir) {
  return existsSync(path.join(projectDir, 'node_modules', 'typescript'))
    && existsSync(path.join(projectDir, 'node_modules', 'rollup'))
    && existsSync(path.join(projectDir, 'node_modules', '@sdkwork', 'sdk-common'));
}

`;

const STANDARDIZE_PUBLISH_CORE_RUN_TYPE_SCRIPT = `function runTypeScript(ctx) {
  const packageFile = path.join(ctx.projectDir, 'package.json');
  ensureFile(packageFile, 'package.json');
  const packageJson = loadJson(packageFile);
  const hasBuildScript = Boolean(packageJson?.scripts?.build);

  if (!hasTypeScriptSdkDependencies(ctx.projectDir)) {
    run('npm', ['install', '--ignore-scripts'], { cwd: ctx.projectDir });
  } else {
    log('TypeScript dependencies already installed, skipping npm install.');
  }
  if (hasBuildScript) {
    run('npm', ['run', 'build'], { cwd: ctx.projectDir });
  } else {
    log('No build script found in package.json, skipping build.');
  }

  if (ctx.action === 'check') {
    run('npm', ['pack', '--dry-run'], { cwd: ctx.projectDir });
    return;
  }

  if (ctx.action === 'build') {
    return;
  }

  const registry = process.env.NPM_REGISTRY_URL || 'https://registry.npmjs.org/';
  const args = ['publish', '--access', 'public', '--registry', registry];
  if (ctx.channel === 'test') {
    args.push('--tag', 'next');
  }
  if (ctx.dryRun) {
    args.push('--dry-run');
  }
  run('npm', args, { cwd: ctx.projectDir });
}`;

function standardizePublishCoreContent(content) {
  let updated = content;
  if (!updated.includes("function hasTypeScriptSdkDependencies(projectDir)")) {
    const marker = "function runTypeScript(ctx) {";
    updated = updated.replace(marker, `${STANDARDIZE_PUBLISH_CORE_HELPER}${marker}`);
  }
  return replaceJavaScriptFunction(updated, "runTypeScript", STANDARDIZE_PUBLISH_CORE_RUN_TYPE_SCRIPT);
}

function replaceJavaScriptFunction(source, functionName, replacement) {
  const marker = `function ${functionName}(`;
  const start = source.indexOf(marker);
  if (start < 0) {
    return source;
  }

  const openBrace = source.indexOf("{", start);
  if (openBrace < 0) {
    return source;
  }

  let depth = 0;
  for (let index = openBrace; index < source.length; index += 1) {
    const character = source[index];
    if (character === "{") {
      depth += 1;
    } else if (character === "}") {
      depth -= 1;
      if (depth === 0) {
        return `${source.slice(0, start)}${replacement}${source.slice(index + 1)}`;
      }
    }
  }

  return source;
}

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
    const languageArgs = [
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
    ];
    if (language === "typescript") {
      languageArgs.push("--common-package", SDK_COMMON_SPEC);
    }
    runNode(languageArgs);
    if (language === "typescript") {
      standardizeTypeScriptGeneratedOutput(outputPath);
    }
  }
}

function standardizeTypeScriptGeneratedOutput(outputPath) {
  standardizeTypeScriptPackageJson(path.join(outputPath, "package.json"));
  const publishCorePath = path.join(outputPath, "bin", "publish-core.mjs");
  if (existsSync(publishCorePath)) {
    writeFileSync(publishCorePath, standardizePublishCoreContent(readFileSync(publishCorePath, "utf8")));
  }
}

function standardizeTypeScriptPackageJson(packageJsonPath) {
  const packageJson = JSON.parse(readFileSync(packageJsonPath, "utf8"));
  const devDependencies = { ...(packageJson.devDependencies ?? {}) };
  devDependencies["@types/node"] = SDK_TYPES_NODE_VERSION;
  devDependencies.typescript = SDK_TYPESCRIPT_VERSION;
  devDependencies.rollup = SDK_ROLLUP_VERSION;
  packageJson.devDependencies = devDependencies;
  writeFileSync(packageJsonPath, `${JSON.stringify(packageJson, null, 2)}\n`);
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
