/**
 * Game API Service
 * Provides methods to interact with the Tic Tac Toe game server
 */

export type Player = "X" | "O";
export type Point = [number, number];

export interface ImageSize {
  width?: number;
  height?: number;
}

export interface GameRules {
  game_size: [number, number];
  winning_length: number;
}

export interface CreateGameParams {
  name: string;
  width?: number;
  height?: number;
  winning_length?: number;
  first_player?: Player;
}

export interface CreateGameResponse {
  success: boolean;
  name: string;
}

export interface MakeMoveOptions {
  redirect?: string;
}

export interface GameApiConfig {
  baseUrl: string;
}

/**
 * Game API Service
 * Handles all interactions with the game server
 */
export class GameApiService {
  private baseUrl: string;

  constructor(config: GameApiConfig) {
    this.baseUrl = config.baseUrl.replace(/\/$/, ""); // Remove trailing slash
  }

  /**
   * Update the base URL
   */
  setBaseUrl(baseUrl: string): void {
    this.baseUrl = baseUrl.replace(/\/$/, "");
  }

  /**
   * Get the base URL
   */
  getBaseUrl(): string {
    return this.baseUrl;
  }

  // ==================== URL Generators ====================

  /**
   * Get URL for the main page
   */
  getMainPageUrl(): string {
    return `${this.baseUrl}/`;
  }

  /**
   * Get URL for current player image
   */
  getCurrentPlayerUrl(room: string, size?: ImageSize): string {
    return this.withImageSize(
      `${this.baseUrl}/api/v1/${room}/get-current-player`,
      size,
    );
  }

  /**
   * Get URL for a specific field image
   */
  getFieldUrl(room: string, field: Point, size?: ImageSize): string {
    const fieldId = this.pointToFieldId(field);
    return this.withImageSize(
      `${this.baseUrl}/api/v1/${room}/get-field/${fieldId}`,
      size,
    );
  }

  /**
   * Get URL to make a move
   */
  getMakeMoveUrl(room: string, field: Point, redirect?: string): string {
    const fieldId = this.pointToFieldId(field);
    const url = new URL(`${this.baseUrl}/api/v1/${room}/make-move/${fieldId}`);
    if (redirect) {
      url.searchParams.set("r", redirect);
    }
    return url.toString();
  }

  /**
   * Get URL for game rules
   */
  getGameRulesUrl(room: string): string {
    return `${this.baseUrl}/api/v1/${room}/rules`;
  }

  /**
   * Get URL to create a game
   */
  getCreateGameUrl(params: CreateGameParams): string {
    const url = new URL(`${this.baseUrl}/api/v1/create-game`);
    url.searchParams.set("name", params.name);
    if (params.width !== undefined) {
      url.searchParams.set("width", params.width.toString());
    }
    if (params.height !== undefined) {
      url.searchParams.set("height", params.height.toString());
    }
    if (params.winning_length !== undefined) {
      url.searchParams.set("winning_length", params.winning_length.toString());
    }
    if (params.first_player !== undefined) {
      url.searchParams.set("first_player", params.first_player);
    }
    return url.toString();
  }

  // ==================== API Methods ====================

  /**
   * Fetch the main HTML page
   */
  async fetchMainPage(): Promise<string> {
    const response = await fetch(this.getMainPageUrl());
    if (!response.ok) {
      throw new Error(`Failed to fetch main page: ${response.statusText}`);
    }
    return response.text();
  }

  /**
   * Get current player image as blob
   */
  async getCurrentPlayerImage(room: string, size?: ImageSize): Promise<Blob> {
    const response = await fetch(this.getCurrentPlayerUrl(room, size));
    if (!response.ok) {
      throw new Error(`Failed to get current player: ${response.statusText}`);
    }
    return response.blob();
  }

  /**
   * Get field image as blob
   */
  async getFieldImage(
    room: string,
    field: Point,
    size?: ImageSize,
  ): Promise<Blob> {
    const response = await fetch(this.getFieldUrl(room, field, size));
    if (!response.ok) {
      throw new Error(`Failed to get field image: ${response.statusText}`);
    }
    return response.blob();
  }

  /**
   * Make a move on the game board
   */
  async makeMove(
    room: string,
    field: Point,
    options?: MakeMoveOptions,
  ): Promise<Response> {
    const url = this.getMakeMoveUrl(room, field, options?.redirect);

    try {
      const response = await fetch(url);

      // Success statuses: 200 OK, 204 No Content, 302 Redirect
      if (response.ok || response.status === 302) {
        return response;
      }

      // Only throw error for actual error responses
      const errorText = await response.text().catch(() => "");
      throw new Error(
        `HTTP ${response.status}: ${response.statusText}${errorText ? " - " + errorText : ""}`,
      );
    } catch (error) {
      throw error;
    }
  }

  /**
   * Create a new game
   */
  async createGame(params: CreateGameParams): Promise<CreateGameResponse> {
    const url = this.getCreateGameUrl(params);
    const response = await fetch(url, {
      method: "POST",
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Failed to create game: ${errorText}`);
    }

    return response.json();
  }

  /**
   * Get game rules
   */
  async getGameRules(room: string): Promise<GameRules> {
    const response = await fetch(this.getGameRulesUrl(room));
    if (!response.ok) {
      throw new Error(`Failed to get game rules: ${response.statusText}`);
    }
    return response.json();
  }

  /**
   * Create or update a game (convenience method)
   */
  async createOrUpdateGame(
    room: string,
    params: Omit<CreateGameParams, "name">,
  ): Promise<CreateGameResponse> {
    return this.createGame({
      ...params,
      name: room,
    });
  }

  // ==================== Helper Methods ====================

  /**
   * Convert Point to field ID string format "x,y"
   */
  private pointToFieldId(point: Point): string {
    return `${point[0]},${point[1]}`;
  }

  /**
   * Convert linear index to Point based on grid width
   */
  indexToPoint(index: number, width: number): Point {
    const x = index % width;
    const y = Math.floor(index / width);
    return [x, y];
  }

  /**
   * Convert Point to linear index based on grid width
   */
  pointToIndex(point: Point, width: number): number {
    return point[1] * width + point[0];
  }

  /**
   * Add cache-busting timestamp to URL
   */
  withTimestamp(url: string): string {
    const urlObj = new URL(url);
    urlObj.searchParams.set("t", Date.now().toString());
    return urlObj.toString();
  }

  withImageSize(url: string, size?: ImageSize): string {
    if (!size) {
      return url;
    }

    const urlObj = new URL(url);
    if (size.width) {
      urlObj.searchParams.set("w", size.width.toString());
    }
    if (size.height) {
      urlObj.searchParams.set("h", size.height.toString());
    }
    return urlObj.toString();
  }

  /**
   * Preload an image URL
   */
  async preloadImage(url: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve();
      img.onerror = reject;
      img.src = url;
    });
  }

  /**
   * Preload all field images for a game
   */
  async preloadGameImages(
    room: string,
    width: number,
    height: number,
    size?: ImageSize,
  ): Promise<void> {
    const promises: Promise<void>[] = [];

    // Preload current player
    promises.push(this.preloadImage(this.getCurrentPlayerUrl(room, size)));

    // Preload all fields
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        promises.push(this.preloadImage(this.getFieldUrl(room, [x, y], size)));
      }
    }

    await Promise.all(promises);
  }
}

/**
 * Create a singleton instance with default configuration
 */
export const gameApi = new GameApiService({
  baseUrl: window.location.origin,
});

export default GameApiService;
