import {
    AbstractStatefulApplicationAction,
    StatefulApplicationAction,
} from "../../../../../../z_vendor/getto-application/action/action"
import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../../../z_vendor/getto-application/board/input/action"
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
} from "../../../../../../z_vendor/getto-application/board/validate_field/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../../../z_vendor/getto-application/board/observe_field/action"

import { initBoardFieldObserver } from "../../../../../../z_vendor/getto-application/board/observe_field/init/observer"
import { resetTokenDestinationBoardConverter } from "./convert"
import { toBoardValue } from "../../../../../../z_vendor/getto-application/board/kernel/convert"

import { BoardFieldChecker } from "../../../../../../z_vendor/getto-application/board/validate_field/infra"
import { BoardValueStore } from "../../../../../../z_vendor/getto-application/board/input/infra"

import { emptyBoardValue } from "../../../../../../z_vendor/getto-application/board/kernel/data"
import { ValidateResetTokenDestinationError } from "./data"
import { ResetTokenDestination } from "../kernel/data"

export interface InputResetTokenDestinationAction
    extends StatefulApplicationAction<InputResetTokenDestinationState> {
    readonly destinationType: InputBoardAction
    readonly email: InputBoardAction
    readonly validate: ValidateResetTokenDestinationAction
    readonly observe: ObserveBoardFieldAction

    reset(destination: ResetTokenDestination): InputResetTokenDestinationState
}

export type InputResetTokenDestinationState = Readonly<{ type: ResetTokenDestination["type"] }>

const initialState: InputResetTokenDestinationState = { type: "none" }

export type ValidateResetTokenDestinationAction =
    ValidateBoardFieldAction<ValidateResetTokenDestinationError>
export type ValidateResetTokenDestinationState =
    ValidateBoardFieldState<ValidateResetTokenDestinationError>

export function initInputResetTokenDestinationAction(): Readonly<{
    input: InputResetTokenDestinationAction
    checker: BoardFieldChecker<ResetTokenDestination, ValidateResetTokenDestinationError>
}> {
    const input = new DestinationAction()
    return {
        input,
        checker: input.checker,
    }
}

class DestinationAction
    extends AbstractStatefulApplicationAction<InputResetTokenDestinationState>
    implements InputResetTokenDestinationAction
{
    initialState = initialState

    readonly destinationType: InputBoardAction
    readonly email: InputBoardAction
    readonly validate: ValidateResetTokenDestinationAction
    readonly observe: ObserveBoardFieldAction

    readonly checker: BoardFieldChecker<ResetTokenDestination, ValidateResetTokenDestinationError>

    readonly store: Readonly<{
        destinationType: BoardValueStore
        input: BoardValueStore
    }>

    constructor() {
        super({
            terminate: () => {
                destinationType.subscriber.terminate()
                email.subscriber.terminate()
                validate.terminate()
            },
        })

        const destinationType = initInputBoardAction()
        const email = initInputBoardAction()

        const observe = initObserveBoardFieldAction({
            observer: initBoardFieldObserver({
                current: () => ({
                    destinationType: destinationType.store.get(),
                    email: email.store.get(),
                }),
                isSame: (a, b) => {
                    if (a.destinationType !== b.destinationType) {
                        return false
                    }
                    switch (a.destinationType) {
                        case "email":
                            return a.email === b.email

                        default:
                            return true
                    }
                },
            }),
        })

        const { validate, checker } = initValidateBoardFieldAction({
            converter: () =>
                resetTokenDestinationBoardConverter({
                    type: destinationType.store.get(),
                    email: email.store.get(),
                }),
        })

        destinationType.subscriber.subscribe(() => {
            observe.check()
            switch (destinationType.store.get()) {
                case "email":
                    this.post({ type: "email" })
                    return

                default:
                    checker.check()
                    this.post({ type: "none" })
                    return
            }
        })
        email.subscriber.subscribe(() => {
            checker.check()
            observe.check()
        })

        this.destinationType = destinationType.input
        this.email = email.input
        this.validate = validate
        this.observe = observe

        this.checker = checker
        this.store = {
            destinationType: destinationType.store,
            input: email.store,
        }
    }

    reset(destination: ResetTokenDestination): InputResetTokenDestinationState {
        const destinationType = destination.type
        this.store.destinationType.set(toBoardValue(destinationType))
        this.store.input.set(
            (() => {
                switch (destinationType) {
                    case "none":
                        return emptyBoardValue

                    case "email":
                        return toBoardValue(destination.email)
                }
            })(),
        )
        this.validate.clear()
        this.observe.pin()
        return this.post({ type: destinationType })
    }
}
