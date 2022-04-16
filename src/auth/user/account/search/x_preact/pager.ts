import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { fieldError } from "../../../../../z_vendor/getto-css/preact/design/form"
import { pagerCount, pagerParams } from "../../../../../core/x_preact/design/table"

import { SearchOffset } from "../../../../../z_lib/ui/search/offset/x_preact/offset"
import { LoadButton } from "../../../../../core/x_preact/button/load_button"

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
            return html``

        case "try":
        case "take-longtime":
            if (state.previousResponse) {
                return pagerForm({ page: state.previousResponse.page, isConnecting: true })
            } else {
                return html``
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
            return h(LoadButton, { isConnecting, onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.list.load()
            }
        }
    }
}

function searchError(err: RemoteCommonError) {
    return remoteCommonErrorReason(err, (reason) => [
        `${reason.message}により検索に失敗しました`,
        ...reason.detail,
    ])
}
