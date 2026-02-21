import { GameAPI } from "../main";

export interface GameFormData {
  serverURL: string;
  room: string;
  redirect: string;
  height: number;
  width: number;
}

export class GameForm extends HTMLElement {
  private form: HTMLFormElement;
  private gameAPI?: GameAPI;

  constructor() {
    super();
    this.attachShadow({ mode: "open" });

    // Create form structure
    this.form = document.createElement("form");
    this.form.className = "game-form";

    const style = document.createElement("style");
    style.textContent = `
      :host {
        display: block;
      }

      .game-form {
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
        <label for="room">Room Name *</label>
        <input
          type="text"
          id="room"
          name="room"
          placeholder="my-game-room"
          required
          pattern="[a-zA-Z0-9_-]+"
        />
        <small>Unique identifier for your game room (alphanumeric, _, -)</small>
      </div>

      <div class="form-group">
        <label for="redirect">Redirect URL</label>
        <input
          type="url"
          id="redirect"
          name="redirect"
          placeholder="https://example.com"
        />
        <small>Optional: Where to redirect after game action</small>
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

  setGameAPI(api: GameAPI): void {
    this.gameAPI = api;
    this.populateForm();
  }

  private populateForm(): void {
    if (!this.gameAPI) return;

    const serverURLInput = this.form.querySelector(
      "#serverURL",
    ) as HTMLInputElement;
    const roomInput = this.form.querySelector("#room") as HTMLInputElement;
    const redirectInput = this.form.querySelector(
      "#redirect",
    ) as HTMLInputElement;

    if (serverURLInput) serverURLInput.value = this.gameAPI.base;
    if (roomInput) roomInput.value = this.gameAPI.room;
    if (redirectInput) redirectInput.value = this.gameAPI.backref || "";
  }

  private handleSubmit(e: Event): void {
    e.preventDefault();

    if (!this.form.checkValidity()) {
      this.showError("Please fill in all required fields correctly.");
      return;
    }

    const formData = new FormData(this.form);
    const data: GameFormData = {
      serverURL: formData.get("serverURL") as string,
      room: formData.get("room") as string,
      redirect: formData.get("redirect") as string,
      height: parseInt(formData.get("height") as string),
      width: parseInt(formData.get("width") as string),
    };

    // Validate dimensions
    if (
      data.height < 3 ||
      data.height > 10 ||
      data.width < 3 ||
      data.width > 10
    ) {
      this.showError("Width and height must be between 3 and 10.");
      return;
    }

    this.hideError();
    this.dispatchEvent(
      new CustomEvent("game-submit", {
        detail: data,
        bubbles: true,
        composed: true,
      }),
    );
  }

  private handleCancel(): void {
    this.dispatchEvent(
      new CustomEvent("game-cancel", {
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

customElements.define("game-form", GameForm);
