import { ApplicationAbstractStateAction } from "../../../../../../ui/vendor/getto-application/action/init"

import { initSignLink } from "../../../../_ui/common/nav/action_nav/init"
import { initInputLoginIDAction } from "../../../../login_id/_ui/action_input/init"
import { initInputPasswordAction } from "../../../_ui/action_input/init"
import { initValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/init"

import { resetPassword } from "../reset/method"
import {
    GetScriptPathDetecter,
    getScriptPath,
} from "../../../../_ui/common/secure/get_script_path/method"
import {
    saveAuthTicket,
    startContinuousRenew,
} from "../../../../auth_ticket/_ui/start_continuous_renew/method"

import { StartContinuousRenewInfra } from "../../../../auth_ticket/_ui/start_continuous_renew/infra"
import { GetScriptPathInfra } from "../../../../_ui/common/secure/get_script_path/infra"
import { ResetPasswordInfra } from "../reset/infra"

import {
    ResetPasswordMaterial,
    ResetPasswordAction,
    ResetPasswordState,
    initialResetPasswordState,
    resetPasswordFields,
    ValidateResetAction,
} from "./action"

import { ResetPasswordDetecter } from "../reset/method"

import { LoadScriptError } from "../../../../_ui/common/secure/get_script_path/data"
import { ResetPasswordFields } from "../reset/data"
import { AuthTicket } from "../../../../auth_ticket/_ui/kernel/data"
import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export type ResetPasswordActionInfra = Readonly<{
    startContinuousRenew: StartContinuousRenewInfra
    getSecureScriptPath: GetScriptPathInfra
    reset: ResetPasswordInfra
}>

// TODO Detecter について考え直す必要がある
export type ResetPasswordCoreForegroundDetecter = Readonly<{
    getSecureScriptPath: GetScriptPathDetecter
}>

export function initResetPasswordMaterial(
    infra: ResetPasswordActionInfra,
    detecter: ResetPasswordCoreForegroundDetecter,
): ResetPasswordMaterial {
    return {
        save: saveAuthTicket(infra.startContinuousRenew),
        startContinuousRenew: startContinuousRenew(infra.startContinuousRenew),
        getSecureScriptPath: getScriptPath(infra.getSecureScriptPath)(detecter.getSecureScriptPath),
        reset: resetPassword(infra.reset),
    }
}

export function initResetPasswordAction(
    material: ResetPasswordMaterial,
    detecter: ResetPasswordDetecter,
): ResetPasswordAction {
    return new Action(material, detecter)
}

class Action
    extends ApplicationAbstractStateAction<ResetPasswordState>
    implements ResetPasswordAction
{
    readonly initialState = initialResetPasswordState

    readonly link = initSignLink()

    readonly loginID = initInputLoginIDAction()
    readonly password = initInputPasswordAction()
    readonly validate: ValidateResetAction

    material: ResetPasswordMaterial
    detecter: ResetPasswordDetecter

    constructor(material: ResetPasswordMaterial, detecter: ResetPasswordDetecter) {
        super()
        this.material = material
        this.detecter = detecter

        this.validate = initValidateBoardAction({
            fields: resetPasswordFields,
            converter: (): ConvertBoardResult<ResetPasswordFields> => {
                const loginIDResult = this.loginID.validate.get()
                const passwordResult = this.password.validate.get()
                if (!loginIDResult.valid || !passwordResult.valid) {
                    return { valid: false }
                }
                return {
                    valid: true,
                    value: {
                        loginID: loginIDResult.value,
                        password: passwordResult.value,
                    },
                }
            },
        })

        this.loginID.validate.subscriber.subscribe(this.validate.updateValidateState("loginID"))
        this.password.validate.subscriber.subscribe(this.validate.updateValidateState("password"))

        this.terminateHook(() => {
            this.loginID.terminate()
            this.password.terminate()
            this.validate.terminate()
        })
    }

    clear(): void {
        this.loginID.clear()
        this.password.clear()
        this.validate.clear()
    }
    async submit(fields: ConvertBoardResult<ResetPasswordFields>): Promise<ResetPasswordState> {
        // TODO fields は this.validate から取得したい
        return this.material.reset(this.detecter(), fields, (event) => {
            switch (event.type) {
                case "succeed-to-reset":
                    return this.startContinuousRenew(event.auth)

                default:
                    return this.post(event)
            }
        })
    }
    async startContinuousRenew(info: AuthTicket): Promise<ResetPasswordState> {
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

    async loadError(err: LoadScriptError): Promise<ResetPasswordState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return this.material.getSecureScriptPath()
    }
}
