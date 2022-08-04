import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { pagerCount, pagerParams } from "../../../../../common/x_preact/design/table"

import { SearchOffset } from "../../../../../z_lib/ui/search/offset/x_preact/offset"
import { LoadButton } from "../../../../../common/x_preact/button/load_button"

import { SearchAuthUserAccountAction } from "../action"

import { pagerOptions } from "../../../../../z_vendor/getto-css/preact/design/table"
import { SearchPageResponse } from "../../../../../z_lib/ui/search/kernel/data"
import { RemoteCommonError } from "../../../../../z_lib/ui/remote/data"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function SearchAuthUserAccountPager(props: Props): VNode {
    const state = useApplicationAction(props.search.list)

    if (!state.isLoad) {
        return html``
    }

    switch (state.data.type) {
        case "success":
            return pagerForm({ page: state.data.response.page })

        case "failed":
            return fieldHelp_error(searchError(state.data.err))
    }

    type Content = Readonly<{
        page: SearchPageResponse
    }>
    function pagerForm({ page }: Content): VNode {
        return h(SearchOffset, {
            field: props.search.offset,
            count: pagerCount(page.all),
            options: pagerOptions(pagerParams(page)),
            button: button(),
        })

        function button(): VNode {
            return h(LoadButton, { onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.search.load()
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
