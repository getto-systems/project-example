import { checkTakeLongtime } from "../../../../../z_lib/ui/timer/helper"

import {
    ApplicationStateAction,
    initApplicationStateAction,
    StatefulApplicationAction,
} from "../../../../../z_vendor/getto-application/action/action"

import { LoginIdFieldAction, initLoginIdFieldAction } from "../../../login_id/input/action"
import {
    ValidateBoardAction,
    initValidateBoardAction,
} from "../../../../../z_vendor/getto-application/board/validate_board/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../../z_vendor/getto-application/board/observe_board/action"

import { RequestResetTokenRemote } from "./infra"
import { WaitTime } from "../../../../../z_lib/ui/config/infra"

import { RequestResetTokenError, RequestResetTokenFields } from "./data"
import { ConvertBoardResult } from "../../../../../z_vendor/getto-application/board/kernel/data"

export interface RequestResetTokenAction extends StatefulApplicationAction<RequestResetTokenState> {
    readonly loginId: LoginIdFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): RequestResetTokenState
    submit(onSuccess: { (): void }): Promise<RequestResetTokenState>
}

export type RequestResetTokenState = Readonly<{ type: "initial" }> | RequestResetTokenEvent

export type RequestResetTokenMaterial = Readonly<{
    infra: RequestResetTokenInfra
    config: RequestResetTokenConfig
}>

export type RequestResetTokenInfra = Readonly<{
    requestTokenRemote: RequestResetTokenRemote
}>

export type RequestResetTokenConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
}>

const initialState: RequestResetTokenState = { type: "initial" }

export function initRequestResetTokenAction(
    material: RequestResetTokenMaterial,
): RequestResetTokenAction {
    return new Action(material)
}

class Action implements RequestResetTokenAction {
    readonly material: RequestResetTokenMaterial
    readonly state: ApplicationStateAction<RequestResetTokenState>
    readonly post: (state: RequestResetTokenState) => RequestResetTokenState

    readonly loginId: LoginIdFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    convert: { (): ConvertBoardResult<RequestResetTokenFields> }

    constructor(material: RequestResetTokenMaterial) {
        const { state, post } = initApplicationStateAction({ initialState })
        this.material = material
        this.state = state
        this.post = post

        const loginId = initLoginIdFieldAction()

        // TODO register field を使う
        const fields = ["loginId"] as const
        const convert = (): ConvertBoardResult<RequestResetTokenFields> => {
            const loginIdResult = loginId.validate.check()
            if (!loginIdResult.valid) {
                return { valid: false }
            }
            return {
                valid: true,
                value: {
                    loginId: loginIdResult.value,
                },
            }
        }

        const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.loginId = loginId
        this.validate = validate
        this.observe = observe
        this.convert = convert

        fields.forEach((field) => {
            this[field].validate.state.subscribe((state) => {
                validateChecker.update(field, state)
            })
            this[field].observe.state.subscribe((result) => {
                observeChecker.update(field, result.hasChanged)
            })
        })
    }

    clear(): RequestResetTokenState {
        this.loginId.clear()
        this.validate.clear()
        return this.state.currentState()
    }
    async submit(onSuccess: { (): void }): Promise<RequestResetTokenState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.state.currentState()
        }
        return requestResetToken(this.material, fields.value, onSuccess, this.post)
    }
}

type RequestResetTokenEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: RequestResetTokenError }>
    | Readonly<{ type: "success" }>

async function requestResetToken<S>(
    { infra, config }: RequestResetTokenMaterial,
    fields: RequestResetTokenFields,
    onSuccess: { (): void },
    post: Post<RequestResetTokenEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { requestTokenRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        requestTokenRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    onSuccess()
    return post({ type: "success" })
}

interface Post<E, S> {
    (event: E): S
}
