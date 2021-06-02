import { menuTargetPathLocationConverter } from "./converter"
import { LoadMenuLocationDetectMethod, LoadMenuLocationKeys } from "../method"

interface Detecter {
    (keys: LoadMenuLocationKeys): LoadMenuLocationDetectMethod
}
export const detectMenuTargetPath: Detecter = (keys) => (currentURL) => {
    return menuTargetPathLocationConverter(currentURL, keys.version)
}
