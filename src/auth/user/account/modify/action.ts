import { checkTakeLongtime, ticker } from "../../../../common/util/timer/helper"

import { ALL_AUTH_PERMISSIONS } from "../../../../x_content/permission"

import { Atom, initAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import { LoadState, loadState_loaded } from "../../../../common/util/load/data"
import { AuthUserTextField, initAuthUserTextField } from "../input/field/action"
import {
    AuthPermissionGrantedField,
    initAuthPermissionGrantedField,
} from "../../kernel/input/field/action"
import { EditableBoardAction } from "../../../../common/util/board/editable/action"
import { ValidateBoardState } from "../../../../common/util/board/validate/action"
import { ObserveBoardState } from "../../../../common/util/board/observe/action"
import { LoadableListAtomUpdater } from "../../../../common/util/list/action"
import { composeModifyFieldBoard } from "../../../../common/util/board/field/action"

import { ModifyAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../common/util/config/infra"

import { ConvertBoardResult } from "../../../../common/util/board/kernel/data"
import { AuthUserAccount } from "../kernel/data"
import { ModifyAuthUserAccountError, ModifyAuthUserAccountFields } from "./data"
import { ConnectState } from "../../../../common/util/connect/data"

export interface ModifyAuthUserAccountAction {
    readonly state: Atom<ModifyAuthUserAccountState>
    readonly connect: Atom<ConnectState>
    readonly validate: Atom<ValidateBoardState>
    readonly observe: Atom<ObserveBoardState>
    readonly editable: EditableBoardAction

    readonly memo: AuthUserTextField<"memo">
    readonly granted: AuthPermissionGrantedField

    reset(): void
    submit(): Promise<ModifyAuthUserAccountState>
}

export type ModifyAuthUserAccountState = ModifyUserEvent

const initialState: ModifyAuthUserAccountState = { type: "initial" }

export type ModifyAuthUserAccountMaterial = Readonly<{
    infra: ModifyAuthUserAccountInfra
    config: ModifyAuthUserAccountConfig
}>

export type ModifyAuthUserAccountInfra = Readonly<{
    modifyUserRemote: ModifyAuthUserAccountRemote
}>

export type ModifyAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initModifyAuthUserAccountAction(
    data: Atom<LoadState<AuthUserAccount>>,
    updater: LoadableListAtomUpdater<AuthUserAccount>,
    material: ModifyAuthUserAccountMaterial,
): ModifyAuthUserAccountAction {
    const modify = initAtom({ initialState })
    async function modifyWithCurrentState(): Promise<ModifyAuthUserAccountState> {
        const element = data.currentState()
        if (!element.isLoad) {
            return modify.state.currentState()
        }

        const fields = currentFields()
        if (!fields.valid) {
            return modify.state.currentState()
        }
        return modifyAuthUser(material, element.data, fields.value, modify.post)
    }

    const grantedOptions = initAtom({ initialState: loadState_loaded(ALL_AUTH_PERMISSIONS) })

    const memo = initAuthUserTextField("memo")
    const granted = initAuthPermissionGrantedField(grantedOptions.state)

    const currentFields = (): ConvertBoardResult<ModifyAuthUserAccountFields> => {
        const result = {
            memo: memo[0].validate.currentState(),
            granted: granted[0].validate.currentState(),
        }
        if (!result.memo.valid || !result.granted.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                memo: result.memo.value,
                granted: result.granted.value,
            },
        }
    }

    const { editable, validate, observe, reset } = composeModifyFieldBoard(data, [
        [memo, (data: AuthUserAccount) => data.memo],
        [granted, (data: AuthUserAccount) => data.granted],
    ])

    modify.state.subscribe((state) => {
        if (state.type === "success") {
            updater.update((list) =>
                list.map((item) => {
                    return item.loginId === state.data.loginId ? state.data : item
                }),
            )
        }
    })

    const connect = mapAtom(modify.state, (state): ConnectState => {
        if (state.type === "try") {
            return { isConnecting: true, hasTakenLongtime: state.hasTakenLongtime }
        } else {
            return { isConnecting: false }
        }
    })

    return {
        state: modify.state,
        connect,
        validate,
        observe,
        editable,

        memo: memo[0],
        granted: granted[0],

        reset,
        submit: modifyWithCurrentState,
    }
}

type ModifyUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ModifyAuthUserAccountError }>
    | Readonly<{ type: "success"; data: AuthUserAccount }>
    | Readonly<{ type: "initial" }>

async function modifyAuthUser<S>(
    { infra, config }: ModifyAuthUserAccountMaterial,
    user: AuthUserAccount,
    fields: ModifyAuthUserAccountFields,
    post: Post<ModifyUserEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { modifyUserRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        modifyUserRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    post({ type: "success", data: { ...user, ...fields } })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
