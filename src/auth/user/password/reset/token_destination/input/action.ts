import {
    ApplicationState,
    initApplicationState,
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

export interface ResetTokenDestinationFieldAction {
    readonly state: ApplicationState<ResetTokenDestinationFieldState>
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
    const { state, post } = initApplicationState({ initialState })

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
                post({ type: "email" })
                return

            default:
                validate.check()
                post({ type: "none" })
                return
        }
    })
    email.subscriber.subscribe(() => {
        validate.check()
        observe.check()
    })

    return {
        state,

        destinationType: destinationType.input,
        email: email.input,

        validate,
        observe,

        reset,
        clear,
    }

    function reset(destination: ResetTokenDestination): ResetTokenDestinationFieldState {
        const type = destination.type
        destinationType.store.set(type)
        email.store.set(
            (() => {
                switch (type) {
                    case "none":
                        return ""

                    case "email":
                        return destination.email
                }
            })(),
        )
        validate.clear()
        observe.pin()
        return post({ type })
    }
    function clear(): ResetTokenDestinationFieldState {
        return reset({ type: "none" })
    }
}
