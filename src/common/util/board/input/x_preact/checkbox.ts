import { html } from "htm/preact"
import { PreactNode } from "../../../../x_preact/node"

import { useLayoutEffect, useMemo, useState } from "preact/hooks"

import { PreactContent, PreactKey } from "../../../../../z_vendor/getto-css/preact/common"
import { checkbox, checkbox_block } from "../../../../../z_vendor/getto-css/preact/design/form"

import { InputBoardAction } from "../action"

import { BoardStoreConnector, MultipleBoardStore } from "../infra"

export type CheckboxBoardContent = Readonly<{
    key: PreactKey
    value: string
    label: PreactContent
}>

type Props = Readonly<{
    input: InputBoardAction<MultipleBoardStore>
    options: readonly CheckboxBoardContent[]
}> &
    Partial<{
        block: boolean
    }>
export function CheckboxBoard(props: Props): PreactNode {
    const [current, store] = useCheckboxStore(props.input.connector)

    return html`${content()}`

    function content(): readonly PreactNode[] {
        return props.options.map(({ key, value, label }) => {
            const isChecked = current.has(value)

            const input = html`<input
                    type="checkbox"
                    checked=${isChecked}
                    onInput=${onInput}
                />${label}`

            const content = { isChecked, input, key }

            if (props.block) {
                return checkbox_block(content)
            } else {
                return checkbox(content)
            }

            function onInput(e: Event) {
                const target = e.target
                if (target instanceof HTMLInputElement) {
                    store.setChecked(value, target.checked)
                }
                props.input.onInput()
            }
        })
    }
}

interface CheckboxStore {
    setChecked(value: string, isChecked: boolean): void
}

function useCheckboxStore(
    connector: BoardStoreConnector<MultipleBoardStore>,
): [ReadonlySet<string>, CheckboxStore] {
    const [current, setValue] = useState<ReadonlySet<string>>(new Set())
    const { check, store, connect } = useMemo(() => initCheckboxStore(), [])

    useLayoutEffect(() => {
        connect(setValue)
        connector.connect(store)
        return () => connector.terminate()
    }, [connector, store, connect])

    return [current, check]
}

function initCheckboxStore(): Readonly<{
    check: CheckboxStore
    store: MultipleBoardStore
    connect: (setValue: (value: ReadonlySet<string>) => void) => void
}> {
    let value: Set<string> = new Set()
    let setValue: { (value: ReadonlySet<string>): void } = () => null

    return {
        check: {
            setChecked(item: string, isChecked: boolean): void {
                if (isChecked) {
                    value.add(item)
                } else {
                    value.delete(item)
                }
                setValue(new Set(value))
            },
        },
        store: {
            get(): readonly string[] {
                return Array.from(value.values())
            },
            set(newValue: readonly string[]): void {
                value = new Set(newValue)
                setValue(value)
            },
        },
        connect(newSetValue: { (value: ReadonlySet<string>): void }): void {
            newSetValue(value)
            setValue = newSetValue
        },
    }
}
