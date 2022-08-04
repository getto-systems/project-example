import { h, VNode } from "preact"
import { useEffect } from "preact/hooks"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"
import { useRegisteredAuthUserAccountTableStructure } from "./structure"
import { scrollToFocused } from "../../../../../z_lib/ui/search/sidebar/x_preact/helper"

import { ListRegisteredAuthUserAccountTable } from "./table"

import { RegisterAuthUserAccountAction } from "../action"

type Props = Readonly<{
    register: RegisterAuthUserAccountAction
}>
export function ListRegisteredAuthUserAccount(props: Props): VNode {
    const structure = useRegisteredAuthUserAccountTableStructure(props.register.list)
    useScrollToFocused(props.register)

    return h(ListRegisteredAuthUserAccountTable, { structure, list: props.register.list })
}

function useScrollToFocused(register: RegisterAuthUserAccountAction): void {
    const state = useApplicationAction(register.list.focus)
    useEffect(() => {
        scrollToFocused({
            sidebarId: "sidebar",
            focusedId: "focused",
            isFirstTime: false,
        })
    }, [state])
}
