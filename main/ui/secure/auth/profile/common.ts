import { initBaseView } from "../../../../../src/example/action_base/init"

import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"
import { ProfilePageResource } from "./resource"

export type ProfileViewProps =
    | Readonly<{ resource: ProfilePageResource }>
    | Readonly<{ resource: ProfilePageResource; terminate: { (): void } }>

export function toProfileView(props: ProfileViewProps): ApplicationView<ProfilePageResource> {
    const { resource } = props
    return initBaseView(resource, () => {
        if ("terminate" in props) {
            props.terminate()
        }
        resource.change.terminate()
        resource.requestToken.terminate()
    })
}
