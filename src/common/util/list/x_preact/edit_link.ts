import { html } from "htm/preact"
import { PreactNode } from "../../../x_preact/node"

import { useAtom } from "../../../../z_vendor/getto-atom/x_preact/hooks"
import { focusClass, listEditLabel } from "../../../x_preact/design/table"

import { FocusModifyListAction, FocusRegisterListAction } from "../action"
import { ScrollPosition } from "../../scroll/data"

export function EditLinkForRegisterList<T>(
    props: Readonly<{
        focus: FocusRegisterListAction<T>
        data: T
    }>,
): PreactNode {
    const detect = useAtom(props.focus.detect)

    const isFocused = props.focus.isEntryFocused(props.data, detect)

    return html`<a
        href="#"
        id="${isFocused ? "focused" : undefined}"
        class="${focusClass(isFocused)}"
        onClick=${onClick}
    >
        ${listEditLabel()}
    </a>`

    function onClick(e: Event) {
        e.preventDefault()
        if (e.target instanceof HTMLElement) {
            e.target.blur()
        }

        props.focus.focusTo(props.data)
    }
}

export function EditLinkForModifyList<T>(
    props: Readonly<{
        focus: FocusModifyListAction<T>
        data: T
    }>,
): PreactNode {
    const isFocused = useAtom(props.focus.isSomeEntryFocused)
    const detect = useAtom(props.focus.detect)

    const isFocusTarget = props.focus.isEntryFocused(props.data, detect)

    return html`<a
        href="#"
        id="${isFocusTarget ? "focused" : undefined}"
        class="${focusClass(isFocusTarget)}"
        onClick=${onClick}
    >
        ${listEditLabel()}
    </a>`

    function onClick(e: Event) {
        e.preventDefault()
        if (e.target instanceof HTMLElement) {
            e.target.blur()
        }

        props.focus.focusTo(props.data, currentScrollPosition())

        function currentScrollPosition(): ScrollPosition {
            return {
                y: isFocused
                    ? document.getElementById("sidebar")?.scrollTop || 0
                    : document.documentElement.scrollTop,
            }
        }
    }
}
