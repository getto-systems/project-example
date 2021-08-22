import { VNode } from "preact"
import { useLayoutEffect, useRef } from "preact/hooks"
import { html } from "htm/preact"

import { InputBoardValueAction } from "../core/action"

import { BoardValue, emptyBoardValue } from "../../kernel/data"
import { InputBoardAction, InputBoardValueResource } from "../action"
import { readBoardValue } from "../../kernel/convert"
import { BoardValueStoreConnector } from "../../input/infra"

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
    // TODO これの story を作る
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

type Props_legacy = InputBoardValueResource
export function InputBoardComponent_legacy(props: Props_legacy): VNode {
    return html`<input
        ref=${useBoardValueStore(props.input)}
        type=${props.type}
        onInput=${onInput}
    />`

    function onInput() {
        props.input.triggerInputEvent()
    }
}

function useBoardValueStore(input: InputBoardValueAction) {
    const REF = useRef<HTMLInputElement>()
    useLayoutEffect(() => {
        input.storeLinker.link(store())
        return () => input.storeLinker.unlink()
    }, [input.storeLinker])

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
