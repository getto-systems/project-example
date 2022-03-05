import { VNode } from "preact"
import { useEffect, useLayoutEffect, useRef } from "preact/hooks"
import { html } from "htm/preact"

import { readBoardValue } from "../../kernel/convert"

import { InputBoardAction } from "../action"

import { BoardValueStoreConnector } from "../infra"

import { BoardValue, emptyBoardValue } from "../../kernel/data"

type Props = Readonly<{
    input: InputBoardAction
    options: readonly VNode[]
}>
export function SelectBoardComponent({ input, options }: Props): VNode {
    return html`<select ref=${useSelectRef(input.connector)} onInput=${onInput}>
        ${options}
    </select>`

    function onInput() {
        input.publisher.post()
    }
}

function useSelectRef(connector: BoardValueStoreConnector) {
    const REF = useRef<HTMLSelectElement>()
    const temporaryStore = useRef<BoardValue>()

    useLayoutEffect(() => {
        connector.connect({
            get: () => {
                if (REF.current) {
                    return readBoardValue(REF.current)
                }
                return emptyBoardValue
            },
            set: (value) => {
                if (REF.current) {
                    REF.current.value = value
                } else {
                    temporaryStore.current = value
                }
            },
        })
        return () => connector.terminate()
    }, [connector])

    useEffect(() => {
        if (REF.current && temporaryStore.current !== undefined) {
            REF.current.value = temporaryStore.current
        }
    }, [])

    return REF
}
