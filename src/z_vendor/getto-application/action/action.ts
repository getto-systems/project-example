export interface StatefulApplicationAction<S> {
    readonly subscriber: ApplicationStateSubscriber<S>
    readonly ignitionState: Promise<S>
    currentState(): S
}

export interface ApplicationStateSubscriber<S> {
    subscribe(handler: ApplicationStateHandler<S>): void
    unsubscribe(target: ApplicationStateHandler<S>): void
}
export interface ApplicationStatePublisher<S> {
    post(state: S): S
}
export interface ApplicationStateHandler<S> {
    (state: S): void
}
export type ApplicationActionHook<S> = Partial<{
    ignite: ApplicationActionIgniteHook<S>
}>

export interface ApplicationActionIgniteHook<S> {
    (): Promise<S>
}

export abstract class AbstractStatefulApplicationAction<S> implements StatefulApplicationAction<S> {
    abstract readonly initialState: S

    readonly subscriber: ApplicationStateSubscriber<S>

    readonly ignitionState: Promise<S>
    readonly currentState: { (): S }

    // this.material.doSomething(this.post) できるようにプロパティとして提供
    readonly post: Post<S>

    constructor(hook: ApplicationActionHook<S> = {}) {
        const { pub, sub } = new PubSub<S>()
        this.subscriber = sub
        this.post = (state: S) => pub.post(state)

        this.ignitionState = new Promise((resolve) => {
            // コンストラクタが重くならないように初期 action は setTimeout で呼び出す
            // 状態は currentState() で最新のものを参照するので subscribe を待つ必要はない
            setTimeout(async () => {
                resolve(hook.ignite ? await hook.ignite() : this.initialState)
            }, 0)
        })

        // sub class から currentState に手出しできないようにコンストラクタの中で構築する
        let currentState: S | null = null
        sub.subscribe((state) => {
            currentState = state
        })
        this.currentState = () => {
            if (currentState === null) {
                return this.initialState
            } else {
                return currentState
            }
        }
    }
}

class PubSub<S> {
    handlers: ApplicationStateHandler<S>[] = []

    pub: ApplicationStatePublisher<S> = {
        post: (state) => {
            this.handlers.forEach((post) => post(state))
            return state
        },
    }
    sub: ApplicationStateSubscriber<S> = {
        subscribe: (handler) => {
            this.handlers = [...this.handlers, handler]
        },
        unsubscribe: (target) => {
            this.handlers = this.handlers.filter((handler) => handler !== target)
        },
    }
}

interface Post<S> {
    (state: S): S
}
