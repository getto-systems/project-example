import { VNode } from "preact"
import { useLayoutEffect, useRef } from "preact/hooks"
import { html } from "htm/preact"

import { readBoardValue } from "../../kernel/convert"

import { InputBoardAction } from "../action"

import { BoardValueStoreConnector } from "../../input/infra"

import { emptyBoardValue } from "../../kernel/data"

type Props = Readonly<{
    input: InputBoardAction
    options: VNode[]
}>
export function SelectBoardComponent({ input, options }: Props): VNode {
    return html`<select ref=${useInputRef(input.connector)} onInput=${onInput}>
        ${options}
    </select>`

    function onInput() {
        input.publisher.post()
    }
}

function useInputRef(connector: BoardValueStoreConnector) {
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

    return REF
}
