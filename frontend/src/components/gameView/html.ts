import { store } from "../../core/store";
import { gameApi } from "../../services/gameApi";
import type { Point } from "../../services/gameApi";

/**
 * HTML Game View Component
 * Displays embeddable HTML code for the game
 */
export class HtmlGameView extends HTMLElement {
  private container: HTMLDivElement;
  private codeBlock: HTMLPreElement;
  private copyButton: HTMLButtonElement;
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
        gap: 1rem;
      }

      .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
      }

      .title {
        font-size: 1.125rem;
        font-weight: 600;
        color: #374151;
      }

      .copy-button {
        padding: 0.5rem 1rem;
        background-color: #3b82f6;
        color: white;
        border: none;
        border-radius: 6px;
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        transition: background-color 0.2s;
      }

      .copy-button:hover {
        background-color: #2563eb;
      }

      .copy-button:active {
        transform: scale(0.98);
      }

      .copy-button.copied {
        background-color: #10b981;
      }

      .code-container {
        position: relative;
        background-color: #1f2937;
        border-radius: 8px;
        overflow: hidden;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
      }

      pre {
        margin: 0;
        padding: 1.5rem;
        overflow-x: auto;
        color: #e5e7eb;
        font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
        font-size: 0.875rem;
        line-height: 1.5;
      }

      code {
        color: #e5e7eb;
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

      .info {
        padding: 1rem;
        background-color: #eff6ff;
        border-left: 4px solid #3b82f6;
        border-radius: 4px;
        color: #1e40af;
        font-size: 0.875rem;
      }

      @media (max-width: 640px) {
        pre {
          font-size: 0.75rem;
          padding: 1rem;
        }
      }
    `;

    this.container = document.createElement("div");
    this.container.className = "container";

    const header = document.createElement("div");
    header.className = "header";

    const title = document.createElement("div");
    title.className = "title";
    title.textContent = "HTML Embed Code";

    this.copyButton = document.createElement("button");
    this.copyButton.className = "copy-button";
    this.copyButton.textContent = "Copy to Clipboard";
    this.copyButton.addEventListener("click", () => this.copyToClipboard());

    header.appendChild(title);
    header.appendChild(this.copyButton);

    const info = document.createElement("div");
    info.className = "info";
    info.innerHTML = `
      Copy this HTML code to embed the game in any website or HTML document.
    `;

    const codeContainer = document.createElement("div");
    codeContainer.className = "code-container";

    this.codeBlock = document.createElement("pre");
    this.codeBlock.innerHTML = "<code>Loading...</code>";
    codeContainer.appendChild(this.codeBlock);

    this.container.appendChild(header);
    this.container.appendChild(info);
    this.container.appendChild(codeContainer);

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
    const unsubGameName = store.onChange("gameName", () => this.render());
    const unsubServerUrl = store.onChange("serverUrl", () => this.render());
    const unsubWidth = store.onChange("width", () => this.render());
    const unsubHeight = store.onChange("height", () => this.render());

    this.unsubscribers.push(
      unsubGameName,
      unsubServerUrl,
      unsubWidth,
      unsubHeight,
    );
  }

  private async render(): Promise<void> {
    const gameName = store.get<string>("gameName");
    const serverUrl = store.get<string>("serverUrl");
    const width = store.get<number>("width") || 3;
    const height = store.get<number>("height") || 3;

    if (!gameName || !serverUrl) {
      this.showError("Game not configured. Please set up the game first.");
      return;
    }

    gameApi.setBaseUrl(serverUrl);

    try {
      const html = this.generateHtml(gameName, width, height);
      this.displayHtml(html);
    } catch (error) {
      console.error("Failed to generate HTML:", error);
      this.showError("Failed to generate HTML code.");
    }
  }

  private generateHtml(
    gameName: string,
    width: number,
    height: number,
  ): string {
    const currentPlayerUrl = gameApi.getCurrentPlayerUrl(gameName);

    // Generate table rows
    const rows: string[] = [];
    for (let y = 0; y < height; y++) {
      const cells: string[] = [];
      for (let x = 0; x < width; x++) {
        const point: Point = [x, y];
        const fieldUrl = gameApi.getFieldUrl(gameName, point);
        const makeMoveUrl = gameApi.getMakeMoveUrl(gameName, point);
        cells.push(`      <td>
        <a href="${makeMoveUrl}">
          <img src="${fieldUrl}" width="100" alt="Field ${x},${y}" />
        </a>
      </td>`);
      }
      rows.push(`    <tr>
${cells.join("\n")}
    </tr>`);
    }

    return `<div class="tic-tac-toe-game">
  <div class="current-player">
    <span>Current Player: </span>
    <img src="${currentPlayerUrl}" height="24" alt="Current player" />
  </div>
  <table class="game-board">
    <tbody>
${rows.join("\n")}
    </tbody>
  </table>
  <style>
    .tic-tac-toe-game {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 1rem;
      padding: 1rem;
    }
    .tic-tac-toe-game .current-player {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      font-weight: 500;
    }
    .tic-tac-toe-game .game-board {
      border-collapse: separate;
      border-spacing: 0.5rem;
    }
    .tic-tac-toe-game .game-board td {
      padding: 0;
    }
    .tic-tac-toe-game .game-board img {
      display: block;
      border-radius: 4px;
      transition: transform 0.2s;
    }
    .tic-tac-toe-game .game-board a:hover img {
      transform: scale(1.05);
    }
  </style>
</div>`;
  }

  private displayHtml(html: string): void {
    const escapedHtml = this.escapeHtml(html);
    this.codeBlock.innerHTML = `<code>${escapedHtml}</code>`;
  }

  private escapeHtml(text: string): string {
    const div = document.createElement("div");
    div.textContent = text;
    return div.innerHTML;
  }

  private async copyToClipboard(): Promise<void> {
    const code = this.codeBlock.textContent || "";

    try {
      await navigator.clipboard.writeText(code);
      this.copyButton.textContent = "Copied!";
      this.copyButton.classList.add("copied");

      setTimeout(() => {
        this.copyButton.textContent = "Copy to Clipboard";
        this.copyButton.classList.remove("copied");
      }, 2000);
    } catch (error) {
      console.error("Failed to copy to clipboard:", error);
      // Fallback for older browsers
      this.fallbackCopyToClipboard(code);
    }
  }

  private fallbackCopyToClipboard(text: string): void {
    const textarea = document.createElement("textarea");
    textarea.value = text;
    textarea.style.position = "fixed";
    textarea.style.opacity = "0";
    document.body.appendChild(textarea);
    textarea.select();

    try {
      document.execCommand("copy");
      this.copyButton.textContent = "Copied!";
      this.copyButton.classList.add("copied");

      setTimeout(() => {
        this.copyButton.textContent = "Copy to Clipboard";
        this.copyButton.classList.remove("copied");
      }, 2000);
    } catch (error) {
      console.error("Fallback copy failed:", error);
      alert("Failed to copy to clipboard");
    } finally {
      document.body.removeChild(textarea);
    }
  }

  private showError(message: string): void {
    this.codeBlock.innerHTML = `<code class="error">${this.escapeHtml(message)}</code>`;
  }
}

customElements.define("html-game-view", HtmlGameView);
