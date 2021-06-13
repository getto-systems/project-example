import { ApplicationAbstractStateAction } from "../../../../../../../ui/vendor/getto-application/action/impl"

import { resetPassword } from "../../reset/method"
import {
    GetScriptPathDetecter,
    getScriptPath,
} from "../../../../../_ui/common/secure/get_script_path/method"
import {
    saveAuthTicket,
    startContinuousRenew,
} from "../../../../../auth_ticket/_ui/start_continuous_renew/method"

import { StartContinuousRenewInfra } from "../../../../../auth_ticket/_ui/start_continuous_renew/infra"
import { GetScriptPathInfra } from "../../../../../_ui/common/secure/get_script_path/infra"
import { ResetPasswordInfra } from "../../reset/infra"

import {
    ResetPasswordCoreMaterial,
    ResetPasswordCoreAction,
    ResetPasswordCoreState,
    initialResetPasswordCoreState,
} from "./action"

import { ResetPasswordDetecter } from "../../reset/method"

import { LoadScriptError } from "../../../../../_ui/common/secure/get_script_path/data"
import { ResetPasswordFields } from "../../reset/data"
import { AuthTicket } from "../../../../../auth_ticket/_ui/kernel/data"
import { ConvertBoardResult } from "../../../../../../../ui/vendor/getto-application/board/kernel/data"

export type ResetPasswordCoreInfra = Readonly<{
    startContinuousRenew: StartContinuousRenewInfra
    getSecureScriptPath: GetScriptPathInfra
    reset: ResetPasswordInfra
}>

export type ResetPasswordCoreForegroundDetecter = Readonly<{
    getSecureScriptPath: GetScriptPathDetecter
}>
export type ResetPasswordCoreBackgroundDetecter = Readonly<{
    reset: ResetPasswordDetecter
}>

export function initResetPasswordCoreMaterial(
    infra: ResetPasswordCoreInfra,
    detecter: ResetPasswordCoreForegroundDetecter & ResetPasswordCoreBackgroundDetecter,
): ResetPasswordCoreMaterial {
    return {
        save: saveAuthTicket(infra.startContinuousRenew),
        startContinuousRenew: startContinuousRenew(infra.startContinuousRenew),
        getSecureScriptPath: getScriptPath(infra.getSecureScriptPath)(detecter.getSecureScriptPath),
        reset: resetPassword(infra.reset)(detecter.reset),
    }
}

export function initResetPasswordCoreAction(
    material: ResetPasswordCoreMaterial,
): ResetPasswordCoreAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<ResetPasswordCoreState>
    implements ResetPasswordCoreAction
{
    readonly initialState = initialResetPasswordCoreState

    material: ResetPasswordCoreMaterial

    constructor(material: ResetPasswordCoreMaterial) {
        super()
        this.material = material
    }

    async submit(fields: ConvertBoardResult<ResetPasswordFields>): Promise<ResetPasswordCoreState> {
        return this.material.reset(fields, (event) => {
            switch (event.type) {
                case "succeed-to-reset":
                    return this.startContinuousRenew(event.auth)

                default:
                    return this.post(event)
            }
        })
    }
    async startContinuousRenew(info: AuthTicket): Promise<ResetPasswordCoreState> {
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

    async loadError(err: LoadScriptError): Promise<ResetPasswordCoreState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return this.material.getSecureScriptPath()
    }
}
