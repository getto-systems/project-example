import {
    ApplicationStateAction,
    initApplicationStateAction,
    StatefulApplicationAction,
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
} from "./data"
import { SearchSortOrder } from "../../../../z_lib/ui/search/sort/data"
import { AuthUserAccount } from "../kernel/data"
import { LoginId } from "../../login_id/kernel/data"

export interface SearchAuthUserAccountAction extends ListAuthUserAccountAction {
    readonly loginId: TextFilterAction
    readonly grantedRoles: AuthUserGrantedRolesFilterAction
    readonly observe: ObserveBoardAction

    clear(): void
    search(): Promise<SearchAuthUserAccountState>
}
export interface ListAuthUserAccountAction
    extends StatefulApplicationAction<SearchAuthUserAccountState> {
    readonly focused: FocusedAuthUserAccountAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction

    currentSort(): SearchAuthUserAccountSort

    load(): Promise<SearchAuthUserAccountState>
    sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState>

    searchResponse(
        state: SearchAuthUserAccountState,
    ): Readonly<{ response?: SearchAuthUserAccountRemoteResponse }>
}
export interface FocusedAuthUserAccountAction
    extends StatefulApplicationAction<FocusedAuthUserAccountState> {
    focus(user: AuthUserAccount): FocusedAuthUserAccountState
    update(loginId: LoginId, user: AuthUserAccount): FocusedAuthUserAccountState
    remove(loginId: LoginId): FocusedAuthUserAccountState
    close(): FocusedAuthUserAccountState

    isFocused(user: AuthUserAccount): boolean
}

export type SearchAuthUserAccountState = Readonly<{ type: "initial" }> | SearchAuthUserAccountEvent

const initialSearchState: SearchAuthUserAccountState = { type: "initial" }

export type FocusedAuthUserAccountState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "focus-failed" }>
    | Readonly<{ type: "focus-detected"; user: AuthUserAccount }>
    | Readonly<{ type: "focus-on"; user: AuthUserAccount }>

const initialFocusedState: FocusedAuthUserAccountState = { type: "initial" }

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
    readonly state: ApplicationStateAction<SearchAuthUserAccountState>
    readonly post: (state: SearchAuthUserAccountState) => SearchAuthUserAccountState

    readonly focused: FocusedAuthUserAccountAction

    readonly loginId: TextFilterAction
    readonly grantedRoles: AuthUserGrantedRolesFilterAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction
    readonly observe: ObserveBoardAction

    filter: SearchFilter<SearchAuthUserAccountSortKey, SearchAuthUserAccountFilterProps>
    clear: () => void

    response?: SearchAuthUserAccountRemoteResponse

    constructor(material: SearchAuthUserAccountMaterial) {
        const { state, post } = initApplicationStateAction({
            initialState: initialSearchState,
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

        this.material = material
        this.filter = filter
        this.clear = clear

        this.loginId = loginId.input
        this.grantedRoles = grantedRoles.input
        this.offset = offset
        this.columns = columns
        this.observe = observe

        this.focused = new FocusedAction({
            infra: {
                detectUser: async (loginId): Promise<DetectUserResult> => {
                    const result = this.searchResponse(await this.state.ignitionState)
                    if (!result.response) {
                        return { found: false }
                    }
                    const user = result.response.users.find((user) => user.loginId === loginId)
                    if (user === undefined) {
                        return { found: false }
                    }
                    return { found: true, user }
                },
                updateUser: (loginId, user) => {
                    this.update(loginId, user)
                },
                removeUser: (loginId) => {
                    this.remove(loginId)
                },
            },
            shell: material.shell,
        })
    }

    currentSort(): SearchAuthUserAccountSort {
        return this.filter.get().sort
    }

    async search(): Promise<SearchAuthUserAccountState> {
        return this.updateResponse(
            await search(this.material, this.filter.search(), this.response, this.post),
        )
    }
    async load(): Promise<SearchAuthUserAccountState> {
        return this.updateResponse(
            await search(this.material, this.filter.load(), this.response, this.post),
        )
    }
    async sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState> {
        return this.updateResponse(
            await search(this.material, this.filter.sort(key), this.response, this.post),
        )
    }
    updateResponse(state: SearchAuthUserAccountState): SearchAuthUserAccountState {
        switch (state.type) {
            case "success":
                this.filter.setSort(state.response.sort)
                this.response = state.response
                break
        }
        return state
    }

    update(loginId: LoginId, user: AuthUserAccount): SearchAuthUserAccountState {
        if (!this.response) {
            return this.state.currentState()
        }

        this.response = {
            ...this.response,
            users: this.response.users.map((row) => {
                if (row.loginId !== loginId) {
                    return row
                }
                return user
            }),
        }

        const state = this.state.currentState()
        switch (state.type) {
            case "initial":
                return state

            case "try":
            case "failed":
                return this.post({ ...state, previousResponse: this.response })

            case "success":
                return this.post({ ...state, response: this.response })
        }
    }
    remove(loginId: LoginId): SearchAuthUserAccountState {
        if (!this.response) {
            return this.state.currentState()
        }

        this.response = {
            ...this.response,
            page: {
                ...this.response.page,
                all: this.response.page.all - 1,
            },
            users: this.response.users.filter((row) => row.loginId !== loginId),
        }

        const state = this.state.currentState()
        switch (state.type) {
            case "initial":
                return state

            case "try":
            case "failed":
                return this.post({ ...state, previousResponse: this.response })

            case "success":
                return this.post({ ...state, response: this.response })
        }
    }

    searchResponse(state: SearchAuthUserAccountState): Readonly<{
        response?:
            | Readonly<{
                  page: Readonly<{ offset: number; limit: number; all: number }>
                  sort: Readonly<{ key: "loginId"; order: SearchSortOrder }>
                  users: readonly AuthUserAccount[]
              }>
            | undefined
    }> {
        switch (state.type) {
            case "initial":
                return { response: undefined }

            case "try":
            case "failed":
                return { response: state.previousResponse }

            case "success":
                return { response: state.response }
        }
    }
}

type SearchAuthUserAccountEvent =
    | Readonly<{
          type: "try"
          hasTakenLongtime: boolean
          previousResponse?: SearchAuthUserAccountRemoteResponse
      }>
    | Readonly<{
          type: "failed"
          err: RemoteCommonError
          previousResponse?: SearchAuthUserAccountRemoteResponse
      }>
    | Readonly<{ type: "success"; response: SearchAuthUserAccountRemoteResponse }>

async function search<S>(
    { infra, shell, config }: SearchAuthUserAccountMaterial,
    fields: SearchAuthUserAccountFilter,
    previousResponse: SearchAuthUserAccountRemoteResponse | undefined,
    post: Post<SearchAuthUserAccountEvent, S>,
): Promise<S> {
    shell.updateQuery(fields)
    post({ type: "try", hasTakenLongtime: false, previousResponse })

    const { searchRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        searchRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true, previousResponse }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err, previousResponse })
    }

    return post({ type: "success", response: response.value })
}

type FocusedMaterial = Readonly<{
    infra: FocusedInfra
    shell: FocusedShell
}>

interface FocusedInfra {
    detectUser(loginId: string): Promise<DetectUserResult>
    updateUser(loginId: LoginId, user: AuthUserAccount): void
    removeUser(loginId: LoginId): void
}
type DetectUserResult =
    | Readonly<{ found: false }>
    | Readonly<{ found: true; user: AuthUserAccount }>

type FocusedShell = Readonly<{
    detectFocus: FocusAuthUserAccountDetecter
    updateFocus: UpdateFocusAuthUserAccountQuery
}>

class FocusedAction implements FocusedAuthUserAccountAction {
    readonly material: FocusedMaterial
    readonly state: ApplicationStateAction<FocusedAuthUserAccountState>
    readonly post: (state: FocusedAuthUserAccountState) => FocusedAuthUserAccountState

    constructor(material: FocusedMaterial) {
        const { state, post } = initApplicationStateAction({
            initialState: initialFocusedState,
            ignite: () => this.load(),
        })
        this.material = material
        this.state = state
        this.post = post
    }
    async load(): Promise<FocusedAuthUserAccountState> {
        const focus = this.material.shell.detectFocus()
        if (!focus.found) {
            return this.state.currentState()
        }
        const user = await this.material.infra.detectUser(focus.loginId)
        if (!user.found) {
            return this.post({ type: "focus-failed" })
        }
        return this.post({ type: "focus-detected", user: user.user })
    }

    focus(user: AuthUserAccount): FocusedAuthUserAccountState {
        this.material.shell.updateFocus.focus(user)
        return this.post({ type: "focus-on", user })
    }
    update(loginId: LoginId, user: AuthUserAccount): FocusedAuthUserAccountState {
        this.material.infra.updateUser(loginId, user)
        this.material.shell.updateFocus.focus(user)
        return this.post({ type: "focus-on", user })
    }
    remove(loginId: LoginId): FocusedAuthUserAccountState {
        this.material.infra.removeUser(loginId)
        return this.close()
    }
    close(): FocusedAuthUserAccountState {
        this.material.shell.updateFocus.clear()
        return this.post({ type: "initial" })
    }

    isFocused(user: AuthUserAccount): boolean {
        const state = this.state.currentState()
        switch (state.type) {
            case "initial":
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
