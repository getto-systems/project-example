import { initBaseView } from "../../../../core/action_base/init"

import { ApplicationView } from "../../../../z_vendor/getto-application/action/action"
import { ProfilePageResource } from "./resource"

export type ProfileViewProps = Readonly<{ resource: ProfilePageResource; terminate?: { (): void } }>

export function toProfileView(props: ProfileViewProps): ApplicationView<ProfilePageResource> {
    const { resource } = props
    return initBaseView(resource, () => {
        if (props.terminate) {
            props.terminate()
        }
        resource.change.terminate()
        resource.requestToken.terminate()
    })
}
