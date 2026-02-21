import { store } from "../core/store";
import { gameApi } from "../services/gameApi";

export interface CreateGameFormData {
  serverURL: string;
  gameName: string;
  width: number;
  height: number;
  winning_length: number;
  first_player: "X" | "O";
}

export class CreateGameForm extends HTMLElement {
  private form: HTMLFormElement;

  constructor() {
    super();
    this.attachShadow({ mode: "open" });

    // Create form structure
    this.form = document.createElement("form");
    this.form.className = "create-game-form";

    const style = document.createElement("style");
    style.textContent = `
      :host {
        display: block;
      }

      .create-game-form {
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
      }

      .form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
      }

      .form-group label {
        font-weight: 500;
        color: #374151;
        font-size: 0.875rem;
      }

      .form-group input {
        padding: 0.625rem;
        border: 1px solid #d1d5db;
        border-radius: 6px;
        font-size: 1rem;
        transition: border-color 0.2s, box-shadow 0.2s;
      }

      .form-group input:focus {
        outline: none;
        border-color: #3b82f6;
        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
      }

      .form-group input:invalid:not(:placeholder-shown) {
        border-color: #ef4444;
      }

      .form-group small {
        color: #6b7280;
        font-size: 0.75rem;
      }

      .form-row {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1rem;
      }

      .button-group {
        display: flex;
        gap: 0.75rem;
        margin-top: 0.5rem;
      }

      button {
        padding: 0.625rem 1.25rem;
        border-radius: 6px;
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        border: none;
      }

      .btn-primary {
        background-color: #3b82f6;
        color: white;
        flex: 1;
      }

      .btn-primary:hover {
        background-color: #2563eb;
      }

      .btn-primary:active {
        transform: scale(0.98);
      }

      .btn-secondary {
        background-color: #f3f4f6;
        color: #374151;
        flex: 1;
      }

      .btn-secondary:hover {
        background-color: #e5e7eb;
      }

      .error-message {
        color: #ef4444;
        font-size: 0.875rem;
        padding: 0.75rem;
        background-color: #fef2f2;
        border-radius: 6px;
        border: 1px solid #fecaca;
        display: none;
      }

      .error-message.show {
        display: block;
      }

      @media (max-width: 640px) {
        .form-row {
          grid-template-columns: 1fr;
        }
      }
    `;

    this.form.innerHTML = `
      <div class="error-message" id="error-message"></div>

      <div class="form-group">
        <label for="serverURL">Server URL *</label>
        <input
          type="url"
          id="serverURL"
          name="serverURL"
          placeholder="http://localhost:8128"
          required
        />
        <small>The base URL of your Tic Tac Toe API server</small>
      </div>

      <div class="form-group">
        <label for="gameName">Game Name *</label>
        <input
          type="text"
          id="gameName"
          name="gameName"
          placeholder="my-game-room"
          required
          pattern="[a-zA-Z0-9_-]+"
        />
        <small>Unique identifier for your game room (alphanumeric, _, -)</small>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label for="width">Width *</label>
          <input
            type="number"
            id="width"
            name="width"
            min="3"
            max="10"
            value="3"
            required
          />
          <small>Grid width (3-10)</small>
        </div>

        <div class="form-group">
          <label for="height">Height *</label>
          <input
            type="number"
            id="height"
            name="height"
            min="3"
            max="10"
            value="3"
            required
          />
          <small>Grid height (3-10)</small>
        </div>

        <div class="form-group">
          <label for="winning_length">Winning Length *</label>
          <input
            type="number"
            id="winning_length"
            name="winning_length"
            min="3"
            max="10"
            value="3"
            required
          />
          <small>Number in a row to win (3-10)</small>
        </div>
      </div>

      <div class="form-group">
        <label for="first_player">First Player *</label>
        <select id="first_player" name="first_player" required>
          <option value="X">X</option>
          <option value="O">O</option>
        </select>
        <small>Which player starts the game</small>
      </div>

      <div class="button-group">
        <button type="submit" class="btn-primary">Create Game</button>
        <button type="button" class="btn-secondary" id="cancel-btn">Cancel</button>
      </div>
    `;

    this.shadowRoot!.appendChild(style);
    this.shadowRoot!.appendChild(this.form);

    // Event listeners
    this.form.addEventListener("submit", (e) => this.handleSubmit(e));

    const cancelBtn = this.form.querySelector("#cancel-btn");
    cancelBtn?.addEventListener("click", () => this.handleCancel());
  }

  connectedCallback(): void {
    this.populateForm();
  }

  disconnectedCallback(): void {
    // Cleanup if needed
  }

  private populateForm(): void {
    const serverURLInput = this.form.querySelector(
      "#serverURL",
    ) as HTMLInputElement;
    const gameNameInput = this.form.querySelector(
      "#gameName",
    ) as HTMLInputElement;

    if (serverURLInput) {
      serverURLInput.value =
        store.get<string>("serverUrl") || window.location.origin;
    }
    if (gameNameInput) {
      gameNameInput.value = "";
    }
  }

  private async handleSubmit(e: Event): Promise<void> {
    e.preventDefault();

    if (!this.form.checkValidity()) {
      this.showError("Please fill in all required fields correctly.");
      return;
    }

    const formData = new FormData(this.form);
    const data: CreateGameFormData = {
      serverURL: formData.get("serverURL") as string,
      gameName: formData.get("gameName") as string,
      width: parseInt(formData.get("width") as string),
      height: parseInt(formData.get("height") as string),
      winning_length: parseInt(formData.get("winning_length") as string),
      first_player: formData.get("first_player") as "X" | "O",
    };

    // Validate dimensions
    if (
      data.height < 3 ||
      data.height > 10 ||
      data.width < 3 ||
      data.width > 10 ||
      data.winning_length < 3 ||
      data.winning_length > 10
    ) {
      this.showError("All dimensions must be between 3 and 10.");
      return;
    }

    // Validate winning length doesn't exceed dimensions
    if (data.winning_length > Math.max(data.width, data.height)) {
      this.showError("Winning length cannot exceed grid dimensions.");
      return;
    }

    this.hideError();

    // Update game API base URL
    gameApi.setBaseUrl(data.serverURL);

    // Create the game on the server
    try {
      await gameApi.createOrUpdateGame(data.gameName, {
        width: data.width,
        height: data.height,
        winning_length: data.winning_length,
        first_player: data.first_player,
      });

      // Update store - this will trigger effects to fetch rules
      store.set("serverUrl", data.serverURL);
      store.set("gameName", data.gameName);

      // Dispatch success event
      this.dispatchEvent(
        new CustomEvent("game-create", {
          detail: data,
          bubbles: true,
          composed: true,
        }),
      );

      // Reset form
      this.reset();
    } catch (error) {
      console.error("Failed to create game:", error);
      this.showError(
        "Failed to create game. Please check your settings and try again.",
      );
    }
  }

  private handleCancel(): void {
    this.dispatchEvent(
      new CustomEvent("game-create-cancel", {
        bubbles: true,
        composed: true,
      }),
    );
  }

  private showError(message: string): void {
    const errorElement = this.form.querySelector(
      "#error-message",
    ) as HTMLElement;
    if (errorElement) {
      errorElement.textContent = message;
      errorElement.classList.add("show");
    }
  }

  private hideError(): void {
    const errorElement = this.form.querySelector(
      "#error-message",
    ) as HTMLElement;
    if (errorElement) {
      errorElement.classList.remove("show");
    }
  }

  reset(): void {
    this.form.reset();
    this.hideError();
  }
}

customElements.define("create-game-form", CreateGameForm);
