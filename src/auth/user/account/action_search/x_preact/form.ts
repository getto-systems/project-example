import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_search,
    button_undo,
} from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { box_grow } from "../../../../../../ui/vendor/getto-css/preact/design/box"

import { spinner } from "../../../../../example/x_preact/design/icon"

import { SearchLoginIDComponent } from "../../../login_id/input/action_search/x_preact/search"

import { SearchUserAccountResource, SearchUserAccountFormResourceState } from "../resource"

export function SearchUserAccountFormEntry({ search }: SearchUserAccountResource): VNode {
    return h(SearchUserAccountFormComponent, {
        search,
        state: useApplicationAction(search),
        observe: useApplicationAction(search.observe),
    })
}

type Props = SearchUserAccountResource & SearchUserAccountFormResourceState
export function SearchUserAccountFormComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, observe }: SearchUserAccountFormResourceState): VNode {
        if (observe.hasChanged) {
            return searchForm({ type: "has-changed" })
        }

        switch (state.type) {
            case "initial-search":
            case "succeed-to-search":
                return searchForm({ type: "initial" })

            case "try-to-search":
            case "take-longtime-to-search":
                return searchForm({ type: "connecting" })

            case "failed-to-search":
                return searchForm({ type: "failed" })
        }
    }

    type Content =
        | Readonly<{ type: "has-changed" }>
        | Readonly<{ type: "initial" }>
        | Readonly<{ type: "connecting" }>
        | Readonly<{ type: "failed" }>

    function searchForm(content: Content): VNode {
        return box_grow({
            body: [
                h(SearchLoginIDComponent, { field: props.search.loginID }),
                // TODO granted role (checkbox)
            ],
            footer: buttons({ left: button(), right: clearButton() }),
            form: true,
        })

        function clearButton(): VNode {
            const label = "検索項目をクリア"

            return button_undo({ label, onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.search.clear()
            }
        }

        function button(): VNode {
            const label = "検索"

            switch (content.type) {
                case "has-changed":
                    return button_search({ state: "confirm", label, onClick })

                case "initial":
                    return button_search({ state: "normal", label, onClick })

                case "connecting":
                    return button_search({ state: "connect", label: html`検索中 ${spinner}` })

                case "failed":
                    return button_search({ state: "confirm", label, onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.search.submit()
            }
        }
    }
}
