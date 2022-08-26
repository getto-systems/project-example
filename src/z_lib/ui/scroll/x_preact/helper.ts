import { ScrollPosition } from "../data"

export function scrollToPosition(element: HTMLElement | null, position: ScrollPosition): void {
    if (element !== null) {
        element.scrollTop = position.y
    }
}
export function scrollToFocused(
    props: Readonly<{
        container: HTMLElement | null
        element: HTMLElement | null
    }>,
): void {
    if (props.container === null || props.element === null) {
        return
    }

    const threshold = 0.175

    const element = props.element.getBoundingClientRect()
    const container = props.container.getBoundingClientRect()

    scrollToPosition(props.container, getScrollPosition(threshold))

    function getScrollPosition(threshold: number): ScrollPosition {
        return { y: element.top - (container.top + container.height * threshold) }
    }
}
