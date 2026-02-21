import { store } from "../core/store";
import { gameApi } from "../services/gameApi";

/**
 * Game Selector Component
 * Inline selector for switching between games (no modal)
 * Self-contained: handles store updates internally
 */
export class GameSelector extends HTMLElement {
  private serverUrlInput: HTMLInputElement;
  private gameNameInput: HTMLInputElement;
  private redirectUrlInput: HTMLInputElement;
  private unsubscribers: Array<() => void> = [];

  constructor() {
    super();
    this.attachShadow({ mode: "open" });

    const style = document.createElement("style");
    style.textContent = `
      :host {
        display: block;
        width: 100%;
      }

      .game-selector {
        display: flex;
        gap: 0.75rem;
        align-items: end;
        padding: 1rem;
        background-color: #f9fafb;
        border-radius: 8px;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
      }

      .field-group {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        flex: 1;
      }

      .field-group label {
        font-size: 0.75rem;
        font-weight: 500;
        color: #6b7280;
        text-transform: uppercase;
        letter-spacing: 0.025em;
      }

      .field-group input {
        padding: 0.5rem 0.75rem;
        border: 1px solid #d1d5db;
        border-radius: 6px;
        font-size: 0.875rem;
        background-color: white;
        transition: border-color 0.2s, box-shadow 0.2s;
      }

      .field-group input:focus {
        outline: none;
        border-color: #3b82f6;
        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
      }

      .field-group input:invalid {
        border-color: #ef4444;
      }

      .button-group {
        display: flex;
        gap: 0.5rem;
      }

      button {
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        border: none;
        white-space: nowrap;
      }

      .btn-primary {
        background-color: #3b82f6;
        color: white;
      }

      .btn-primary:hover {
        background-color: #2563eb;
      }

      .btn-primary:active {
        transform: scale(0.98);
      }

      .btn-secondary {
        background-color: white;
        color: #374151;
        border: 1px solid #d1d5db;
      }

      .btn-secondary:hover {
        background-color: #f9fafb;
      }

      @media (max-width: 768px) {
        .game-selector {
          flex-direction: column;
          align-items: stretch;
        }

        .button-group {
          flex-direction: column;
        }
      }
    `;

    const container = document.createElement("div");
    container.className = "game-selector";

    // Redirect URL field
    const redirectUrlGroup = document.createElement("div");
    redirectUrlGroup.className = "field-group";

    const redirectUrlLabel = document.createElement("label");
    redirectUrlLabel.textContent = "Redirect URL";
    redirectUrlLabel.htmlFor = "redirectUrl";

    this.redirectUrlInput = document.createElement("input");
    this.redirectUrlInput.type = "url";
    this.redirectUrlInput.id = "redirectUrl";
    this.redirectUrlInput.placeholder = "https://github.com/user/repo";
    this.redirectUrlInput.required = true;

    redirectUrlGroup.appendChild(redirectUrlLabel);
    redirectUrlGroup.appendChild(this.redirectUrlInput);

    // Server URL field
    const serverUrlGroup = document.createElement("div");
    serverUrlGroup.className = "field-group";

    const serverUrlLabel = document.createElement("label");
    serverUrlLabel.textContent = "Server URL";
    serverUrlLabel.htmlFor = "serverUrl";

    this.serverUrlInput = document.createElement("input");
    this.serverUrlInput.type = "url";
    this.serverUrlInput.id = "serverUrl";
    this.serverUrlInput.placeholder = "http://localhost:8128";
    this.serverUrlInput.required = true;

    serverUrlGroup.appendChild(serverUrlLabel);
    serverUrlGroup.appendChild(this.serverUrlInput);

    // Game name field
    const gameNameGroup = document.createElement("div");
    gameNameGroup.className = "field-group";

    const gameNameLabel = document.createElement("label");
    gameNameLabel.textContent = "Game Name";
    gameNameLabel.htmlFor = "gameName";

    this.gameNameInput = document.createElement("input");
    this.gameNameInput.type = "text";
    this.gameNameInput.id = "gameName";
    this.gameNameInput.placeholder = "default";
    this.gameNameInput.pattern = "[a-zA-Z0-9_-]+";
    this.gameNameInput.required = true;

    gameNameGroup.appendChild(gameNameLabel);
    gameNameGroup.appendChild(this.gameNameInput);

    // Button group
    const buttonGroup = document.createElement("div");
    buttonGroup.className = "button-group";

    const selectButton = document.createElement("button");
    selectButton.className = "btn-primary";
    selectButton.textContent = "Load Game";
    selectButton.addEventListener("click", () => this.handleSelect());

    const createButton = document.createElement("button");
    createButton.className = "btn-secondary";
    createButton.textContent = "Create New";
    createButton.addEventListener("click", () => this.handleCreate());

    buttonGroup.appendChild(selectButton);
    buttonGroup.appendChild(createButton);

    container.appendChild(redirectUrlGroup);
    container.appendChild(serverUrlGroup);
    container.appendChild(gameNameGroup);
    container.appendChild(buttonGroup);

    this.shadowRoot!.appendChild(style);
    this.shadowRoot!.appendChild(container);

    // Handle Enter key
    this.redirectUrlInput.addEventListener("keypress", (e) => {
      if (e.key === "Enter") {
        e.preventDefault();
        this.handleSelect();
      }
    });

    this.serverUrlInput.addEventListener("keypress", (e) => {
      if (e.key === "Enter") {
        e.preventDefault();
        this.handleSelect();
      }
    });

    this.gameNameInput.addEventListener("keypress", (e) => {
      if (e.key === "Enter") {
        e.preventDefault();
        this.handleSelect();
      }
    });
  }

  connectedCallback(): void {
    this.loadFromStore();
    this.setupStoreListeners();
  }

  disconnectedCallback(): void {
    this.unsubscribers.forEach((unsub) => unsub());
    this.unsubscribers = [];
  }

  private setupStoreListeners(): void {
    // Update inputs when store changes
    const unsubRedirectUrl = store.onChange("redirectUrl", (value) => {
      if (value && this.redirectUrlInput.value !== value) {
        this.redirectUrlInput.value = value;
      }
    });

    const unsubServerUrl = store.onChange("serverUrl", (value) => {
      if (value && this.serverUrlInput.value !== value) {
        this.serverUrlInput.value = value;
      }
    });

    const unsubGameName = store.onChange("gameName", (value) => {
      if (value && this.gameNameInput.value !== value) {
        this.gameNameInput.value = value;
      }
    });

    this.unsubscribers.push(unsubRedirectUrl, unsubServerUrl, unsubGameName);
  }

  private loadFromStore(): void {
    const redirectUrl = store.get<string>("redirectUrl");
    const serverUrl = store.get<string>("serverUrl");
    const gameName = store.get<string>("gameName");

    if (redirectUrl) {
      this.redirectUrlInput.value = redirectUrl;
    } else {
      this.redirectUrlInput.value = window.location.href;
    }

    if (serverUrl) {
      this.serverUrlInput.value = serverUrl;
    } else {
      this.serverUrlInput.value = window.location.origin;
    }

    if (gameName) {
      this.gameNameInput.value = gameName;
    } else {
      this.gameNameInput.value = "default";
    }
  }

  private handleSelect(): void {
    if (
      !this.redirectUrlInput.checkValidity() ||
      !this.serverUrlInput.checkValidity() ||
      !this.gameNameInput.checkValidity()
    ) {
      this.redirectUrlInput.reportValidity();
      this.serverUrlInput.reportValidity();
      this.gameNameInput.reportValidity();
      return;
    }

    const redirectUrl = this.redirectUrlInput.value;
    const serverUrl = this.serverUrlInput.value;
    const gameName = this.gameNameInput.value;

    // Update store directly - this will trigger effects in main.ts
    store.set("redirectUrl", redirectUrl);
    store.set("serverUrl", serverUrl);
    store.set("gameName", gameName);

    // Update game API base URL
    gameApi.setBaseUrl(serverUrl);

    // Dispatch event for UI feedback (optional)
    this.dispatchEvent(
      new CustomEvent("game-select", {
        detail: {
          redirectUrl: redirectUrl,
          serverURL: serverUrl,
          gameName: gameName,
        },
        bubbles: true,
        composed: true,
      }),
    );
  }

  private handleCreate(): void {
    this.dispatchEvent(
      new CustomEvent("open-create-game", {
        bubbles: true,
        composed: true,
      }),
    );
  }
}

customElements.define("game-selector", GameSelector);
