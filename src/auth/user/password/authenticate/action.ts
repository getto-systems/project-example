import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"
import { initSignLink } from "../../../sign/action_nav/init"
import { initInputLoginIDAction } from "../../login_id/input/action_input/init"
import { initInputPasswordAction } from "../input/action_input/init"
import { initValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/init"

import { SignLink } from "../../../sign/action_nav/resource"
import { InputLoginIDAction } from "../../login_id/input/action_input/action"
import { InputPasswordAction } from "../input/action_input/action"
import { ValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"
import { getScriptPath } from "../../../sign/get_script_path/method"
import { startContinuousRenew } from "../../../ticket/start_continuous_renew/method"

import {
    StartContinuousRenewConfig,
    StartContinuousRenewInfra,
} from "../../../ticket/start_continuous_renew/infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"
import { GetScriptPathConfig, GetScriptPathShell } from "../../../sign/get_script_path/infra"
import { ValidateBoardChecker } from "../../../../../ui/vendor/getto-application/board/validate_board/infra"
import { AuthenticatePasswordRemote } from "./infra"

import { StartContinuousRenewEvent } from "../../../ticket/start_continuous_renew/event"

import { LoadScriptError, ConvertScriptPathResult } from "../../../sign/get_script_path/data"
import { AuthenticatePasswordError, AuthenticatePasswordFields } from "./data"
import { ConvertBoardResult } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { AuthTicket } from "../../../ticket/kernel/data"
import { RepositoryError } from "../../../../z_lib/ui/repository/data"

export interface AuthenticatePasswordAction
    extends ApplicationStateAction<AuthenticatePasswordState> {
    readonly link: SignLink

    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateBoardAction

    clear(): AuthenticatePasswordState
    submit(): Promise<AuthenticatePasswordState>
    loadError(err: LoadScriptError): Promise<AuthenticatePasswordState>
}

export const authenticatePasswordFieldNames = ["loginID", "password"] as const
export type AuthenticatePasswordFieldName = typeof authenticatePasswordFieldNames[number]

export type AuthenticatePasswordState =
    | Readonly<{ type: "initial-login" }>
    | AuthenticateEvent
    | Exclude<StartContinuousRenewEvent, { type: "succeed-to-start-continuous-renew" }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

export const initialAuthenticatePasswordState: AuthenticatePasswordState = {
    type: "initial-login",
}

export type AuthenticatePasswordConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}> &
    StartContinuousRenewConfig &
    GetScriptPathConfig

export type AuthenticatePasswordInfra = Readonly<{
    authenticateRemote: AuthenticatePasswordRemote
}> &
    StartContinuousRenewInfra

export type AuthenticatePasswordShell = GetScriptPathShell

export function initAuthenticatePasswordAction(
    config: AuthenticatePasswordConfig,
    infra: AuthenticatePasswordInfra,
    shell: AuthenticatePasswordShell,
): AuthenticatePasswordAction {
    return new Action(config, infra, shell)
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

    config: AuthenticatePasswordConfig
    infra: AuthenticatePasswordInfra
    shell: AuthenticatePasswordShell
    checker: ValidateBoardChecker<AuthenticatePasswordFieldName, AuthenticatePasswordFields>

    constructor(
        config: AuthenticatePasswordConfig,
        infra: AuthenticatePasswordInfra,
        shell: AuthenticatePasswordShell,
    ) {
        super()
        this.config = config
        this.infra = infra
        this.shell = shell

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
        const result = await authenticate(this.config, this.infra, this.checker.get(), this.post)
        if (!result.success) {
            return result.state
        }
        return this.startContinuousRenew(result.ticket)
    }
    async startContinuousRenew(ticket: AuthTicket): Promise<AuthenticatePasswordState> {
        return await startContinuousRenew(
            this.config,
            this.infra,
            { hold: true, ticket },
            (event) => {
                switch (event.type) {
                    case "succeed-to-start-continuous-renew":
                        return this.post({
                            type: "try-to-load",
                            scriptPath: this.secureScriptPath(),
                        })
                    default:
                        return this.post(event)
                }
            },
        )
    }

    async loadError(err: LoadScriptError): Promise<AuthenticatePasswordState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return getScriptPath(this.config, this.shell)
    }
}

type AuthenticateEvent =
    | Readonly<{ type: "try-to-login" }>
    | Readonly<{ type: "take-longtime-to-login" }>
    | Readonly<{ type: "failed-to-login"; err: AuthenticatePasswordError }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

type AuthenticateResult<S> =
    | Readonly<{ success: true; ticket: AuthTicket }>
    | Readonly<{ success: false; state: S }>

async function authenticate<S>(
    config: AuthenticatePasswordConfig,
    infra: AuthenticatePasswordInfra,
    fields: ConvertBoardResult<AuthenticatePasswordFields>,
    post: Post<AuthenticateEvent, S>,
): Promise<AuthenticateResult<S>> {
    if (!fields.valid) {
        return {
            success: false,
            state: post({ type: "failed-to-login", err: { type: "validation-error" } }),
        }
    }

    post({ type: "try-to-login" })

    const { authenticateRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        authenticateRemote(fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-login" }),
    )
    if (!response.success) {
        return { success: false, state: post({ type: "failed-to-login", err: response.err }) }
    }

    return { success: true, ticket: response.value }
}

interface Post<E, S> {
    (event: E): S
}
