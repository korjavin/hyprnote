// Mock Tauri APIs for development without the Tauri backend

// Mock the invoke function
(window as any).__TAURI_INVOKE__ = async (cmd: string, args?: any) => {
  console.log(`Mock Tauri invoke: ${cmd}`, args);
  
  // Handle specific commands
  if (cmd === "plugin:auth|getFromStore") {
    return "mock-user-id";
  }
  
  if (cmd === "sentry_dsn") {
    return "https://mock-sentry-dsn@sentry.io/123456";
  }
  
  // Default return
  return null;
};

// Mock the window API
(window as any).__TAURI__ = {
  window: {
    appWindow: {
      label: "main",
      metadata: {}
    }
  }
};

export const mockTauriApis = () => {
  console.log("Tauri APIs mocked for development");
};
