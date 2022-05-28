import { VNode } from "preact"
import { useLayoutEffect, useMemo, useState } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent, VNodeKey } from "../../../../getto-css/preact/common"

import { InputBoardAction } from "../action"

import { initBoardValueStore } from "../init/store"

import { BoardValueStore, BoardValueStoreConnector } from "../../input/infra"

import { radio, radio_block } from "../../../../getto-css/preact/design/form"

export type RadioBoardContent = Readonly<{
    key: VNodeKey
    value: string
    label: VNodeContent
}>

type Props = Readonly<{
    input: InputBoardAction<BoardValueStore>
    name: string
    options: readonly RadioBoardContent[]
}> &
    Partial<{
        block: boolean
        autoFocus: boolean
        onKeyDown: { (e: KeyboardEvent): void }
    }>
export function RadioBoard(props: Props): VNode {
    const [current, store] = useRadioStore(props.input.connector)

    return html`${content()}`

    function content(): VNode[] {
        return props.options.map(({ key, value, label }, i) => {
            const isChecked = value === current

            const input = html`<input
                    type="radio"
                    name="${props.name}"
                    checked=${isChecked}
                    onInput=${onInput}
                    onKeyDown=${props.onKeyDown}
                    autofocus=${i === 0 && props.autoFocus}
                />${label}`

            const content = { isChecked, input, key }

            if (props.block) {
                return radio_block(content)
            } else {
                return radio(content)
            }

            function onInput(e: Event) {
                const target = e.target
                if (target instanceof HTMLInputElement) {
                    if (target.checked) {
                        store.set(value)
                    }
                }
                props.input.publisher.post()
            }
        })
    }
}

interface RadioStore {
    set(value: string): void
}

function useRadioStore(connector: BoardValueStoreConnector<BoardValueStore>): [string, RadioStore] {
    const [current, setValue] = useState("")
    const { store, connect } = useMemo(() => initBoardValueStore(), [])

    useLayoutEffect(() => {
        connect(setValue)
        connector.connect(store)
        return () => connector.terminate()
    }, [connector, store, connect])

    return [current, store]
}
