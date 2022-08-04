import {
    ApplicationStateAction,
    initApplicationStateAction,
    StatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../z_lib/ui/timer/helper"

import { initLoginIdFieldAction, LoginIdFieldAction } from "../input/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardAction } from "../../../../z_vendor/getto-application/board/observe_board/action"
import { EditableBoardAction } from "../../../../z_vendor/getto-application/board/editable/action"
import {
    initModifyField,
    modifyField,
    ModifyFieldHandler,
} from "../../../../z_lib/ui/modify/action"

import { ChangeLoginIdError, OverwriteLoginIdFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"

import { OverwriteLoginIdRemote } from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { LoginId } from "../kernel/data"
import { PrepareElementState } from "../../../../z_lib/ui/prepare/data"

export interface OverwriteLoginIdAction extends StatefulApplicationAction<OverwriteLoginIdState> {
    readonly newLoginId: LoginIdFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

    onSuccess(handler: (data: OverwriteLoginIdEntry) => void): void

    data(): PrepareElementState<OverwriteLoginIdEntry>

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
    const action = new OverwriteAction(material)
    return { action, handler: action.handler }
}

class OverwriteAction implements OverwriteLoginIdAction {
    readonly material: OverwriteLoginIdMaterial
    readonly state: ApplicationStateAction<OverwriteLoginIdState>
    readonly post: (state: OverwriteLoginIdState) => OverwriteLoginIdState

    readonly newLoginId: LoginIdFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

    readonly convert: () => ConvertBoardResult<OverwriteLoginIdFields>
    readonly data: () => PrepareElementState<OverwriteLoginIdEntry>
    readonly handler: ModifyFieldHandler<OverwriteLoginIdEntry>
    readonly reset: () => void

    constructor(material: OverwriteLoginIdMaterial) {
        const { state, post } = initApplicationStateAction({ initialState })
        this.material = material
        this.state = state
        this.post = post

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

        this.newLoginId = newLoginId
        this.validate = validate
        this.observe = observe
        this.editable = editable
        this.convert = convert
        this.data = data
        this.handler = handler
        this.reset = reset

        this.onSuccess(() => {
            this.editable.close()
        })
    }

    onSuccess(handler: (data: OverwriteLoginIdEntry) => void): void {
        this.state.subscribe((state) => {
            if (state.type === "success") {
                handler(state.entry)
            }
        })
    }

    async submit(): Promise<OverwriteLoginIdState> {
        const element = this.data()
        if (!element.isLoad) {
            return this.state.currentState()
        }

        const fields = this.convert()
        if (!fields.valid) {
            return this.state.currentState()
        }

        return overwriteLoginId(this.material, element.data, fields.value, this.post)
    }
}

type OverwriteLoginIdEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangeLoginIdError }>
    | Readonly<{ type: "success"; entry: OverwriteLoginIdEntry }>
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

    post({ type: "success", entry: { loginId: fields.newLoginId } })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
