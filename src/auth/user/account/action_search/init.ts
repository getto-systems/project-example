import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { initObserveBoardAction } from "../../../../../ui/vendor/getto-application/board/action_observe_board/init"
import { initSearchLoginIDAction } from "../../login_id/input/action_search/init"
import { initSearchOffsetAction } from "../../../../z_lib/ui/search/action_offset/init"

import {
    SearchUserAccountMaterial,
    SearchUserAccountAction,
    SearchUserAccountState,
    initialSearchUserAccountState,
    searchUserAccountFieldNames,
} from "./action"
import { SearchLoginIDAction } from "../../login_id/input/action_search/action"
import { SearchOffsetAction } from "../../../../z_lib/ui/search/action_offset/action"
import { SearchColumnsAction } from "../../../../z_lib/ui/search/action_columns/action"
import { ObserveBoardAction } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"

import { searchUserAccount } from "../search/method"

import {
    SearchUserAccountFieldsDetecter,
    SearchUserAccountInfra,
    UpdateSearchUserAccountFieldsQuery,
} from "../search/infra"

import { SearchUserAccountFields } from "../search/data"
import {
    initSearchColumnsAction,
    initSearchColumnsMaterial,
} from "../../../../z_lib/ui/search/action_columns/init"
import { SearchColumnsInfra } from "../../../../z_lib/ui/search/columns/infra"
import { SearchSort } from "../../../../z_lib/ui/search/sort/data"
import { nextSort } from "../../../../z_lib/ui/search/sort/helper"

export type SearchUserAccountActionInfra = Readonly<{
    search: SearchUserAccountInfra
    columns: SearchColumnsInfra
}>

export function initSearchUserAccountMaterial(
    infra: SearchUserAccountActionInfra,
): SearchUserAccountMaterial {
    return {
        search: searchUserAccount(infra.search),
        columns: initSearchColumnsMaterial(infra.columns),
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
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction
    readonly observe: ObserveBoardAction

    material: SearchUserAccountMaterial

    searchFields: { (): SearchUserAccountFields }
    loadFields: { (): SearchUserAccountFields }
    sortFields: { (key: string): SearchUserAccountFields }

    updateQuery: UpdateSearchUserAccountFieldsQuery
    sortStore: SearchSort

    constructor(
        material: SearchUserAccountMaterial,
        detecter: SearchUserAccountFieldsDetecter,
        updateQuery: UpdateSearchUserAccountFieldsQuery,
    ) {
        super(async () => this.load())
        this.material = material

        const initialFields = detecter({ defaultSortKey: "login-id" })
        const loginID = initSearchLoginIDAction(initialFields.loginID)
        const offset = initSearchOffsetAction(initialFields.offset)
        const columns = initSearchColumnsAction(this.material.columns)
        const { observe, checker } = initObserveBoardAction({
            fields: searchUserAccountFieldNames,
        })

        this.searchFields = () => ({
            offset: offset.reset(),
            sort: this.currentSort(),
            loginID: loginID.pin(),
        })
        this.loadFields = () => ({
            offset: offset.get(),
            sort: this.currentSort(),
            loginID: loginID.peek(),
        })
        this.sortFields = (key: string) => ({
            offset: offset.reset(),
            sort: this.updateSort(key),
            loginID: loginID.peek(),
        })

        this.loginID = loginID.input
        this.offset = offset.input
        this.columns = columns
        this.observe = observe

        this.updateQuery = updateQuery
        this.sortStore = initialFields.sort

        this.loginID.observe.subscriber.subscribe((result) =>
            checker.update("loginID", result.hasChanged),
        )

        this.terminateHook(() => {
            this.loginID.terminate()
            this.observe.terminate()
        })
    }

    currentSort(): SearchSort {
        return this.sortStore
    }
    updateSort(key: string): SearchSort {
        this.sortStore = nextSort(this.currentSort(), key)
        return this.sortStore
    }

    clear(): SearchUserAccountState {
        this.loginID.clear()
        return this.initialState
    }
    async submit(): Promise<SearchUserAccountState> {
        return this.material.search(this.updateQuery, this.searchFields(), this.post)
    }
    async load(): Promise<SearchUserAccountState> {
        return this.material.search(this.updateQuery, this.loadFields(), this.post)
    }
    async sort(key: string): Promise<SearchUserAccountState> {
        return this.material.search(this.updateQuery, this.sortFields(key), this.post)
    }
}
