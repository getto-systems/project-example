import { h, VNode } from "preact"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { button_search, fieldError } from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { box } from "../../../../../../ui/vendor/getto-css/preact/design/box"
import { pagerCount, pagerParams } from "../../../../../example/x_preact/design/table"

import { SearchOffsetComponent } from "../../../../../z_lib/ui/search/offset/x_preact/offset"

import { SearchAuthUserAccountAction, SearchAuthUserAccountState } from "../action"

import { pagerOptions } from "../../../../../../ui/vendor/getto-css/preact/design/data"
import { SearchPageResponse } from "../../../../../z_lib/ui/search/data"
import { RemoteCommonError } from "../../../../../z_lib/ui/remote/data"

type EntryProps = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function SearchAuthUserAccountPagerEntry({ search }: EntryProps): VNode {
    return h(SearchAuthUserAccountPagerComponent, {
        search,
        state: useApplicationAction(search),
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
            case "initial-search":
            case "try-to-search":
                return EMPTY_BOX

            case "succeed-to-search":
                return pagerForm({ page: state.response.page })

            case "take-longtime-to-search":
                return connectingMessage()

            case "failed-to-search":
                return errorMessage({ err: state.err })
        }
    }

    type Content = Readonly<{ page: SearchPageResponse }>

    function pagerForm({ page }: Content): VNode {
        return box({
            body: [
                h(SearchOffsetComponent, {
                    field: props.search.offset,
                    count: pagerCount(page.all),
                    options: pagerOptions(pagerParams(page)),
                    button: button_search({ state: "normal", label: "読み込み", onClick }),
                }),
            ],
            form: true,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.search.load()
        }
    }

    function connectingMessage(): VNode {
        return box({
            body: fieldError([
                "検索中です",
                "30秒以上かかる場合は何かがおかしいので、",
                "お手数ですが管理者に連絡お願いします",
            ]),
        })
    }

    type ErrorContent = Readonly<{ err: RemoteCommonError }>
    function errorMessage({ err }: ErrorContent): VNode {
        return box({ body: fieldError(searchError(err)) })
    }
}

const EMPTY_BOX = box({ body: "" })

function searchError(err: RemoteCommonError) {
    return remoteCommonErrorReason(err, (reason) => [
        `${reason.message}により検索に失敗しました`,
        ...reason.detail,
    ])
}
