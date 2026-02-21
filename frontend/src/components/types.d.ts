import { ModalWindow } from "./modal-window";
import { CreateGameForm, CreateGameFormData } from "./game-form";
import { SelectGameForm, SelectGameFormData } from "./select-game-form";
import { GameSelector } from "./game-selector";
import { TabContainer, TabButton, TabPanel } from "./tab-container";
import { PlayableGameView } from "./gameView/playable";
import { MarkdownGameView } from "./gameView/md";
import { HtmlGameView } from "./gameView/html";

declare global {
  interface HTMLElementTagNameMap {
    "modal-window": ModalWindow;
    "create-game-form": CreateGameForm;
    "select-game-form": SelectGameForm;
    "game-selector": GameSelector;
    "tab-container": TabContainer;
    "tab-button": TabButton;
    "tab-panel": TabPanel;
    "playable-game-view": PlayableGameView;
    "markdown-game-view": MarkdownGameView;
    "html-game-view": HtmlGameView;
  }

  interface HTMLElementEventMap {
    "modal-open": CustomEvent<void>;
    "modal-close": CustomEvent<void>;
    "game-create": CustomEvent<CreateGameFormData>;
    "game-create-cancel": CustomEvent<void>;
    "game-select": CustomEvent<SelectGameFormData>;
    "game-select-cancel": CustomEvent<void>;
    "open-create-game": CustomEvent<void>;
    "tab-change": CustomEvent<{ tabId: string }>;
    "tab-select": CustomEvent<string>;
  }
}

export {};
