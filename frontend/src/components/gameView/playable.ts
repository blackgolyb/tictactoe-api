import {
  getCurrentPlayerImageSize,
  getFieldImageSize,
  store,
} from "../../core/store";
import { gameApi } from "../../services/gameApi";
import type { ImageSize, Point } from "../../services/gameApi";

/**
 * Playable Game View Component
 * Displays an interactive tic-tac-toe game board
 */
export class PlayableGameView extends HTMLElement {
  private container: HTMLDivElement;
  private gameBoard: HTMLDivElement;
  private currentPlayerIndicator: HTMLDivElement;
  private unsubscribers: Array<() => void> = [];

  constructor() {
    super();
    this.attachShadow({ mode: "open" });

    const style = document.createElement("style");
    style.textContent = `
      :host {
        display: block;
        width: 100%;
        padding: 1.5rem;
      }

      .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.5rem;
      }

      .current-player {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        font-size: 1.125rem;
        font-weight: 500;
        color: #374151;
      }

      .current-player img {
        height: 24px;
        width: auto;
      }

      .game-board {
        display: inline-grid;
        gap: 0.5rem;
        padding: 1rem;
        background-color: #f9fafb;
        border-radius: 8px;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
      }

      .field {
        width: 100px;
        height: 100px;
        cursor: pointer;
        border-radius: 4px;
        transition: transform 0.2s, box-shadow 0.2s;
        box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
        background-color: white;
        display: flex;
        align-items: center;
        justify-content: center;
        overflow: hidden;
      }

      .field:hover {
        transform: scale(1.05);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
      }

      .field:active {
        transform: scale(0.98);
      }

      .field img {
        width: 100%;
        height: 100%;
        object-fit: contain;
        pointer-events: none;
      }

      .loading {
        text-align: center;
        padding: 2rem;
        color: #6b7280;
      }

      .error {
        text-align: center;
        padding: 2rem;
        color: #ef4444;
        background-color: #fef2f2;
        border-radius: 8px;
        border: 1px solid #fecaca;
      }

      @media (max-width: 640px) {
        .field {
          width: 80px;
          height: 80px;
        }
      }

      @media (max-width: 480px) {
        .field {
          width: 60px;
          height: 60px;
        }
      }
    `;

    this.container = document.createElement("div");
    this.container.className = "container";

    this.currentPlayerIndicator = document.createElement("div");
    this.currentPlayerIndicator.className = "current-player";

    this.gameBoard = document.createElement("div");
    this.gameBoard.className = "game-board";

    this.container.appendChild(this.currentPlayerIndicator);
    this.container.appendChild(this.gameBoard);

    this.shadowRoot!.appendChild(style);
    this.shadowRoot!.appendChild(this.container);
  }

  connectedCallback(): void {
    this.setupStoreListeners();
    this.render();
  }

  disconnectedCallback(): void {
    this.unsubscribers.forEach((unsub) => unsub());
    this.unsubscribers = [];
  }

  private setupStoreListeners(): void {
    // Listen for game configuration changes
    const unsubGameName = store.onChange("gameName", () => this.render());
    const unsubServerUrl = store.onChange("serverUrl", () => this.render());
    const unsubWidth = store.onChange("width", () => this.render());
    const unsubHeight = store.onChange("height", () => this.render());
    const unsubImageWidth = store.onChange("imageWidth", () => this.render());
    const unsubImageHeight = store.onChange("imageHeight", () => this.render());
    const unsubCurrentPlayerImageWidth = store.onChange(
      "currentPlayerImageWidth",
      () => this.render(),
    );
    const unsubCurrentPlayerImageHeight = store.onChange(
      "currentPlayerImageHeight",
      () => this.render(),
    );

    this.unsubscribers.push(
      unsubGameName,
      unsubServerUrl,
      unsubWidth,
      unsubHeight,
      unsubImageWidth,
      unsubImageHeight,
      unsubCurrentPlayerImageWidth,
      unsubCurrentPlayerImageHeight,
    );
  }

  private async render(): Promise<void> {
    const gameName = store.get<string>("gameName");
    const serverUrl = store.get<string>("serverUrl");
    const width = store.get<number>("width") || 3;
    const height = store.get<number>("height") || 3;
    const fieldImageSize = getFieldImageSize();
    const currentPlayerImageSize = getCurrentPlayerImageSize();

    if (!gameName || !serverUrl) {
      this.showError("Game not configured. Please set up the game first.");
      return;
    }

    // Update API base URL
    gameApi.setBaseUrl(serverUrl);

    try {
      await this.renderCurrentPlayer(gameName, currentPlayerImageSize);
      await this.renderGameBoard(gameName, width, height, fieldImageSize);
    } catch (error) {
      console.error("Failed to render game:", error);
      this.showError("Failed to load game. Please try again.");
    }
  }

  private async renderCurrentPlayer(
    gameName: string,
    imageSize?: ImageSize,
  ): Promise<void> {
    const currentPlayerUrl = gameApi.withTimestamp(
      gameApi.getCurrentPlayerUrl(gameName, imageSize),
    );
    const imageStyle = imageSize
      ? ` style="width: ${imageSize.width}px; height: ${imageSize.height}px;"`
      : "";

    this.currentPlayerIndicator.innerHTML = `
      <span>Current Player:</span>
      <img src="${currentPlayerUrl}"${imageStyle} alt="Current player" />
    `;
  }

  private async renderGameBoard(
    gameName: string,
    width: number,
    height: number,
    imageSize?: ImageSize,
  ): Promise<void> {
    const fieldWidth = imageSize?.width || 100;
    this.gameBoard.style.gridTemplateColumns = `repeat(${width}, ${fieldWidth}px)`;
    this.gameBoard.innerHTML = "";

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const field = this.createFieldElement(gameName, [x, y], imageSize);
        this.gameBoard.appendChild(field);
      }
    }
  }

  private createFieldElement(
    gameName: string,
    point: Point,
    imageSize?: ImageSize,
  ): HTMLDivElement {
    const field = document.createElement("div");
    field.className = "field";
    field.setAttribute("data-x", point[0].toString());
    field.setAttribute("data-y", point[1].toString());
    if (imageSize) {
      field.style.width = `${imageSize.width}px`;
      field.style.height = `${imageSize.height}px`;
    }

    const img = document.createElement("img");
    img.src = gameApi.withTimestamp(
      gameApi.getFieldUrl(gameName, point, imageSize),
    );
    img.alt = `Field ${point[0]},${point[1]}`;
    field.appendChild(img);

    field.addEventListener("click", async () => {
      await this.handleFieldClick(gameName, point);
    });

    return field;
  }

  private async handleFieldClick(
    gameName: string,
    point: Point,
  ): Promise<void> {
    try {
      // Make the move
      await gameApi.makeMove(gameName, point);

      // Reload all field images
      await this.reloadBoard(gameName);
    } catch (error) {
      console.error("Failed to make move:", error);
      this.showError("Failed to make move. Please try again.");
    }
  }

  private async reloadBoard(gameName: string): Promise<void> {
    // Reload current player indicator
    const currentPlayerImg = this.currentPlayerIndicator.querySelector("img");
    const fieldImageSize = getFieldImageSize();
    const currentPlayerImageSize = getCurrentPlayerImageSize();
    if (currentPlayerImg) {
      (currentPlayerImg as HTMLImageElement).src = gameApi.withTimestamp(
        gameApi.getCurrentPlayerUrl(gameName, currentPlayerImageSize),
      );
    }

    // Reload all field images
    const fields = this.gameBoard.querySelectorAll(".field");
    fields.forEach((field) => {
      const x = parseInt(field.getAttribute("data-x") || "0");
      const y = parseInt(field.getAttribute("data-y") || "0");
      const img = field.querySelector("img") as HTMLImageElement;
      if (img) {
        img.src = gameApi.withTimestamp(
          gameApi.getFieldUrl(gameName, [x, y], fieldImageSize),
        );
      }
    });
  }

  private showError(message: string): void {
    this.container.innerHTML = `<div class="error">${message}</div>`;
  }
}

customElements.define("playable-game-view", PlayableGameView);
