/**
 * Describes an SDK family consumed by the membership PC composition.
 * Mirrors the `sdkDependencies` declared in `specs/component.spec.json`.
 */
export interface SdkworkMembershipPcSdkDescriptor {
  readonly composedFacade: string;
  readonly family: string;
  readonly surface: "app-api" | "backend-api" | "open-api";
  readonly credentialMode: "authenticated-app-api" | "authenticated-backend-api" | "anonymous";
  readonly status: "available";
}

const SDKWORK_MEMBERSHIP_PC_SDK_INVENTORY: readonly SdkworkMembershipPcSdkDescriptor[] = [
  {
    composedFacade: "@sdkwork/membership-app-sdk",
    family: "sdkwork-membership-app-sdk",
    surface: "app-api",
    credentialMode: "authenticated-app-api",
    status: "available",
  },
  {
    composedFacade: "@sdkwork/iam-app-sdk",
    family: "sdkwork-iam-app-sdk",
    surface: "app-api",
    credentialMode: "authenticated-app-api",
    status: "available",
  },
];

/**
 * Lists the SDK families that the membership PC composition depends on.
 * Used by composition bootstrap to verify SDK availability before mounting
 * the shell.
 */
export function listSdkworkCoreSdkInventory(): readonly SdkworkMembershipPcSdkDescriptor[] {
  return SDKWORK_MEMBERSHIP_PC_SDK_INVENTORY;
}
