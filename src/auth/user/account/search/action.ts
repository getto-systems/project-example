import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"
import { nextSort } from "../../../../z_lib/ui/search/sort/helper"

import { initSearchLoginIdAction, SearchLoginIdAction } from "../../login_id/input/action"
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

import { searchResponse } from "../../../../z_lib/ui/search/kernel/x_preact/helper"

import {
    FocusAuthUserAccountDetecter,
    SearchAuthUserAccountFilterDetecter,
    SearchAuthUserAccountRemote,
    UpdateFocusAuthUserAccountQuery,
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
import { AuthUserAccount } from "../kernel/data"

export interface SearchAuthUserAccountAction extends ListAuthUserAccountAction {
    readonly loginId: SearchLoginIdAction
    readonly observe: ObserveBoardAction

    clear(): SearchAuthUserAccountState
    search(): Promise<SearchAuthUserAccountState>
}
export interface ListAuthUserAccountAction
    extends StatefulApplicationAction<SearchAuthUserAccountState> {
    readonly detail: DetailAuthUserAccountAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction

    currentSort(): SearchAuthUserAccountSort

    load(): Promise<SearchAuthUserAccountState>
    sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState>
}
export interface DetailAuthUserAccountAction
    extends StatefulApplicationAction<DetailAuthUserAccountState> {
    focus(user: AuthUserAccount): DetailAuthUserAccountState
    update(user: AuthUserAccount): DetailAuthUserAccountState
    close(): DetailAuthUserAccountState

    isFocused(user: AuthUserAccount): boolean
}

export type SearchAuthUserAccountState =
    | Readonly<{ type: "initial-search" }>
    | SearchAuthUserAccountEvent

const initialSearchState: SearchAuthUserAccountState = { type: "initial-search" }

export type DetailAuthUserAccountState =
    | Readonly<{ type: "initial-detail" }>
    | Readonly<{ type: "focus-failed" }>
    | Readonly<{ type: "focus-detected"; user: AuthUserAccount }>
    | Readonly<{ type: "focus-on"; user: AuthUserAccount }>

const initialDetailState: DetailAuthUserAccountState = { type: "initial-detail" }

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
    detectFilter: SearchAuthUserAccountFilterDetecter
    updateQuery: UpdateSearchAuthUserAccountFieldsQuery
    detectFocus: FocusAuthUserAccountDetecter
    updateFocus: UpdateFocusAuthUserAccountQuery
}>

export type SearchAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export function initSearchAuthUserAccountAction(
    material: SearchAuthUserAccountMaterial,
): SearchAuthUserAccountAction {
    return new Action(material)
}

class Action
    extends AbstractStatefulApplicationAction<SearchAuthUserAccountState>
    implements SearchAuthUserAccountAction
{
    readonly initialState = initialSearchState

    readonly detail: DetailAuthUserAccountAction

    readonly loginId: SearchLoginIdAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction
    readonly observe: ObserveBoardAction

    material: SearchAuthUserAccountMaterial

    filter: SearchAuthUserAccountFilter
    setFilterOnSearch: { (): SearchAuthUserAccountFilter }
    setFilterOnLoad: { (): SearchAuthUserAccountFilter }
    setFilterOnSort: { (key: SearchAuthUserAccountSortKey): SearchAuthUserAccountFilter }

    response?: SearchAuthUserAccountRemoteResponse

    constructor(material: SearchAuthUserAccountMaterial) {
        super({
            ignite: async () => this.load(),
            terminate: () => {
                this.detail.terminate()
                this.loginId.terminate()
                this.offset.terminate()
                this.columns.terminate()
                this.observe.terminate()
            },
        })

        const initialFilter = material.shell.detectFilter()

        const fields = ["loginId"] as const

        const loginId = initSearchLoginIdAction(initialFilter.loginId)
        const offset = initSearchOffsetAction(initialFilter.offset)
        const columns = initSearchColumnsAction(material.infra)
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.setFilterOnSearch = () =>
            this.setFilter({
                offset: offset.reset(),
                loginId: loginId.pin(),
            })
        this.setFilterOnLoad = () =>
            this.setFilter({
                offset: offset.get(),
            })
        this.setFilterOnSort = (key: SearchAuthUserAccountSortKey) =>
            this.setFilter({
                offset: offset.reset(),
                sort: nextSort(this.currentSort(), key),
            })

        this.material = material

        this.detail = new DetailAction({
            infra: {
                detectUser: async (loginId): Promise<DetectUserResult> => {
                    const response = searchResponse(await this.ignitionState)
                    if (!response.found) {
                        return { found: false }
                    }
                    const user = response.response.users.find((user) => user.loginId === loginId)
                    if (user === undefined) {
                        return { found: false }
                    }
                    return { found: true, user }
                },
                updateUser: (user) => this.update(user),
            },
            shell: material.shell,
        })

        this.loginId = loginId.input
        this.offset = offset.input
        this.columns = columns
        this.observe = observe

        this.filter = initialFilter

        this.loginId.observe.subscriber.subscribe((result) =>
            observeChecker.update("loginId", result.hasChanged),
        )
    }

    setFilter(filter: Partial<SearchAuthUserAccountFilter>): SearchAuthUserAccountFilter {
        this.filter = { ...this.filter, ...filter }
        return this.filter
    }

    currentSort(): SearchAuthUserAccountSort {
        return this.filter.sort
    }

    clear(): SearchAuthUserAccountState {
        this.loginId.clear()
        return this.currentState()
    }

    async search(): Promise<SearchAuthUserAccountState> {
        return this.updateResponse(
            await search(this.material, this.setFilterOnSearch(), this.response, this.post),
        )
    }
    async load(): Promise<SearchAuthUserAccountState> {
        return this.updateResponse(
            await search(this.material, this.setFilterOnLoad(), this.response, this.post),
        )
    }
    async sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState> {
        return this.updateResponse(
            await search(this.material, this.setFilterOnSort(key), this.response, this.post),
        )
    }
    updateResponse(state: SearchAuthUserAccountState): SearchAuthUserAccountState {
        switch (state.type) {
            case "succeed-to-search":
                this.setFilter({ sort: state.response.sort })
                this.response = state.response
                break
        }
        return state
    }

    update(user: AuthUserAccount): SearchAuthUserAccountState {
        if (!this.response) {
            return this.currentState()
        }

        this.response = {
            ...this.response,
            users: this.response.users.map((row) => {
                if (row.loginId !== user.loginId) {
                    return row
                }
                return user
            }),
        }

        const state = this.currentState()
        switch (state.type) {
            case "initial-search":
                return state

            case "try-to-search":
            case "take-longtime-to-search":
            case "failed-to-search":
                return this.post({ ...state, previousResponse: this.response })

            case "succeed-to-search":
                return this.post({ ...state, response: this.response })
        }
    }
}

type SearchAuthUserAccountEvent =
    | Readonly<{ type: "try-to-search"; previousResponse?: SearchAuthUserAccountRemoteResponse }>
    | Readonly<{
          type: "take-longtime-to-search"
          previousResponse?: SearchAuthUserAccountRemoteResponse
      }>
    | Readonly<{
          type: "failed-to-search"
          err: RemoteCommonError
          previousResponse?: SearchAuthUserAccountRemoteResponse
      }>
    | Readonly<{ type: "succeed-to-search"; response: SearchAuthUserAccountRemoteResponse }>

async function search<S>(
    { infra, shell, config }: SearchAuthUserAccountMaterial,
    fields: SearchAuthUserAccountFilter,
    previousResponse: SearchAuthUserAccountRemoteResponse | undefined,
    post: Post<SearchAuthUserAccountEvent, S>,
): Promise<S> {
    shell.updateQuery(fields)
    post({ type: "try-to-search", previousResponse })

    const { searchRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(searchRemote(fields), config.takeLongtimeThreshold, () =>
        post({ type: "take-longtime-to-search", previousResponse }),
    )
    if (!response.success) {
        return post({ type: "failed-to-search", err: response.err, previousResponse })
    }

    return post({ type: "succeed-to-search", response: response.value })
}

type DetailMaterial = Readonly<{
    infra: DetailInfra
    shell: DetailShell
}>

interface DetailInfra {
    detectUser(loginId: string): Promise<DetectUserResult>
    updateUser(user: AuthUserAccount): void
}
type DetectUserResult =
    | Readonly<{ found: false }>
    | Readonly<{ found: true; user: AuthUserAccount }>

type DetailShell = Readonly<{
    detectFocus: FocusAuthUserAccountDetecter
    updateFocus: UpdateFocusAuthUserAccountQuery
}>

class DetailAction
    extends AbstractStatefulApplicationAction<DetailAuthUserAccountState>
    implements DetailAuthUserAccountAction
{
    readonly initialState = initialDetailState

    material: DetailMaterial

    constructor(material: DetailMaterial) {
        super({
            ignite: async () => {
                const focus = this.material.shell.detectFocus()
                if (!focus.found) {
                    return this.currentState()
                }
                const user = await this.material.infra.detectUser(focus.loginId)
                if (!user.found) {
                    return this.post({ type: "focus-failed" })
                }
                return this.post({ type: "focus-detected", user: user.user })
            },
        })
        this.material = material
    }

    focus(user: AuthUserAccount): DetailAuthUserAccountState {
        this.material.shell.updateFocus.focus(user)
        return this.post({ type: "focus-on", user })
    }
    update(user: AuthUserAccount): DetailAuthUserAccountState {
        this.material.infra.updateUser(user)
        return this.post({ type: "focus-on", user })
    }
    close(): DetailAuthUserAccountState {
        this.material.shell.updateFocus.clear()
        return this.post({ type: "initial-detail" })
    }

    isFocused(user: AuthUserAccount): boolean {
        const state = this.currentState()
        switch (state.type) {
            case "initial-detail":
            case "focus-failed":
                return false

            case "focus-detected":
            case "focus-on":
                return user.loginId === state.user.loginId
        }
    }
}

interface Post<E, S> {
    (event: E): S
}
