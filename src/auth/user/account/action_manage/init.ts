import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { initObserveBoardAction } from "../../../../../ui/vendor/getto-application/board/action_observe_board/init"
import { initSearchLoginIDAction } from "../../login_id/input/action_search/init"

import {
    ManageUserAccountMaterial,
    ManageUserAccountAction,
    ManageUserAccountState,
    initialManageUserAccountState,
    manageUserAccountFieldNames,
} from "./action"
import { SearchLoginIDAction } from "../../login_id/input/action_search/action"
import { ObserveBoardAction } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"

import { searchUserAccount } from "../search/method"

import {
    SearchUserAccountFieldsDetecter,
    SearchUserAccountInfra,
    UpdateSearchUserAccountFieldsQuery,
} from "../search/infra"

import { SearchUserAccountFields } from "../search/data"

export type ManageUserAccountActionInfra = Readonly<{
    search: SearchUserAccountInfra
}>

export function initAuthenticatePasswordMaterial(
    infra: ManageUserAccountActionInfra,
): ManageUserAccountMaterial {
    return {
        search: searchUserAccount(infra.search),
    }
}

export function initAuthenticatePasswordAction(
    material: ManageUserAccountMaterial,
    detecter: SearchUserAccountFieldsDetecter,
    updateQuery: UpdateSearchUserAccountFieldsQuery,
): ManageUserAccountAction {
    return new Action(material, detecter, updateQuery)
}

class Action
    extends ApplicationAbstractStateAction<ManageUserAccountState>
    implements ManageUserAccountAction
{
    readonly initialState = initialManageUserAccountState

    readonly loginID: SearchLoginIDAction
    readonly observe: ObserveBoardAction

    material: ManageUserAccountMaterial

    fields: { (): SearchUserAccountFields }

    constructor(
        material: ManageUserAccountMaterial,
        detecter: SearchUserAccountFieldsDetecter,
        updateQuery: UpdateSearchUserAccountFieldsQuery,
    ) {
        super()
        this.material = material

        const initialFields = detecter()
        const loginID = initSearchLoginIDAction(initialFields.loginID)
        const { observe, checker } = initObserveBoardAction({
            fields: manageUserAccountFieldNames,
        })

        this.fields = () => {
            const fields = {
                loginID: loginID.pin(),
            }
            updateQuery(fields)
            return fields
        }

        this.loginID = loginID.input
        this.observe = observe

        this.loginID.observe.subscriber.subscribe((result) =>
            checker.update("loginID", result.hasChanged),
        )

        this.terminateHook(() => {
            this.loginID.terminate()
            this.observe.terminate()
        })
    }

    clear(): ManageUserAccountState {
        this.loginID.clear()
        return this.initialState
    }
    async submit(): Promise<ManageUserAccountState> {
        return this.material.search(this.fields(), this.post)
    }
}
