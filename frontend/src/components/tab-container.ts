export class TabContainer extends HTMLElement {
  private selectedTab: string | null = null;
  private tabButtons: Map<string, TabButton> = new Map();
  private tabPanels: Map<string, TabPanel> = new Map();
  private tabList: HTMLDivElement;
  private panelContainer: HTMLDivElement;

  constructor() {
    super();
    this.attachShadow({ mode: "open" });

    const style = document.createElement("style");
    style.textContent = `
      :host {
        display: block;
        width: 100%;
      }

      .tab-container {
        display: flex;
        flex-direction: column;
        width: 100%;
      }

      .tab-list {
        display: flex;
        gap: 0.25rem;
        border-bottom: 2px solid #e5e7eb;
        background-color: #f9fafb;
        padding: 0.5rem 0.5rem 0;
      }

      .panel-container {
        position: relative;
        width: 100%;
        overflow: hidden;
      }

      ::slotted(tab-button) {
        flex-shrink: 0;
      }

      ::slotted(tab-panel) {
        width: 100%;
      }
    `;

    const container = document.createElement("div");
    container.className = "tab-container";

    this.tabList = document.createElement("div");
    this.tabList.className = "tab-list";
    const tabSlot = document.createElement("slot");
    tabSlot.name = "tabs";
    this.tabList.appendChild(tabSlot);

    this.panelContainer = document.createElement("div");
    this.panelContainer.className = "panel-container";
    const panelSlot = document.createElement("slot");
    panelSlot.name = "panels";
    this.panelContainer.appendChild(panelSlot);

    container.appendChild(this.tabList);
    container.appendChild(this.panelContainer);

    this.shadowRoot!.appendChild(style);
    this.shadowRoot!.appendChild(container);
  }

  connectedCallback(): void {
    // Use setTimeout to ensure all child elements are fully registered
    setTimeout(() => {
      this.setupTabs();
      this.selectInitialTab();
    }, 0);
  }

  private setupTabs(): void {
    // Find all tab buttons and panels
    const tabButtons = Array.from(
      this.querySelectorAll("tab-button"),
    ) as TabButton[];
    const tabPanels = Array.from(
      this.querySelectorAll("tab-panel"),
    ) as TabPanel[];

    // Register tab buttons
    tabButtons.forEach((button) => {
      const tabId = button.getAttribute("for");
      if (tabId) {
        this.tabButtons.set(tabId, button);
        button.addEventListener("tab-select", (e) => {
          const event = e as CustomEvent<string>;
          this.selectTab(event.detail);
        });
      }
    });

    // Register tab panels
    tabPanels.forEach((panel) => {
      const tabId = panel.getAttribute("tab-id");
      if (tabId) {
        this.tabPanels.set(tabId, panel);
      }
    });
  }

  private selectInitialTab(): void {
    // Find the first tab with 'selected' attribute
    const tabButtonsArray = Array.from(this.querySelectorAll("tab-button"));
    const firstSelectedButton = tabButtonsArray.find((btn) =>
      btn.hasAttribute("selected"),
    );

    if (firstSelectedButton) {
      const tabId = firstSelectedButton.getAttribute("for");
      if (tabId) {
        this.selectTab(tabId);
        return;
      }
    }

    // Fallback to first tab
    if (this.tabButtons.size > 0) {
      const firstTabId = Array.from(this.tabButtons.keys())[0];
      this.selectTab(firstTabId);
    }
  }

  selectTab(tabId: string): void {
    if (this.selectedTab === tabId) return;

    const button = this.tabButtons.get(tabId);
    const panel = this.tabPanels.get(tabId);

    if (!button || !panel) {
      console.warn(`Tab "${tabId}" not found`);
      return;
    }

    // Get all tab IDs in order
    const tabIds = Array.from(this.tabButtons.keys());
    const currentIndex = tabIds.indexOf(tabId);
    const previousIndex = this.selectedTab
      ? tabIds.indexOf(this.selectedTab)
      : -1;

    // Deactivate all tabs
    this.tabButtons.forEach((btn, id) => {
      btn.setActive(id === tabId);
    });

    // Update panels with direction
    this.tabPanels.forEach((pnl, id) => {
      if (id === tabId) {
        pnl.setActive(true, currentIndex > previousIndex ? "right" : "left");
      } else {
        const panelIndex = tabIds.indexOf(id);
        let direction: "left" | "right" = "left";
        if (panelIndex < currentIndex) {
          direction = "left";
        } else if (panelIndex > currentIndex) {
          direction = "right";
        }
        pnl.setActive(false, direction);
      }
    });

    this.selectedTab = tabId;

    // Dispatch event
    this.dispatchEvent(
      new CustomEvent("tab-change", {
        detail: { tabId },
        bubbles: true,
        composed: true,
      }),
    );
  }

  getSelectedTab(): string | null {
    return this.selectedTab;
  }
}

export class TabButton extends HTMLElement {
  private button: HTMLButtonElement;

  constructor() {
    super();
    this.attachShadow({ mode: "open" });

    const style = document.createElement("style");
    style.textContent = `
      :host {
        display: inline-block;
      }

      button {
        padding: 0.75rem 1.25rem;
        border: none;
        background-color: transparent;
        color: #6b7280;
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        border-bottom: 2px solid transparent;
        transition: all 0.2s;
        position: relative;
        white-space: nowrap;
      }

      button:hover {
        color: #111827;
        background-color: #f3f4f6;
      }

      button.active {
        color: #3b82f6;
        border-bottom-color: #3b82f6;
        background-color: white;
      }

      button:focus {
        outline: none;
        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
      }

      button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
      }
    `;

    this.button = document.createElement("button");
    const slot = document.createElement("slot");
    this.button.appendChild(slot);

    this.shadowRoot!.appendChild(style);
    this.shadowRoot!.appendChild(this.button);

    this.button.addEventListener("click", () => {
      if (!this.hasAttribute("disabled")) {
        this.handleClick();
      }
    });
  }

  private handleClick(): void {
    const tabId = this.getAttribute("for");
    if (tabId) {
      this.dispatchEvent(
        new CustomEvent("tab-select", {
          detail: tabId,
          bubbles: true,
          composed: true,
        }),
      );
    }
  }

  setActive(active: boolean): void {
    if (active) {
      this.button.classList.add("active");
      this.button.setAttribute("aria-selected", "true");
    } else {
      this.button.classList.remove("active");
      this.button.setAttribute("aria-selected", "false");
    }
  }

  static get observedAttributes(): string[] {
    return ["disabled"];
  }

  attributeChangedCallback(
    name: string,
    _oldValue: string,
    newValue: string,
  ): void {
    if (name === "disabled") {
      if (newValue !== null) {
        this.button.setAttribute("disabled", "");
      } else {
        this.button.removeAttribute("disabled");
      }
    }
  }
}

export class TabPanel extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });

    const style = document.createElement("style");
    style.textContent = `
      :host {
        display: block;
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        min-height: 200px;
        padding: 1.5rem;
        background-color: white;
        transition: transform 0.3s ease-in-out, opacity 0.3s ease-in-out;
        opacity: 0;
        transform: translateX(100%);
      }

      :host(.active) {
        position: relative;
        opacity: 1;
        transform: translateX(0);
      }

      :host(.left) {
        transform: translateX(-100%);
      }

      :host(.right) {
        transform: translateX(100%);
      }

      :host(.active.from-left) {
        animation: slideInFromLeft 0.3s ease-out forwards;
      }

      :host(.active.from-right) {
        animation: slideInFromRight 0.3s ease-out forwards;
      }

      @keyframes slideInFromLeft {
        from {
          transform: translateX(-100%);
          opacity: 0;
        }
        to {
          transform: translateX(0);
          opacity: 1;
        }
      }

      @keyframes slideInFromRight {
        from {
          transform: translateX(100%);
          opacity: 0;
        }
        to {
          transform: translateX(0);
          opacity: 1;
        }
      }

      .panel-content {
        width: 100%;
      }
    `;

    const content = document.createElement("div");
    content.className = "panel-content";
    const slot = document.createElement("slot");
    content.appendChild(slot);

    this.shadowRoot!.appendChild(style);
    this.shadowRoot!.appendChild(content);
  }

  setActive(active: boolean, direction: "left" | "right"): void {
    if (active) {
      this.classList.add("active");
      this.classList.remove("left", "right");
      this.classList.add(direction === "left" ? "from-left" : "from-right");
      this.setAttribute("aria-hidden", "false");
    } else {
      this.classList.remove("active", "from-left", "from-right");
      this.classList.add(direction);
      this.setAttribute("aria-hidden", "true");
    }
  }

  connectedCallback(): void {
    this.setAttribute("role", "tabpanel");
    if (!this.classList.contains("active")) {
      this.setAttribute("aria-hidden", "true");
    }
  }
}

customElements.define("tab-container", TabContainer);
customElements.define("tab-button", TabButton);
customElements.define("tab-panel", TabPanel);
