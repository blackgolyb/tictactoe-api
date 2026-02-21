import "./components/modal-window";
import "./components/game-form";
import "./components/game-selector";
import "./components/tab-container";
import "./components/gameView/playable";
import "./components/gameView/html";
import "./components/gameView/md";

import { store } from "./core/store";
import { gameApi } from "./services/gameApi";
import { DEFAULT_PARAMS } from "./core/constants";
import type { ModalWindow } from "./components/modal-window";

/**
 * Wire up component interactions
 */
function setupComponentWiring(): void {
  const gameSelector = document.querySelector("game-selector");
  const createModal = document.getElementById(
    "create-game-modal",
  ) as ModalWindow;
  const createForm = document.querySelector("create-game-form");

  // Wire game-selector to open create modal
  if (gameSelector && createModal) {
    gameSelector.addEventListener("open-create-game", () => {
      createModal.open();
    });
  }

  // Wire create form to close modal on success or cancel
  if (createForm && createModal) {
    createForm.addEventListener("game-create", () => {
      createModal.close();
    });
    createForm.addEventListener("game-create-cancel", () => {
      createModal.close();
    });
  }
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

  // Wire up component interactions
  setupComponentWiring();

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
