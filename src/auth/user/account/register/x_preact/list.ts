import { h, VNode } from "preact"
import { useEffect } from "preact/hooks"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"
import { useRegisteredAuthUserAccountTableStructure } from "./structure"
import { scrollToFocused } from "../../../../../z_lib/ui/search/sidebar/x_preact/helper"

import { ListRegisteredAuthUserAccountTable } from "./table"

import {
    FocusedRegisteredAuthUserAccountAction,
    ListRegisteredAuthUserAccountAction,
} from "../action"

type Props = Readonly<{
    list: ListRegisteredAuthUserAccountAction
}>
export function ListRegisteredAuthUserAccount(resource: Props): VNode {
    const structure = useRegisteredAuthUserAccountTableStructure(resource.list)
    useScrollToFocused(resource.list.focused)

    return h(ListRegisteredAuthUserAccountTable, { structure, ...resource })
}

function useScrollToFocused(detail: FocusedRegisteredAuthUserAccountAction): void {
    const state = useApplicationAction(detail)
    useEffect(() => {
        scrollToFocused({
            sidebarId: "sidebar",
            focusedId: "focused",
            isFirstTime: false,
        })
    }, [state])
}
