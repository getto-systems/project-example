import { checkTakeLongtime } from "../../../../../common/util/timer/helper"

import { Atom, initAtom, mapAtom } from "../../../../../z_vendor/getto-atom/atom"
import { ValidateBoardState } from "../../../../../common/util/board/validate/action"
import { ObserveBoardState } from "../../../../../common/util/board/observe/action"
import { composeRegisterFieldBoard } from "../../../../../common/util/board/field/action"
import { LoginIdField, initLoginIdField } from "../../../login_id/input/field/action"
import {
    EditableBoardAction,
    initEditableBoardAction,
} from "../../../../../common/util/board/editable/action"

import { RequestResetTokenRemote } from "./infra"
import { WaitTime } from "../../../../../common/util/config/infra"

import { RequestResetTokenError, RequestResetTokenFields } from "./data"
import { ConvertBoardResult } from "../../../../../common/util/board/kernel/data"
import { ConnectState } from "../../../../../common/util/connect/data"

export interface RequestResetTokenAction {
    readonly state: Atom<RequestResetTokenState>
    readonly connect: Atom<ConnectState>
    readonly validate: Atom<ValidateBoardState>
    readonly observe: Atom<ObserveBoardState>
    readonly editable: EditableBoardAction

    readonly loginId: LoginIdField

    reset(): void
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
    const request = initAtom({ initialState })
    async function requestWithCurrentState(): Promise<RequestResetTokenState> {
        const fields = currentFields()
        if (!fields.valid) {
            return request.state.currentState()
        }
        return requestResetToken(material, fields.value, request.post)
    }

    const loginId = initLoginIdField()

    const currentFields = (): ConvertBoardResult<RequestResetTokenFields> => {
        const result = {
            loginId: loginId[0].validate.currentState(),
        }
        if (!result.loginId.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                loginId: result.loginId.value,
            },
        }
    }

    const { validate, observe, reset } = composeRegisterFieldBoard([loginId])

    const editable = initEditableBoardAction()
    editable.state.subscribe((state) => {
        if (state.isEditable) {
            reset()
        }
    })

    const connect = mapAtom(request.state, (state): ConnectState => {
        if (state.type === "try") {
            return { isConnecting: true, hasTakenLongtime: state.hasTakenLongtime }
        } else {
            return { isConnecting: false }
        }
    })

    return {
        state: request.state,
        connect,
        validate,
        observe,
        editable,

        loginId: loginId[0],

        reset,
        submit: requestWithCurrentState,
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
