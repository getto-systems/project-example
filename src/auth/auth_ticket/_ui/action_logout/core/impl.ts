import { ApplicationAbstractStateAction } from "../../../../../../ui/vendor/getto-application/action/impl"
import { clearAuthTicket } from "../../clear/method"
import { ClearAuthTicketInfra } from "../../clear/infra"

import {
    initialLogoutCoreState,
    LogoutCoreAction,
    LogoutCoreMaterial,
    LogoutCoreState,
} from "./action"

export function initLogoutCoreMaterial(infra: ClearAuthTicketInfra): LogoutCoreMaterial {
    return {
        clear: clearAuthTicket(infra),
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
