import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { initObserveBoardAction } from "../../../../../ui/vendor/getto-application/board/action_observe_board/init"
import { initSearchLoginIDAction } from "../../login_id/input/action_search/init"
import { initSearchOffsetAction } from "../../../../z_lib/ui/search/action_offset/init"

import {
    SearchAuthUserAccountMaterial,
    SearchAuthUserAccountAction,
    SearchAuthUserAccountState,
    initialSearchAuthUserAccountState,
    searchAuthUserAccountFieldNames,
} from "./action"
import { SearchLoginIDAction } from "../../login_id/input/action_search/action"
import { SearchOffsetAction } from "../../../../z_lib/ui/search/action_offset/action"
import { SearchColumnsAction } from "../../../../z_lib/ui/search/action_columns/action"
import { ObserveBoardAction } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"

import { searchAuthUserAccount } from "../search/method"

import {
    SearchAuthUserAccountFieldsDetecter,
    SearchAuthUserAccountInfra,
    UpdateSearchAuthUserAccountFieldsQuery,
} from "../search/infra"

import { SearchAuthUserAccountFields } from "../search/data"
import {
    initSearchColumnsAction,
    initSearchColumnsMaterial,
} from "../../../../z_lib/ui/search/action_columns/init"
import { SearchColumnsInfra } from "../../../../z_lib/ui/search/columns/infra"
import { SearchSort } from "../../../../z_lib/ui/search/sort/data"
import { nextSort } from "../../../../z_lib/ui/search/sort/helper"

export type SearchAuthUserAccountActionInfra = Readonly<{
    search: SearchAuthUserAccountInfra
    columns: SearchColumnsInfra
}>

export function initSearchAuthUserAccountMaterial(
    infra: SearchAuthUserAccountActionInfra,
): SearchAuthUserAccountMaterial {
    return {
        search: searchAuthUserAccount(infra.search),
        columns: initSearchColumnsMaterial(infra.columns),
    }
}

export function initSearchAuthUserAccountAction(
    material: SearchAuthUserAccountMaterial,
    detecter: SearchAuthUserAccountFieldsDetecter,
    updateQuery: UpdateSearchAuthUserAccountFieldsQuery,
): SearchAuthUserAccountAction {
    return new Action(material, detecter, updateQuery)
}

class Action
    extends ApplicationAbstractStateAction<SearchAuthUserAccountState>
    implements SearchAuthUserAccountAction
{
    readonly initialState = initialSearchAuthUserAccountState

    readonly loginID: SearchLoginIDAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction
    readonly observe: ObserveBoardAction

    material: SearchAuthUserAccountMaterial

    searchFields: { (): SearchAuthUserAccountFields }
    loadFields: { (): SearchAuthUserAccountFields }
    sortFields: { (key: string): SearchAuthUserAccountFields }

    updateQuery: UpdateSearchAuthUserAccountFieldsQuery
    sortStore: SearchSort

    constructor(
        material: SearchAuthUserAccountMaterial,
        detecter: SearchAuthUserAccountFieldsDetecter,
        updateQuery: UpdateSearchAuthUserAccountFieldsQuery,
    ) {
        super(async () => this.load())
        this.material = material

        const initialFields = detecter({ defaultSortKey: "login-id" })
        const loginID = initSearchLoginIDAction(initialFields.loginID)
        const offset = initSearchOffsetAction(initialFields.offset)
        const columns = initSearchColumnsAction(this.material.columns)
        const { observe, checker } = initObserveBoardAction({
            fields: searchAuthUserAccountFieldNames,
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

    clear(): SearchAuthUserAccountState {
        this.loginID.clear()
        return this.initialState
    }
    async submit(): Promise<SearchAuthUserAccountState> {
        return this.material.search(this.updateQuery, this.searchFields(), this.post)
    }
    async load(): Promise<SearchAuthUserAccountState> {
        return this.material.search(this.updateQuery, this.loadFields(), this.post)
    }
    async sort(key: string): Promise<SearchAuthUserAccountState> {
        return this.material.search(this.updateQuery, this.sortFields(key), this.post)
    }
}
