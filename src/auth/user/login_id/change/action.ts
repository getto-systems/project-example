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

import { AuthUserAccountBasket } from "../../account/kernel/data"

export interface OverrideLoginIdAction extends StatefulApplicationAction<OverrideLoginIdState> {
    readonly newLoginId: InputLoginIdAction
    readonly validate: ValidateBoardAction

    clear(): OverrideLoginIdState
    submit(user: AuthUserAccountBasket): Promise<OverrideLoginIdState>
}

export type OverrideLoginIdState =
    | Readonly<{ type: "initial-override-login-id" }>
    | OverrideLoginIdEvent

const initialOverrideState: OverrideLoginIdState = { type: "initial-override-login-id" }

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
    async submit(user: AuthUserAccountBasket): Promise<OverrideLoginIdState> {
        return overrideLoginId(this.material, user, this.convert(), this.post)
    }
}

type OverrideLoginIdEvent =
    | Readonly<{ type: "try-to-override-login-id" }>
    | Readonly<{ type: "take-longtime-to-override-login-id" }>
    | Readonly<{ type: "failed-to-override-login-id"; err: ChangeLoginIdError }>
    | Readonly<{ type: "succeed-to-override-login-id" }>

async function overrideLoginId<S>(
    { infra, config }: OverrideLoginIdMaterial,
    user: AuthUserAccountBasket,
    fields: ConvertBoardResult<OverrideLoginIdFields>,
    post: Post<OverrideLoginIdEvent, S>,
): Promise<S> {
    if (!fields.valid) {
        return post({ type: "failed-to-override-login-id", err: { type: "validation-error" } })
    }

    post({ type: "try-to-override-login-id" })

    const { overrideLoginIdRemote: overridePasswordRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        overridePasswordRemote(user, fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-override-login-id" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-override-login-id", err: response.err })
    }

    return post({ type: "succeed-to-override-login-id" })
}

interface Post<E, S> {
    (event: E): S
}
