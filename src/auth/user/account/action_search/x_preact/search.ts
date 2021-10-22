import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/reason"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_search,
    button_undo,
    fieldError,
    form,
} from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { box_grow, container } from "../../../../../../ui/vendor/getto-css/preact/design/box"

import { spinner } from "../../../../../example/x_preact/design/icon"

import { SearchLoginIDComponent } from "../../../login_id/input/action_search/x_preact/search"

import { SearchUserAccountResource, SearchUserAccountResourceState } from "../resource"

import { SearchUserAccountError } from "../../search/data"

export function SearchUserAccountEntry({ search }: SearchUserAccountResource): VNode {
    return container([
        h(SearchUserAccountComponent, {
            search,
            state: useApplicationAction(search),
            observe: useApplicationAction(search.observe),
        }),
    ])
}

type Props = SearchUserAccountResource & SearchUserAccountResourceState
export function SearchUserAccountComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, observe }: SearchUserAccountResourceState): VNode {
        if (observe.hasChanged) {
            return searchForm({ type: "has-changed" })
        }

        switch (state.type) {
            case "initial-search":
            case "succeed-to-search":
                return searchForm({ type: "initial" })

            case "try-to-search":
                return searchForm({ type: "connecting", hasTakeLongtime: false })

            case "take-longtime-to-search":
                return searchForm({ type: "connecting", hasTakeLongtime: true })

            case "failed-to-search":
                return searchForm({ type: "failed", err: state.err })
        }
    }

    type Content =
        | Readonly<{ type: "has-changed" }>
        | Readonly<{ type: "initial" }>
        | Readonly<{ type: "connecting"; hasTakeLongtime: boolean }>
        | Readonly<{ type: "failed"; err: SearchUserAccountError }>

    function searchForm(content: Content): VNode {
        return form(
            box_grow({
                body: [
                    h(SearchLoginIDComponent, { field: props.search.loginID }),
                    // TODO granted role (checkbox)
                ],
                footer: buttons({ left: button(), right: clearButton() }),
            }),
        )

        function clearButton(): VNode {
            const label = "検索項目をクリア"

            if (content.type === "has-changed") {
                return button_undo({ label, onClick })
            } else {
                return button_disabled({ label })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.search.clear()
            }
        }

        function button(): VNode[] {
            const label = "検索"

            switch (content.type) {
                case "has-changed":
                    return [button_search({ state: "confirm", label, onClick })]

                case "initial":
                    return [button_search({ state: "normal", label, onClick })]

                case "connecting":
                    return [
                        button_search({ state: "connect", label: html`検索中 ${spinner}` }),
                        ...takeLongtimeMessage(content.hasTakeLongtime),
                    ]

                case "failed":
                    return [
                        button_search({ state: "confirm", label, onClick }),
                        error(content.err),
                    ]
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.search.submit()
            }

            function takeLongtimeMessage(hasTakeLongtime: boolean): VNode[] {
                if (!hasTakeLongtime) {
                    return []
                }
                return [
                    html`<p>検索中です</p>`,
                    html`<p>
                        30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします
                    </p>`,
                ]
            }
        }

        function error(err: SearchUserAccountError): VNode {
            return fieldError(searchError(err))
        }
    }
}

function searchError(err: SearchUserAccountError) {
    return remoteCommonErrorReason(err, (reason) => [
        `${reason.message}により検索に失敗しました`,
        ...reason.detail,
    ])
}
