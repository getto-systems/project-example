import { VNode } from "preact"
import { useLayoutEffect, useRef } from "preact/hooks"
import { html } from "htm/preact"

import { readBoardValue } from "../../kernel/convert"

import { InputBoardAction } from "../action"

import { BoardValueStoreConnector } from "../infra"

import { emptyBoardValue } from "../../kernel/data"

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
    input: InputBoardAction
}>
export function InputBoardComponent({ type, input }: Props): VNode {
    return html`<input ref=${useInputRef(input.connector)} type=${type} onInput=${onInput} />`

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
