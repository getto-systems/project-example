import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"
import { nextSort } from "../../../../z_lib/ui/search/sort/helper"

import { initSearchLoginIDAction, SearchLoginIDAction } from "../../login_id/input/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"
import {
    initSearchOffsetAction,
    SearchOffsetAction,
} from "../../../../z_lib/ui/search/offset/action"
import {
    initSearchColumnsAction,
    SearchColumnsAction,
    SearchColumnsInfra,
} from "../../../../z_lib/ui/search/columns/action"

import {
    SearchAuthUserAccountFilterDetecter,
    SearchAuthUserAccountRemote,
    UpdateSearchAuthUserAccountFieldsQuery,
} from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"

import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import {
    SearchAuthUserAccountFilter,
    SearchAuthUserAccountRemoteResponse,
    SearchAuthUserAccountSort,
    SearchAuthUserAccountSortKey,
} from "./data"

export interface SearchAuthUserAccountAction
    extends StatefulApplicationAction<SearchAuthUserAccountState> {
    readonly loginID: SearchLoginIDAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction
    readonly observe: ObserveBoardAction

    currentSort(): SearchAuthUserAccountSort

    clear(): SearchAuthUserAccountState
    submit(): Promise<SearchAuthUserAccountState>
    load(): Promise<SearchAuthUserAccountState>
    sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState>
}

export type SearchAuthUserAccountMaterial = Readonly<{
    infra: SearchAuthUserAccountInfra
    shell: SearchAuthUserAccountShell
    config: SearchAuthUserAccountConfig
}>

export type SearchAuthUserAccountInfra = Readonly<{
    searchRemote: SearchAuthUserAccountRemote
}> &
    SearchColumnsInfra

export type SearchAuthUserAccountShell = Readonly<{
    detectFields: SearchAuthUserAccountFilterDetecter
    updateQuery: UpdateSearchAuthUserAccountFieldsQuery
}>

export type SearchAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export type SearchAuthUserAccountState =
    | Readonly<{ type: "initial-search" }>
    | (SearchAuthUserAccountEvent &
          Readonly<{ previousResponse?: SearchAuthUserAccountRemoteResponse }>)

export const initialSearchAuthUserAccountState: SearchAuthUserAccountState = {
    type: "initial-search",
}

export function initSearchAuthUserAccountAction(
    material: SearchAuthUserAccountMaterial,
): SearchAuthUserAccountAction {
    return new Action(material)
}

const searchAuthUserAccountFieldNames = ["loginID"] as const

class Action
    extends AbstractStatefulApplicationAction<SearchAuthUserAccountState>
    implements SearchAuthUserAccountAction
{
    readonly initialState = initialSearchAuthUserAccountState

    readonly loginID: SearchLoginIDAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction
    readonly observe: ObserveBoardAction

    material: SearchAuthUserAccountMaterial

    searchFields: { (): SearchAuthUserAccountFilter }
    loadFields: { (): SearchAuthUserAccountFilter }
    sortFields: { (key: SearchAuthUserAccountSortKey): SearchAuthUserAccountFilter }

    sortStore: SearchAuthUserAccountSort
    response?: SearchAuthUserAccountRemoteResponse

    constructor(material: SearchAuthUserAccountMaterial) {
        super({
            ignite: async () => this.load(),
            terminate: () => {
                this.loginID.terminate()
                this.observe.terminate()
            },
        })
        this.material = material

        const initialFields = material.shell.detectFields()

        const loginID = initSearchLoginIDAction(initialFields.loginID)
        const offset = initSearchOffsetAction(initialFields.offset)
        const columns = initSearchColumnsAction(this.material.infra)
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
        this.sortFields = (key: SearchAuthUserAccountSortKey) => ({
            offset: offset.reset(),
            sort: this.updateSort(key),
            loginID: loginID.peek(),
        })

        this.loginID = loginID.input
        this.offset = offset.input
        this.columns = columns
        this.observe = observe

        this.sortStore = initialFields.sort

        this.loginID.observe.subscriber.subscribe((result) =>
            checker.update("loginID", result.hasChanged),
        )
    }

    currentSort(): SearchAuthUserAccountSort {
        return this.sortStore
    }
    updateSort(key: SearchAuthUserAccountSortKey): SearchAuthUserAccountSort {
        this.sortStore = nextSort(this.currentSort(), key)
        return this.sortStore
    }

    clear(): SearchAuthUserAccountState {
        this.loginID.clear()
        return this.currentState()
    }
    async submit(): Promise<SearchAuthUserAccountState> {
        return search(this.material, this.searchFields(), (e) => this.searchResult(e))
    }
    async load(): Promise<SearchAuthUserAccountState> {
        return search(this.material, this.loadFields(), (e) => this.searchResult(e))
    }
    async sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState> {
        return search(this.material, this.sortFields(key), (e) => this.searchResult(e))
    }

    searchResult(e: SearchAuthUserAccountEvent): SearchAuthUserAccountState {
        const previousInfo = {
            previousResponse: this.response,
        }
        switch (e.type) {
            case "succeed-to-search":
                this.sortStore = e.response.sort
                this.response = e.response
                break
        }
        return this.post({
            ...e,
            ...previousInfo,
        })
    }
}

type SearchAuthUserAccountEvent =
    | Readonly<{ type: "try-to-search" }>
    | Readonly<{ type: "take-longtime-to-search" }>
    | Readonly<{ type: "failed-to-search"; err: RemoteCommonError }>
    | Readonly<{ type: "succeed-to-search"; response: SearchAuthUserAccountRemoteResponse }>

async function search<S>(
    { infra, shell, config }: SearchAuthUserAccountMaterial,
    fields: SearchAuthUserAccountFilter,
    post: Post<SearchAuthUserAccountEvent, S>,
): Promise<S> {
    shell.updateQuery(fields)
    post({ type: "try-to-search" })

    const { searchRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(searchRemote(fields), config.takeLongtimeThreshold, () =>
        post({ type: "take-longtime-to-search" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-search", err: response.err })
    }

    return post({ type: "succeed-to-search", response: response.value })
}

interface Post<E, S> {
    (event: E): S
}
