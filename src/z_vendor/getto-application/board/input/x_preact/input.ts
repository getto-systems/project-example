import { VNode } from "preact"
import { html } from "htm/preact"

import { useInputRef } from "./hooks"

import { InputBoardAction } from "../action"

import { BoardValueStore } from "../infra"

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
