import { VNode } from "preact"
import { useLayoutEffect, useMemo, useState } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent, VNodeKey } from "../../../../../common/x_preact/vnode"

import { InputBoardAction } from "../action"

import { readBoardValue } from "../../kernel/convert"

import { BoardValueStore, BoardValueStoreConnector } from "../infra"
import { initBoardValueStore } from "../init/store"

export type SelectBoardContent = Readonly<{
    key: VNodeKey
    value: string
    label: VNodeContent
}>

type Props = Readonly<{
    input: InputBoardAction<BoardValueStore>
    options: readonly SelectBoardContent[]
}>
export function SelectBoard({ input, options }: Props): VNode {
    const [current, store] = useSelectStore(input.connector)

    return html`<select onInput=${onInput}>
        ${content()}
    </select>`

    function content(): VNode[] {
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
        input.publisher.post()
    }
}

interface SelectStore {
    set(value: string): void
}

function useSelectStore(
    connector: BoardValueStoreConnector<BoardValueStore>,
): [string, SelectStore] {
    const [current, setValue] = useState("")
    const { store, connect } = useMemo(() => initBoardValueStore(), [])

    useLayoutEffect(() => {
        connect(setValue)
        connector.connect(store)
        return () => connector.terminate()
    }, [connector, store, connect])

    return [current, store]
}
