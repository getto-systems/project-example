import { h, VNode } from "preact"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { button_search, fieldError } from "../../../../../z_vendor/getto-css/preact/design/form"
import {
    pagerCount,
    pagerParams,
    PAGER_BUTTON_CONNECT,
    PAGER_BUTTON_STATIC,
} from "../../../../../core/x_preact/design/table"

import { SearchOffsetComponent } from "../../../../../z_lib/ui/search/offset/x_preact/offset"

import { ListAuthUserAccountAction, SearchAuthUserAccountState } from "../action"

import { pagerOptions } from "../../../../../z_vendor/getto-css/preact/design/table"
import { SearchPageResponse } from "../../../../../z_lib/ui/search/kernel/data"
import { RemoteCommonError } from "../../../../../z_lib/ui/remote/data"
import { html } from "htm/preact"

type EntryProps = Readonly<{
    list: ListAuthUserAccountAction
}>
export function SearchAuthUserAccountPagerEntry({ list }: EntryProps): VNode {
    return h(SearchAuthUserAccountPagerComponent, {
        list,
        state: useApplicationAction(list),
    })
}

type Props = EntryProps &
    Readonly<{
        state: SearchAuthUserAccountState
    }>
export function SearchAuthUserAccountPagerComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state }: Props): VNode {
        switch (state.type) {
            case "initial":
                return EMPTY_CONTENT

            case "try":
            case "take-longtime":
                if (state.previousResponse) {
                    return pagerForm({ page: state.previousResponse.page, isConnecting: true })
                } else {
                    return EMPTY_CONTENT
                }

            case "success":
                return pagerForm({ page: state.response.page, isConnecting: false })

            case "failed":
                return errorMessage({ err: state.err })
        }
    }

    type Content = Readonly<{ page: SearchPageResponse; isConnecting: boolean }>

    function pagerForm({ page, isConnecting }: Content): VNode {
        return h(SearchOffsetComponent, {
            field: props.list.offset,
            count: pagerCount(page.all),
            options: pagerOptions(pagerParams(page)),
            button: button(),
        })

        function button(): VNode {
            if (isConnecting) {
                return button_search({ state: "connect", label: PAGER_BUTTON_CONNECT })
            } else {
                return button_search({ state: "normal", label: PAGER_BUTTON_STATIC, onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.list.load()
            }
        }
    }

    type ErrorContent = Readonly<{ err: RemoteCommonError }>
    function errorMessage({ err }: ErrorContent): VNode {
        return fieldError(searchError(err))
    }
}

const EMPTY_CONTENT = html``

function searchError(err: RemoteCommonError) {
    return remoteCommonErrorReason(err, (reason) => [
        `${reason.message}により検索に失敗しました`,
        ...reason.detail,
    ])
}
