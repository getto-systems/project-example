import { h } from "preact"
import { useMemo, useState } from "preact/hooks"
import { html } from "htm/preact"
import { PreactNode } from "../../../../x_preact/node"

import { useInputRef } from "./hooks"

import { InputBoardAction } from "../action"
import { initSingleBoardStoreConnector } from "../detail/connector"

import { SingleBoardStore, BoardStoreConnector, MultipleBoardStore } from "../infra"

export const vectorInputTypes = [
    "text",
    "search",
    "number",
    "tel",
    "email",
    "date",
    "time",
] as const
export type VectorInputType = (typeof vectorInputTypes)[number]

type Props = Readonly<{
    type: VectorInputType
    input: InputBoardAction<MultipleBoardStore>
    button: VectorButton
}>
export type VectorButton = Readonly<{
    add: (props: Readonly<{ onClick: () => void }>) => PreactNode
    remove: (props: Readonly<{ onClick: () => void }>) => PreactNode
    undoRemove: (props: Readonly<{ onClick: () => void }>) => PreactNode
}>
export function VectorBoard({ type, input, button }: Props): PreactNode {
    const [connectors, vector] = useVectorConnector(input.connector)
    return html`<ul>
        ${connectors.map((item, index) => {
            return html`<li key=${index}>
                ${[
                    h(VectorInputBoard, {
                        type,
                        connector: item.connector,
                        disabled: item.isDeleting,
                        onInput,
                    }),
                    " ",
                    item.isDeleting ? undoRemoveButton() : removeButton(),
                ]}
            </li>`

            function removeButton(): PreactNode {
                return h(button.remove, { onClick })

                function onClick() {
                    vector.remove(index)
                }
            }
            function undoRemoveButton(): PreactNode {
                return h(button.undoRemove, { onClick })

                function onClick() {
                    vector.undoRemove(index)
                }
            }
        })}
        <li>${addButton()}</li>
    </ul>`

    function onInput() {
        input.onInput()
    }

    function addButton(): PreactNode {
        return h(button.add, { onClick })

        function onClick() {
            vector.push()
        }
    }
}

type InputProps = Readonly<{
    type: VectorInputType
    connector: BoardStoreConnector<SingleBoardStore>
    disabled: boolean
    onInput: () => void
}>
function VectorInputBoard({ type, connector, disabled, onInput }: InputProps): PreactNode {
    return html`<input
        ref=${useInputRef(connector)}
        type=${type}
        disabled=${disabled}
        onInput=${onInput}
    />`
}

interface Vector {
    push(): void
    remove(index: number): void
    undoRemove(index: number): void
}

type VectorItem = Readonly<{
    connector: BoardStoreConnector<SingleBoardStore>
    store: SingleBoardStore
    isDeleting: boolean
}>

function useVectorConnector(
    connector: BoardStoreConnector<MultipleBoardStore>,
): [readonly VectorItem[], Vector] {
    const [connectors, setConnectors] = useState<readonly VectorItem[]>([])
    const store = useMemo(() => initVector(connector, setConnectors), [connector])
    return [connectors, store]
}

function initVector(
    connector: BoardStoreConnector<MultipleBoardStore>,
    setConnectors: (connectors: readonly VectorItem[]) => void,
): Vector {
    const store: VectorItem[] = []
    let length = 0

    connector.connect({
        get: () => {
            return current()
                .filter((item) => !item.isDeleting)
                .map((item) => item.store.get())
        },
        set: (value) => {
            extend(value.length)
            value.forEach((value, index) => {
                store[index].store.set(value)
                store[index] = { ...store[index], isDeleting: false }
            })
            postCurrent()
        },
    })

    return {
        push(): void {
            extend(length + 1)
            postCurrent()
        },
        remove(index: number): void {
            setDeleting(index, true)
        },
        undoRemove(index: number): void {
            setDeleting(index, false)
        },
    }

    function current(): readonly VectorItem[] {
        return store.slice(0, length)
    }
    function postCurrent(): void {
        setConnectors(current())
    }

    function extend(newLength: number): void {
        while (store.length < newLength) {
            // infra の実装扱いなので、例外的に init を使用していい
            store.push({
                ...initSingleBoardStoreConnector(),
                isDeleting: false,
            })
        }

        length = newLength
    }

    function setDeleting(index: number, isDeleting: boolean): void {
        if (index < 0 || index >= store.length) {
            return
        }
        store[index] = { ...store[index], isDeleting }
        postCurrent()
    }
}
