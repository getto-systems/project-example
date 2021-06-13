import { ApplicationAbstractStateAction } from "../../../../../../ui/vendor/getto-application/action/impl"

import {
    saveAuthTicket,
    startContinuousRenew,
} from "../../../../auth_ticket/_ui/start_continuous_renew/method"
import {
    GetScriptPathDetecter,
    getScriptPath,
} from "../../../../_ui/common/secure/get_script_path/method"
import { authenticatePassword } from "../../authenticate/impl/core"

import { AuthenticatePasswordInfra } from "../../authenticate/infra"
import { StartContinuousRenewInfra } from "../../../../auth_ticket/_ui/start_continuous_renew/infra"
import { GetScriptPathInfra } from "../../../../_ui/common/secure/get_script_path/infra"

import {
    AuthenticatePasswordCoreMaterial,
    AuthenticatePasswordCoreAction,
    AuthenticatePasswordCoreState,
    initialAuthenticatePasswordCoreState,
} from "./action"

import { LoadScriptError } from "../../../../_ui/common/secure/get_script_path/data"
import { AuthenticatePasswordFields } from "../../authenticate/data"
import { AuthTicket } from "../../../../auth_ticket/_ui/kernel/data"
import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export type AuthenticatePasswordCoreInfra = Readonly<{
    startContinuousRenew: StartContinuousRenewInfra
    getSecureScriptPath: GetScriptPathInfra
    authenticate: AuthenticatePasswordInfra
}>

export function initAuthenticatePasswordCoreMaterial(
    infra: AuthenticatePasswordCoreInfra,
    locationInfo: GetScriptPathDetecter,
): AuthenticatePasswordCoreMaterial {
    return {
        save: saveAuthTicket(infra.startContinuousRenew),
        startContinuousRenew: startContinuousRenew(infra.startContinuousRenew),
        getSecureScriptPath: getScriptPath(infra.getSecureScriptPath)(locationInfo),
        authenticate: authenticatePassword(infra.authenticate),
    }
}

export function initAuthenticatePasswordCoreAction(
    material: AuthenticatePasswordCoreMaterial,
): AuthenticatePasswordCoreAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<AuthenticatePasswordCoreState>
    implements AuthenticatePasswordCoreAction
{
    readonly initialState = initialAuthenticatePasswordCoreState

    material: AuthenticatePasswordCoreMaterial

    constructor(material: AuthenticatePasswordCoreMaterial) {
        super()
        this.material = material
    }

    async submit(
        fields: ConvertBoardResult<AuthenticatePasswordFields>,
    ): Promise<AuthenticatePasswordCoreState> {
        return this.material.authenticate(fields, (event) => {
            switch (event.type) {
                case "succeed-to-login":
                    return this.startContinuousRenew(event.auth)

                default:
                    return this.post(event)
            }
        })
    }
    async startContinuousRenew(info: AuthTicket): Promise<AuthenticatePasswordCoreState> {
        return this.material.save(info, (event) => {
            switch (event.type) {
                case "failed-to-save":
                    return this.post({ type: "repository-error", continue: false, err: event.err })

                case "succeed-to-save":
                    return this.material.startContinuousRenew((event) => {
                        switch (event.type) {
                            case "succeed-to-start-continuous-renew":
                                return this.post({
                                    type: "try-to-load",
                                    scriptPath: this.secureScriptPath(),
                                })

                            default:
                                return this.post(event)
                        }
                    })
            }
        })
    }

    async loadError(err: LoadScriptError): Promise<AuthenticatePasswordCoreState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return this.material.getSecureScriptPath()
    }
}
