import { initBaseView } from "../../../../../common/base/init"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { RegisterUserAccountPageResource } from "./resource"

export type RegisterUserAccountViewProps = Readonly<{ resource: RegisterUserAccountPageResource }> &
    Partial<{ terminate: { (): void } }>

export function toRegisterUserAccountView(
    props: RegisterUserAccountViewProps,
): ApplicationView<RegisterUserAccountPageResource> {
    const { resource } = props
    return initBaseView(resource, () => {
        if (props.terminate) {
            props.terminate()
        }
        resource.register.terminate()
    })
}
