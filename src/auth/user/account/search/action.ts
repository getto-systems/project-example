import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"
import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"
import { nextSort } from "../../../../z_lib/ui/search/sort/helper"

import { initSearchLoginIDAction, SearchLoginIDAction } from "../../login_id/input/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../../ui/vendor/getto-application/board/observe_board/action"
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
    SearchAuthUserAccountFieldsDetecter,
    SearchAuthUserAccountRemote,
    UpdateSearchAuthUserAccountFieldsQuery,
} from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"

import { SearchSort } from "../../../../z_lib/ui/search/sort/data"
import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { SearchAuthUserAccountFields, SearchAuthUserAccountRemoteResponse } from "./data"

export interface SearchAuthUserAccountAction
    extends ApplicationStateAction<SearchAuthUserAccountState> {
    readonly loginID: SearchLoginIDAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction
    readonly observe: ObserveBoardAction

    currentSort(): SearchSort

    clear(): SearchAuthUserAccountState
    submit(): Promise<SearchAuthUserAccountState>
    load(): Promise<SearchAuthUserAccountState>
    sort(key: string): Promise<SearchAuthUserAccountState>
}

export type SearchAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export type SearchAuthUserAccountInfra = Readonly<{
    searchRemote: SearchAuthUserAccountRemote
}> &
    SearchColumnsInfra

export type SearchAuthUserAccountShell = Readonly<{
    detectFields: SearchAuthUserAccountFieldsDetecter
    updateQuery: UpdateSearchAuthUserAccountFieldsQuery
}>

export type SearchAuthUserAccountState =
    | Readonly<{ type: "initial-search" }>
    | SearchAuthUserAccountEvent

export const initialSearchAuthUserAccountState: SearchAuthUserAccountState = {
    type: "initial-search",
}

export function initSearchAuthUserAccountAction(
    config: SearchAuthUserAccountConfig,
    infra: SearchAuthUserAccountInfra,
    shell: SearchAuthUserAccountShell,
): SearchAuthUserAccountAction {
    return new Action(config, infra, shell)
}

const searchAuthUserAccountFieldNames = ["loginID"] as const

class Action
    extends ApplicationAbstractStateAction<SearchAuthUserAccountState>
    implements SearchAuthUserAccountAction
{
    readonly initialState = initialSearchAuthUserAccountState

    readonly loginID: SearchLoginIDAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction
    readonly observe: ObserveBoardAction

    config: SearchAuthUserAccountConfig
    infra: SearchAuthUserAccountInfra
    shell: SearchAuthUserAccountShell

    searchFields: { (): SearchAuthUserAccountFields }
    loadFields: { (): SearchAuthUserAccountFields }
    sortFields: { (key: string): SearchAuthUserAccountFields }

    sortStore: SearchSort

    constructor(
        config: SearchAuthUserAccountConfig,
        infra: SearchAuthUserAccountInfra,
        shell: SearchAuthUserAccountShell,
    ) {
        super(async () => this.load())
        this.config = config
        this.infra = infra
        this.shell = shell

        const initialFields = shell.detectFields({ defaultSortKey: "login-id" })

        const loginID = initSearchLoginIDAction(initialFields.loginID)
        const offset = initSearchOffsetAction(initialFields.offset)
        const columns = initSearchColumnsAction(this.infra)
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
        return search(this.config, this.infra, this.shell, this.searchFields(), this.post)
    }
    async load(): Promise<SearchAuthUserAccountState> {
        return search(this.config, this.infra, this.shell, this.loadFields(), this.post)
    }
    async sort(key: string): Promise<SearchAuthUserAccountState> {
        return search(this.config, this.infra, this.shell, this.sortFields(key), this.post)
    }
}

type SearchAuthUserAccountEvent =
    | Readonly<{ type: "try-to-search" }>
    | Readonly<{ type: "take-longtime-to-search" }>
    | Readonly<{ type: "failed-to-search"; err: RemoteCommonError }>
    | Readonly<{ type: "succeed-to-search"; response: SearchAuthUserAccountRemoteResponse }>

async function search<S>(
    config: SearchAuthUserAccountConfig,
    infra: SearchAuthUserAccountInfra,
    shell: SearchAuthUserAccountShell,
    fields: SearchAuthUserAccountFields,
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
