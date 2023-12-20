import { html } from "htm/preact"
import { PreactNode } from "../../../../x_preact/vnode"

import { useLayoutEffect, useRef } from "preact/hooks"

import { readBoardValue } from "../../kernel/x_dom/input"

import { InputBoardAction } from "../action"

import { SingleBoardStore, BoardStoreConnector } from "../infra"

type Props = Readonly<{ input: InputBoardAction<SingleBoardStore> }> &
    Partial<Readonly<{ rows: number; cols: number; disabled: boolean }>>
export function TextareaBoard({ input, rows, cols, disabled }: Props): PreactNode {
    return html`<textarea
        ref=${useInputRef(input.connector)}
        onInput=${onInput}
        rows=${rows}
        cols=${cols}
        disabled=${disabled}
    ></textarea>`

    function onInput() {
        input.onInput()
    }
}

function useInputRef(connector: BoardStoreConnector<SingleBoardStore>) {
    const REF = useRef<HTMLInputElement>()

    useLayoutEffect(() => {
        connector.connect({
            get: () => {
                if (!REF.current) {
                    return ""
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
