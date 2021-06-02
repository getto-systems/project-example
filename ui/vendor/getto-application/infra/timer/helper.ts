import { DelayTime, WaitTime } from "../config/infra"

export async function delayedChecker<T>(
    promise: Promise<T>,
    time: DelayTime,
    handler: { (): void },
): Promise<T> {
    // winner のチェックのため、実行ごとに新しいオブジェクトを作成する必要がある
    const DELAYED_MARKER = { DELAYED: true }

    const winner = await Promise.race([
        promise,
        ticker({ wait_millisecond: time.delay_millisecond }, () => DELAYED_MARKER),
    ])

    if (winner === DELAYED_MARKER) {
        handler()
    }

    return await promise
}

export function ticker<T>(time: WaitTime, content: { (): T }): Promise<T> {
    return new Promise((resolve) => {
        setTimeout(() => resolve(content()), time.wait_millisecond)
    })
}
