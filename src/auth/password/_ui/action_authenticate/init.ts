import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { initSignLink } from "../../../_ui/common/nav/action_nav/init"
import { initInputLoginIDAction } from "../../../login_id/_ui/action_input/init"
import { initInputPasswordAction } from "../action_input/init"
import { initValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/init"

import {
    saveAuthTicket,
    startContinuousRenew,
} from "../../../auth_ticket/_ui/start_continuous_renew/method"
import {
    GetScriptPathDetecter,
    getScriptPath,
} from "../../../_ui/common/secure/get_script_path/method"
import { authenticatePassword } from "../authenticate/method"

import { AuthenticatePasswordInfra } from "../authenticate/infra"
import { StartContinuousRenewInfra } from "../../../auth_ticket/_ui/start_continuous_renew/infra"
import { GetScriptPathInfra } from "../../../_ui/common/secure/get_script_path/infra"

import {
    AuthenticatePasswordMaterial,
    AuthenticatePasswordAction,
    AuthenticatePasswordState,
    initialAuthenticatePasswordState,
    ValidateAuthenticatePasswordFieldsAction,
    authenticatePasswordFields,
} from "./action"

import { LoadScriptError } from "../../../_ui/common/secure/get_script_path/data"
import { AuthenticatePasswordFields } from "../authenticate/data"
import { AuthTicket } from "../../../auth_ticket/_ui/kernel/data"
import { ConvertBoardResult } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export type AuthenticatePasswordActionInfra = Readonly<{
    startContinuousRenew: StartContinuousRenewInfra
    getSecureScriptPath: GetScriptPathInfra
    authenticate: AuthenticatePasswordInfra
}>

export function initAuthenticatePasswordMaterial(
    infra: AuthenticatePasswordActionInfra,
    locationInfo: GetScriptPathDetecter,
): AuthenticatePasswordMaterial {
    return {
        save: saveAuthTicket(infra.startContinuousRenew),
        startContinuousRenew: startContinuousRenew(infra.startContinuousRenew),
        getSecureScriptPath: getScriptPath(infra.getSecureScriptPath)(locationInfo),
        authenticate: authenticatePassword(infra.authenticate),
    }
}

export function initAuthenticatePasswordAction(
    material: AuthenticatePasswordMaterial,
): AuthenticatePasswordAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<AuthenticatePasswordState>
    implements AuthenticatePasswordAction
{
    readonly initialState = initialAuthenticatePasswordState

    readonly link = initSignLink()

    readonly loginID = initInputLoginIDAction()
    readonly password = initInputPasswordAction()
    readonly validate: ValidateAuthenticatePasswordFieldsAction

    material: AuthenticatePasswordMaterial

    constructor(material: AuthenticatePasswordMaterial) {
        super()
        this.material = material

        this.validate = initValidateBoardAction({
            fields: authenticatePasswordFields,
            converter: (): ConvertBoardResult<AuthenticatePasswordFields> => {
                const result = {
                    loginID: this.loginID.validate.get(),
                    password: this.password.validate.get(),
                }
                if (!result.loginID.valid || !result.password.valid) {
                    return { valid: false }
                }
                return {
                    valid: true,
                    value: {
                        loginID: result.loginID.value,
                        password: result.password.value,
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
    async submit(
        fields: ConvertBoardResult<AuthenticatePasswordFields>,
    ): Promise<AuthenticatePasswordState> {
        return this.material.authenticate(fields, (event) => {
            switch (event.type) {
                case "succeed-to-login":
                    return this.startContinuousRenew(event.auth)

                default:
                    return this.post(event)
            }
        })
    }
    async startContinuousRenew(info: AuthTicket): Promise<AuthenticatePasswordState> {
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

    async loadError(err: LoadScriptError): Promise<AuthenticatePasswordState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return this.material.getSecureScriptPath()
    }
}
