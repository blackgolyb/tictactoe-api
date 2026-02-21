import "./components/modal-window";
import "./components/select-game-form";
import "./components/game-form";
import "./components/game-selector";
import "./components/tab-container";
import "./components/gameView/playable";
import "./components/gameView/html";
import "./components/gameView/md";

import type { SelectGameFormData } from "./components/select-game-form";
import type { CreateGameFormData } from "./components/game-form";
import type { ModalWindow } from "./components/modal-window";

import { store } from "./core/store";
import { gameApi } from "./services/gameApi";
import { DEFAULT_PARAMS } from "./core/constants";

/**
 * Initialize the select game modal
 */
function initSelectGameModal(): ModalWindow {
  const modal = document.createElement("modal-window") as ModalWindow;
  const selectGameForm = document.createElement("select-game-form");

  const modalTitle = document.createElement("h2");
  modalTitle.slot = "title";
  modalTitle.textContent = "Select Game";

  modal.appendChild(modalTitle);
  modal.appendChild(selectGameForm);
  document.body.appendChild(modal);

  // Handle form submission
  selectGameForm.addEventListener(
    "game-select",
    async (e: CustomEvent<SelectGameFormData>) => {
      const data = e.detail;

      // Update store - this will trigger the effect to fetch rules
      store.set("serverUrl", data.serverURL);
      store.set("gameName", data.gameName);

      // Update game API base URL
      gameApi.setBaseUrl(data.serverURL);

      modal.close();
    },
  );

  // Handle form cancellation
  selectGameForm.addEventListener("game-select-cancel", () => {
    modal.close();
  });

  return modal;
}

/**
 * Initialize the create game modal
 */
function initCreateGameModal(): ModalWindow {
  const modal = document.createElement("modal-window") as ModalWindow;
  const createGameForm = document.createElement("create-game-form");

  const modalTitle = document.createElement("h2");
  modalTitle.slot = "title";
  modalTitle.textContent = "Create New Game";

  modal.appendChild(modalTitle);
  modal.appendChild(createGameForm);
  document.body.appendChild(modal);

  // Handle form submission
  createGameForm.addEventListener(
    "game-create",
    async (e: CustomEvent<CreateGameFormData>) => {
      const data = e.detail;

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

        // Update store - this will trigger the effect to fetch rules
        store.set("serverUrl", data.serverURL);
        store.set("gameName", data.gameName);

        modal.close();
      } catch (error) {
        console.error("Failed to create game:", error);
        alert(
          "Failed to create game. Please check your settings and try again.",
        );
      }
    },
  );

  // Handle form cancellation
  createGameForm.addEventListener("game-create-cancel", () => {
    modal.close();
  });

  return modal;
}

/**
 * Initialize the game selector in the main window
 */
function initGameSelector(
  _selectGameModal: ModalWindow,
  createGameModal: ModalWindow,
): void {
  const gameSelector = document.createElement("game-selector");

  // Insert at the top of the content
  const content = document.querySelector(".content");
  if (content) {
    content.insertBefore(gameSelector, content.firstChild);
  }

  // Handle game select
  gameSelector.addEventListener(
    "game-select",
    (e: CustomEvent<SelectGameFormData>) => {
      const data = e.detail;

      // Update store - this will trigger the effect to fetch rules
      store.set("serverUrl", data.serverURL);
      store.set("gameName", data.gameName);

      // Update game API base URL
      gameApi.setBaseUrl(data.serverURL);
    },
  );

  // Handle create new game
  gameSelector.addEventListener("open-create-game", () => {
    createGameModal.open();
  });
}

/**
 * Effect: Fetch game rules when serverUrl or gameName changes
 */
function setupGameRulesEffect(): void {
  let previousServerUrl: string | undefined = undefined;
  let previousGameName: string | undefined = undefined;

  const fetchRules = async () => {
    const serverUrl = store.get<string>("serverUrl");
    const gameName = store.get<string>("gameName");

    // Only fetch if both values are set and at least one has changed
    if (!serverUrl || !gameName) {
      return;
    }

    if (serverUrl === previousServerUrl && gameName === previousGameName) {
      return;
    }

    previousServerUrl = serverUrl;
    previousGameName = gameName;

    console.log(`Fetching rules for game: ${gameName} from ${serverUrl}`);

    try {
      // Update API base URL
      gameApi.setBaseUrl(serverUrl);

      // Fetch game rules
      const rules = await gameApi.getGameRules(gameName);

      // Update store with rules
      store.set("width", rules.game_size[0]);
      store.set("height", rules.game_size[1]);
      store.set("winningLength", rules.winning_length);

      console.log("Game rules loaded:", rules);
    } catch (error) {
      console.error("Failed to fetch game rules:", error);
      console.warn(
        "Game may not exist. You can create it using the Create New button.",
      );

      // Set default values if fetch fails
      store.set("width", DEFAULT_PARAMS.width);
      store.set("height", DEFAULT_PARAMS.height);
      store.set("winningLength", DEFAULT_PARAMS.winningLength);
    }
  };

  // Listen for changes to serverUrl and gameName
  store.onChange("serverUrl", fetchRules);
  store.onChange("gameName", fetchRules);

  // Initial fetch
  fetchRules();
}

/**
 * Initialize the application
 */
async function main(): Promise<void> {
  console.log("Initializing Tic Tac Toe application...");

  // Initialize modals
  const selectGameModal = initSelectGameModal();
  const createGameModal = initCreateGameModal();

  // Initialize game selector
  initGameSelector(selectGameModal, createGameModal);

  // Setup effect to fetch game rules when serverUrl or gameName changes
  setupGameRulesEffect();

  // Try to create default game if needed (silently fail if it already exists)
  const gameName = store.get<string>("gameName");
  if (!gameName) {
    try {
      await gameApi.createOrUpdateGame(DEFAULT_PARAMS.name, {
        width: DEFAULT_PARAMS.width,
        height: DEFAULT_PARAMS.height,
        winning_length: DEFAULT_PARAMS.winningLength,
        first_player: "X",
      });
      console.log("Default game created");
    } catch (error) {
      console.log("Default game may already exist or server unavailable");
    }
  }

  console.log("Application initialized");
}

// Start the application
main().catch((error) => {
  console.error("Failed to initialize application:", error);
});
