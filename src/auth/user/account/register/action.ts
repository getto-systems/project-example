import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"

import { initInputGrantedRolesAction, InputGrantedRolesAction } from "../input/action"
import {
    ValidateBoardAction,
    initValidateBoardAction,
} from "../../../../z_vendor/getto-application/board/validate_board/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"
import { initInputLoginIdAction, InputLoginIdAction } from "../../login_id/input/action"
import {
    initInputResetTokenDestinationAction,
    InputResetTokenDestinationAction,
} from "../../password/reset/token_destination/input/action"

import { RegisterAuthUserAccountRemote } from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"
import { BoardConverter } from "../../../../z_vendor/getto-application/board/kernel/infra"

import { RegisterAuthUserAccountError } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { AuthUserAccount } from "../kernel/data"
import { LoginId } from "../../login_id/kernel/data"

export interface RegisterAuthUserAccountAction
    extends StatefulApplicationAction<RegisterAuthUserAccountState> {
    readonly list: ListRegisteredAuthUserAccountAction

    readonly loginId: InputLoginIdAction
    readonly grantedRoles: InputGrantedRolesAction
    readonly resetTokenDestination: InputResetTokenDestinationAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): RegisterAuthUserAccountState
    submit(): Promise<RegisterAuthUserAccountState>
}
export interface ListRegisteredAuthUserAccountAction
    extends StatefulApplicationAction<ListRegisteredAuthUserAccountState> {
    readonly focused: FocusedRegisteredAuthUserAccountAction
}
export interface FocusedRegisteredAuthUserAccountAction
    extends StatefulApplicationAction<FocusedRegisteredAuthUserAccountState> {
    focus(user: AuthUserAccount): FocusedRegisteredAuthUserAccountState
    update(loginId: LoginId, user: AuthUserAccount): FocusedRegisteredAuthUserAccountState
    close(): FocusedRegisteredAuthUserAccountState

    isFocused(user: AuthUserAccount): boolean
}

export type RegisterAuthUserAccountState = Readonly<{ type: "initial" }> | RegisterUserEvent

const initialState: RegisterAuthUserAccountState = { type: "initial" }

export type ListRegisteredAuthUserAccountState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "registered"; users: readonly AuthUserAccount[] }>

const initialListState: ListRegisteredAuthUserAccountState = { type: "initial" }

export type FocusedRegisteredAuthUserAccountState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "focus-on"; user: AuthUserAccount }>

const initialFocusedState: FocusedRegisteredAuthUserAccountState = { type: "initial" }

export type RegisterAuthUserAccountMaterial = Readonly<{
    infra: RegisterAuthUserAccountInfra
    config: RegisterAuthUserAccountConfig
}>

export type RegisterAuthUserAccountInfra = Readonly<{
    registerUserRemote: RegisterAuthUserAccountRemote
}>

export type RegisterAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export function initRegisterAuthUserAccountAction(
    material: RegisterAuthUserAccountMaterial,
): RegisterAuthUserAccountAction {
    return new Action(material)
}

class Action
    extends AbstractStatefulApplicationAction<RegisterAuthUserAccountState>
    implements RegisterAuthUserAccountAction
{
    readonly initialState = initialState

    readonly list: ListAction

    readonly loginId: InputLoginIdAction
    readonly grantedRoles: InputGrantedRolesAction
    readonly resetTokenDestination: InputResetTokenDestinationAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: RegisterAuthUserAccountMaterial
    convert: BoardConverter<AuthUserAccount>

    constructor(material: RegisterAuthUserAccountMaterial) {
        super({
            terminate: () => {
                this.list.terminate()
                this.loginId.terminate()
                this.grantedRoles.terminate()
                this.resetTokenDestination.terminate()
                this.validate.terminate()
                this.observe.terminate()
            },
        })
        this.material = material

        const fields = ["login-id", "granted-roles", "reset-token-destination"] as const

        const loginId = initInputLoginIdAction()
        const grantedRoles = initInputGrantedRolesAction()
        const resetTokenDestination = initInputResetTokenDestinationAction()
        const { validate, validateChecker } = initValidateBoardAction(
            { fields },
            {
                converter: (): ConvertBoardResult<AuthUserAccount> => {
                    const result = {
                        loginId: loginId.checker.check(),
                        grantedRoles: grantedRoles.convert(),
                        resetTokenDestination: resetTokenDestination.checker.check(),
                    }
                    if (!result.loginId.valid || !result.resetTokenDestination.valid) {
                        return { valid: false }
                    }
                    return {
                        valid: true,
                        value: {
                            loginId: result.loginId.value,
                            grantedRoles: result.grantedRoles,
                            resetTokenDestination: result.resetTokenDestination.value,
                        },
                    }
                },
            },
        )
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.list = new ListAction()

        this.loginId = loginId.input
        this.grantedRoles = grantedRoles.input
        this.resetTokenDestination = resetTokenDestination.input
        this.validate = validate
        this.observe = observe
        this.convert = () => validateChecker.get()

        this.loginId.validate.subscriber.subscribe((result) => {
            validateChecker.update("login-id", result.valid)
        })
        this.loginId.observe.subscriber.subscribe((result) => {
            observeChecker.update("login-id", result.hasChanged)
        })
        this.grantedRoles.observe.subscriber.subscribe((result) => {
            observeChecker.update("granted-roles", result.hasChanged)
        })
        this.resetTokenDestination.validate.subscriber.subscribe((result) => {
            validateChecker.update("reset-token-destination", result.valid)
        })
        this.resetTokenDestination.observe.subscriber.subscribe((result) => {
            observeChecker.update("reset-token-destination", result.hasChanged)
        })
    }

    clear(): RegisterAuthUserAccountState {
        this.loginId.clear()
        this.grantedRoles.reset([])
        this.resetTokenDestination.reset({ type: "none" })
        this.validate.clear()
        this.observe.clear()
        return this.currentState()
    }
    async submit(): Promise<RegisterAuthUserAccountState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        return registerUser(this.material, fields.value, this.post).then((state) => {
            switch (state.type) {
                case "success":
                    this.clear()
                    this.list.push(state.data)
                    break
            }
            return state
        })
    }
}

type RegisterUserEvent =
    | Readonly<{ type: "try" }>
    | Readonly<{ type: "take-longtime" }>
    | Readonly<{ type: "failed"; err: RegisterAuthUserAccountError }>
    | Readonly<{ type: "success"; data: AuthUserAccount }>

async function registerUser<S>(
    { infra, config }: RegisterAuthUserAccountMaterial,
    fields: AuthUserAccount,
    post: Post<RegisterUserEvent, S>,
): Promise<S> {
    post({ type: "try" })

    const { registerUserRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        registerUserRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime" }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success", data: fields })
}

class ListAction
    extends AbstractStatefulApplicationAction<ListRegisteredAuthUserAccountState>
    implements ListRegisteredAuthUserAccountAction
{
    readonly initialState = initialListState

    readonly focused: FocusedRegisteredAuthUserAccountAction

    list: AuthUserAccount[] = []

    constructor() {
        super({
            terminate: () => {
                this.focused.terminate()
            },
        })

        this.focused = new FocusedAction({
            updateUser: (loginId, user) => {
                this.update(loginId, user)
            },
        })
    }

    push(user: AuthUserAccount): ListRegisteredAuthUserAccountState {
        // 最新のものが上に表示されるように上から追加する
        this.list.unshift(user)
        return this.post({ type: "registered", users: this.list })
    }

    update(loginId: LoginId, user: AuthUserAccount): ListRegisteredAuthUserAccountState {
        this.list = this.list.map((row) => {
            if (row.loginId !== loginId) {
                return row
            }
            return user
        })
        return this.post({ type: "registered", users: this.list })
    }
}

type FocusedMaterial = Readonly<{
    updateUser(loginId: LoginId, user: AuthUserAccount): void
}>

class FocusedAction
    extends AbstractStatefulApplicationAction<FocusedRegisteredAuthUserAccountState>
    implements FocusedRegisteredAuthUserAccountAction
{
    readonly initialState = initialFocusedState

    readonly material: FocusedMaterial

    constructor(material: FocusedMaterial) {
        super()
        this.material = material
    }

    focus(user: AuthUserAccount): FocusedRegisteredAuthUserAccountState {
        return this.post({ type: "focus-on", user })
    }
    update(loginId: LoginId, user: AuthUserAccount): FocusedRegisteredAuthUserAccountState {
        this.material.updateUser(loginId, user)
        return this.post({ type: "focus-on", user })
    }
    close(): FocusedRegisteredAuthUserAccountState {
        return this.post({ type: "initial" })
    }

    isFocused(user: AuthUserAccount): boolean {
        const state = this.currentState()
        switch (state.type) {
            case "initial":
                return false

            case "focus-on":
                return user.loginId === state.user.loginId
        }
    }
}

interface Post<E, S> {
    (event: E): S
}