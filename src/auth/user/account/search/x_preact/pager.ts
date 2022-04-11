import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { button_search, fieldError } from "../../../../../z_vendor/getto-css/preact/design/form"
import {
    pagerCount,
    pagerParams,
    PAGER_BUTTON_CONNECT,
    PAGER_BUTTON_STATIC,
} from "../../../../../core/x_preact/design/table"

import { SearchOffset } from "../../../../../z_lib/ui/search/offset/x_preact/offset"

import { ListAuthUserAccountAction } from "../action"

import { pagerOptions } from "../../../../../z_vendor/getto-css/preact/design/table"
import { SearchPageResponse } from "../../../../../z_lib/ui/search/kernel/data"
import { RemoteCommonError } from "../../../../../z_lib/ui/remote/data"

type Props = Readonly<{
    list: ListAuthUserAccountAction
}>
export function SearchAuthUserAccountPager(props: Props): VNode {
    const state = useApplicationAction(props.list)

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
            return fieldError(searchError(state.err))
    }

    type Content = Readonly<{ page: SearchPageResponse; isConnecting: boolean }>

    function pagerForm({ page, isConnecting }: Content): VNode {
        return h(SearchOffset, {
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
}

const EMPTY_CONTENT = html``

function searchError(err: RemoteCommonError) {
    return remoteCommonErrorReason(err, (reason) => [
        `${reason.message}により検索に失敗しました`,
        ...reason.detail,
    ])
}
