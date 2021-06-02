import { GetCurrentVersionResource } from "./resource"
import { GetCurrentVersionCoreAction } from "./core/action"

export function initGetCurrentVersionResource(
    version: GetCurrentVersionCoreAction,
): GetCurrentVersionResource {
    return { version }
}
