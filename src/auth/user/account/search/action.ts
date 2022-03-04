import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"
import { nextSort } from "../../../../z_lib/ui/search/sort/helper"

import {
    initSearchLoginIDAction,
    InputLoginIDAction,
    SearchLoginIDAction,
} from "../../login_id/input/action"
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
import { InputPasswordAction } from "../../password/input/action"
import { AuthUserAccountBasket } from "../kernel/data"

export interface SearchAuthUserAccountAction extends ListAuthUserAccountAction {
    readonly loginID: SearchLoginIDAction
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
    // readonly loginID: ChangeAuthUserLoginIDAction
    // readonly password: ChangeAuthUserPasswordAction
    //readonly grantedRoles: ChangeAuthUserGrantedRolesAction

    focus(user: AuthUserAccountBasket): DetailAuthUserAccountState
    close(): DetailAuthUserAccountState

    isFocused(user: AuthUserAccountBasket): boolean
}
export interface ChangeAuthUserLoginIDAction
    extends StatefulApplicationAction<ChangeAuthUserLoginIDState> {
    readonly loginID: InputLoginIDAction

    changeLoginID(): Promise<ChangeAuthUserLoginIDState>
}
export interface ChangeAuthUserPasswordAction
    extends StatefulApplicationAction<ChangeAuthUserPasswordState> {
    readonly password: InputPasswordAction

    changePassword(): Promise<ChangeAuthUserPasswordState>
}

export type SearchAuthUserAccountState =
    | Readonly<{ type: "initial-search" }>
    | (SearchAuthUserAccountEvent &
          Readonly<{
              previousResponse?: SearchAuthUserAccountRemoteResponse
          }>)

const initialSearchState: SearchAuthUserAccountState = { type: "initial-search" }

export type DetailAuthUserAccountState =
    | Readonly<{ type: "initial-detail" }>
    | Readonly<{ type: "focus-on"; user: AuthUserAccountBasket }>

const initialDetailState: DetailAuthUserAccountState = { type: "initial-detail" }

export type ChangeAuthUserLoginIDState = Readonly<{ type: "initial-change-login-id" }>

const initialChangeLoginIDState: ChangeAuthUserLoginIDState = { type: "initial-change-login-id" }

export type ChangeAuthUserPasswordState = Readonly<{ type: "initial-change-password" }>

const initialChangePasswordState: ChangeAuthUserPasswordState = { type: "initial-change-password" }

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
}>

export type SearchAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

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
    readonly initialState = initialSearchState

    readonly detail: DetailAuthUserAccountAction

    readonly loginID: SearchLoginIDAction
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
                this.loginID.terminate()
                this.observe.terminate()
            },
        })

        const initialFilter = material.shell.detectFilter()

        const loginID = initSearchLoginIDAction(initialFilter.loginID)
        const offset = initSearchOffsetAction(initialFilter.offset)
        const columns = initSearchColumnsAction(material.infra)
        const { observe, checker } = initObserveBoardAction({
            fields: searchAuthUserAccountFieldNames,
        })

        this.setFilterOnSearch = () =>
            this.setFilter({
                offset: offset.reset(),
                loginID: loginID.pin(),
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

        this.detail = new DetailAction()

        this.loginID = loginID.input
        this.offset = offset.input
        this.columns = columns
        this.observe = observe

        this.filter = initialFilter

        this.loginID.observe.subscriber.subscribe((result) =>
            checker.update("loginID", result.hasChanged),
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
        this.loginID.clear()
        return this.currentState()
    }
    async search(): Promise<SearchAuthUserAccountState> {
        return search(this.material, this.setFilterOnSearch(), (e) => this.searchResult(e))
    }
    async load(): Promise<SearchAuthUserAccountState> {
        return search(this.material, this.setFilterOnLoad(), (e) => this.searchResult(e))
    }
    async sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState> {
        return search(this.material, this.setFilterOnSort(key), (e) => this.searchResult(e))
    }

    searchResult(e: SearchAuthUserAccountEvent): SearchAuthUserAccountState {
        const previousInfo = {
            previousResponse: this.response,
        }
        switch (e.type) {
            case "succeed-to-search":
                this.setFilter({ sort: e.response.sort })
                this.response = e.response
                break
        }
        return this.post({
            ...e,
            ...previousInfo,
        })
    }

    async focus(_loginID: string): Promise<SearchAuthUserAccountState> {
        // TODO 指定された login id に focus する
        return this.currentState()
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

class DetailAction
    extends AbstractStatefulApplicationAction<DetailAuthUserAccountState>
    implements DetailAuthUserAccountAction
{
    readonly initialState = initialDetailState

    focus(user: AuthUserAccountBasket): DetailAuthUserAccountState {
        return this.post({ type: "focus-on", user })
    }
    close(): DetailAuthUserAccountState {
        return this.post({ type: "initial-detail" })
    }

    isFocused(user: AuthUserAccountBasket): boolean {
        const state = this.currentState()
        switch (state.type) {
            case "initial-detail":
                return false

            case "focus-on":
                return user.loginID === state.user.loginID
        }
    }
}

interface Post<E, S> {
    (event: E): S
}
