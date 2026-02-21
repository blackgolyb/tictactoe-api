import { ModalWindow } from "./modal-window";
import { GameForm, GameFormData } from "./game-form";

declare global {
  interface HTMLElementTagNameMap {
    "modal-window": ModalWindow;
    "game-form": GameForm;
  }

  interface HTMLElementEventMap {
    "modal-open": CustomEvent<void>;
    "modal-close": CustomEvent<void>;
    "game-submit": CustomEvent<GameFormData>;
    "game-cancel": CustomEvent<void>;
  }
}

export {};
