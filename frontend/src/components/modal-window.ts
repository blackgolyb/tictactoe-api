export class ModalWindow extends HTMLElement {
  private isOpen: boolean = false;
  private overlay: HTMLDivElement;
  private dialog: HTMLDivElement;
  private closeButton: HTMLButtonElement;

  constructor() {
    super();
    this.attachShadow({ mode: "open" });

    // Create modal structure
    this.overlay = document.createElement("div");
    this.overlay.className = "modal-overlay";

    this.dialog = document.createElement("div");
    this.dialog.className = "modal-dialog";

    const header = document.createElement("div");
    header.className = "modal-header";

    const titleSlot = document.createElement("slot");
    titleSlot.name = "title";
    header.appendChild(titleSlot);

    this.closeButton = document.createElement("button");
    this.closeButton.className = "modal-close";
    this.closeButton.innerHTML = "&times;";
    this.closeButton.setAttribute("aria-label", "Close modal");
    header.appendChild(this.closeButton);

    const content = document.createElement("div");
    content.className = "modal-content";
    const contentSlot = document.createElement("slot");
    content.appendChild(contentSlot);

    this.dialog.appendChild(header);
    this.dialog.appendChild(content);
    this.overlay.appendChild(this.dialog);

    // Add styles
    const style = document.createElement("style");
    style.textContent = `
      :host {
        display: none;
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: 1000;
      }

      :host([open]) {
        display: block;
      }

      .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        animation: fadeIn 0.2s ease-out;
      }

      .modal-dialog {
        background: white;
        border-radius: 8px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1), 0 10px 20px rgba(0, 0, 0, 0.15);
        max-width: 500px;
        width: 90%;
        max-height: 90vh;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        animation: slideIn 0.3s ease-out;
      }

      .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem;
        border-bottom: 1px solid #e5e7eb;
      }

      .modal-header slot[name="title"] {
        font-size: 1.25rem;
        font-weight: 600;
        color: #111827;
      }

      .modal-close {
        background: none;
        border: none;
        font-size: 2rem;
        line-height: 1;
        color: #6b7280;
        cursor: pointer;
        padding: 0;
        width: 2rem;
        height: 2rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 4px;
        transition: background-color 0.2s, color 0.2s;
      }

      .modal-close:hover {
        background-color: #f3f4f6;
        color: #111827;
      }

      .modal-content {
        padding: 1.5rem;
        overflow-y: auto;
      }

      @keyframes fadeIn {
        from {
          opacity: 0;
        }
        to {
          opacity: 1;
        }
      }

      @keyframes slideIn {
        from {
          transform: translateY(-20px);
          opacity: 0;
        }
        to {
          transform: translateY(0);
          opacity: 1;
        }
      }
    `;

    this.shadowRoot!.appendChild(style);
    this.shadowRoot!.appendChild(this.overlay);

    // Event listeners
    this.closeButton.addEventListener("click", () => this.close());
    this.overlay.addEventListener("click", (e) => {
      if (e.target === this.overlay) {
        this.close();
      }
    });

    // Handle ESC key
    document.addEventListener("keydown", (e) => {
      if (e.key === "Escape" && this.isOpen) {
        this.close();
      }
    });
  }

  open(): void {
    this.isOpen = true;
    this.setAttribute("open", "");
    this.dispatchEvent(new CustomEvent("modal-open", { bubbles: true }));
    document.body.style.overflow = "hidden";
  }

  close(): void {
    this.isOpen = false;
    this.removeAttribute("open");
    this.dispatchEvent(new CustomEvent("modal-close", { bubbles: true }));
    document.body.style.overflow = "";
  }

  toggle(): void {
    if (this.isOpen) {
      this.close();
    } else {
      this.open();
    }
  }

  static get observedAttributes(): string[] {
    return ["open"];
  }

  attributeChangedCallback(
    name: string,
    _oldValue: string,
    newValue: string,
  ): void {
    if (name === "open") {
      this.isOpen = newValue !== null;
      if (this.isOpen) {
        document.body.style.overflow = "hidden";
      } else {
        document.body.style.overflow = "";
      }
    }
  }

  disconnectedCallback(): void {
    document.body.style.overflow = "";
  }
}

customElements.define("modal-window", ModalWindow);
