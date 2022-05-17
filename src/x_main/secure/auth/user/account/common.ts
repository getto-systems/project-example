import { initBaseView } from "../../../../../common/base/init"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { ManageUserAccountPageResource } from "./resource"

export type ManageUserAccountViewProps = Readonly<{ resource: ManageUserAccountPageResource }> &
    Partial<{ terminate: { (): void } }>

export function toManageUserAccountView(
    props: ManageUserAccountViewProps,
): ApplicationView<ManageUserAccountPageResource> {
    const { resource } = props
    return initBaseView(resource, () => {
        if (props.terminate) {
            props.terminate()
        }
        resource.search.terminate()
    })
}
