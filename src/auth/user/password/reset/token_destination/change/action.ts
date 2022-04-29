import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../../../z_vendor/getto-application/action/action"

import { delayedChecker } from "../../../../../../z_lib/ui/timer/helper"

import {
    initInputResetTokenDestinationAction,
    InputResetTokenDestinationAction,
} from "../input/action"
import {
    ValidateBoardAction,
    initValidateBoardAction,
} from "../../../../../../z_vendor/getto-application/board/validate_board/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../../../z_vendor/getto-application/board/observe_board/action"

import { ChangeResetTokenDestinationError } from "./data"
import { ConvertBoardResult } from "../../../../../../z_vendor/getto-application/board/kernel/data"

import { ChangeResetTokenDestinationRemote } from "./infra"
import { DelayTime } from "../../../../../../z_lib/ui/config/infra"

import { ResetTokenDestination } from "../kernel/data"
import { LoginId } from "../../../../login_id/kernel/data"

export interface ChangeResetTokenDestinationAction
    extends StatefulApplicationAction<ChangeResetTokenDestinationState> {
    readonly destination: InputResetTokenDestinationAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    reset(destination: ResetTokenDestination): ChangeResetTokenDestinationState
    submit(
        user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
    ): Promise<ChangeResetTokenDestinationState>
}

export type ChangeResetTokenDestinationState =
    | Readonly<{ type: "initial" }>
    | ChangeDestinationEvent

const initialState: ChangeResetTokenDestinationState = { type: "initial" }

export type ChangeResetTokenDestinationMaterial = Readonly<{
    infra: ChangeResetTokenDestinationInfra
    config: ChangeResetTokenDestinationConfig
}>

export type ChangeResetTokenDestinationInfra = Readonly<{
    changeDestinationRemote: ChangeResetTokenDestinationRemote
}>

export type ChangeResetTokenDestinationConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export function initChangeResetTokenDestinationAction(
    material: ChangeResetTokenDestinationMaterial,
): ChangeResetTokenDestinationAction {
    return new Action(material)
}

class Action
    extends AbstractStatefulApplicationAction<ChangeResetTokenDestinationState>
    implements ChangeResetTokenDestinationAction
{
    readonly initialState = initialState

    readonly destination: InputResetTokenDestinationAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: ChangeResetTokenDestinationMaterial
    convert: { (): ConvertBoardResult<ResetTokenDestination> }

    constructor(material: ChangeResetTokenDestinationMaterial) {
        super({
            terminate: () => {
                this.destination.terminate()
                this.validate.terminate()
            },
        })
        this.material = material

        const destination = initInputResetTokenDestinationAction()

        const fields = ["destination"] as const
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

        const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.destination = destination
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

    reset(destination: ResetTokenDestination): ChangeResetTokenDestinationState {
        this.destination.reset(destination)
        this.validate.clear()
        this.observe.clear()
        return this.post(this.initialState)
    }
    async submit(
        user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
    ): Promise<ChangeResetTokenDestinationState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        return changeDestination(this.material, user, fields.value, this.post)
    }
}

type ChangeDestinationEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangeResetTokenDestinationError }>
    | Readonly<{ type: "success"; data: ResetTokenDestination }>

async function changeDestination<S>(
    { infra, config }: ChangeResetTokenDestinationMaterial,
    user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
    fields: ResetTokenDestination,
    post: Post<ChangeDestinationEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { changeDestinationRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        changeDestinationRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success", data: fields })
}

interface Post<E, S> {
    (event: E): S
}
