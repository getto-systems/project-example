import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { logout } from "../logout/method"

import { LogoutInfra } from "../logout/infra"

import {
    initialLogoutState,
    LogoutAction,
    LogoutMaterial,
    LogoutState,
} from "./action"

export function initLogoutMaterial(infra: LogoutInfra): LogoutMaterial {
    return {
        clear: logout(infra),
    }
}

export function initLogoutAction(material: LogoutMaterial): LogoutAction {
    return new Action(material)
}

class Action extends ApplicationAbstractStateAction<LogoutState> implements LogoutAction {
    readonly initialState = initialLogoutState

    material: LogoutMaterial

    constructor(material: LogoutMaterial) {
        super()
        this.material = material
    }

    submit(): Promise<LogoutState> {
        return this.material.clear(this.post)
    }
}
