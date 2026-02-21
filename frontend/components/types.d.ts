import { ModalWindow } from "./modal-window";
import { GameForm, GameFormData } from "./game-form";
import { TabContainer, TabButton, TabPanel } from "./tab-container";

declare global {
  interface HTMLElementTagNameMap {
    "modal-window": ModalWindow;
    "game-form": GameForm;
    "tab-container": TabContainer;
    "tab-button": TabButton;
    "tab-panel": TabPanel;
  }

  interface HTMLElementEventMap {
    "modal-open": CustomEvent<void>;
    "modal-close": CustomEvent<void>;
    "game-submit": CustomEvent<GameFormData>;
    "game-cancel": CustomEvent<void>;
    "tab-change": CustomEvent<{ tabId: string }>;
    "tab-select": CustomEvent<string>;
  }
}

export {};
