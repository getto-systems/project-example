import { env } from "../../../y_environment/env"

import { initGetCurrentVersionResource } from "./impl"
import { initGetCurrentVersionCoreAction } from "./core/impl"

import { GetCurrentVersionResource } from "./resource"

export function newGetCurrentVersionResource(): GetCurrentVersionResource {
    return initGetCurrentVersionResource(
        initGetCurrentVersionCoreAction({
            version: env.version,
        }),
    )
}
