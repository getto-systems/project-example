import {
    ApplicationStateAction,
    initApplicationStateAction,
    StatefulApplicationAction,
} from "../../../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../../../z_lib/ui/timer/helper"

import {
    initResetTokenDestinationFieldAction,
    ResetTokenDestinationFieldAction,
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
import { WaitTime } from "../../../../../../z_lib/ui/config/infra"

import { ResetTokenDestination } from "../kernel/data"
import { LoginId } from "../../../../login_id/kernel/data"

export interface ChangeResetTokenDestinationAction
    extends StatefulApplicationAction<ChangeResetTokenDestinationState> {
    readonly destination: ResetTokenDestinationFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    reset(destination: ResetTokenDestination): ChangeResetTokenDestinationState
    submit(
        user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
        onSuccess: { (data: ResetTokenDestination): void },
    ): Promise<ChangeResetTokenDestinationState>
}

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
): ChangeResetTokenDestinationAction {
    return new Action(material)
}

class Action implements ChangeResetTokenDestinationAction {
    readonly material: ChangeResetTokenDestinationMaterial
    readonly state: ApplicationStateAction<ChangeResetTokenDestinationState>
    readonly post: (state: ChangeResetTokenDestinationState) => ChangeResetTokenDestinationState

    readonly destination: ResetTokenDestinationFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    convert: { (): ConvertBoardResult<ResetTokenDestination> }

    constructor(material: ChangeResetTokenDestinationMaterial) {
        const { state, post } = initApplicationStateAction({ initialState })
        this.material = material
        this.state = state
        this.post = post

        const destination = initResetTokenDestinationFieldAction()

        // TODO modify field を使う
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
            this[field].validate.state.subscribe((state) => {
                validateChecker.update(field, state)
            })
            this[field].observe.state.subscribe((result) => {
                observeChecker.update(field, result.hasChanged)
            })
        })
    }

    reset(destination: ResetTokenDestination): ChangeResetTokenDestinationState {
        this.destination.reset(destination)
        this.validate.clear()
        this.observe.clear()
        return this.post(initialState)
    }
    async submit(
        user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
        onSuccess: { (data: ResetTokenDestination): void },
    ): Promise<ChangeResetTokenDestinationState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.state.currentState()
        }
        return changeDestination(this.material, user, fields.value, onSuccess, this.post)
    }
}

type ChangeDestinationEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangeResetTokenDestinationError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function changeDestination<S>(
    { infra, config }: ChangeResetTokenDestinationMaterial,
    user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
    fields: ResetTokenDestination,
    onSuccess: { (data: ResetTokenDestination): void },
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

    onSuccess(fields)
    post({ type: "success" })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
