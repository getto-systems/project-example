import {
    ApplicationState,
    initApplicationState,
} from "../../../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../../../z_lib/ui/timer/helper"

import {
    initResetTokenDestinationFieldAction,
    ResetTokenDestinationFieldAction,
} from "../input/action"
import { ValidateBoardAction } from "../../../../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardAction } from "../../../../../../z_vendor/getto-application/board/observe_board/action"
import {
    initModifyField,
    modifyField,
    ModifyFieldHandler,
} from "../../../../../../z_lib/ui/modify/action"
import { EditableBoardAction } from "../../../../../../z_vendor/getto-application/board/editable/action"

import { ChangeResetTokenDestinationRemote } from "./infra"
import { WaitTime } from "../../../../../../z_lib/ui/config/infra"

import { ResetTokenDestination } from "../kernel/data"
import { LoginId } from "../../../../login_id/kernel/data"
import { ChangeResetTokenDestinationError } from "./data"
import { ConvertBoardResult } from "../../../../../../z_vendor/getto-application/board/kernel/data"
import { PrepareElementState } from "../../../../../../z_lib/ui/prepare/data"

export interface ChangeResetTokenDestinationAction {
    readonly state: ApplicationState<ChangeResetTokenDestinationState>
    readonly destination: ResetTokenDestinationFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

    onSuccess(handler: (data: ChangeResetTokenDestinationEntry) => void): void

    data(): PrepareElementState<ChangeResetTokenDestinationEntry>
    reset(): void
    submit(): Promise<ChangeResetTokenDestinationState>
}

export type ChangeResetTokenDestinationEntry = Readonly<{
    loginId: LoginId
    resetTokenDestination: ResetTokenDestination
}>

export type ChangeResetTokenDestinationState = ChangeDestinationEvent

const initialState: ChangeResetTokenDestinationState = { type: "initial" }

export type ChangeResetTokenDestinationMaterial = Readonly<{
    infra: ChangeResetTokenDestinationInfra
    config: ChangeResetTokenDestinationConfig
}>

export type ChangeResetTokenDestinationInfra = Readonly<{
    changeDestinationRemote: ChangeResetTokenDestinationRemote
}>

export type ChangeResetTokenDestinationConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initChangeResetTokenDestinationAction(
    material: ChangeResetTokenDestinationMaterial,
): Readonly<{
    action: ChangeResetTokenDestinationAction
    handler: ModifyFieldHandler<ChangeResetTokenDestinationEntry>
}> {
    const { state, post } = initApplicationState({ initialState })

    const destination = initResetTokenDestinationFieldAction()

    const convert = (): ConvertBoardResult<ResetTokenDestination> => {
        const result = destination.validate.check()
        if (!result.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: result.value,
        }
    }

    const { validate, observe, editable, data, handler, reset } = initModifyField(
        [
            modifyField(
                "destination",
                destination,
                (data: ChangeResetTokenDestinationEntry) => data.resetTokenDestination,
            ),
        ],
        convert,
    )

    onSuccess(() => {
        editable.close()
    })

    return {
        action: {
            state,

            destination,

            validate,
            observe,
            editable,

            data,
            reset,

            onSuccess,

            async submit(): Promise<ChangeResetTokenDestinationState> {
                const element = data()
                if (!element.isLoad) {
                    return state.currentState()
                }

                const fields = convert()
                if (!fields.valid) {
                    return state.currentState()
                }
                return changeDestination(material, element.data, fields.value, post)
            },
        },
        handler,
    }

    function onSuccess(handler: (data: ChangeResetTokenDestinationEntry) => void): void {
        state.subscribe((state) => {
            if (state.type === "success") {
                handler(state.data)
            }
        })
    }
}

type ChangeDestinationEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangeResetTokenDestinationError }>
    | Readonly<{ type: "success"; data: ChangeResetTokenDestinationEntry }>
    | Readonly<{ type: "initial" }>

async function changeDestination<S>(
    { infra, config }: ChangeResetTokenDestinationMaterial,
    user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
    fields: ResetTokenDestination,
    post: Post<ChangeDestinationEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { changeDestinationRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        changeDestinationRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    post({ type: "success", data: { loginId: user.loginId, resetTokenDestination: fields } })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
