import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { initInputLoginIdAction } from "../../login_id/input/action"
import { initInputPasswordAction } from "../input/action"
import { initValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"

import { InputLoginIdAction } from "../../login_id/input/action"
import { InputPasswordAction } from "../input/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"
import { getScriptPath } from "../../../sign/get_script_path/method"
import {
    startContinuousRenew,
    StartContinuousRenewConfig,
    StartContinuousRenewInfra,
    StartContinuousRenewEvent,
} from "../../../ticket/check/method"

import { DelayTime } from "../../../../z_lib/ui/config/infra"
import { GetScriptPathConfig, GetScriptPathShell } from "../../../sign/get_script_path/infra"
import { AuthenticatePasswordRemote } from "./infra"
import { BoardConverter } from "../../../../z_vendor/getto-application/board/kernel/infra"

import { LoadScriptError, ConvertScriptPathResult } from "../../../sign/get_script_path/data"
import { AuthenticatePasswordError, AuthenticatePasswordFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { AuthTicket } from "../../../ticket/kernel/data"
import { RepositoryError } from "../../../../z_lib/ui/repository/data"

export interface AuthenticatePasswordAction
    extends StatefulApplicationAction<AuthenticatePasswordState> {
    readonly loginId: InputLoginIdAction
    readonly password: InputPasswordAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): AuthenticatePasswordState
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
    takeLongtimeThreshold: DelayTime
}> &
    StartContinuousRenewConfig &
    GetScriptPathConfig

export function initAuthenticatePasswordAction(
    material: AuthenticatePasswordMaterial,
): AuthenticatePasswordAction {
    return new Action(material)
}

class Action
    extends AbstractStatefulApplicationAction<AuthenticatePasswordState>
    implements AuthenticatePasswordAction
{
    readonly initialState = initialState

    readonly loginId: InputLoginIdAction
    readonly password: InputPasswordAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: AuthenticatePasswordMaterial
    convert: BoardConverter<AuthenticatePasswordFields>

    constructor(material: AuthenticatePasswordMaterial) {
        super({
            terminate: () => {
                this.loginId.terminate()
                this.password.terminate()
                this.validate.terminate()
                this.observe.terminate()
            },
        })
        this.material = material

        const loginId = initInputLoginIdAction()
        const password = initInputPasswordAction()

        const fields = ["loginId", "password"] as const
        const convert = (): ConvertBoardResult<AuthenticatePasswordFields> => {
            const result = {
                loginId: loginId.checker.check(),
                password: password.checker.check(),
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

        const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.loginId = loginId.input
        this.password = password.input
        this.validate = validate
        this.observe = observe
        this.convert = convert

        this.loginId.validate.subscriber.subscribe((result) =>
            validateChecker.update("loginId", result.valid),
        )
        this.loginId.observe.subscriber.subscribe((result) =>
            observeChecker.update("loginId", result.hasChanged),
        )
        this.password.validate.subscriber.subscribe((result) =>
            validateChecker.update("password", result.valid),
        )
        this.password.observe.subscriber.subscribe((result) =>
            observeChecker.update("password", result.hasChanged),
        )
    }

    clear(): AuthenticatePasswordState {
        this.loginId.clear()
        this.password.clear()
        this.validate.clear()
        return this.initialState
    }
    async submit(): Promise<AuthenticatePasswordState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        const result = await authenticate(this.material, fields.value, this.post)
        if (!result.success) {
            return result.state
        }
        return this.startContinuousRenew(result.ticket)
    }
    async startContinuousRenew(ticket: AuthTicket): Promise<AuthenticatePasswordState> {
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

    async loadError(err: LoadScriptError): Promise<AuthenticatePasswordState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return getScriptPath(this.material)
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
    const response = await delayedChecker(
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
