import { SearchSidebarState } from "../action"

export function sidebarExpand(state: SearchSidebarState): boolean {
    switch (state.type) {
        case "initial-sidebar":
        case "repository-error":
            return true

        case "succeed-to-load":
        case "succeed-to-save":
            return state.state.isExpand
    }
}

export type ScrollToFocusedProps = Readonly<{
    sidebarId: string
    focusedId: string
    isFirstTime: boolean
}>
export function scrollToFocused({ sidebarId, focusedId, isFirstTime }: ScrollToFocusedProps) {
    const sidebar = document.getElementById(sidebarId)
    const focused = document.getElementById(focusedId)
    if (sidebar && focused) {
        const props: ScrollDiffProps = {
            sidebar: sidebar.getBoundingClientRect(),
            focused: focused.getBoundingClientRect(),
        }
        const threshold = 0.175

        if (isFirstTime) {
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
