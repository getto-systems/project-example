import { checkTakeLongtime, ticker } from "../../../../common/util/timer/helper"
import { emptyLoginId } from "../kernel/convert"

import { Atom, initAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../common/util/load/data"
import { EditableBoardAction } from "../../../../common/util/board/editable/action"
import { ValidateBoardState } from "../../../../common/util/board/validate/action"
import { ObserveBoardState } from "../../../../common/util/board/observe/action"
import { composeModifyFieldBoard } from "../../../../common/util/board/field/action"
import { LoadableListAtomUpdater } from "../../../../common/util/list/action"
import { initLoginIdField, LoginIdField } from "../input/field/action"

import { OverwriteLoginIdRemote } from "./infra"
import { WaitTime } from "../../../../common/util/config/infra"

import { ConvertBoardResult } from "../../../../common/util/board/kernel/data"
import { ChangeLoginIdError, OverwriteLoginIdFields } from "./data"
import { LoginId } from "../kernel/data"
import { AuthUserAccount } from "../../account/kernel/data"
import { ConnectState } from "../../../../common/util/connect/data"

export interface OverwriteLoginIdAction {
    readonly state: Atom<OverwriteLoginIdState>
    readonly connect: Atom<ConnectState>
    readonly validate: Atom<ValidateBoardState>
    readonly observe: Atom<ObserveBoardState>
    readonly editable: EditableBoardAction

    readonly newLoginId: LoginIdField

    reset(): void
    submit(): Promise<OverwriteLoginIdState>
}

export type OverwriteLoginIdEntry = Readonly<{ loginId: LoginId }>

export type OverwriteLoginIdState = OverwriteLoginIdEvent

const initialState: OverwriteLoginIdState = { type: "initial" }

export type OverwriteLoginIdMaterial = Readonly<{
    infra: OverwriteLoginIdInfra
    config: OverwriteLoginIdConfig
}>

export type OverwriteLoginIdInfra = Readonly<{
    overwriteLoginIdRemote: OverwriteLoginIdRemote
}>

export type OverwriteLoginIdConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initOverwriteLoginIdAction(
    data: Atom<LoadState<AuthUserAccount>>,
    updater: LoadableListAtomUpdater<AuthUserAccount>,
    material: OverwriteLoginIdMaterial,
): OverwriteLoginIdAction {
    const modify = initAtom({ initialState })
    async function modifyWithCurrentState(): Promise<OverwriteLoginIdState> {
        const element = data.currentState()
        if (!element.isLoad) {
            return modify.state.currentState()
        }

        const fields = currentFields()
        if (!fields.valid) {
            return modify.state.currentState()
        }
        return overwriteLoginId(material, element.data, fields.value, modify.post)
    }

    const newLoginId = initLoginIdField()

    const currentFields = (): ConvertBoardResult<OverwriteLoginIdFields> => {
        const result = {
            newLoginId: newLoginId[0].validate.currentState(),
        }
        if (!result.newLoginId.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                newLoginId: result.newLoginId.value,
            },
        }
    }

    const { editable, validate, observe, reset } = composeModifyFieldBoard(data, [
        [newLoginId, (_data: AuthUserAccount) => emptyLoginId()],
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

        newLoginId: newLoginId[0],

        reset,
        submit: modifyWithCurrentState,
    }
}

type OverwriteLoginIdEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangeLoginIdError }>
    | Readonly<{ type: "success"; data: AuthUserAccount }>
    | Readonly<{ type: "initial" }>

async function overwriteLoginId<S>(
    { infra, config }: OverwriteLoginIdMaterial,
    user: AuthUserAccount,
    fields: OverwriteLoginIdFields,
    post: Post<OverwriteLoginIdEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { overwriteLoginIdRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        overwriteLoginIdRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    post({ type: "success", data: { ...user, loginId: fields.newLoginId } })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
