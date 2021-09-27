import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { initSignLink } from "../../../sign/action_nav/init"
import { initInputLoginIDAction } from "../../login_id/input/action_input/init"
import { initInputPasswordAction } from "../action_input/init"
import { initValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/init"

import {
    saveAuthTicket,
    startContinuousRenew,
} from "../../../ticket/start_continuous_renew/method"
import {
    GetScriptPathDetecter,
    getScriptPath,
} from "../../../sign/get_script_path/method"
import { authenticatePassword } from "../authenticate/method"

import { AuthenticatePasswordInfra } from "../authenticate/infra"
import { StartContinuousRenewInfra } from "../../../ticket/start_continuous_renew/infra"
import { GetScriptPathInfra } from "../../../sign/get_script_path/infra"

import {
    AuthenticatePasswordMaterial,
    AuthenticatePasswordAction,
    AuthenticatePasswordState,
    initialAuthenticatePasswordState,
    authenticatePasswordFieldNames,
    AuthenticatePasswordFieldName,
} from "./action"
import { ValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { LoadScriptError } from "../../../sign/get_script_path/data"
import { AuthenticatePasswordFields } from "../authenticate/data"
import { AuthTicket } from "../../../ticket/kernel/data"
import { ConvertBoardResult } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { InputLoginIDAction } from "../../login_id/input/action_input/action"
import { InputPasswordAction } from "../action_input/action"
import { ValidateBoardChecker } from "../../../../../ui/vendor/getto-application/board/validate_board/infra"

export type AuthenticatePasswordActionInfra = Readonly<{
    startContinuousRenew: StartContinuousRenewInfra
    getSecureScriptPath: GetScriptPathInfra
    authenticate: AuthenticatePasswordInfra
}>

export function initAuthenticatePasswordMaterial(
    infra: AuthenticatePasswordActionInfra,
): AuthenticatePasswordMaterial {
    return {
        save: saveAuthTicket(infra.startContinuousRenew),
        startContinuousRenew: startContinuousRenew(infra.startContinuousRenew),
        getSecureScriptPath: getScriptPath(infra.getSecureScriptPath),
        authenticate: authenticatePassword(infra.authenticate),
    }
}

export function initAuthenticatePasswordAction(
    material: AuthenticatePasswordMaterial,
    detecter: GetScriptPathDetecter,
): AuthenticatePasswordAction {
    return new Action(material, detecter)
}

class Action
    extends ApplicationAbstractStateAction<AuthenticatePasswordState>
    implements AuthenticatePasswordAction
{
    readonly initialState = initialAuthenticatePasswordState

    readonly link = initSignLink()

    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateBoardAction

    material: AuthenticatePasswordMaterial
    detecter: GetScriptPathDetecter
    checker: ValidateBoardChecker<AuthenticatePasswordFieldName, AuthenticatePasswordFields>

    constructor(material: AuthenticatePasswordMaterial, detecter: GetScriptPathDetecter) {
        super()
        this.material = material
        this.detecter = detecter

        const loginID = initInputLoginIDAction()
        const password = initInputPasswordAction()
        const { validate, checker } = initValidateBoardAction({
            fields: authenticatePasswordFieldNames,
            converter: (): ConvertBoardResult<AuthenticatePasswordFields> => {
                const result = {
                    loginID: loginID.checker.get(),
                    password: password.checker.get(),
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

        this.loginID = loginID.input
        this.password = password.input
        this.validate = validate
        this.checker = checker

        this.loginID.validate.subscriber.subscribe((result) =>
            checker.update("loginID", result.valid),
        )
        this.password.validate.subscriber.subscribe((result) =>
            checker.update("password", result.valid),
        )

        this.terminateHook(() => {
            this.loginID.terminate()
            this.password.terminate()
            this.validate.terminate()
        })
    }

    clear(): AuthenticatePasswordState {
        this.loginID.clear()
        this.password.clear()
        this.validate.clear()
        return this.initialState
    }
    async submit(): Promise<AuthenticatePasswordState> {
        return this.material.authenticate(this.checker.get(), (event) => {
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
        return this.material.getSecureScriptPath(this.detecter())
    }
}
