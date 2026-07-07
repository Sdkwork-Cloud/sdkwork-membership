/**
 * Relative path from this composition module to the component spec that
 * declares SDK dependencies and surface metadata.
 */
export const sdkworkComponentSpecPath = "../../../specs/component.spec.json" as const;

/**
 * Runtime packages that the membership PC composition layer depends on.
 * These are the composed facade packages consumed by the shell and feature
 * modules — never generator transport package names.
 */
export const sdkworkCoreCompositionDependencies = [
  "@sdkwork/membership-pc-core",
  "@sdkwork/membership-pc-membership",
  "@sdkwork/membership-pc-subscription",
  "@sdkwork/membership-pc-shell",
  "@sdkwork/membership-service",
  "@sdkwork/ui-pc-react",
] as const;

export interface SdkworkCoreDependencyManifest {
  readonly componentSpecPath: string;
  readonly runtime: readonly string[];
}

/**
 * Resolves the dependency manifest for the membership PC composition.
 * Consumed by governance checks and composition bootstrap to verify that
 * all required composed facades are present in the workspace.
 */
export function resolveSdkworkCoreDependencyManifest(): SdkworkCoreDependencyManifest {
  return {
    componentSpecPath: sdkworkComponentSpecPath,
    runtime: sdkworkCoreCompositionDependencies,
  };
}
