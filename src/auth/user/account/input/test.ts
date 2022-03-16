import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"

import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { initInputResetTokenDestinationAction } from "./action"
import { AuthUserAccountBasket } from "../kernel/data"

describe("InputLoginId", () => {
    test("validate; valid input", async () => {
        const { action, store } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            store.destinationType.set(markBoardValue("email"))
            store.input.set(markBoardValue("valid"))
            action.input.publisher.post()
            return action.validate.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ valid: true }])
        })
    })

    test("validate; invalid : empty", async () => {
        const { action, store } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            store.destinationType.set(markBoardValue("email"))
            store.input.set(markBoardValue(""))
            action.input.publisher.post()
            return action.validate.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ valid: false, err: [{ type: "empty" }] }])
        })
    })

    test("validate; invalid : too-long", async () => {
        const { action, store } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            store.destinationType.set(markBoardValue("email"))
            store.input.set(markBoardValue("a".repeat(100 + 1)))
            action.input.publisher.post()
            return action.validate.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ valid: false, err: [{ type: "too-long", maxLength: 100 }] }])
        })
    })

    test("validate; valid : just max-length", async () => {
        const { action, store } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            store.destinationType.set(markBoardValue("email"))
            store.input.set(markBoardValue("a".repeat(100)))
            action.input.publisher.post()
            return action.validate.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ valid: true }])
        })
    })

    test("reset; has email", () => {
        const { action, store } = standard()
        const user = standard_user()

        store.input.set(markBoardValue("valid"))
        action.reset(user)

        expect(store.destinationType.get()).toEqual("email")
        expect(store.input.get()).toEqual("admin@example.com")
    })
    test("reset; none", () => {
        const { action, store } = standard()
        const user = noDestination_user()

        store.input.set(markBoardValue("valid"))
        action.reset(user)

        expect(store.destinationType.get()).toEqual("none")
        expect(store.input.get()).toEqual("")
    })

    test("terminate", async () => {
        const { action } = standard()

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                action.validate.subscriber.subscribe(handler)
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            action.terminate()
            action.input.publisher.post()
            return action.validate.currentState()
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const { input: action } = initInputResetTokenDestinationAction()
    const store = {
        destinationType: mockBoardValueStore(),
        input: mockBoardValueStore(),
    }
    action.destinationType.connector.connect(store.destinationType)
    action.input.connector.connect(store.input)

    return { action, store }
}

function standard_user(): AuthUserAccountBasket {
    return {
        loginId: "login-id",
        grantedRoles: ["user"],
        resetTokenDestination: { type: "email", email: "user@example.com" },
    }
}
function noDestination_user(): AuthUserAccountBasket {
    return {
        loginId: "login-id",
        grantedRoles: ["user"],
        resetTokenDestination: { type: "none" },
    }
}
