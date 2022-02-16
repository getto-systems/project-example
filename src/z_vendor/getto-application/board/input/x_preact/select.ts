import { VNode } from "preact"
import { useEffect, useLayoutEffect, useRef } from "preact/hooks"
import { html } from "htm/preact"

import { readBoardValue } from "../../kernel/convert"

import { InputBoardAction } from "../action"

import { BoardValueStoreConnector } from "../infra"

import { BoardValue, emptyBoardValue } from "../../kernel/data"

type Props = Readonly<{
    input: InputBoardAction
    selected: BoardValue
    options: readonly VNode[]
}>
export function SelectBoardComponent({ input, selected, options }: Props): VNode {
    return html`<select ref=${useInputRef(input.connector, selected)} onInput=${onInput}>
        ${options}
    </select>`

    function onInput() {
        input.publisher.post()
    }
}

function useInputRef(connector: BoardValueStoreConnector, selected: BoardValue) {
    const REF = useRef<HTMLSelectElement>()

    useLayoutEffect(() => {
        connector.connect({
            get: () => {
                if (!REF.current) {
                    return emptyBoardValue
                }
                return readBoardValue(REF.current)
            },
            set: (value) => {
                if (REF.current) {
                    REF.current.value = value
                }
            },
        })
        return () => connector.terminate()
    }, [connector])

    useEffect(() => {
        if (REF.current) {
            REF.current.value = selected
        }
    }, [selected])

    return REF
}
