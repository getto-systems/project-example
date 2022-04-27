import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"

import { InputLoginIdAction, initInputLoginIdAction } from "../input/action"
import {
    ValidateBoardAction,
    initValidateBoardAction,
} from "../../../../z_vendor/getto-application/board/validate_board/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"

import { ChangeLoginIdError, OverrideLoginIdFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"

import { OverrideLoginIdRemote } from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"

import { LoginId } from "../kernel/data"

export interface OverrideLoginIdAction extends StatefulApplicationAction<OverrideLoginIdState> {
    readonly newLoginId: InputLoginIdAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): OverrideLoginIdState
    submit(user: Readonly<{ loginId: LoginId }>): Promise<OverrideLoginIdState>
}

export type OverrideLoginIdState = Readonly<{ type: "initial" }> | OverrideLoginIdEvent

const initialOverrideState: OverrideLoginIdState = { type: "initial" }

export type OverrideLoginIdMaterial = Readonly<{
    infra: OverrideLoginIdInfra
    config: OverrideLoginIdConfig
}>

export type OverrideLoginIdInfra = Readonly<{
    overrideLoginIdRemote: OverrideLoginIdRemote
}>

export type OverrideLoginIdConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export function initOverrideLoginIdAction(
    material: OverrideLoginIdMaterial,
): OverrideLoginIdAction {
    return new OverrideAction(material)
}

class OverrideAction
    extends AbstractStatefulApplicationAction<OverrideLoginIdState>
    implements OverrideLoginIdAction
{
    readonly initialState = initialOverrideState

    readonly newLoginId: InputLoginIdAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: OverrideLoginIdMaterial
    convert: { (): ConvertBoardResult<OverrideLoginIdFields> }

    constructor(material: OverrideLoginIdMaterial) {
        super({
            terminate: () => {
                this.newLoginId.terminate()
                this.validate.terminate()
                this.observe.terminate()
            },
        })
        this.material = material

        const newLoginId = initInputLoginIdAction()

        const fields = ["newLoginId"] as const
        const convert = (): ConvertBoardResult<OverrideLoginIdFields> => {
            const result = {
                newLoginId: newLoginId.checker.check(),
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

        this.newLoginId = newLoginId.input
        this.validate = validate
        this.observe = observe
        this.convert = convert

        this.newLoginId.validate.subscriber.subscribe((result) =>
            validateChecker.update("newLoginId", result.valid),
        )
        this.newLoginId.observe.subscriber.subscribe((result) => {
            observeChecker.update("newLoginId", result.hasChanged)
        })
    }

    clear(): OverrideLoginIdState {
        this.newLoginId.clear()
        this.validate.clear()
        return this.post(this.initialState)
    }
    async submit(user: Readonly<{ loginId: LoginId }>): Promise<OverrideLoginIdState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        return overrideLoginId(this.material, user, fields.value, this.post)
    }
}

type OverrideLoginIdEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangeLoginIdError }>
    | Readonly<{ type: "success"; loginId: LoginId }>

async function overrideLoginId<S>(
    { infra, config }: OverrideLoginIdMaterial,
    user: Readonly<{ loginId: LoginId }>,
    fields: OverrideLoginIdFields,
    post: Post<OverrideLoginIdEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { overrideLoginIdRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        overrideLoginIdRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success", loginId: fields.newLoginId })
}

interface Post<E, S> {
    (event: E): S
}
