import { html } from "htm/preact"
import { PreactNode } from "../../../../x_preact/vnode"

import { useInputRef } from "./hooks"

import { InputBoardAction } from "../action"

import { SingleBoardStore } from "../infra"

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
export type InputType = (typeof inputTypes)[number]

type Props = Readonly<{
    type: InputType
    input: InputBoardAction<SingleBoardStore>
}> &
    Partial<{
        autocomplete: string
    }>
export function InputBoard({ type, input, autocomplete }: Props): PreactNode {
    return html`<input
        ref=${useInputRef(input.connector)}
        type=${type}
        onInput=${onInput}
        autocomplete=${autocomplete}
    />`

    function onInput() {
        input.onInput()
    }
}
