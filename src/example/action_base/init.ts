import { ApplicationView } from "../../z_vendor/getto-application/action/action"
import { BaseResource } from "./resource"

export function initBaseView<R>(
    resource: R & BaseResource,
    terminate: { (): void },
): ApplicationView<R & BaseResource> {
    return {
        resource,
        terminate: () => {
            resource.menu.terminate()
            terminate()
        },
    }
}
