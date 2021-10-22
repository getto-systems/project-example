import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { initObserveBoardAction } from "../../../../../ui/vendor/getto-application/board/action_observe_board/init"
import { initSearchLoginIDAction } from "../../login_id/input/action_search/init"

import {
    SearchUserAccountMaterial,
    SearchUserAccountAction,
    SearchUserAccountState,
    initialSearchUserAccountState,
    searchUserAccountFieldNames,
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

export type SearchUserAccountActionInfra = Readonly<{
    search: SearchUserAccountInfra
}>

export function initSearchUserAccountMaterial(
    infra: SearchUserAccountActionInfra,
): SearchUserAccountMaterial {
    return {
        search: searchUserAccount(infra.search),
    }
}

export function initSearchUserAccountAction(
    material: SearchUserAccountMaterial,
    detecter: SearchUserAccountFieldsDetecter,
    updateQuery: UpdateSearchUserAccountFieldsQuery,
): SearchUserAccountAction {
    return new Action(material, detecter, updateQuery)
}

class Action
    extends ApplicationAbstractStateAction<SearchUserAccountState>
    implements SearchUserAccountAction
{
    readonly initialState = initialSearchUserAccountState

    readonly loginID: SearchLoginIDAction
    readonly observe: ObserveBoardAction

    material: SearchUserAccountMaterial

    fields: { (): SearchUserAccountFields }

    constructor(
        material: SearchUserAccountMaterial,
        detecter: SearchUserAccountFieldsDetecter,
        updateQuery: UpdateSearchUserAccountFieldsQuery,
    ) {
        super(async () => this.submit())
        this.material = material

        const initialFields = detecter()
        const loginID = initSearchLoginIDAction(initialFields.loginID)
        const { observe, checker } = initObserveBoardAction({
            fields: searchUserAccountFieldNames,
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

    clear(): SearchUserAccountState {
        this.loginID.clear()
        return this.initialState
    }
    async submit(): Promise<SearchUserAccountState> {
        return this.material.search(this.fields(), this.post)
    }
}
