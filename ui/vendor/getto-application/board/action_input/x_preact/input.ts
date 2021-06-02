import { VNode } from "preact"
import { useLayoutEffect, useRef } from "preact/hooks"
import { html } from "htm/preact"

import { InputBoardValueAction } from "../core/action"

import { BoardValue, emptyBoardValue } from "../../kernel/data"
import { InputBoardValueResource } from "../action"
import { readBoardValue } from "../../kernel/converter"

type Props = InputBoardValueResource
export function InputBoardComponent(props: Props): VNode {
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
