import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/reason"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    button_search,
    field,
    fieldError,
    pager,
} from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { box } from "../../../../../../ui/vendor/getto-css/preact/design/box"

import { pagerCount, pagerParams } from "../../../../../example/x_preact/design/table"

import { SearchUserAccountPagerResourceState, SearchUserAccountResource } from "../resource"

import { SearchUserAccountError } from "../../search/data"
import { pagerOptions } from "../../../../../../ui/vendor/getto-css/preact/design/data"
import { SearchPage } from "../../../../../z_lib/ui/remote/data"

export function SearchUserAccountPagerEntry({ search }: SearchUserAccountResource): VNode {
    return h(SearchUserAccountPagerComponent, {
        search,
        state: useApplicationAction(search),
    })
}

type Props = SearchUserAccountResource & SearchUserAccountPagerResourceState
export function SearchUserAccountPagerComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state }: SearchUserAccountPagerResourceState): VNode {
        switch (state.type) {
            case "initial-search":
                return EMPTY_CONTENT

            case "succeed-to-search":
                return pagerForm({ page: state.response.page })

            case "try-to-search":
                return connectingMessage({ hasTakeLongtime: false })

            case "take-longtime-to-search":
                return connectingMessage({ hasTakeLongtime: true })

            case "failed-to-search":
                return errorMessage({ err: state.err })
        }
    }

    type Content = Readonly<{ page: SearchPage }>

    function pagerForm({ page }: Content): VNode {
        return box({
            body: [field({ title: pagerCount(page.all), body: [pagerSelect(), pagerButton()] })],
            form: true,
        })

        function pagerSelect() {
            return pager(html`<select value=${page.offset}>
                ${options()}
            </select>`)

            function options() {
                return pagerOptions(pagerParams(page))
            }
        }
        function pagerButton() {
            return button_search({ state: "normal", label: "移動", onClick: () => null })
        }
    }

    type ConnectingContent = Readonly<{ hasTakeLongtime: boolean }>
    function connectingMessage({ hasTakeLongtime }: ConnectingContent): VNode {
        if (!hasTakeLongtime) {
            return EMPTY_CONTENT
        }
        return box({
            body: fieldError([
                "検索中です",
                "30秒以上かかる場合は何かがおかしいので、",
                "お手数ですが管理者に連絡お願いします",
            ]),
        })
    }

    type ErrorContent = Readonly<{ err: SearchUserAccountError }>
    function errorMessage({ err }: ErrorContent): VNode {
        return box({ body: fieldError(searchError(err)) })
    }
}

const EMPTY_CONTENT = box({ body: "" })

function searchError(err: SearchUserAccountError) {
    return remoteCommonErrorReason(err, (reason) => [
        `${reason.message}により検索に失敗しました`,
        ...reason.detail,
    ])
}
