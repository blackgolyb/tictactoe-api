import { BASE_URL, DEFAULT_PARAMS } from "./constants";
import Storage from "../shared/storage";

export const store = new Storage({
  serverUrl: BASE_URL,
  gameName: DEFAULT_PARAMS.name,
  imageSize: 100,
  currentPlayerImageSize: 24,
});

export function getFieldImageSize():
  | { width?: number; height?: number }
  | undefined {
  return getImageHeight(store.get<number>("imageSize"));
}

export function getCurrentPlayerImageSize():
  | { width?: number; height?: number }
  | undefined {
  return getImageHeight(store.get<number>("currentPlayerImageSize"));
}

function getImageHeight(size: number | undefined): { height: number } | undefined {
  if (!size) {
    return undefined;
  }

  return { height: size };
}
