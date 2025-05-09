// Import UI styles - temporarily commented out due to build issues
// import "@hypr/ui/globals.css";
import "./styles/globals.css";

// Import mock Tauri APIs for development
import { mockTauriApis } from "./mocks/tauri";

// Initialize mock Tauri APIs when running in development without Tauri
if (import.meta.env.DEV && !window.hasOwnProperty("__TAURI_IPC__")) {
  mockTauriApis();
}

import { i18n } from "@lingui/core";
import { I18nProvider } from "@lingui/react";
import { QueryClient, QueryClientProvider, useQuery, useQueryClient } from "@tanstack/react-query";
import { CatchBoundary, createRouter, ErrorComponent, RouterProvider } from "@tanstack/react-router";
import { useEffect } from "react";
import ReactDOM from "react-dom/client";

import type { Context } from "@/types";
import { commands } from "@/types";
import { commands as authCommands } from "@hypr/plugin-auth";
import { Toaster } from "@hypr/ui/components/ui/toast";
import { TooltipProvider } from "@hypr/ui/components/ui/tooltip";
import { ThemeProvider } from "@hypr/ui/contexts/theme";
import { broadcastQueryClient } from "./utils";
import { EncryptionProvider, useEncryption } from "./contexts/encryption";
import { PasswordModal } from "./components/password-modal";

import { messages as enMessages } from "./locales/en/messages";
import { messages as koMessages } from "./locales/ko/messages";

import { createOngoingSessionStore, createSessionsStore } from "@hypr/utils/stores";
import { routeTree } from "./routeTree.gen";

import * as Sentry from "@sentry/react";
import { defaultOptions } from "tauri-plugin-sentry-api";

i18n.load({
  en: enMessages,
  ko: koMessages,
});

// TODO: load language from user settings
i18n.activate("en");

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // for most case, we don't want cache
      gcTime: 0,
    },
  },
});

const sessionsStore = createSessionsStore();
const ongoingSessionStore = createOngoingSessionStore(sessionsStore);

const context: Context = {
  queryClient,
  ongoingSessionStore,
  sessionsStore,
};

const router = createRouter({
  routeTree,
  context: context as Required<Context>,
  defaultPreload: "intent",
  defaultViewTransition: false,
  // Since we're using React Query, we don't want loader calls to ever be stale
  // This will ensure that the loader is always called when the route is preloaded or visited
  defaultPreloadStaleTime: 0,
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

commands.sentryDsn().then((dsn) => {
  // https://docs.sentry.io/platforms/javascript/guides/react/features/tanstack-router/
  Sentry.init({
    ...defaultOptions,
    dsn,
    integrations: [Sentry.tanstackRouterBrowserTracingIntegration(router)],
    tracesSampleRate: 1.0,
  });
});

const rootElement = document.getElementById("root")!;

function App() {
  const queryClient = useQueryClient();

  useEffect(() => {
    return broadcastQueryClient(queryClient);
  }, [queryClient]);

  const userId = useQuery({
    queryKey: ["userId"],
    queryFn: () => authCommands.getFromStore("auth-user-id"),
    staleTime: Infinity,
  });

  if (!userId.data) {
    return null;
  }

  return (
    <EncryptionProvider>
      <RouterProvider router={router} context={{ ...context, userId: userId.data }} />
      <PasswordModalContainer />
    </EncryptionProvider>
  );
}

function PasswordModalContainer() {
  const { isPasswordModalOpen, closePasswordModal } = useEncryption();

  return (
    <PasswordModal
      isOpen={isPasswordModalOpen}
      onClose={closePasswordModal}
    />
  );
}

if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <CatchBoundary getResetKey={() => "error"} errorComponent={ErrorComponent}>
      <TooltipProvider delayDuration={700} skipDelayDuration={300}>
        <ThemeProvider defaultTheme="light">
          <QueryClientProvider client={queryClient}>
            <I18nProvider i18n={i18n}>
              <App />
              <Toaster
                position="bottom-left"
                expand={true}
                offset={16}
                duration={Infinity}
                swipeDirections={[]}
              />
            </I18nProvider>
          </QueryClientProvider>
        </ThemeProvider>
      </TooltipProvider>
    </CatchBoundary>,
  );
}
