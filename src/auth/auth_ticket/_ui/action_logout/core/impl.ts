import { ApplicationAbstractStateAction } from "../../../../../../ui/vendor/getto-application/action/init"
import { logout } from "../../logout/method"
import { LogoutInfra } from "../../logout/infra"

import {
    initialLogoutCoreState,
    LogoutCoreAction,
    LogoutCoreMaterial,
    LogoutCoreState,
} from "./action"

export function initLogoutCoreMaterial(infra: LogoutInfra): LogoutCoreMaterial {
    return {
        clear: logout(infra),
    }
}

export function initLogoutCoreAction(material: LogoutCoreMaterial): LogoutCoreAction {
    return new Action(material)
}

class Action extends ApplicationAbstractStateAction<LogoutCoreState> implements LogoutCoreAction {
    readonly initialState = initialLogoutCoreState

    material: LogoutCoreMaterial

    constructor(material: LogoutCoreMaterial) {
        super()
        this.material = material
    }

    submit(): Promise<LogoutCoreState> {
        return this.material.clear(this.post)
    }
}
