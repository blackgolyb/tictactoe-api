import { store } from "../core/store";

export interface SelectGameFormData {
  serverURL: string;
  gameName: string;
}

/**
 * Select Game Form Component
 * Simple form to select/switch between existing games
 */
export class SelectGameForm extends HTMLElement {
  private form: HTMLFormElement;

  constructor() {
    super();
    this.attachShadow({ mode: "open" });

    const style = document.createElement("style");
    style.textContent = `
      :host {
        display: block;
      }

      .select-game-form {
        display: flex;
        flex-direction: column;
        gap: 1rem;
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
    `;

    this.form = document.createElement("form");
    this.form.className = "select-game-form";

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
          placeholder="default"
          required
          pattern="[a-zA-Z0-9_-]+"
        />
        <small>Name of the game room (alphanumeric, _, -)</small>
      </div>

      <div class="button-group">
        <button type="submit" class="btn-primary">Select Game</button>
        <button type="button" class="btn-secondary" id="cancel-btn">Cancel</button>
      </div>
    `;

    this.shadowRoot!.appendChild(style);
    this.shadowRoot!.appendChild(this.form);

    this.form.addEventListener("submit", (e) => this.handleSubmit(e));

    const cancelBtn = this.form.querySelector("#cancel-btn");
    cancelBtn?.addEventListener("click", () => this.handleCancel());
  }

  connectedCallback(): void {
    this.populateForm();
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
      gameNameInput.value = store.get<string>("gameName") || "default";
    }
  }

  private handleSubmit(e: Event): void {
    e.preventDefault();

    if (!this.form.checkValidity()) {
      this.showError("Please fill in all required fields correctly.");
      return;
    }

    const formData = new FormData(this.form);
    const data: SelectGameFormData = {
      serverURL: formData.get("serverURL") as string,
      gameName: formData.get("gameName") as string,
    };

    this.hideError();
    this.dispatchEvent(
      new CustomEvent("game-select", {
        detail: data,
        bubbles: true,
        composed: true,
      }),
    );
  }

  private handleCancel(): void {
    this.dispatchEvent(
      new CustomEvent("game-select-cancel", {
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

customElements.define("select-game-form", SelectGameForm);
