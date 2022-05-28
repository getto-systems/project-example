import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../z_lib/ui/timer/helper"

import { InputLoginIdAction, initInputLoginIdAction } from "../input/action"
import {
    ValidateBoardAction,
    initValidateBoardAction,
} from "../../../../z_vendor/getto-application/board/validate_board/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"

import { ChangeLoginIdError, OverwriteLoginIdFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"

import { OverwriteLoginIdRemote } from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { LoginId } from "../kernel/data"

export interface OverwriteLoginIdAction extends StatefulApplicationAction<OverwriteLoginIdState> {
    readonly newLoginId: InputLoginIdAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): OverwriteLoginIdState
    submit(
        user: Readonly<{ loginId: LoginId }>,
        onSuccess: { (loginId: LoginId): void },
    ): Promise<OverwriteLoginIdState>
}

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

export function initOverwriteLoginIdAction(
    material: OverwriteLoginIdMaterial,
): OverwriteLoginIdAction {
    return new OverwriteAction(material)
}

class OverwriteAction
    extends AbstractStatefulApplicationAction<OverwriteLoginIdState>
    implements OverwriteLoginIdAction
{
    readonly initialState = initialState

    readonly newLoginId: InputLoginIdAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: OverwriteLoginIdMaterial
    convert: { (): ConvertBoardResult<OverwriteLoginIdFields> }

    constructor(material: OverwriteLoginIdMaterial) {
        super()
        this.material = material

        const newLoginId = initInputLoginIdAction()

        const fields = ["newLoginId"] as const
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

        const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.newLoginId = newLoginId
        this.validate = validate
        this.observe = observe
        this.convert = convert

        fields.forEach((field) => {
            this[field].validate.subscriber.subscribe((state) => {
                validateChecker.update(field, state)
            })
            this[field].observe.subscriber.subscribe((result) => {
                observeChecker.update(field, result.hasChanged)
            })
        })
    }

    clear(): OverwriteLoginIdState {
        this.newLoginId.clear()
        this.validate.clear()
        return this.post(this.initialState)
    }
    async submit(
        user: Readonly<{ loginId: LoginId }>,
        onSuccess: { (loginId: LoginId): void },
    ): Promise<OverwriteLoginIdState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        return overwriteLoginId(this.material, user, fields.value, onSuccess, this.post)
    }
}

type OverwriteLoginIdEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangeLoginIdError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function overwriteLoginId<S>(
    { infra, config }: OverwriteLoginIdMaterial,
    user: Readonly<{ loginId: LoginId }>,
    fields: OverwriteLoginIdFields,
    onSuccess: { (loginId: LoginId): void },
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

    onSuccess(fields.newLoginId)
    post({ type: "success" })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
