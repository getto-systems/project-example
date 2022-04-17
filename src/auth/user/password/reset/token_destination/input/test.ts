import { setupActionTestRunner } from "../../../../../../z_vendor/getto-application/action/test_helper"

import { markBoardValue } from "../../../../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../../../../z_vendor/getto-application/board/input/test_helper"

import { initInputResetTokenDestinationAction } from "./action"

import { restoreResetTokenDestination } from "../kernel/convert"

import { ResetTokenDestination } from "../kernel/data"

test("validate; valid input", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.destinationType.set(markBoardValue("email"))
        store.email.set(markBoardValue("user@example.com"))
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
        store.email.set(markBoardValue(""))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ valid: false, err: [{ type: "empty-email" }] }])
    })
})

test("validate; invalid : invalid", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.destinationType.set(markBoardValue("email"))
        store.email.set(markBoardValue("invalid-email; not includes at-mark"))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ valid: false, err: [{ type: "invalid-email" }] }])
    })
})

test("validate; invalid : too-long", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.destinationType.set(markBoardValue("email"))
        store.email.set(markBoardValue("@".repeat(255 + 1)))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ valid: false, err: [{ type: "too-long-email", maxLength: 255 }] }])
    })
})

test("validate; valid : just max-length", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.destinationType.set(markBoardValue("email"))
        store.email.set(markBoardValue("@".repeat(255)))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ valid: true }])
    })
})

test("reset; has email", () => {
    const { action, store } = standard()
    const destination = standard_destination()

    store.email.set(markBoardValue("some-user@example.com"))
    action.reset(destination)

    expect(store.destinationType.get()).toEqual("email")
    expect(store.email.get()).toEqual("user@example.com")
})
test("reset; none", () => {
    const { action, store } = standard()
    const destination = no_destination()

    store.email.set(markBoardValue("some-user@example.com"))
    action.reset(destination)

    expect(store.destinationType.get()).toEqual("none")
    expect(store.email.get()).toEqual("")
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
        action.email.publisher.post()
        return action.validate.currentState()
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    const { input: action } = initInputResetTokenDestinationAction()
    const store = {
        destinationType: mockBoardValueStore(action.destinationType),
        email: mockBoardValueStore(action.email),
    }

    return { action, store }
}

function standard_destination(): ResetTokenDestination {
    return restoreResetTokenDestination({ type: "email", email: "user@example.com" })
}
function no_destination(): ResetTokenDestination {
    return { type: "none" }
}
