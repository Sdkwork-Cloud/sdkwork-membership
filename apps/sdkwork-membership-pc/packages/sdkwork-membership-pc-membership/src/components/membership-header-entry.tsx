import {
  Suspense,
  useEffect,
  useRef,
  useState,
} from "react";
import { Crown } from "lucide-react";
import {
  createSdkworkMembershipToneStyle,
} from "../membership-appearance.ts";
import type { SdkworkMembershipMessagesOverrides } from "../membership-copy.ts";
import type { SdkworkMembershipController } from "../membership-controller.ts";
import {
  useSdkworkMembershipController,
  useSdkworkMembershipControllerState,
} from "../membership-controller.ts";
import {
  SdkworkMembershipIntlProvider,
  useSdkworkMembershipIntl,
} from "../membership-intl.tsx";
import { SdkworkMembershipHeaderMenu } from "./membership-header-menu.tsx";

export interface SdkworkMembershipHeaderEntryProps {
  checkoutBasePath?: string;
  controller?: SdkworkMembershipController;
  locale?: string | null;
  menuClassName?: string;
  messages?: SdkworkMembershipMessagesOverrides;
  onNavigate?: (route: string) => void;
  onOpenCenter?: () => void;
}

function SdkworkMembershipHeaderEntryContent({
  checkoutBasePath,
  controller: controllerProp,
  menuClassName,
  onNavigate,
  onOpenCenter,
}: Omit<SdkworkMembershipHeaderEntryProps, "locale" | "messages">) {
  const controller = useSdkworkMembershipController(controllerProp);
  const state = useSdkworkMembershipControllerState(controller);
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const entryRef = useRef<HTMLDivElement>(null);
  const { copy } = useSdkworkMembershipIntl();
  const label = state.dashboard.summary.isAuthenticated && state.dashboard.summary.currentLevelName
    ? state.dashboard.summary.currentLevelName
    : copy.headerEntry.fallbackLevel;

  useEffect(() => {
    if (!state.isBootstrapped && !state.isLoading && !state.lastError) {
      void controller.bootstrap().catch(() => undefined);
    }
  }, [controller, state.isBootstrapped, state.isLoading, state.lastError]);

  useEffect(() => {
    if (!isMenuOpen) {
      return undefined;
    }

    const handlePointerDown = (event: PointerEvent) => {
      if (entryRef.current?.contains(event.target as Node)) {
        return;
      }

      setIsMenuOpen(false);
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setIsMenuOpen(false);
      }
    };

    document.addEventListener("pointerdown", handlePointerDown, true);
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("pointerdown", handlePointerDown, true);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [isMenuOpen]);

  return (
    <div className="relative flex items-center" ref={entryRef}>
      <button
        aria-expanded={isMenuOpen}
        aria-haspopup="dialog"
        aria-label={copy.headerEntry.ariaLabel}
        className="inline-flex h-9 items-center gap-2 rounded-[1rem] border px-3 text-sm font-medium"
        onClick={() => setIsMenuOpen((current) => !current)}
        style={createSdkworkMembershipToneStyle("accent", {
          backgroundWeight: 12,
          borderWeight: 24,
        })}
        type="button"
      >
        <Crown className="h-4 w-4" />
        {label}
      </button>

      {isMenuOpen ? (
        <div
          className={menuClassName ?? "absolute right-0 top-[calc(100%+0.75rem)] z-50"}
          role="dialog"
          aria-label={copy.headerEntry.ariaLabel}
        >
          <Suspense fallback={null}>
            <SdkworkMembershipHeaderMenu
              checkoutBasePath={checkoutBasePath}
              controller={controller}
              onNavigate={(route) => {
                setIsMenuOpen(false);
                onNavigate?.(route);
              }}
              onOpenCenter={onOpenCenter
                ? () => {
                  setIsMenuOpen(false);
                  onOpenCenter();
                }
                : undefined}
            />
          </Suspense>
        </div>
      ) : null}
    </div>
  );
}

export function SdkworkMembershipHeaderEntry({
  locale,
  messages,
  ...props
}: SdkworkMembershipHeaderEntryProps) {
  const content = <SdkworkMembershipHeaderEntryContent {...props} />;

  if (locale || messages) {
    return (
      <SdkworkMembershipIntlProvider locale={locale} messages={messages}>
        {content}
      </SdkworkMembershipIntlProvider>
    );
  }

  return content;
}

/** Token Plan header entry alias for membership plan selection in app shells. */
export const SdkworkTokenPlanHeaderEntry = SdkworkMembershipHeaderEntry;
