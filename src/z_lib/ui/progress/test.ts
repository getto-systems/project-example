import { test, expect } from "vitest"
import { ProgressInfo } from "./data"
import { initProgressCounter } from "./helper"

test("add", async () => {
    const stack: ProgressInfo[] = []
    const counter = initProgressCounter({
        all: 10,
        step: 5,
        post: (info) => stack.push(info),
    })
    for (let i = 0; i < 10; i++) {
        counter.add()
    }
    expect(stack).toEqual([
        { all: 10, current: 0 },
        { all: 10, current: 5 },
        { all: 10, current: 10 },
    ])
})
