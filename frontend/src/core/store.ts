import { BASE_URL, DEFAULT_PARAMS } from "./constants";
import Storage from "../shared/storage";

export const store = new Storage({
  serverUrl: BASE_URL,
  gameName: DEFAULT_PARAMS.name,
  imageWidth: 100,
  imageHeight: 100,
  currentPlayerImageWidth: 24,
  currentPlayerImageHeight: 24,
});

export function getFieldImageSize():
  | { width: number; height: number }
  | undefined {
  const width = store.get<number>("imageWidth");
  const height = store.get<number>("imageHeight");

  return getImageSize(width, height);
}

export function getCurrentPlayerImageSize():
  | { width: number; height: number }
  | undefined {
  const width = store.get<number>("currentPlayerImageWidth");
  const height = store.get<number>("currentPlayerImageHeight");

  return getImageSize(width, height);
}

function getImageSize(
  width: number | undefined,
  height: number | undefined,
): { width: number; height: number } | undefined {
  if (!width || !height) {
    return undefined;
  }

  return { width, height };
}
