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

import { ChangeLoginIdError, OverrideLoginIdFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"

import { OverrideLoginIdRemote } from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"
import { BoardConverter } from "../../../../z_vendor/getto-application/board/kernel/infra"

import { LoginId } from "../input/data"

export interface OverrideLoginIdAction extends StatefulApplicationAction<OverrideLoginIdState> {
    readonly newLoginId: InputLoginIdAction
    readonly validate: ValidateBoardAction

    clear(): OverrideLoginIdState
    submit(user: Readonly<{ loginId: LoginId }>): Promise<OverrideLoginIdState>
}

export type OverrideLoginIdState =
    | Readonly<{ type: "initial" }>
    | OverrideLoginIdEvent

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

    material: OverrideLoginIdMaterial
    convert: BoardConverter<OverrideLoginIdFields>

    constructor(material: OverrideLoginIdMaterial) {
        super({
            terminate: () => {
                this.newLoginId.terminate()
                this.validate.terminate()
            },
        })
        this.material = material

        const fields = ["newLoginId"] as const

        const newLoginId = initInputLoginIdAction()
        const { validate, validateChecker } = initValidateBoardAction(
            { fields },
            {
                converter: (): ConvertBoardResult<OverrideLoginIdFields> => {
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
                },
            },
        )

        this.newLoginId = newLoginId.input
        this.validate = validate
        this.convert = () => validateChecker.get()

        this.newLoginId.validate.subscriber.subscribe((result) =>
            validateChecker.update("newLoginId", result.valid),
        )
    }

    clear(): OverrideLoginIdState {
        this.newLoginId.clear()
        this.validate.clear()
        return this.post(this.initialState)
    }
    async submit(user: Readonly<{ loginId: LoginId }>): Promise<OverrideLoginIdState> {
        return overrideLoginId(this.material, user, this.convert(), this.post)
    }
}

type OverrideLoginIdEvent =
    | Readonly<{ type: "try" }>
    | Readonly<{ type: "take-longtime" }>
    | Readonly<{ type: "failed"; err: ChangeLoginIdError }>
    | Readonly<{ type: "success"; loginId: LoginId }>

async function overrideLoginId<S>(
    { infra, config }: OverrideLoginIdMaterial,
    user: Readonly<{ loginId: LoginId }>,
    fields: ConvertBoardResult<OverrideLoginIdFields>,
    post: Post<OverrideLoginIdEvent, S>,
): Promise<S> {
    if (!fields.valid) {
        return post({ type: "failed", err: { type: "validation-error" } })
    }

    post({ type: "try" })

    const { overrideLoginIdRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        overrideLoginIdRemote(user, fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime" }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success", loginId: fields.value.newLoginId })
}

interface Post<E, S> {
    (event: E): S
}
