import {
    ApplicationState,
    initApplicationState,
    StatefulApplicationAction,
} from "../../../../../../z_vendor/getto-application/action/action"
import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../../../z_vendor/getto-application/board/input/action"
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
} from "../../../../../../z_vendor/getto-application/board/validate_field/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../../../z_vendor/getto-application/board/observe_field/action"

import { initBoardFieldObserver } from "../../../../../../z_vendor/getto-application/board/observe_field/init/observer"
import { resetTokenDestinationBoardConverter } from "../kernel/convert"

import { BoardValueStore } from "../../../../../../z_vendor/getto-application/board/input/infra"

import { ValidateResetTokenDestinationError } from "./data"
import { ResetTokenDestination } from "../kernel/data"

export interface ResetTokenDestinationFieldAction
    extends StatefulApplicationAction<ResetTokenDestinationFieldState> {
    readonly destinationType: InputBoardAction<BoardValueStore>
    readonly email: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<
        ResetTokenDestination,
        ValidateResetTokenDestinationError
    >
    readonly observe: ObserveBoardFieldAction

    reset(destination: ResetTokenDestination): ResetTokenDestinationFieldState
    clear(): ResetTokenDestinationFieldState
}

export type ResetTokenDestinationFieldState = Readonly<{ type: ResetTokenDestination["type"] }>

const initialState: ResetTokenDestinationFieldState = { type: "none" }

export function initResetTokenDestinationFieldAction(): ResetTokenDestinationFieldAction {
    return new DestinationAction()
}

class DestinationAction implements ResetTokenDestinationFieldAction {
    readonly state: ApplicationState<ResetTokenDestinationFieldState>
    readonly post: (state: ResetTokenDestinationFieldState) => ResetTokenDestinationFieldState

    readonly destinationType: InputBoardAction<BoardValueStore>
    readonly email: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<
        ResetTokenDestination,
        ValidateResetTokenDestinationError
    >
    readonly observe: ObserveBoardFieldAction

    readonly store: Readonly<{
        destinationType: BoardValueStore
        input: BoardValueStore
    }>

    constructor() {
        const { state, post } = initApplicationState({ initialState })
        this.state = state
        this.post = post

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

        const validate = initValidateBoardFieldAction({
            convert: () =>
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
                    validate.check()
                    this.post({ type: "none" })
                    return
            }
        })
        email.subscriber.subscribe(() => {
            validate.check()
            observe.check()
        })

        this.destinationType = destinationType.input
        this.email = email.input
        this.validate = validate
        this.observe = observe

        this.store = {
            destinationType: destinationType.store,
            input: email.store,
        }
    }

    reset(destination: ResetTokenDestination): ResetTokenDestinationFieldState {
        const destinationType = destination.type
        this.store.destinationType.set(destinationType)
        this.store.input.set(
            (() => {
                switch (destinationType) {
                    case "none":
                        return ""

                    case "email":
                        return destination.email
                }
            })(),
        )
        this.validate.clear()
        this.observe.pin()
        return this.post({ type: destinationType })
    }
    clear(): ResetTokenDestinationFieldState {
        return this.reset({ type: "none" })
    }
}
