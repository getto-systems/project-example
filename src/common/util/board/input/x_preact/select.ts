import { html } from "htm/preact"
import { PreactContent, PreactKey, PreactNode } from "../../../../x_preact/vnode"

import { useLayoutEffect, useMemo, useState } from "preact/hooks"

import { readBoardValue } from "../../kernel/x_dom/input"

import { InputBoardAction } from "../action"

import { SingleBoardStore, BoardStoreConnector } from "../infra"
import { initSingleBoardStore } from "../detail/store"

export type SelectBoardContent = Readonly<{
    key: PreactKey
    value: string
    label: PreactContent
}>

type Props = Readonly<{
    input: InputBoardAction<SingleBoardStore>
    options: readonly SelectBoardContent[]
}>
export function SelectBoard({ input, options }: Props): PreactNode {
    const [current, store] = useSelectStore(input.connector)

    return html`<select onInput=${onInput}>
        ${content()}
    </select>`

    function content(): PreactNode[] {
        return options.map(({ key, value, label }) => {
            return html`<option key=${key} value=${value} selected=${value === current}>
                ${label}
            </option>`
        })
    }

    function onInput(e: Event) {
        const target = e.target
        if (target instanceof HTMLSelectElement) {
            store.set(readBoardValue(target))
        }
        input.onInput()
    }
}

interface SelectStore {
    set(value: string): void
}

function useSelectStore(connector: BoardStoreConnector<SingleBoardStore>): [string, SelectStore] {
    const [current, setValue] = useState("")
    const { store, connect } = useMemo(() => initSingleBoardStore(), [])

    useLayoutEffect(() => {
        connect(setValue)
        connector.connect(store)
        return () => connector.terminate()
    }, [connector, store, connect])

    return [current, store]
}
