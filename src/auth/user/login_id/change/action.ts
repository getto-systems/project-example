import {
    ApplicationState,
    initApplicationState,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../common/util/timer/helper"

import { initLoginIdFieldAction, LoginIdFieldAction } from "../input/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardAction } from "../../../../z_vendor/getto-application/board/observe_board/action"
import { EditableBoardAction } from "../../../../z_vendor/getto-application/board/editable/action"
import {
    initModifyField,
    modifyField,
    ModifyFieldHandler,
} from "../../../../common/util/modify/action"

import { ChangeLoginIdError, OverwriteLoginIdFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"

import { OverwriteLoginIdRemote } from "./infra"
import { WaitTime } from "../../../../common/util/config/infra"

import { LoginId } from "../kernel/data"

export interface OverwriteLoginIdAction {
    readonly state: ApplicationState<OverwriteLoginIdState>
    readonly newLoginId: LoginIdFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

    onSuccess(handler: (data: OverwriteLoginIdEntry) => void): void

    reset(): void
    submit(): Promise<OverwriteLoginIdState>
}

export type OverwriteLoginIdEntry = Readonly<{ loginId: LoginId }>

export type OverwriteLoginIdState = OverwriteLoginIdEvent

const initialState: OverwriteLoginIdState = { type: "initial" }

export type OverwriteLoginIdMaterial = Readonly<{
    infra: OverwriteLoginIdInfra
    config: OverwriteLoginIdConfig
}>

export type OverwriteLoginIdInfra = Readonly<{
    overwriteLoginIdRemote: OverwriteLoginIdRemote
}>

export type OverwriteLoginIdConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initOverwriteLoginIdAction(material: OverwriteLoginIdMaterial): Readonly<{
    action: OverwriteLoginIdAction
    handler: ModifyFieldHandler<OverwriteLoginIdEntry>
}> {
    const { state, post } = initApplicationState({ initialState })

    const newLoginId = initLoginIdFieldAction()

    const convert = (): ConvertBoardResult<OverwriteLoginIdFields> => {
        const result = {
            newLoginId: newLoginId.validate.check(),
        }
        if (!result.newLoginId.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                newLoginId: result.newLoginId.value,
            },
        }
    }

    const { validate, observe, editable, data, handler, reset } = initModifyField(
        [modifyField("newLoginId", newLoginId, (_data: OverwriteLoginIdEntry) => "")],
        convert,
    )

    onSuccess(() => {
        editable.close()
    })

    return {
        action: {
            state,

            newLoginId,

            validate,
            observe,
            editable,

            reset,

            onSuccess,

            async submit(): Promise<OverwriteLoginIdState> {
                const element = data()
                if (!element.isLoad) {
                    return state.currentState()
                }

                const fields = convert()
                if (!fields.valid) {
                    return state.currentState()
                }

                return overwriteLoginId(material, element.data, fields.value, post)
            },
        },
        handler,
    }

    function onSuccess(handler: (data: OverwriteLoginIdEntry) => void): void {
        state.subscribe((state) => {
            if (state.type === "success") {
                handler(state.data)
            }
        })
    }
}

type OverwriteLoginIdEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangeLoginIdError }>
    | Readonly<{ type: "success"; data: OverwriteLoginIdEntry }>
    | Readonly<{ type: "initial" }>

async function overwriteLoginId<S>(
    { infra, config }: OverwriteLoginIdMaterial,
    user: Readonly<{ loginId: LoginId }>,
    fields: OverwriteLoginIdFields,
    post: Post<OverwriteLoginIdEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { overwriteLoginIdRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        overwriteLoginIdRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    post({ type: "success", data: { loginId: fields.newLoginId } })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
