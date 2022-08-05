import {
    ApplicationState,
    initApplicationStateAction,
    StatefulApplicationAction,
} from "../../../../../z_vendor/getto-application/action/action"

import { LoginIdFieldAction, initLoginIdFieldAction } from "../../../login_id/input/action"
import { PasswordFieldAction, initPasswordFieldAction } from "../../input/action"
import { ValidateBoardAction } from "../../../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardAction } from "../../../../../z_vendor/getto-application/board/observe_board/action"
import { initRegisterField } from "../../../../../z_lib/ui/register/action"

import { checkTakeLongtime } from "../../../../../z_lib/ui/timer/helper"

import { getScriptPath } from "../../../../sign/get_script_path/method"
import {
    startContinuousRenew,
    StartContinuousRenewConfig,
    StartContinuousRenewInfra,
    StartContinuousRenewEvent,
} from "../../../../ticket/check/method"

import { GetScriptPathConfig, GetScriptPathShell } from "../../../../sign/get_script_path/infra"
import { WaitTime } from "../../../../../z_lib/ui/config/infra"
import { ResetPasswordRemote, ResetTokenDetecter } from "./infra"

import { LoadScriptError, ConvertScriptPathResult } from "../../../../sign/get_script_path/data"
import { ResetPasswordError, ResetPasswordFields } from "./data"
import { AuthTicket } from "../../../../ticket/kernel/data"
import { ConvertBoardResult } from "../../../../../z_vendor/getto-application/board/kernel/data"
import { RepositoryError } from "../../../../../z_lib/ui/repository/data"

export interface ResetPasswordAction extends StatefulApplicationAction<ResetPasswordState> {
    readonly loginId: LoginIdFieldAction
    readonly password: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): void
    submit(): Promise<ResetPasswordState>
    loadError(err: LoadScriptError): Promise<ResetPasswordState>
}

export type ResetPasswordState =
    | Readonly<{ type: "initial-reset" }>
    | ResetEvent
    | Exclude<StartContinuousRenewEvent, { type: "succeed-to-start-continuous-renew" }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

const initialState: ResetPasswordState = { type: "initial-reset" }

export type ResetPasswordMaterial = Readonly<{
    infra: ResetPasswordInfra
    shell: ResetPasswordShell
    config: ResetPasswordConfig
}>

export type ResetPasswordInfra = Readonly<{
    resetRemote: ResetPasswordRemote
}> &
    StartContinuousRenewInfra

export type ResetPasswordShell = Readonly<{
    detectResetToken: ResetTokenDetecter
}> &
    GetScriptPathShell

export type ResetPasswordConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
}> &
    StartContinuousRenewConfig &
    GetScriptPathConfig

export function initResetPasswordAction(material: ResetPasswordMaterial): ResetPasswordAction {
    return new Action(material)
}

class Action implements ResetPasswordAction {
    readonly material: ResetPasswordMaterial
    readonly state: ApplicationState<ResetPasswordState>
    readonly post: (state: ResetPasswordState) => ResetPasswordState

    readonly loginId: LoginIdFieldAction
    readonly password: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    convert: () => ConvertBoardResult<ResetPasswordFields>
    clear: () => void

    constructor(material: ResetPasswordMaterial) {
        const { state, post } = initApplicationStateAction({ initialState })
        this.material = material
        this.state = state
        this.post = post

        const loginId = initLoginIdFieldAction()
        const password = initPasswordFieldAction()

        const convert = (): ConvertBoardResult<ResetPasswordFields> => {
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
                    newPassword: result.password.value,
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

        this.loginId = loginId
        this.password = password
        this.validate = validate
        this.observe = observe
        this.convert = convert
        this.clear = clear
    }

    async submit(): Promise<ResetPasswordState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.state.currentState()
        }
        const result = await reset(this.material, fields.value, this.post)
        if (!result.success) {
            return result.state
        }
        return this.startContinuousRenew(result.ticket)
    }
    async startContinuousRenew(ticket: AuthTicket): Promise<ResetPasswordState> {
        return await startContinuousRenew(this.material, { hasTicket: true, ticket }, (event) => {
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

    async loadError(err: LoadScriptError): Promise<ResetPasswordState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return getScriptPath(this.material)
    }
}

type ResetEvent =
    | Readonly<{ type: "try-to-reset"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed-to-reset"; err: ResetPasswordError }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

type ResetResult<S> =
    | Readonly<{ success: true; ticket: AuthTicket }>
    | Readonly<{ success: false; state: S }>

async function reset<S>(
    { infra, shell, config }: ResetPasswordMaterial,
    fields: ResetPasswordFields,
    post: Post<ResetEvent, S>,
): Promise<ResetResult<S>> {
    const resetToken = shell.detectResetToken()
    if (!resetToken.valid) {
        return {
            success: false,
            state: post({ type: "failed-to-reset", err: { type: "empty-reset-token" } }),
        }
    }

    post({ type: "try-to-reset", hasTakenLongtime: false })

    const { resetRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        resetRemote(resetToken.value, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try-to-reset", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return { success: false, state: post({ type: "failed-to-reset", err: response.err }) }
    }

    return { success: true, ticket: response.value }
}

interface Post<E, S> {
    (event: E): S
}
