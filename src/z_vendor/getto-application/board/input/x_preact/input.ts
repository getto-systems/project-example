import { VNode } from "preact"
import { useLayoutEffect, useRef } from "preact/hooks"
import { html } from "htm/preact"

import { readBoardValue } from "../../kernel/convert"

import { InputBoardAction } from "../action"

import { BoardValueStore, BoardValueStoreConnector } from "../infra"

export const inputTypes = [
    "text",
    "password",
    "search",
    "number",
    "tel",
    "email",
    "date",
    "time",
] as const
export type InputType = typeof inputTypes[number]

type Props = Readonly<{
    type: InputType
    input: InputBoardAction<BoardValueStore>
}> &
    Partial<{
        autocomplete: string
    }>
export function InputBoard({ type, input, autocomplete }: Props): VNode {
    return html`<input
        ref=${useInputRef(input.connector)}
        type=${type}
        onInput=${onInput}
        autocomplete=${autocomplete}
    />`

    function onInput() {
        input.publisher.post()
    }
}

function useInputRef(connector: BoardValueStoreConnector<BoardValueStore>) {
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
