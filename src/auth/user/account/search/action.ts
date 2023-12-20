import { checkTakeLongtime } from "../../../../common/util/timer/helper"
import { nextSort } from "../../../../common/util/search/sort/helper"

import { ALL_AUTH_PERMISSIONS } from "../../../../x_content/permission"

import { Atom, initAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import { LoadState, loadState_loaded, loadState_loading } from "../../../../common/util/load/data"
import {
    FocusModifyListAction,
    FocusModifyListInfra,
    initFocusModifyListAction,
    initLoadableListAtomUpdater,
    LoadableListAtomUpdater,
} from "../../../../common/util/list/action"
import { ObserveBoardState } from "../../../../common/util/board/observe/action"
import {
    composeSearchFilterBoard,
    initMultipleFilterBoard,
    initOffsetFilterBoard,
    initTextFilterBoard,
    MultipleFilterBoard,
    OffsetFilterBoard,
    TextFilterBoard,
} from "../../../../common/util/board/filter/action"

import {
    SearchAuthUserAccountFilterDetecter,
    SearchAuthUserAccountRemote,
    UpdateSearchAuthUserAccountFieldsQuery,
} from "./infra"
import { WaitTime } from "../../../../common/util/config/infra"

import { RemoteCommonError } from "../../../../common/util/remote/data"
import {
    SearchAuthUserAccountFilter,
    SearchAuthUserAccountFilterData,
    SearchAuthUserAccountRemoteResponse,
    SearchAuthUserAccountSort,
    SearchAuthUserAccountSortKey,
} from "./data"
import { AuthUserAccount } from "../kernel/data"
import { SearchPageResponseResult } from "../../../../common/util/search/kernel/data"
import { AuthPermission } from "../../kernel/data"
import { ConnectState } from "../../../../common/util/connect/data"

export interface SearchAuthUserAccountAction {
    readonly focus: FocusModifyListAction<AuthUserAccount>

    readonly state: Atom<SearchAuthUserAccountState>
    readonly list: Atom<LoadState<readonly AuthUserAccount[]>>
    readonly connect: Atom<ConnectState>
    readonly page: Atom<LoadState<SearchPageResponseResult<RemoteCommonError>>>
    readonly sortKey: Atom<SearchAuthUserAccountSort>
    readonly observe: Atom<ObserveBoardState>

    readonly offset: OffsetFilterBoard
    readonly loginId: TextFilterBoard
    readonly granted: MultipleFilterBoard<AuthPermission, AuthPermission>

    reset(): void

    search(): Promise<SearchAuthUserAccountState>
    load(): Promise<SearchAuthUserAccountState>
    sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState>
}

export type SearchAuthUserAccountState = Readonly<{ type: "initial" }> | SearchAuthUserAccountEvent

const initialState: SearchAuthUserAccountState = { type: "initial" }

export type SearchAuthUserAccountMaterial = Readonly<{
    infra: SearchAuthUserAccountInfra
    shell: SearchAuthUserAccountShell
    config: SearchAuthUserAccountConfig
}>

export type SearchAuthUserAccountInfra = Readonly<{
    searchRemote: SearchAuthUserAccountRemote
}>

export type SearchAuthUserAccountShell = Readonly<{
    detectFilter: SearchAuthUserAccountFilterDetecter
    updateQuery: UpdateSearchAuthUserAccountFieldsQuery
    focus: FocusModifyListInfra
}>

export type SearchAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
}>

export function initSearchAuthUserAccountAction(
    material: SearchAuthUserAccountMaterial,
): [SearchAuthUserAccountAction, LoadableListAtomUpdater<AuthUserAccount>] {
    const search = initAtom({ initialState, ignite: searchWithCurrentState })
    function searchWithCurrentState(): Promise<SearchAuthUserAccountState> {
        return searchAuthUserAccount(
            material,
            {
                offset: offset[0].value.currentState(),
                sort: sortKey.state.currentState(),
                filter: currentFilter(),
            },
            search.post,
        )
    }

    const initialSearch = material.shell.detectFilter()
    const sortKey = initAtom({ initialState: initialSearch.sort })

    const offset = initOffsetFilterBoard(initialSearch.offset)

    const grantedOptions = initAtom<LoadState<readonly AuthPermission[]>>({
        initialState: loadState_loaded(ALL_AUTH_PERMISSIONS),
    })

    const loginId = initTextFilterBoard(initialSearch.filter.loginId)
    const granted = initMultipleFilterBoard({
        initial: initialSearch.filter.granted,
        options: grantedOptions.state,
        toFilter: (option) => option,
        toValue: (option) => option,
    })

    const currentFilter = (): SearchAuthUserAccountFilter => ({
        loginId: loginId[0].filter.currentState(),
        granted: granted[0].filter.currentState(),
    })

    const { observe, reset, pin } = composeSearchFilterBoard(offset[0], [loginId, granted])

    const list = initAtom<LoadState<readonly AuthUserAccount[]>>({
        initialState: loadState_loading(),
    })

    search.state.subscribe((state) => {
        switch (state.type) {
            case "success":
                sortKey.post(state.response.sort)
                offset[1].init(`${state.response.page.offset}`)
                list.post(loadState_loaded(state.response.list))
                break
        }
    })

    const page = mapAtom(
        search.state,
        (state): LoadState<SearchPageResponseResult<RemoteCommonError>> => {
            switch (state.type) {
                case "initial":
                case "try":
                    return { isLoad: false }

                case "success":
                    return { isLoad: true, data: { isSuccess: true, page: state.response.page } }

                case "failed":
                    return { isLoad: true, data: { isSuccess: false, err: state.err } }
            }
        },
    )

    const connect = mapAtom(search.state, (state): ConnectState => {
        if (state.type === "try") {
            return { isConnecting: true, hasTakenLongtime: state.hasTakenLongtime }
        } else {
            return { isConnecting: false }
        }
    })

    const focus = initFocusModifyListAction(
        list.state,
        (entry) => `${entry.loginId}`,
        material.shell.focus,
    )

    return [
        {
            focus,

            state: search.state,
            list: list.state,
            connect,
            page,
            sortKey: sortKey.state,
            observe,

            offset: offset[0],
            loginId: loginId[0],
            granted: granted[0],

            reset,

            async search(): Promise<SearchAuthUserAccountState> {
                pin()
                return searchWithCurrentState()
            },
            async load(): Promise<SearchAuthUserAccountState> {
                offset[1].pin()
                return searchWithCurrentState()
            },
            async sort(key: SearchAuthUserAccountSortKey): Promise<SearchAuthUserAccountState> {
                sortKey.post(nextSort(sortKey.state.currentState(), key))
                return searchWithCurrentState()
            },
        },
        initLoadableListAtomUpdater(list),
    ]
}

type SearchAuthUserAccountEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: RemoteCommonError }>
    | Readonly<{ type: "success"; response: SearchAuthUserAccountRemoteResponse }>

async function searchAuthUserAccount<S>(
    { infra, shell, config }: SearchAuthUserAccountMaterial,
    fields: SearchAuthUserAccountFilterData,
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
