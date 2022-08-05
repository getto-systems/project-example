import {
    ApplicationState,
    initApplicationState,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime } from "../../../../z_lib/ui/timer/helper"

import { initTextFilterAction, TextFilterAction } from "../../../../z_lib/ui/input/filter/text"
import { ObserveBoardAction } from "../../../../z_vendor/getto-application/board/observe_board/action"
import { SearchOffsetAction } from "../../../../z_lib/ui/search/offset/action"
import { SearchColumnsAction, SearchColumnsInfra } from "../../../../z_lib/ui/search/columns/action"
import { initSearchFilter, SearchFilter } from "../../../../z_lib/ui/search/filter/action"
import {
    AuthUserGrantedRolesFilterAction,
    initAuthUserGrantedRolesFilterAction,
} from "../input/filter/action"
import { initListSearchedAction, ListSearchedAction } from "../../../../z_lib/ui/list/action"

import { ALL_AUTH_ROLES } from "../../../../x_content/role"

import {
    FocusAuthUserAccountDetecter,
    SearchAuthUserAccountFilterDetecter,
    SearchAuthUserAccountRemote,
    UpdateFocusAuthUserAccountQuery,
    UpdateSearchAuthUserAccountFieldsQuery,
} from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import {
    SearchAuthUserAccountFilter,
    SearchAuthUserAccountFilterProps,
    SearchAuthUserAccountRemoteResponse,
    SearchAuthUserAccountSort,
    SearchAuthUserAccountSortKey,
    SearchAuthUserAccountSummary,
} from "./data"
import { AuthUserAccount } from "../kernel/data"
import { prepared, preparing } from "../../../../z_lib/ui/prepare/data"

export interface SearchAuthUserAccountAction {
    readonly state: ApplicationState<SearchAuthUserAccountState>
    readonly list: ListSearchedAuthUserAccountAction

    readonly loginId: TextFilterAction
    readonly grantedRoles: AuthUserGrantedRolesFilterAction
    readonly observe: ObserveBoardAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction

    currentSort(): SearchAuthUserAccountSort

    clear(): void
    search(): Promise<SearchAuthUserAccountState>
    load(): Promise<SearchAuthUserAccountState>
    sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState>
}

type ListSearchedAuthUserAccountAction = ListSearchedAction<
    AuthUserAccount,
    SearchAuthUserAccountSummary,
    RemoteCommonError
>

export type SearchAuthUserAccountState = Readonly<{ type: "initial" }> | SearchAuthUserAccountEvent

const initialState: SearchAuthUserAccountState = { type: "initial" }

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
    takeLongtimeThreshold: WaitTime
}>

export function initSearchAuthUserAccountAction(
    material: SearchAuthUserAccountMaterial,
): SearchAuthUserAccountAction {
    return new Action(material)
}

class Action implements SearchAuthUserAccountAction {
    readonly material: SearchAuthUserAccountMaterial
    readonly state: ApplicationState<SearchAuthUserAccountState>
    readonly post: (state: SearchAuthUserAccountState) => SearchAuthUserAccountState

    readonly list: ListSearchedAuthUserAccountAction

    readonly loginId: TextFilterAction
    readonly grantedRoles: AuthUserGrantedRolesFilterAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction
    readonly observe: ObserveBoardAction

    filter: SearchFilter<SearchAuthUserAccountSortKey, SearchAuthUserAccountFilterProps>
    clear: () => void

    constructor(material: SearchAuthUserAccountMaterial) {
        const { state, post } = initApplicationState({
            initialState,
            ignite: () => this.load(),
        })
        this.state = state
        this.post = post

        const initialFilter = material.shell.detectFilter()

        const loginId = initTextFilterAction(initialFilter.loginId)
        const grantedRoles = initAuthUserGrantedRolesFilterAction(initialFilter.grantedRoles)

        const { observe, offset, columns, filter, clear } = initSearchFilter(
            material.infra,
            initialFilter,
            [
                ["loginId", loginId.input],
                ["grantedRoles", grantedRoles.input],
            ],
            () => ({
                loginId: loginId.pin(),
                grantedRoles: grantedRoles.pin(),
            }),
        )

        grantedRoles.setOptions(ALL_AUTH_ROLES)

        const list = initListSearchedAction({
            initialSearch: this.state.ignitionState.then((state) => {
                switch (state.type) {
                    case "initial":
                    case "try":
                        return preparing()

                    case "success":
                    case "failed":
                        return prepared(state)
                }
            }),
            detect: {
                get: () => this.material.shell.detectFocus(),
                key: (data: AuthUserAccount) => data.loginId,
            },
        })

        list.action.focus.state.subscribe((state) => {
            switch (state.type) {
                case "change":
                    material.shell.updateFocus.focus(state.data)
                    break

                case "close":
                    material.shell.updateFocus.clear()
                    break
            }
        })

        this.list = list.action

        this.material = material
        this.filter = filter
        this.clear = clear

        this.loginId = loginId.input
        this.grantedRoles = grantedRoles.input
        this.offset = offset
        this.columns = columns
        this.observe = observe

        this.onSuccess((data) => {
            this.filter.setSort(data.sort)
        })
        this.onSearched((state) => {
            list.handler.load({ isLoad: true, data: state })
        })
    }

    onSuccess(handler: (response: SearchAuthUserAccountRemoteResponse) => void): void {
        this.state.subscribe((state) => {
            if (state.type === "success") {
                handler(state.response)
            }
        })
    }
    onSearched(
        handler: (state: Exclude<SearchAuthUserAccountEvent, { type: "try" }>) => void,
    ): void {
        this.state.subscribe((state) => {
            switch (state.type) {
                case "success":
                case "failed":
                    handler(state)
                    break
            }
        })
    }

    currentSort(): SearchAuthUserAccountSort {
        return this.filter.get().sort
    }

    async search(): Promise<SearchAuthUserAccountState> {
        return search(this.material, this.filter.search(), this.post)
    }
    async load(): Promise<SearchAuthUserAccountState> {
        return search(this.material, this.filter.load(), this.post)
    }
    async sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState> {
        return search(this.material, this.filter.sort(key), this.post)
    }
}

type SearchAuthUserAccountEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: RemoteCommonError }>
    | Readonly<{ type: "success"; response: SearchAuthUserAccountRemoteResponse }>

async function search<S>(
    { infra, shell, config }: SearchAuthUserAccountMaterial,
    fields: SearchAuthUserAccountFilter,
    post: Post<SearchAuthUserAccountEvent, S>,
): Promise<S> {
    shell.updateQuery(fields)
    post({ type: "try", hasTakenLongtime: false })

    const { searchRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        searchRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success", response: response.value })
}

interface Post<E, S> {
    (event: E): S
}
