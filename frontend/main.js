// var BASE_URL = "https://tic_tac_toe_api.serveo.net";
var BASE_URL = "http://localhost:8128";
var DEFAULT_PARAMS = {
  name: "default",
  width: 3,
  height: 3,
  winningLength: 3,
  firstPlayer: "X",
};

class GameAPI {
  constructor(base_url, room) {
    this.base = base_url;
    this.room = room;
    this.backref = undefined;
  }

  async _fetch(name, params, requestData) {
    const route = this.getRouter()[name]?.(...params);
    if (!route) {
      throw new Error(`No such method: ${name}`);
    }

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

  fetchGameSettings(name) {
    return this._fetch("getGameSettings", [name]);
  }

  createOrUpdateGame(name, { width, height, winning_length, first_player }) {
    return this._fetch("upsertGame", [name], {
      data: {
        width,
        height,
        winning_length,
        first_player,
      },
    });
  }

  makeStep(name, fieldId) {
    return this._fetch("makeStep", [name, fieldId]);
  }

  getRouter() {
    return {
      getCurrentPlayer: () => [
        "GET",
        `${this.base}/api/v1/${this.room}/get_current_player`,
      ],
      getField: (fieldId) => [
        "GET",
        `${this.base}/api/v1/${this.room}/get_field/${fieldId}`,
      ],
      getGameSettings: () => ["GET", `${this.base}/api/v1/${this.room}/game`],
      makeStep: (fieldId) => {
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

class TabSwitcher {
  constructor(elemId) {
    this.elemId = elemId;
    this.initElements();
  }

  initElements() {
    const elem = document.getElementById(this.elemId);
    const tabsElem = elem.querySelector(".tabs-switcher__variants");
    const bodyElem = elem.querySelector(".tabs-switcher__body");
    const views = Array.from(bodyElem.querySelectorAll(`[data-tab]`));

    this.tabs = {};
    this.selected = null;
    for (const switcher of tabsElem.children) {
      const name = switcher.dataset.for;
      if (switcher.dataset.checked !== undefined) {
        this.selected = name;
      }
      const view = views.find((v) => v.dataset.tab === name);
      this.tabs[name] = { switcher, view };
    }

    if (this.selected === null) {
      this.selected = Object.keys(this.tabs)[0];
    }

    for (const tabName in this.tabs) {
      this.tabs[tabName].switcher.addEventListener("click", (e) =>
        this.switch(e.target.dataset.for),
      );
    }

    this.switch(this.selected);
  }

  switch(tabName) {
    const tabsNames = Object.keys(this.tabs);
    const tab = this.tabs[tabName].view;
    const tabId = tabsNames.indexOf(tabName);

    for (const [id, name] of tabsNames.entries()) {
      const current = this.tabs[name].view;
      if (id < tabId) {
        current.classList.remove("right");
        current.classList.remove("mid");
        current.classList.add("left");
      } else if (id > tabId) {
        current.classList.remove("left");
        current.classList.remove("mid");
        current.classList.add("right");
      }
    }

    tab.classList.remove("right");
    tab.classList.remove("left");
    tab.classList.add("mid");
    this.selected = tabName;
  }
}

var fieldTemplate = document.getElementById("fieldTemplate");
var fieldsContainer = document.getElementById("fields");
var codeHTMLContainer = document.getElementById("html-game-code");
var codeMarkdownContainer = document.getElementById("markdown-game-code");
var gameMakerForm = document.getElementById("game-maker-form");

function initGameMakerForm(gameAPI) {
  gameMakerForm.addEventListener("submit", (e) => {
    e.preventDefault();
    const formData = new FormData(gameMakerForm);
    const data = Object.fromEntries(formData);
    gameAPI.base = data.serverURL;
    gameAPI.room = data.room;
    const backref = data.redirect.trim();
    gameAPI.backref = backref.length ? backref : undefined;
    updateAllGame(gameAPI);
  });
  const serverURLField = gameMakerForm.querySelector('[name="serverURL"]');
  const roomField = gameMakerForm.querySelector('[name="room"]');
  const redirectField = gameMakerForm.querySelector('[name="redirect"]');
  const changeHeight = gameMakerForm.querySelector('[name="height"]');
  const changeWidth = gameMakerForm.querySelector('[name="width"]');
  changeHeight.value = 3;
  changeWidth.value = 3;
  serverURLField.value = gameAPI.base;
  roomField.value = gameAPI.room;
  redirectField.value = "";
}

function getMarkdownGameCode(gameAPI, rows = 3, cols = 3) {
  const router = gameAPI.getRouter();
  const getImg = (id) =>
    `<a href="${router.getField(id)[1]}"><img src="${router.makeStep(id)[1]}" width="100"/></a>`;
  const currentPlayerIndicator = `<img src="${router.getCurrentPlayer()[1]}" height="12"/>`;

  let res = "Current Player: " + currentPlayerIndicator + "\n\n";

  for (let i = 0; i < rows; i++) {
    const cells = Array.from(Array(cols).keys(), (j) => getImg(i * rows + j));
    res += "|" + cells.join("|") + "|\n";
  }
  return res;
}

function getHTMLGameCode(gameAPI, rows = 3, cols = 3) {
  const router = gameAPI.getRouter();
  const getImg = (id) =>
    `<a href="${router.getField(id)[1]}"><img src="${router.makeStep(id)[1]}" width="100"/></a>`;
  const currentPlayerIndicator = `<img src="${router.getCurrentPlayer()[1]}" height="12"/>`;

  const template = (fields, currentPlayer) =>
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

function urlWithTimestamp(url) {
  if (!URL.canParse(url)) {
    console.error("failed to find img with this url: ", url);
  }
  const newUrl = new URL(url);
  newUrl.searchParams.set("reloadT", Date.now().toString());
  return newUrl;
}

async function reloadImg(url) {
  const updatedUrl = urlWithTimestamp(url);
  document.body.querySelectorAll(`img[src^='${url}']`).forEach((img) => {
    img.src = updatedUrl.toString();
  });
}

function reloadGameMap(gameAPI) {
  for (let i = 0; i < 9; i++) {
    const url = gameAPI.getRouter().getField(i)[1];
    reloadImg(url);
  }
}

function createFieldNode(gameAPI, fieldId) {
  const field = fieldTemplate.content.cloneNode(true);
  const img = field.querySelector("img");
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
  img.src = urlWithTimestamp(gameAPI.getRouter().getField(fieldId)[1]);
  return field;
}

function fillGameMap(gameAPI, rows = 3, cols = 3) {
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

function fillMarkdownCode(gameAPI) {
  codeMarkdownContainer.textContent = getMarkdownGameCode(gameAPI);
  // hljs.highlightElement(codeMarkdownContainer);
}

function fillHTMLCode(gameAPI) {
  codeHTMLContainer.textContent = getHTMLGameCode(gameAPI);
  // hljs.highlightElement(codeHTMLContainer);
}

async function fetchGameSettings(gameAPI) {
  return gameAPI.createOrUpdateGame(gameAPI.room, {
    width: 3,
    height: 3,
    winning_length: 3,
    first_player: "X",
  });
}

async function updateAllGame(gameAPI) {
  await fetchGameSettings(gameAPI);
  fillGameMap(gameAPI);
  fillMarkdownCode(gameAPI);
  fillHTMLCode(gameAPI);
}

class App {
  constructor(api) {
    this.api = api;
    this.name = DEFAULT_PARAMS.name;
    this.width = DEFAULT_PARAMS.width;
    this.height = DEFAULT_PARAMS.height;
    this.winningLength = DEFAULT_PARAMS.winningLength;
    this.firstPlayer = DEFAULT_PARAMS.firstPlayer;
  }
}

function main() {
  const gameAPI = new GameAPI(BASE_URL, DEFAULT_PARAMS.name);
  updateAllGame(gameAPI);
  initGameMakerForm(gameAPI);
  const tabSwitcher = new TabSwitcher("game-switcher");
}

main();
