import { h, VNode } from "preact"
import { useEffect } from "preact/hooks"
import { html } from "htm/preact"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountPagerEntry } from "./pager"
import { SearchAuthUserAccountTableEntry } from "./table"

import {
    DetailAuthUserAccountAction,
    DetailAuthUserAccountState,
    ListAuthUserAccountAction,
} from "../action"

import { useAuthUserAccountTableStructure } from "./structure"
import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

type EntryProps = Readonly<{
    list: ListAuthUserAccountAction
}>
export function ListAuthUserAccountEntry(resource: EntryProps): VNode {
    const structure = useAuthUserAccountTableStructure(resource.list)
    useScrollToFocused(resource.list.detail)

    return html`
        ${container([box_grow({ body: h(SearchAuthUserAccountPagerEntry, resource) })])}
        ${h(SearchAuthUserAccountTableEntry, { structure, ...resource })}
    `
}

function useScrollToFocused(detail: DetailAuthUserAccountAction): void {
    const state = useApplicationAction(detail)
    useEffect(() => {
        scrollToFocused(state)
    }, [state])
}

function scrollToFocused(state: DetailAuthUserAccountState) {
    const sidebar = document.getElementById("sidebar")
    const focused = document.getElementById("focused")
    if (sidebar && focused) {
        const props: ScrollDiffProps = {
            sidebar: sidebar.getBoundingClientRect(),
            focused: focused.getBoundingClientRect(),
        }
        const threshold = 0.175

        if (state.type === "focus-detected") {
            sidebar.scrollTop = scrollTop({
                currentScrollTop: 0,
                scrollDiff: scrollDiff(props, threshold),
            })
            return
        }

        const scrollUnder = scrollDiff(props, threshold)
        if (scrollUnder < 0) {
            sidebar.scrollTop = scrollTop({
                currentScrollTop: sidebar.scrollTop,
                scrollDiff: scrollUnder,
            })
            return
        }

        const scrollOver = scrollDiff(props, 1 - threshold)
        if (scrollOver > 0) {
            sidebar.scrollTop = scrollTop({
                currentScrollTop: sidebar.scrollTop,
                scrollDiff: scrollOver,
            })
            return
        }
    }

    type ScrollDiffProps = Readonly<{
        sidebar: DOMRect
        focused: DOMRect
    }>
    function scrollDiff({ sidebar, focused }: ScrollDiffProps, threshold: number): number {
        return focused.top - (sidebar.top + sidebar.height * threshold)
    }
    function scrollTop({
        currentScrollTop,
        scrollDiff,
    }: Readonly<{ currentScrollTop: number; scrollDiff: number }>): number {
        const scrollTop = currentScrollTop + scrollDiff
        if (scrollTop < 0) {
            return 0
        } else {
            return scrollTop
        }
    }
}
