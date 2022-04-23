import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"
import { nextSort } from "../../../../z_lib/ui/search/sort/helper"

import { initSearchLoginIdAction, SearchLoginIdAction } from "../../login_id/input/action"
import { initSearchGrantedRolesAction, SearchGrantedRolesAction } from "../input/action"
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
import { SearchSortOrder } from "../../../../z_lib/ui/search/sort/data"
import { LoginId } from "../../login_id/kernel/data"
import { ResetTokenDestination } from "../../password/reset/token_destination/kernel/data"

export interface SearchAuthUserAccountAction extends ListAuthUserAccountAction {
    readonly loginId: SearchLoginIdAction
    readonly grantedRoles: SearchGrantedRolesAction
    readonly observe: ObserveBoardAction

    clear(): SearchAuthUserAccountState
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

    readonly focused: FocusedAuthUserAccountAction

    readonly loginId: SearchLoginIdAction
    readonly grantedRoles: SearchGrantedRolesAction
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
                this.focused.terminate()
                this.loginId.terminate()
                this.grantedRoles.terminate()
                this.offset.terminate()
                this.columns.terminate()
                this.observe.terminate()
            },
        })

        const initialFilter = material.shell.detectFilter()

        const fields = ["login-id", "granted-roles"] as const

        const loginId = initSearchLoginIdAction(initialFilter.loginId)
        const grantedRoles = initSearchGrantedRolesAction(initialFilter.grantedRoles)
        const offset = initSearchOffsetAction(initialFilter.offset)
        const columns = initSearchColumnsAction(material.infra)
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.setFilterOnSearch = () =>
            this.setFilter({
                offset: offset.reset(),
                loginId: loginId.pin(),
                grantedRoles: grantedRoles.pin(),
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

        this.focused = new FocusedAction({
            infra: {
                detectUser: async (loginId): Promise<DetectUserResult> => {
                    const result = this.searchResponse(await this.ignitionState)
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

        this.loginId = loginId.input
        this.grantedRoles = grantedRoles.input
        this.offset = offset.input
        this.columns = columns
        this.observe = observe

        this.filter = initialFilter

        this.loginId.observe.subscriber.subscribe((result) =>
            observeChecker.update("login-id", result.hasChanged),
        )
        this.grantedRoles.observe.subscriber.subscribe((result) =>
            observeChecker.update("granted-roles", result.hasChanged),
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
            case "success":
                this.setFilter({ sort: state.response.sort })
                this.response = state.response
                break
        }
        return state
    }

    update(loginId: LoginId, user: AuthUserAccount): SearchAuthUserAccountState {
        if (!this.response) {
            return this.currentState()
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

        const state = this.currentState()
        switch (state.type) {
            case "initial":
                return state

            case "try":
            case "take-longtime":
            case "failed":
                return this.post({ ...state, previousResponse: this.response })

            case "success":
                return this.post({ ...state, response: this.response })
        }
    }
    remove(loginId: LoginId): SearchAuthUserAccountState {
        if (!this.response) {
            return this.currentState()
        }

        this.response = {
            ...this.response,
            page: {
                ...this.response.page,
                all: this.response.page.all - 1,
            },
            users: this.response.users.filter((row) => row.loginId !== loginId),
        }

        const state = this.currentState()
        switch (state.type) {
            case "initial":
                return state

            case "try":
            case "take-longtime":
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
                  sort: Readonly<{ key: "login-id"; order: SearchSortOrder }>
                  users: readonly Readonly<{
                      loginId: LoginId
                      grantedRoles: readonly "user"[]
                      resetTokenDestination: ResetTokenDestination
                  }>[]
              }>
            | undefined
    }> {
        switch (state.type) {
            case "initial":
                return { response: undefined }

            case "try":
            case "take-longtime":
            case "failed":
                return { response: state.previousResponse }

            case "success":
                return { response: state.response }
        }
    }
}

type SearchAuthUserAccountEvent =
    | Readonly<{ type: "try"; previousResponse?: SearchAuthUserAccountRemoteResponse }>
    | Readonly<{
          type: "take-longtime"
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
    post({ type: "try", previousResponse })

    const { searchRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(searchRemote(fields), config.takeLongtimeThreshold, () =>
        post({ type: "take-longtime", previousResponse }),
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

class FocusedAction
    extends AbstractStatefulApplicationAction<FocusedAuthUserAccountState>
    implements FocusedAuthUserAccountAction
{
    readonly initialState = initialFocusedState

    material: FocusedMaterial

    constructor(material: FocusedMaterial) {
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
        const state = this.currentState()
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
