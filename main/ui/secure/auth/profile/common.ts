import { initBaseView } from "../../../../../src/example/action_base/init"

import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"
import { ProfilePageResource } from "./resource"

export function toProfileView(
    resource: ProfilePageResource,
    terminate: { (): void },
): ApplicationView<ProfilePageResource> {
    return initBaseView(resource, () => {
        terminate()
        resource.change.terminate()
        resource.requestToken.terminate()
    })
}
