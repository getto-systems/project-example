import { Clock } from "./infra"

export function mockClock(initial: Date, subscriber: ClockSubscriber): Clock {
    let now: Date = initial

    subscriber.subscribe((current: Date) => {
        now = current
    })

    return {
        now: () => now,
    }
}
export function mockClockPubSub(): ClockPubSub {
    let handlers: Handler[] = []

    return {
        update: (current: Date) => {
            handlers.forEach((handler) => handler(current))
        },
        subscribe: (handler: Handler) => {
            handlers = [...handlers, handler]
        },
    }
}
export interface ClockPubSub extends ClockSubscriber {
    update(current: Date): void
}
export interface ClockSubscriber {
    subscribe(handler: Handler): void
}
interface Handler {
    (current: Date): void
}