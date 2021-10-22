import { initBaseView } from "../../../../../../src/example/action_base/init"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { ManageUserAccountPageResource } from "./resource"

export type ManageUserAccountViewProps =
    | Readonly<{ resource: ManageUserAccountPageResource }>
    | Readonly<{ resource: ManageUserAccountPageResource; terminate: { (): void } }>

export function toManageUserAccountView(props: ManageUserAccountViewProps): ApplicationView<ManageUserAccountPageResource> {
    const { resource } = props
    return initBaseView(resource, () => {
        if ("terminate" in props) {
            props.terminate()
        }
        resource.search.terminate()
    })
}
