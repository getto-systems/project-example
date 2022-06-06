import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../z_lib/ui/timer/helper"

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
import {
    AuthUserGrantedRolesFieldAction,
    AuthUserTextFieldAction,
    initAuthUserGrantedRolesFieldAction,
    initAuthUserTextFieldAction,
} from "../input/field/action"

import { ALL_AUTH_ROLES } from "../../../../x_content/role"

import { RegisterAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { RegisterAuthUserAccountError } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { AuthUserAccount } from "../kernel/data"
import { LoginId } from "../../login_id/kernel/data"

export interface RegisterAuthUserAccountAction
    extends StatefulApplicationAction<RegisterAuthUserAccountState> {
    readonly list: ListRegisteredAuthUserAccountAction

    readonly loginId: InputLoginIdAction
    readonly grantedRoles: AuthUserGrantedRolesFieldAction
    readonly resetTokenDestination: InputResetTokenDestinationAction
    readonly memo: AuthUserTextFieldAction<"memo">
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): RegisterAuthUserAccountState
    submit(onSuccess: { (data: AuthUserAccount): void }): Promise<RegisterAuthUserAccountState>
}
export interface ListRegisteredAuthUserAccountAction
    extends StatefulApplicationAction<ListRegisteredAuthUserAccountState> {
    readonly focused: FocusedRegisteredAuthUserAccountAction
}
export interface FocusedRegisteredAuthUserAccountAction
    extends StatefulApplicationAction<FocusedRegisteredAuthUserAccountState> {
    focus(user: AuthUserAccount): FocusedRegisteredAuthUserAccountState
    update(loginId: LoginId, user: AuthUserAccount): FocusedRegisteredAuthUserAccountState
    remove(loginId: LoginId): FocusedRegisteredAuthUserAccountState
    close(): FocusedRegisteredAuthUserAccountState

    isFocused(user: AuthUserAccount): boolean
}

export type RegisterAuthUserAccountState = RegisterUserEvent

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
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
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
    readonly grantedRoles: AuthUserGrantedRolesFieldAction
    readonly resetTokenDestination: InputResetTokenDestinationAction
    readonly memo: AuthUserTextFieldAction<"memo">
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: RegisterAuthUserAccountMaterial
    convert: { (): ConvertBoardResult<AuthUserAccount> }

    constructor(material: RegisterAuthUserAccountMaterial) {
        super()
        this.material = material

        const loginId = initInputLoginIdAction()
        const grantedRoles = initAuthUserGrantedRolesFieldAction()
        const resetTokenDestination = initInputResetTokenDestinationAction()
        const memo = initAuthUserTextFieldAction("memo")

        const fields = ["loginId", "grantedRoles", "resetTokenDestination", "memo"] as const
        const convert = (): ConvertBoardResult<AuthUserAccount> => {
            const result = {
                loginId: loginId.validate.check(),
                grantedRoles: grantedRoles.input.validate.check(),
                resetTokenDestination: resetTokenDestination.validate.check(),
                memo: memo.validate.check(),
            }
            if (
                !result.loginId.valid ||
                !result.grantedRoles.valid ||
                !result.resetTokenDestination.valid ||
                !result.memo.valid
            ) {
                return { valid: false }
            }
            return {
                valid: true,
                value: {
                    loginId: result.loginId.value,
                    grantedRoles: result.grantedRoles.value,
                    resetTokenDestination: result.resetTokenDestination.value,
                    memo: result.memo.value,
                },
            }
        }

        const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        grantedRoles.setOptions(ALL_AUTH_ROLES)

        this.list = new ListAction()

        this.loginId = loginId
        this.grantedRoles = grantedRoles.input
        this.resetTokenDestination = resetTokenDestination
        this.memo = memo
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

        this.clear()
    }

    clear(): RegisterAuthUserAccountState {
        this.loginId.clear()
        this.grantedRoles.reset([])
        this.resetTokenDestination.reset({ type: "none" })
        this.memo.clear()
        this.validate.clear()
        this.observe.clear()
        return this.currentState()
    }
    async submit(onSuccess: {
        (data: AuthUserAccount): void
    }): Promise<RegisterAuthUserAccountState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        return registerUser(
            this.material,
            fields.value,
            (data) => {
                onSuccess(data)
                this.clear()
                this.list.push(data)
            },
            this.post,
        )
    }
}

type RegisterUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: RegisterAuthUserAccountError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function registerUser<S>(
    { infra, config }: RegisterAuthUserAccountMaterial,
    fields: AuthUserAccount,
    onSuccess: { (data: AuthUserAccount): void },
    post: Post<RegisterUserEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { registerUserRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        registerUserRemote(fields),
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

class ListAction
    extends AbstractStatefulApplicationAction<ListRegisteredAuthUserAccountState>
    implements ListRegisteredAuthUserAccountAction
{
    readonly initialState = initialListState

    readonly focused: FocusedRegisteredAuthUserAccountAction

    list: AuthUserAccount[] = []

    constructor() {
        super()

        this.focused = new FocusedAction({
            updateUser: (loginId, user) => {
                this.update(loginId, user)
            },
            removeUser: (loginId) => {
                this.remove(loginId)
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
    remove(loginId: LoginId): ListRegisteredAuthUserAccountState {
        this.list = this.list.filter((row) => row.loginId !== loginId)
        return this.post({ type: "registered", users: this.list })
    }
}

type FocusedMaterial = Readonly<{
    updateUser(loginId: LoginId, user: AuthUserAccount): void
    removeUser(loginId: LoginId): void
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
    remove(loginId: LoginId): FocusedRegisteredAuthUserAccountState {
        this.material.removeUser(loginId)
        return this.close()
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
