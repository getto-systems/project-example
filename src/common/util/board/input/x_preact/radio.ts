import { html } from "htm/preact"
import { PreactNode } from "../../../../x_preact/node"

import { useLayoutEffect, useMemo, useState } from "preact/hooks"

import { PreactContent, PreactKey } from "../../../../../z_vendor/getto-css/preact/common"

import { InputBoardAction } from "../action"

import { initSingleBoardStore } from "../detail/store"

import { SingleBoardStore, BoardStoreConnector } from "../infra"

import { radio, radio_block } from "../../../../../z_vendor/getto-css/preact/design/form"

export type RadioBoardContent = Readonly<{
    key: PreactKey
    value: string
    label: PreactContent
}>

type Props = Readonly<{
    input: InputBoardAction<SingleBoardStore>
    name: string
    options: readonly RadioBoardContent[]
}> &
    Partial<{
        block: boolean
        autoFocus: boolean
        onKeyDown: { (e: KeyboardEvent): void }
    }>
export function RadioBoard(props: Props): PreactNode {
    const [current, store] = useRadioStore(props.input.connector)

    return html`${content()}`

    function content(): PreactNode[] {
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
                props.input.onInput()
            }
        })
    }
}

interface RadioStore {
    set(value: string): void
}

function useRadioStore(connector: BoardStoreConnector<SingleBoardStore>): [string, RadioStore] {
    const [current, setValue] = useState("")
    const { store, connect } = useMemo(() => initSingleBoardStore(), [])

    useLayoutEffect(() => {
        connect(setValue)
        connector.connect(store)
        return () => connector.terminate()
    }, [connector, store, connect])

    return [current, store]
}
