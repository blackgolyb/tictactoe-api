import { BASE_URL, DEFAULT_PARAMS } from "./constants";
import Storage from "../shared/storage";

export const store = new Storage({
  serverUrl: BASE_URL,
  gameName: DEFAULT_PARAMS.name,
});
