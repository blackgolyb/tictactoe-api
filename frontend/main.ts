import "./components/modal-window";
import "./components/game-form";
import "./components/tab-container";
import type { GameFormData } from "./components/game-form";
import type { ModalWindow } from "./components/modal-window";
import type { GameForm } from "./components/game-form";

// const BASE_URL = "https://tic_tac_toe_api.serveo.net";
const BASE_URL = "http://localhost:8128";
const DEFAULT_PARAMS = {
  name: "default",
  width: 3,
  height: 3,
  winningLength: 3,
  firstPlayer: "X",
};

interface GameSettings {
  width: number;
  height: number;
  winning_length: number;
  first_player: string;
}

interface RequestData {
  queryParams?: Record<string, string>;
  data?: unknown;
}

type HttpMethod = "GET" | "POST" | "PUT" | "DELETE";
type Route = [HttpMethod, string];

interface Router {
  getCurrentPlayer: () => Route;
  getField: (fieldId: number) => Route;
  getGameSettings: () => Route;
  makeStep: (fieldId: number) => Route;
  upsertGame: () => Route;
}

export class GameAPI {
  base: string;
  room: string;
  backref: string | undefined;

  constructor(base_url: string, room: string) {
    this.base = base_url;
    this.room = room;
    this.backref = undefined;
  }

  async _fetch(
    name: keyof Router,
    params: any[],
    requestData?: RequestData,
  ): Promise<Response> {
    const routerFunc = this.getRouter()[name];
    if (!routerFunc) {
      throw new Error(`No such method: ${name}`);
    }

    const route = (routerFunc as any)(...params) as Route;
    const url = new URL(route[1]);
    const method = route[0];
    const queryParams = requestData?.queryParams || {};
    for (const key in queryParams) {
      url.searchParams.set(key, queryParams[key]);
    }

    const data = requestData?.data;
    return fetch(url, {
      method,
      body: data ? JSON.stringify(data) : undefined,
      headers: data ? { "Content-Type": "application/json" } : undefined,
    });
  }

  fetchGameSettings(name: string): Promise<Response> {
    return this._fetch("getGameSettings", [name]);
  }

  createOrUpdateGame(
    name: string,
    { width, height, winning_length, first_player }: GameSettings,
  ): Promise<Response> {
    return this._fetch("upsertGame", [name], {
      data: {
        width,
        height,
        winning_length,
        first_player,
      },
    });
  }

  makeStep(name: string, fieldId: number): Promise<Response> {
    return this._fetch("makeStep", [name, fieldId]);
  }

  getRouter(): Router {
    return {
      getCurrentPlayer: () => [
        "GET",
        `${this.base}/api/v1/${this.room}/get_current_player`,
      ],
      getField: (fieldId: number) => [
        "GET",
        `${this.base}/api/v1/${this.room}/get_field/${fieldId}`,
      ],
      getGameSettings: () => ["GET", `${this.base}/api/v1/${this.room}/game`],
      makeStep: (fieldId: number) => {
        const url = new URL(
          `${this.base}/api/v1/${this.room}/update_field/${fieldId}`,
        );
        if (this.backref !== undefined) {
          url.searchParams.set("r", this.backref);
        }
        return ["GET", url.toString()];
      },
      upsertGame: () => ["POST", `${this.base}/api/v1/${this.room}/game`],
    };
  }
}

const fieldTemplate = document.getElementById(
  "fieldTemplate",
) as HTMLTemplateElement;
const fieldsContainer = document.getElementById("fields") as HTMLElement;
const codeHTMLContainer = document.getElementById(
  "html-game-code",
) as HTMLElement;
const codeMarkdownContainer = document.getElementById(
  "markdown-game-code",
) as HTMLElement;

function initGameModal(gameAPI: GameAPI): void {
  // Create modal and form elements
  const modal = document.createElement("modal-window") as ModalWindow;
  const gameForm = document.createElement("game-form") as GameForm;

  const modalTitle = document.createElement("h2");
  modalTitle.slot = "title";
  modalTitle.textContent = "Create Your Game";

  modal.appendChild(modalTitle);
  modal.appendChild(gameForm);
  document.body.appendChild(modal);

  // Set initial game API data
  gameForm.setGameAPI(gameAPI);

  // Create button to open modal
  const openModalBtn = document.createElement("button");
  openModalBtn.textContent = "Configure Game";
  openModalBtn.className = "open-modal-btn";
  openModalBtn.style.cssText = `
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    padding: 1rem 1.5rem;
    background-color: #3b82f6;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: all 0.2s;
    z-index: 999;
  `;
  openModalBtn.addEventListener("mouseenter", () => {
    openModalBtn.style.backgroundColor = "#2563eb";
    openModalBtn.style.transform = "translateY(-2px)";
    openModalBtn.style.boxShadow = "0 6px 8px rgba(0, 0, 0, 0.15)";
  });
  openModalBtn.addEventListener("mouseleave", () => {
    openModalBtn.style.backgroundColor = "#3b82f6";
    openModalBtn.style.transform = "translateY(0)";
    openModalBtn.style.boxShadow = "0 4px 6px rgba(0, 0, 0, 0.1)";
  });
  openModalBtn.addEventListener("click", () => {
    modal.open();
  });
  document.body.appendChild(openModalBtn);

  // Handle form submission
  gameForm.addEventListener("game-submit", (e: CustomEvent<GameFormData>) => {
    const data = e.detail;
    gameAPI.base = data.serverURL;
    gameAPI.room = data.room;
    const backref = data.redirect.trim();
    gameAPI.backref = backref.length ? backref : undefined;

    updateAllGame(gameAPI);
    modal.close();
  });

  // Handle form cancellation
  gameForm.addEventListener("game-cancel", () => {
    modal.close();
  });
}

function getMarkdownGameCode(
  gameAPI: GameAPI,
  rows: number = 3,
  cols: number = 3,
): string {
  const router = gameAPI.getRouter();
  const getImg = (id: number): string =>
    `<a href="${router.getField(id)[1]}"><img src="${router.makeStep(id)[1]}" width="100"/></a>`;
  const currentPlayerIndicator = `<img src="${router.getCurrentPlayer()[1]}" height="12"/>`;

  let res = "Current Player: " + currentPlayerIndicator + "\n\n";

  for (let i = 0; i < rows; i++) {
    const cells = Array.from(Array(cols).keys(), (j) => getImg(i * rows + j));
    res += "|" + cells.join("|") + "|\n";
  }
  return res;
}

function getHTMLGameCode(
  gameAPI: GameAPI,
  rows: number = 3,
  cols: number = 3,
): string {
  const router = gameAPI.getRouter();
  const getImg = (id: number): string =>
    `<a href="${router.getField(id)[1]}"><img src="${router.makeStep(id)[1]}" width="100"/></a>`;
  const currentPlayerIndicator = `<img src="${router.getCurrentPlayer()[1]}" height="12"/>`;

  const template = (fields: string, currentPlayer: string): string =>
    `\
        <div class="tic-tac-toe">\n\
        <div>\n\
        \    <span>Current Player: </span>\n\
        \    ${currentPlayer}\n\
        </div>\n\
        <table class="tic-tac-toe__gamemap">\n\
        \    <tbody>\n\
        \        ${fields}\n\
        \    </tbody>\n\
        </table>\n\
        </div>
        `;

  let fields = "";
  for (let i = 0; i < rows; i++) {
    const cells = Array.from(
      Array(cols).keys(),
      (j) => "<td>" + getImg(i * rows + j) + "</td>",
    );
    fields += "<tr>" + cells.join("") + "</tr>";
  }

  return template(fields, currentPlayerIndicator);
}

function urlWithTimestamp(url: string): URL {
  if (!URL.canParse(url)) {
    console.error("failed to find img with this url: ", url);
  }
  const newUrl = new URL(url);
  newUrl.searchParams.set("reloadT", Date.now().toString());
  return newUrl;
}

async function reloadImg(url: string): Promise<void> {
  const updatedUrl = urlWithTimestamp(url);
  document.body.querySelectorAll(`img[src^='${url}']`).forEach((img) => {
    (img as HTMLImageElement).src = updatedUrl.toString();
  });
}

function reloadGameMap(gameAPI: GameAPI): void {
  for (let i = 0; i < 9; i++) {
    const url = gameAPI.getRouter().getField(i)[1];
    reloadImg(url);
  }
}

function createFieldNode(gameAPI: GameAPI, fieldId: number): DocumentFragment {
  const field = fieldTemplate.content.cloneNode(true) as DocumentFragment;
  const img = field.querySelector("img") as HTMLImageElement;
  img.addEventListener("click", () => {
    gameAPI
      .makeStep(gameAPI.room, fieldId)
      .then(() => {
        reloadGameMap(gameAPI);
      })
      .catch((err) => {
        console.error(err);
      });
  });
  img.src = urlWithTimestamp(
    gameAPI.getRouter().getField(fieldId)[1],
  ).toString();
  return field;
}

function fillGameMap(
  gameAPI: GameAPI,
  rows: number = 3,
  cols: number = 3,
): void {
  fieldsContainer.innerHTML = "";
  for (let i = 0; i < rows; i++) {
    const row = document.createElement("tr");
    fieldsContainer.appendChild(row);
    for (let j = 0; j < cols; j++) {
      const col = document.createElement("td");
      const field = createFieldNode(gameAPI, i * rows + j);
      row.appendChild(col);
      col.appendChild(field);
    }
  }
}

function fillMarkdownCode(gameAPI: GameAPI): void {
  codeMarkdownContainer.textContent = getMarkdownGameCode(gameAPI);
  // hljs.highlightElement(codeMarkdownContainer);
}

function fillHTMLCode(gameAPI: GameAPI): void {
  codeHTMLContainer.textContent = getHTMLGameCode(gameAPI);
  // hljs.highlightElement(codeHTMLContainer);
}

async function fetchGameSettings(gameAPI: GameAPI): Promise<Response> {
  return gameAPI.createOrUpdateGame(gameAPI.room, {
    width: 3,
    height: 3,
    winning_length: 3,
    first_player: "X",
  });
}

async function updateAllGame(gameAPI: GameAPI): Promise<void> {
  await fetchGameSettings(gameAPI);
  fillGameMap(gameAPI);
  fillMarkdownCode(gameAPI);
  fillHTMLCode(gameAPI);
}

function main(): void {
  const gameAPI = new GameAPI(BASE_URL, DEFAULT_PARAMS.name);
  updateAllGame(gameAPI);
  initGameModal(gameAPI);
}

main();
