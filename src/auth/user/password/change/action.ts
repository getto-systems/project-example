import { checkTakeLongtime, ticker } from "../../../../common/util/timer/helper"
import { emptyPassword } from "../input/field/convert"

import { Atom, initAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import { LoadState, loadState_loading } from "../../../../common/util/load/data"
import { EditableBoardAction } from "../../../../common/util/board/editable/action"
import { ValidateBoardState } from "../../../../common/util/board/validate/action"
import { ObserveBoardState } from "../../../../common/util/board/observe/action"
import { composeModifyFieldBoard } from "../../../../common/util/board/field/action"
import { initPasswordField, PasswordField } from "../input/field/action"

import { ChangePasswordRemote, OverwritePasswordRemote } from "./infra"
import { WaitTime } from "../../../../common/util/config/infra"

import { ConvertBoardResult } from "../../../../common/util/board/kernel/data"
import { ChangePasswordError, ChangePasswordFields, OverwritePasswordFields } from "./data"
import { LoginId } from "../../login_id/kernel/data"
import { AuthUserAccount } from "../../account/kernel/data"
import { ConnectState } from "../../../../common/util/connect/data"

export interface ChangePasswordAction {
    readonly state: Atom<ChangePasswordState>
    readonly connect: Atom<ConnectState>
    readonly validate: Atom<ValidateBoardState>
    readonly observe: Atom<ObserveBoardState>
    readonly editable: EditableBoardAction

    readonly currentPassword: PasswordField
    readonly newPassword: PasswordField

    reset(): void
    submit(): Promise<ChangePasswordState>
}

export type ChangePasswordState = ChangePasswordEvent

const initialState: ChangePasswordState = { type: "initial" }

export interface OverwritePasswordAction {
    readonly state: Atom<OverwritePasswordState>
    readonly connect: Atom<ConnectState>
    readonly validate: Atom<ValidateBoardState>
    readonly observe: Atom<ObserveBoardState>
    readonly editable: EditableBoardAction

    readonly newPassword: PasswordField

    reset(): void
    submit(): Promise<OverwritePasswordState>
}

export type OverwritePasswordEntry = Readonly<{ loginId: LoginId }>

export type OverwritePasswordState = OverwritePasswordEvent

const initialOverwriteState: OverwritePasswordState = { type: "initial" }

export type ChangePasswordMaterial = Readonly<{
    infra: ChangePasswordInfra
    config: ChangePasswordConfig
}>

export type ChangePasswordInfra = Readonly<{
    changePasswordRemote: ChangePasswordRemote
}>

export type ChangePasswordConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initChangePasswordAction(material: ChangePasswordMaterial): ChangePasswordAction {
    const change = initAtom({ initialState })
    async function changeWithCurrentState(): Promise<ChangePasswordState> {
        const fields = currentFields()
        if (!fields.valid) {
            return change.state.currentState()
        }
        return changePassword(material, fields.value, change.post)
    }

    const currentPassword = initPasswordField()
    const newPassword = initPasswordField()

    const currentFields = (): ConvertBoardResult<ChangePasswordFields> => {
        const result = {
            currentPassword: currentPassword[0].validate.currentState(),
            newPassword: newPassword[0].validate.currentState(),
        }
        if (!result.newPassword.valid || !result.currentPassword.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                currentPassword: result.currentPassword.value,
                newPassword: result.newPassword.value,
            },
        }
    }

    const data = initAtom<LoadState<AuthUserAccount>>({ initialState: loadState_loading() })
    const { editable, validate, observe, reset } = composeModifyFieldBoard(data.state, [
        [currentPassword, (_data: AuthUserAccount) => emptyPassword()],
        [newPassword, (_data: AuthUserAccount) => emptyPassword()],
    ])

    const connect = mapAtom(change.state, (state): ConnectState => {
        if (state.type === "try") {
            return { isConnecting: true, hasTakenLongtime: state.hasTakenLongtime }
        } else {
            return { isConnecting: false }
        }
    })

    return {
        state: change.state,
        connect,
        validate,
        observe,
        editable,

        currentPassword: currentPassword[0],
        newPassword: newPassword[0],

        reset,
        submit: changeWithCurrentState,
    }
}

type ChangePasswordEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangePasswordError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function changePassword<S>(
    { infra, config }: ChangePasswordMaterial,
    fields: ChangePasswordFields,
    post: Post<ChangePasswordEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { changePasswordRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        changePasswordRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    post({ type: "success" })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

export type OverwritePasswordMaterial = Readonly<{
    infra: OverwritePasswordInfra
    config: OverwritePasswordConfig
}>

export type OverwritePasswordInfra = Readonly<{
    overwritePasswordRemote: OverwritePasswordRemote
}>

export type OverwritePasswordConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initOverwritePasswordAction(
    data: Atom<LoadState<AuthUserAccount>>,
    material: OverwritePasswordMaterial,
): OverwritePasswordAction {
    const overwrite = initAtom({ initialState: initialOverwriteState })
    async function overwriteWithCurrentState(): Promise<OverwritePasswordState> {
        const element = data.currentState()
        if (!element.isLoad) {
            return overwrite.state.currentState()
        }

        const fields = currentFields()
        if (!fields.valid) {
            return overwrite.state.currentState()
        }
        return overwritePassword(material, element.data, fields.value, overwrite.post)
    }

    const newPassword = initPasswordField()

    const currentFields = (): ConvertBoardResult<OverwritePasswordFields> => {
        const result = {
            newPassword: newPassword[0].validate.currentState(),
        }
        if (!result.newPassword.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                newPassword: result.newPassword.value,
            },
        }
    }

    const { editable, validate, observe, reset } = composeModifyFieldBoard(data, [
        [newPassword, (_data: AuthUserAccount) => emptyPassword()],
    ])

    const connect = mapAtom(overwrite.state, (state): ConnectState => {
        if (state.type === "try") {
            return { isConnecting: true, hasTakenLongtime: state.hasTakenLongtime }
        } else {
            return { isConnecting: false }
        }
    })

    return {
        state: overwrite.state,
        connect,
        validate,
        observe,
        editable,

        newPassword: newPassword[0],

        reset,
        submit: overwriteWithCurrentState,
    }
}

type OverwritePasswordEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangePasswordError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function overwritePassword<S>(
    { infra, config }: OverwritePasswordMaterial,
    user: AuthUserAccount,
    fields: OverwritePasswordFields,
    post: Post<OverwritePasswordEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { overwritePasswordRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        overwritePasswordRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    post({ type: "success" })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
