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
    const { state, post } = initApplicationState({ initialState })
    const editable = initEditableBoardAction()

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

    onSuccess(() => {
        editable.close()
    })

    return {
        state,

        loginId,

        validate,
        observe,
        editable,

        clear,
        edit(): void {
            editable.open()
            clear()
        },
        async submit(): Promise<RequestResetTokenState> {
            const fields = convert()
            if (!fields.valid) {
                return state.currentState()
            }
            return requestResetToken(material, fields.value, post)
        },
    }

    function onSuccess(handler: () => void): void {
        state.subscribe((state) => {
            switch (state.type) {
                case "success":
                    handler()
                    break
            }
        })
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
