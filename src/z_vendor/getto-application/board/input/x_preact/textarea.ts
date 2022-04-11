import { VNode } from "preact"
import { useLayoutEffect, useRef } from "preact/hooks"
import { html } from "htm/preact"

import { readBoardValue } from "../../kernel/convert"

import { InputBoardAction } from "../action"

import { BoardValueStoreConnector } from "../../input/infra"

import { emptyBoardValue } from "../../kernel/data"

type Props = Readonly<{ input: InputBoardAction }> &
    Partial<Readonly<{ rows: number; cols: number; disabled: boolean }>>
export function TextareaBoard({ input, rows, cols, disabled }: Props): VNode {
    return html`<textarea
        ref=${useInputRef(input.connector)}
        onInput=${onInput}
        rows=${rows}
        cols=${cols}
        disabled=${disabled}
    ></textarea>`

    function onInput() {
        input.publisher.post()
    }
}

function useInputRef(connector: BoardValueStoreConnector) {
    const REF = useRef<HTMLInputElement>()

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
