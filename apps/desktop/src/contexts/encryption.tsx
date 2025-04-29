import React, { createContext, useContext, useEffect, useState } from "react";

// Mock implementation for testing
// In production, this would be imported from the actual plugin
const commands = {
  get_encryption_status: async (): Promise<boolean> => {
    console.log("Mock: get_encryption_status called");
    return false;
  },
  unlock_app: async (password: string): Promise<boolean> => {
    console.log("Mock: unlock_app called with password:", password);
    return password === "test";
  },
  lock_app: async (): Promise<void> => {
    console.log("Mock: lock_app called");
  },
  change_password: async (old_password: string, new_password: string): Promise<void> => {
    console.log("Mock: change_password called with old_password:", old_password, "new_password:", new_password);
  }
};

interface EncryptionContextType {
  isEncryptionEnabled: boolean;
  isUnlocked: boolean;
  unlockApp: (password: string) => Promise<boolean>;
  lockApp: () => Promise<void>;
  isPasswordModalOpen: boolean;
  openPasswordModal: () => void;
  closePasswordModal: () => void;
}

const EncryptionContext = createContext<EncryptionContextType | undefined>(undefined);

export function useEncryption() {
  const context = useContext(EncryptionContext);
  if (context === undefined) {
    throw new Error("useEncryption must be used within an EncryptionProvider");
  }
  return context;
}

interface EncryptionProviderProps {
  children: React.ReactNode;
}

export function EncryptionProvider({ children }: EncryptionProviderProps) {
  const [isEncryptionEnabled, setIsEncryptionEnabled] = useState(false);
  const [isUnlocked, setIsUnlocked] = useState(false);
  const [isPasswordModalOpen, setIsPasswordModalOpen] = useState(false);

  // Check encryption status on mount
  useEffect(() => {
    const checkEncryptionStatus = async () => {
      try {
        const status = await commands.get_encryption_status();
        setIsEncryptionEnabled(true);
        setIsUnlocked(status);

        // If encryption is enabled but not unlocked, show the password modal
        if (status === false) {
          setIsPasswordModalOpen(true);
        }
      } catch (error) {
        console.error("Error checking encryption status:", error);
        setIsEncryptionEnabled(false);
        setIsUnlocked(false);
      }
    };

    checkEncryptionStatus();
  }, []);

  const unlockApp = async (password: string): Promise<boolean> => {
    try {
      const result = await commands.unlock_app(password);
      setIsUnlocked(result);
      if (result) {
        closePasswordModal();
      }
      return result;
    } catch (error) {
      console.error("Error unlocking app:", error);
      return false;
    }
  };

  const lockApp = async (): Promise<void> => {
    try {
      await commands.lock_app();
      setIsUnlocked(false);
    } catch (error) {
      console.error("Error locking app:", error);
    }
  };

  const openPasswordModal = () => setIsPasswordModalOpen(true);
  const closePasswordModal = () => setIsPasswordModalOpen(false);

  return (
    <EncryptionContext.Provider
      value={{
        isEncryptionEnabled,
        isUnlocked,
        unlockApp,
        lockApp,
        isPasswordModalOpen,
        openPasswordModal,
        closePasswordModal,
      }}
    >
      {children}
    </EncryptionContext.Provider>
  );
}
