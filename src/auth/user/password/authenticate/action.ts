import {
    ApplicationState,
    initApplicationState,
} from "../../../../z_vendor/getto-application/action/action"

import { LoginIdFieldAction, initLoginIdFieldAction } from "../../login_id/input/action"
import { PasswordFieldAction, initPasswordFieldAction } from "../input/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardAction } from "../../../../z_vendor/getto-application/board/observe_board/action"
import { initRegisterField } from "../../../../z_lib/ui/register/action"

import { checkTakeLongtime } from "../../../../z_lib/ui/timer/helper"
import { getScriptPath } from "../../../sign/get_script_path/method"
import {
    startContinuousRenew,
    StartContinuousRenewConfig,
    StartContinuousRenewInfra,
    StartContinuousRenewEvent,
} from "../../../ticket/check/method"

import { WaitTime } from "../../../../z_lib/ui/config/infra"
import { GetScriptPathConfig, GetScriptPathShell } from "../../../sign/get_script_path/infra"
import { AuthenticatePasswordRemote } from "./infra"

import { LoadScriptError, ConvertScriptPathResult } from "../../../sign/get_script_path/data"
import { AuthenticatePasswordError, AuthenticatePasswordFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { AuthTicket } from "../../../ticket/kernel/data"
import { RepositoryError } from "../../../../z_lib/ui/repository/data"

export interface AuthenticatePasswordAction {
    readonly state: ApplicationState<AuthenticatePasswordState>
    readonly loginId: LoginIdFieldAction
    readonly password: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): void
    submit(): Promise<AuthenticatePasswordState>
    loadError(err: LoadScriptError): Promise<AuthenticatePasswordState>
}

export type AuthenticatePasswordState =
    | Readonly<{ type: "initial-login" }>
    | AuthenticateEvent
    | Exclude<StartContinuousRenewEvent, { type: "succeed-to-start-continuous-renew" }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

const initialState: AuthenticatePasswordState = { type: "initial-login" }

export type AuthenticatePasswordMaterial = Readonly<{
    infra: AuthenticatePasswordInfra
    shell: AuthenticatePasswordShell
    config: AuthenticatePasswordConfig
}>

export type AuthenticatePasswordInfra = Readonly<{
    authenticateRemote: AuthenticatePasswordRemote
}> &
    StartContinuousRenewInfra

export type AuthenticatePasswordShell = GetScriptPathShell

export type AuthenticatePasswordConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
}> &
    StartContinuousRenewConfig &
    GetScriptPathConfig

export function initAuthenticatePasswordAction(
    material: AuthenticatePasswordMaterial,
): AuthenticatePasswordAction {
    const { state, post } = initApplicationState({ initialState })

    const loginId = initLoginIdFieldAction()
    const password = initPasswordFieldAction()

    const convert = (): ConvertBoardResult<AuthenticatePasswordFields> => {
        const result = {
            loginId: loginId.validate.check(),
            password: password.validate.check(),
        }
        if (!result.loginId.valid || !result.password.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                loginId: result.loginId.value,
                password: result.password.value,
            },
        }
    }

    const { validate, observe, clear } = initRegisterField(
        [
            ["loginId", loginId],
            ["password", password],
        ],
        convert,
    )

    return {
        state,

        loginId,
        password,

        validate,
        observe,

        clear,

        async submit(): Promise<AuthenticatePasswordState> {
            const fields = convert()
            if (!fields.valid) {
                return state.currentState()
            }
            const result = await authenticate(material, fields.value, post)
            if (!result.success) {
                return result.state
            }
            return start(result.ticket)
        },
        async loadError(err: LoadScriptError): Promise<AuthenticatePasswordState> {
            return post({ type: "load-error", err })
        },
    }

    async function start(ticket: AuthTicket): Promise<AuthenticatePasswordState> {
        return await startContinuousRenew(material, { hasTicket: true, ticket }, (event) => {
            switch (event.type) {
                case "succeed-to-start-continuous-renew":
                    return post({
                        type: "try-to-load",
                        scriptPath: scriptPath(),
                    })
                default:
                    return post(event)
            }
        })
    }

    function scriptPath() {
        return getScriptPath(material)
    }
}

type AuthenticateMaterial = Readonly<{
    infra: AuthenticatePasswordInfra
    config: AuthenticatePasswordConfig
}>

type AuthenticateEvent =
    | Readonly<{ type: "try-to-login"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed-to-login"; err: AuthenticatePasswordError }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

type AuthenticateResult<S> =
    | Readonly<{ success: true; ticket: AuthTicket }>
    | Readonly<{ success: false; state: S }>

async function authenticate<S>(
    { infra, config }: AuthenticateMaterial,
    fields: AuthenticatePasswordFields,
    post: Post<AuthenticateEvent, S>,
): Promise<AuthenticateResult<S>> {
    post({ type: "try-to-login", hasTakenLongtime: false })

    const { authenticateRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        authenticateRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try-to-login", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return { success: false, state: post({ type: "failed-to-login", err: response.err }) }
    }

    return { success: true, ticket: response.value }
}

interface Post<E, S> {
    (event: E): S
}
