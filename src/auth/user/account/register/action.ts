import { checkTakeLongtime, ticker } from "../../../../common/util/timer/helper"

import { ALL_AUTH_PERMISSIONS } from "../../../../x_content/permission"

import { Atom, initAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import { LoadState, loadState_loaded, loadState_loading } from "../../../../common/util/load/data"
import { ValidateBoardState } from "../../../../common/util/board/validate/action"
import { ObserveBoardState } from "../../../../common/util/board/observe/action"
import { composeRegisterFieldBoard } from "../../../../common/util/board/field/action"
import { AuthUserTextField, initAuthUserTextField } from "../input/field/action"
import {
    AuthPermissionGrantedField,
    initAuthPermissionGrantedField,
} from "../../kernel/input/field/action"
import { initLoginIdField, LoginIdField } from "../../login_id/input/field/action"
import {
    ResetTokenDestinationField,
    initResetTokenDestinationField,
} from "../../password/reset/token_destination/input/field/action"
import {
    initLoadableListAtomUpdater,
    initPushListAction,
    initFocusRegisterListAction,
    LoadableListAtomUpdater,
    FocusRegisterListAction,
} from "../../../../common/util/list/action"

import { RegisterAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../common/util/config/infra"

import { ConvertBoardResult } from "../../../../common/util/board/kernel/data"
import { RegisterAuthUserAccountError } from "./data"
import { AuthUserAccount } from "../kernel/data"
import { AuthPermission } from "../../kernel/data"
import {
    ResetTokenDestinationType,
    resetTokenDestinationTypeVariants,
} from "../../password/reset/token_destination/kernel/data"
import { ConnectState, SuccessState } from "../../../../common/util/connect/data"

export interface RegisterAuthUserAccountAction {
    readonly focus: FocusRegisterListAction<AuthUserAccount>

    readonly state: Atom<RegisterAuthUserAccountState>
    readonly list: Atom<LoadState<readonly AuthUserAccount[]>>
    readonly success: Atom<SuccessState>
    readonly connect: Atom<ConnectState>
    readonly validate: Atom<ValidateBoardState>
    readonly observe: Atom<ObserveBoardState>

    readonly loginId: LoginIdField
    readonly granted: AuthPermissionGrantedField
    readonly resetTokenDestination: ResetTokenDestinationField
    readonly memo: AuthUserTextField<"memo">

    reset(): void
    submit(): Promise<RegisterAuthUserAccountState>
}

export type RegisterAuthUserAccountState = RegisterUserEvent

const initialState: RegisterAuthUserAccountState = { type: "initial" }

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
): [RegisterAuthUserAccountAction, LoadableListAtomUpdater<AuthUserAccount>] {
    const register = initAtom({ initialState })
    async function registerWithCurrentState(): Promise<RegisterAuthUserAccountState> {
        const fields = currentFields()
        if (!fields.valid) {
            return register.state.currentState()
        }
        return registerAuthUserAccount(material, fields.value, register.post)
    }

    const list = initAtom<LoadState<readonly AuthUserAccount[]>>({
        initialState: loadState_loading(),
    })

    const grantedOptions = initAtom<LoadState<readonly AuthPermission[]>>({
        initialState: loadState_loaded(ALL_AUTH_PERMISSIONS),
    })
    const resetOptions = initAtom<LoadState<readonly ResetTokenDestinationType[]>>({
        initialState: loadState_loaded(resetTokenDestinationTypeVariants),
    })

    const loginId = initLoginIdField()
    const granted = initAuthPermissionGrantedField(grantedOptions.state)
    const resetTokenDestination = initResetTokenDestinationField(resetOptions.state)
    const memo = initAuthUserTextField("memo")

    const currentFields = (): ConvertBoardResult<AuthUserAccount> => {
        const result = {
            loginId: loginId[0].validate.currentState(),
            granted: granted[0].validate.currentState(),
            resetTokenDestination: resetTokenDestination[0].validate.currentState(),
            memo: memo[0].validate.currentState(),
        }
        if (
            !result.loginId.valid ||
            !result.granted.valid ||
            !result.resetTokenDestination.valid ||
            !result.memo.valid
        ) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                loginId: result.loginId.value,
                granted: result.granted.value,
                resetTokenDestination: result.resetTokenDestination.value,
                memo: result.memo.value,
            },
        }
    }

    const { validate, observe, reset } = composeRegisterFieldBoard([
        loginId,
        granted,
        resetTokenDestination,
        memo,
    ])

    const { push } = initPushListAction<AuthUserAccount>(list)

    const focus = initFocusRegisterListAction(list.state, (entry) => `${entry.loginId}`)

    register.state.subscribe((state) => {
        if (state.type === "success") {
            push(state.data)
            reset()
        }
    })

    const success = mapAtom(register.state, (state): SuccessState => {
        return { isSuccess: state.type === "success" }
    })

    const connect = mapAtom(register.state, (state): ConnectState => {
        if (state.type === "try") {
            return { isConnecting: true, hasTakenLongtime: state.hasTakenLongtime }
        } else {
            return { isConnecting: false }
        }
    })

    return [
        {
            focus,

            state: register.state,
            list: list.state,
            success,
            connect,
            validate,
            observe,

            loginId: loginId[0],
            granted: granted[0],
            resetTokenDestination: resetTokenDestination[0],
            memo: memo[0],

            reset,
            submit: registerWithCurrentState,
        },
        initLoadableListAtomUpdater(list),
    ]
}

type RegisterUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: RegisterAuthUserAccountError }>
    | Readonly<{ type: "success"; data: AuthUserAccount }>
    | Readonly<{ type: "initial" }>

async function registerAuthUserAccount<S>(
    { infra, config }: RegisterAuthUserAccountMaterial,
    fields: AuthUserAccount,
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

    post({ type: "success", data: fields })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
