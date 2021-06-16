import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"

import { markBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../ui/vendor/getto-application/board/action_input/mock"

import { initInputPasswordAction } from "./core/impl"

describe("InputPassword", () => {
    test("validate; valid input", async () => {
        const { action } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            action.board.input.set(markBoardValue("valid"))
            return action.validate.initialState
        }).then((stack) => {
            expect(stack).toEqual([{ valid: true }])
            expect(action.validate.get()).toEqual({ valid: true, value: "valid" })
        })
    })

    test("validate; invalid : empty", async () => {
        const { action } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            action.board.input.set(markBoardValue(""))
            return action.validate.initialState
        }).then((stack) => {
            expect(stack).toEqual([{ valid: false, err: [{ type: "empty" }] }])
            expect(action.validate.get()).toEqual({ valid: false, err: [{ type: "empty" }] })
        })
    })

    test("validate; invalid : too-long", async () => {
        const { action } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            action.board.input.set(markBoardValue("a".repeat(100 + 1)))
            return action.validate.initialState
        }).then((stack) => {
            expect(stack).toEqual([{ valid: false, err: [{ type: "too-long", maxLength: 100 }] }])
            expect(action.validate.get()).toEqual({
                valid: false,
                err: [{ type: "too-long", maxLength: 100 }],
            })
        })
    })

    test("validate; valid : just max-length", async () => {
        const { action } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            action.board.input.set(markBoardValue("a".repeat(100)))
            return action.validate.initialState
        }).then((stack) => {
            expect(stack).toEqual([{ valid: true }])
            expect(action.validate.get()).toEqual({ valid: true, value: "a".repeat(100) })
        })
    })

    test("validate; invalid : too-long : multi-byte", async () => {
        const { action } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            action.board.input.set(markBoardValue("あ".repeat(100) + "a"))
            return action.validate.initialState
        }).then((stack) => {
            expect(stack).toEqual([{ valid: false, err: [{ type: "too-long", maxLength: 100 }] }])
            expect(action.validate.get()).toEqual({
                valid: false,
                err: [{ type: "too-long", maxLength: 100 }],
            })
        })
    })

    test("validate; valid : just max-length : multi-byte", async () => {
        const { action } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            action.board.input.set(markBoardValue("あ".repeat(100)))
            return action.validate.initialState
        }).then((stack) => {
            expect(stack).toEqual([{ valid: true }])
            expect(action.validate.get()).toEqual({ valid: true, value: "あ".repeat(100) })
        })
    })

    test("password character state : single byte", () => {
        const { action } = standard()

        action.board.input.set(markBoardValue("password"))

        expect(action.checkCharacter()).toEqual({ multiByte: false })
    })

    test("password character state : multi byte", () => {
        const { action } = standard()

        action.board.input.set(markBoardValue("パスワード"))
        expect(action.checkCharacter()).toEqual({ multiByte: true })
    })

    test("clear", () => {
        const { action } = standard()

        action.board.input.set(markBoardValue("valid"))
        action.clear()

        expect(action.board.input.get()).toEqual("")
    })

    test("terminate", async () => {
        const { action } = standard()

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                action.validate.subscriber.subscribe(handler)
                action.board.input.subscribeInputEvent(() => handler(action.board.input.get()))
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            action.terminate()
            action.board.input.set(markBoardValue("valid"))
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const action = initInputPasswordAction()
    action.board.input.storeLinker.link(mockBoardValueStore())

    return { action }
}
