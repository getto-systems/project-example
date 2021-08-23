import { VNode } from "preact"
import { useLayoutEffect, useRef } from "preact/hooks"
import { html } from "htm/preact"

import { readBoardValue } from "../../kernel/convert"

import { InputBoardAction } from "../action"

import { BoardValueStoreConnector } from "../../input/infra"

import { BoardValue, emptyBoardValue } from "../../kernel/data"

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
export function InputBoardComponent(props: Props): VNode {
    return html`<input
        ref=${useInputRef(props.input.connector)}
        type=${props.type}
        onInput=${onInput}
    />`

    function onInput() {
        props.input.publisher.post()
    }
}

function useInputRef(connector: BoardValueStoreConnector) {
    const REF = useRef<HTMLInputElement>()
    useLayoutEffect(() => {
        connector.connect(store())
        return () => connector.terminate()
    }, [connector])

    return REF

    function store() {
        return { get, set }
        function get() {
            if (!REF.current) {
                return emptyBoardValue
            }
            return readBoardValue(REF.current)
        }
        function set(value: BoardValue) {
            if (REF.current) {
                REF.current.value = value
            }
        }
    }
}
