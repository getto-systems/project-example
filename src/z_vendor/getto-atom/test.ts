import { test, expect } from "vitest"

import { combine3Atom, combineAtom, composeAtom, initAtom, mapAtom, mapAtomStateful } from "./atom"
import { observeAtom } from "./test_helper"

test("atom", async () => {
    const { state, post } = initAtom({ initialState: "data" })

    const store: string[] = []
    const handler = (state: string) => {
        store.push(state)
    }

    state.subscribe(handler)

    expect(state.currentState()).toEqual("data")

    post("data-1")
    expect(state.currentState()).toEqual("data-1")
    expect(store).toEqual(["data-1"])

    post("data-2")
    expect(state.currentState()).toEqual("data-2")
    expect(store).toEqual(["data-1", "data-2"])

    state.unsubscribe(handler)

    post("data-3")
    expect(state.currentState()).toEqual("data-3")
    expect(store).toEqual(["data-1", "data-2"])
})

test("atom; ignite", async () => {
    const atom = initAtom<string>({
        initialState: "data",
        ignite: () => {
            return new Promise((resolve) => {
                setTimeout(() => {
                    resolve(atom.post("done"))
                }, 16)
            })
        },
    })

    const result = observeAtom(atom.state)

    await atom.state.ignitionState

    expect(result()).toEqual(["done"])
})

test("map", async () => {
    const original = initAtom({ initialState: "data" })
    const mapped = mapAtom(original.state, (state) => `mapped; ${state}`)

    expect(mapped.currentState()).toEqual("mapped; data")

    const result = observeAtom(mapped)

    original.post("next")
    original.post("next")

    expect(result()).toEqual(["mapped; next", "mapped; next"])
})

test("map; stateful", async () => {
    const original = initAtom({ initialState: "data" })
    const mapped = mapAtomStateful(original.state, (state) => `mapped; ${state}`)

    expect(mapped.currentState()).toEqual("mapped; data")

    const result = observeAtom(mapped)

    original.post("next")
    original.post("next")

    expect(result()).toEqual(["mapped; next"])
})

test("combine", async () => {
    const originalA = initAtom({ initialState: "data-A" })
    const originalB = initAtom({ initialState: "data-B" })
    const combined = combineAtom(
        originalA.state,
        originalB.state,
        (stateA, stateB) => `combined; ${stateA}/${stateB}`,
    )

    expect(combined.currentState()).toEqual("combined; data-A/data-B")

    const result = observeAtom(combined)

    originalA.post("next-A")
    originalB.post("next-B")

    expect(result()).toEqual(["combined; next-A/data-B", "combined; next-A/next-B"])
})

test("combine 3", async () => {
    const originalA = initAtom({ initialState: "data-A" })
    const originalB = initAtom({ initialState: "data-B" })
    const originalC = initAtom({ initialState: "data-C" })
    const combined = combine3Atom(
        originalA.state,
        originalB.state,
        originalC.state,
        (stateA, stateB, stateC) => `combined; ${stateA}/${stateB}/${stateC}`,
    )

    expect(combined.currentState()).toEqual("combined; data-A/data-B/data-C")

    const result = observeAtom(combined)

    originalA.post("next-A")
    originalB.post("next-B")
    originalC.post("next-C")

    expect(result()).toEqual([
        "combined; next-A/data-B/data-C",
        "combined; next-A/next-B/data-C",
        "combined; next-A/next-B/next-C",
    ])
})

test("compose", async () => {
    const originalA = initAtom({ initialState: "data-A" })
    const originalB = initAtom({ initialState: "data-B" })
    const originalC = initAtom({ initialState: "data-C" })
    const composed = composeAtom(
        [originalA.state, originalB.state, originalC.state],
        (stateArr) => `composed; ${stateArr.join("/")}`,
    )

    expect(composed.currentState()).toEqual("composed; data-A/data-B/data-C")

    const result = observeAtom(composed)

    originalA.post("next-A")
    originalB.post("next-B")
    originalC.post("next-C")

    expect(result()).toEqual([
        "composed; next-A/data-B/data-C",
        "composed; next-A/next-B/data-C",
        "composed; next-A/next-B/next-C",
    ])
})
