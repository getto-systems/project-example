import {
    AbstractStatefulApplicationAction,
    ApplicationAction,
    StatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"
import {
    initInputBoardAction,
    initMultipleInputBoardAction,
    InputBoardAction,
    MultipleInputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
} from "../../../../z_vendor/getto-application/board/validate_field/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/observe_field/action"

import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"
import { resetTokenDestinationBoardConverter } from "./convert"
import { toBoardValue } from "../../../../z_vendor/getto-application/board/kernel/convert"
import { isSameMultipleBoardValue } from "../../../../z_vendor/getto-application/board/observe_field/helper"

import { BoardFieldChecker } from "../../../../z_vendor/getto-application/board/validate_field/infra"
import {
    BoardValueStore,
    MultipleBoardValueStore,
} from "../../../../z_vendor/getto-application/board/input/infra"

import { emptyBoardValue } from "../../../../z_vendor/getto-application/board/kernel/data"
import { ResetTokenDestination, ValidateResetTokenDestinationError } from "./data"
import { AuthUserAccountBasket } from "../kernel/data"

export interface InputGrantedRolesAction extends ApplicationAction {
    readonly grantedRoles: MultipleInputBoardAction
    readonly observe: ObserveBoardFieldAction

    reset(user: AuthUserAccountBasket): void
}

export interface InputResetTokenDestinationAction
    extends StatefulApplicationAction<InputResetTokenDestinationState> {
    readonly destinationType: InputBoardAction
    readonly input: InputBoardAction
    readonly validate: ValidateResetTokenDestinationAction
    readonly observe: ObserveBoardFieldAction

    reset(user: AuthUserAccountBasket): InputResetTokenDestinationState
}

export type InputResetTokenDestinationState = Readonly<{ type: ResetTokenDestination["type"] }>

const initialState: InputResetTokenDestinationState = { type: "none" }

export type ValidateResetTokenDestinationAction =
    ValidateBoardFieldAction<ValidateResetTokenDestinationError>
export type ValidateResetTokenDestinationState =
    ValidateBoardFieldState<ValidateResetTokenDestinationError>

export function initInputGrantedRolesAction(): InputGrantedRolesAction {
    return new GrantedRolesAction()
}

class GrantedRolesAction implements InputGrantedRolesAction {
    readonly grantedRoles: MultipleInputBoardAction
    readonly observe: ObserveBoardFieldAction

    readonly store: Readonly<{
        grantedRoles: MultipleBoardValueStore
    }>

    terminate: { (): void }

    constructor() {
        const grantedRoles = initMultipleInputBoardAction()
        const observe = initObserveBoardFieldAction({
            observer: initBoardFieldObserver({
                current: () => grantedRoles.store.get(),
                isSame: isSameMultipleBoardValue,
            }),
        })

        this.grantedRoles = grantedRoles.input
        this.observe = observe

        this.store = {
            grantedRoles: grantedRoles.store,
        }

        grantedRoles.subscriber.subscribe(() => {
            observe.check()
        })

        this.terminate = () => {
            grantedRoles.subscriber.terminate()
            observe.terminate()
        }
    }

    reset(user: AuthUserAccountBasket): void {
        this.store.grantedRoles.set(user.grantedRoles.map(toBoardValue))
    }
}

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
    readonly input: InputBoardAction
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
                input.subscriber.terminate()
                validate.terminate()
            },
        })

        const destinationType = initInputBoardAction()
        const input = initInputBoardAction()

        const observe = initObserveBoardFieldAction({
            observer: initBoardFieldObserver({
                current: () => ({
                    destinationType: destinationType.store.get(),
                    input: input.store.get(),
                }),
                isSame: (a, b) => a.destinationType === b.destinationType && a.input === b.input,
            }),
        })

        const { validate, checker } = initValidateBoardFieldAction({
            converter: () => resetTokenDestinationBoardConverter(input.store.get()),
        })

        destinationType.subscriber.subscribe(() => {
            observe.check()
            switch (destinationType.store.get()) {
                case "email":
                    this.post({ type: "email" })
                    return

                default:
                    this.post({ type: "none" })
                    return
            }
        })
        input.subscriber.subscribe(() => {
            checker.check()
            observe.check()
        })

        this.destinationType = destinationType.input
        this.input = input.input
        this.validate = validate
        this.observe = observe

        this.checker = checker
        this.store = {
            destinationType: destinationType.store,
            input: input.store,
        }
    }

    reset(user: AuthUserAccountBasket): InputResetTokenDestinationState {
        const destinationType = user.resetTokenDestination.type
        this.store.destinationType.set(toBoardValue(destinationType))
        this.store.input.set(
            (() => {
                switch (destinationType) {
                    case "none":
                        return emptyBoardValue

                    case "email":
                        return toBoardValue(user.resetTokenDestination.email)
                }
            })(),
        )
        this.validate.clear()
        this.observe.pin()
        return this.post({ type: destinationType })
    }
}
