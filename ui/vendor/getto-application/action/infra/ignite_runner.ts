import { ApplicationActionIgniteHook, ApplicationActionIgniteRunner } from "../infra"

export function initActionIgniteRunner<S>(
    hook: ApplicationActionIgniteHook<S>,
): ApplicationActionIgniteRunner<S> {
    return new Runner(hook)
}

class Runner<S> implements ApplicationActionIgniteRunner<S> {
    state: IgniteHookState<S>

    constructor(hook: ApplicationActionIgniteHook<S>) {
        this.state = { done: false, hook }
    }

    ignite(): Promise<S> {
        // 一回 ignite したら ignite 済みとしてマーク
        // 複数の箇所から複数回コールされても hook の実行は一回だけ
        if (this.state.done) {
            return this.state.promise
        }
        this.state = { done: true, promise: this.state.hook() }
        return this.state.promise
    }
}

type IgniteHookState<S> =
    | Readonly<{ done: false; hook: ApplicationActionIgniteHook<S> }>
    | Readonly<{ done: true; promise: Promise<S> }>
