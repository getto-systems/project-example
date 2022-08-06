import { checkTakeLongtime } from "../../../../../z_lib/ui/timer/helper"

import {
    ApplicationState,
    initApplicationState,
} from "../../../../../z_vendor/getto-application/action/action"

import { LoginIdFieldAction, initLoginIdFieldAction } from "../../../login_id/input/action"
import { ValidateBoardAction } from "../../../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardAction } from "../../../../../z_vendor/getto-application/board/observe_board/action"
import { initRegisterField } from "../../../../../z_lib/ui/register/action"
import {
    EditableBoardAction,
    initEditableBoardAction,
} from "../../../../../z_vendor/getto-application/board/editable/action"

import { RequestResetTokenRemote } from "./infra"
import { WaitTime } from "../../../../../z_lib/ui/config/infra"

import { RequestResetTokenError, RequestResetTokenFields } from "./data"
import { ConvertBoardResult } from "../../../../../z_vendor/getto-application/board/kernel/data"

export interface RequestResetTokenAction {
    readonly state: ApplicationState<RequestResetTokenState>
    readonly loginId: LoginIdFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

    edit(): void
    clear(): void
    submit(): Promise<RequestResetTokenState>
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
    readonly state: ApplicationState<RequestResetTokenState>
    readonly post: (state: RequestResetTokenState) => RequestResetTokenState

    readonly loginId: LoginIdFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

    convert: () => ConvertBoardResult<RequestResetTokenFields>
    clear: () => void

    constructor(material: RequestResetTokenMaterial) {
        const { state, post } = initApplicationState({ initialState })
        this.material = material
        this.state = state
        this.post = post
        this.editable = initEditableBoardAction()

        const loginId = initLoginIdFieldAction()

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

        const { validate, observe, clear } = initRegisterField([["loginId", loginId]], convert)

        this.loginId = loginId
        this.validate = validate
        this.observe = observe
        this.convert = convert
        this.clear = clear

        this.onSuccess(() => {
            this.editable.close()
        })
    }

    onSuccess(handler: () => void): void {
        this.state.subscribe((state) => {
            switch (state.type) {
                case "success":
                    handler()
                    break
            }
        })
    }

    edit(): void {
        this.editable.open()
        this.clear()
    }
    async submit(): Promise<RequestResetTokenState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.state.currentState()
        }
        return requestResetToken(this.material, fields.value, this.post)
    }
}

type RequestResetTokenEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: RequestResetTokenError }>
    | Readonly<{ type: "success" }>

async function requestResetToken<S>(
    { infra, config }: RequestResetTokenMaterial,
    fields: RequestResetTokenFields,
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

    return post({ type: "success" })
}

interface Post<E, S> {
    (event: E): S
}
