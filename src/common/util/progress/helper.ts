import { ProgressInfo } from "./data"

export interface ProgressCounter {
    add(): void
}

export type ProgressProps = Readonly<{
    all: number
    step: number
    post: Post<ProgressInfo>
}>
export function initProgressCounter({ all, step, post }: ProgressProps): ProgressCounter {
    let current = 0
    post({ all, current })

    return {
        add: () => {
            current++

            if (current % step === 0) {
                post({ all, current })
            }
        },
    }
}

interface Post<E> {
    (event: E): void
}
