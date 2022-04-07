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
import { BoardConverter } from "../../../../../../z_vendor/getto-application/board/kernel/infra"

import { ResetTokenDestination } from "../kernel/data"
import { LoginId } from "../../../../login_id/input/data"

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
    convert: BoardConverter<ResetTokenDestination>

    constructor(material: ChangeResetTokenDestinationMaterial) {
        super({
            terminate: () => {
                this.destination.terminate()
                this.validate.terminate()
            },
        })
        this.material = material

        const fields = ["destination"] as const

        const destination = initInputResetTokenDestinationAction()
        const { validate, validateChecker } = initValidateBoardAction(
            { fields },
            {
                converter: (): ConvertBoardResult<ResetTokenDestination> =>
                    destination.checker.check(),
            },
        )

        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.destination = destination.input
        this.validate = validate
        this.observe = observe
        this.convert = () => validateChecker.get()

        this.destination.validate.subscriber.subscribe((result) =>
            validateChecker.update("destination", result.valid),
        )

        this.destination.observe.subscriber.subscribe((result) => {
            observeChecker.update("destination", result.hasChanged)
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
        return changeDestination(this.material, user, this.convert(), this.post)
    }
}

type ChangeDestinationEvent =
    | Readonly<{ type: "try" }>
    | Readonly<{ type: "take-longtime" }>
    | Readonly<{ type: "failed"; err: ChangeResetTokenDestinationError }>
    | Readonly<{ type: "success"; data: ResetTokenDestination }>

async function changeDestination<S>(
    { infra, config }: ChangeResetTokenDestinationMaterial,
    user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
    fields: ConvertBoardResult<ResetTokenDestination>,
    post: Post<ChangeDestinationEvent, S>,
): Promise<S> {
    if (!fields.valid) {
        return post({ type: "failed", err: { type: "validation-error" } })
    }

    post({ type: "try" })

    const { changeDestinationRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        changeDestinationRemote(user, fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime" }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success", data: fields.value })
}

interface Post<E, S> {
    (event: E): S
}
