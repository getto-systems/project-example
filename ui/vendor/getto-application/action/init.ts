import { initActionIgniteRunner } from "./init/ignite_runner"
import { initActionStatePubSub } from "./init/state_pub_sub"
import { initActionTerminateRunner } from "./init/terminate_runner"

import {
    ApplicationActionIgniteHook,
    ApplicationActionIgniteRunner,
    ApplicationActionTerminateHook,
    ApplicationActionTerminateRunner,
} from "./infra"

import { ApplicationStateSubscriber, ApplicationStateAction } from "./action"

export abstract class ApplicationAbstractStateAction<S> implements ApplicationStateAction<S> {
    abstract readonly initialState: S

    readonly subscriber: ApplicationStateSubscriber<S>

    // this.material.doSomething(this.post) できるようにプロパティとして提供
    readonly post: Post<S>

    readonly igniteRunner: ApplicationActionIgniteRunner<S>
    readonly terminateRunner: ApplicationActionTerminateRunner = initActionTerminateRunner()

    // コンストラクタの中で state をアップデートするためにプロパティとして定義
    readonly currentState: { (): S }

    constructor(hook: ApplicationActionIgniteHook<S> = async () => this.initialState) {
        const { pub, sub } = initActionStatePubSub<S>()
        this.subscriber = sub
        this.post = (state: S) => pub.post(state)

        this.igniteRunner = initActionIgniteRunner(hook)
        this.terminateHook(() => {
            pub.terminate()
        })

        // 動的に初期化される action でも最新の state で始められるようにする
        // sub class では currentState に手出しできないようにコンストラクタの中で構築する
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

    terminateHook(hook: ApplicationActionTerminateHook): void {
        this.terminateRunner.register(hook)
    }

    ignite(): Promise<S> {
        return new Promise((resolve) => {
            // すべての subscriber が登録された後で ignite するための setTimeout
            setTimeout(() => resolve(this.igniteRunner.ignite()))
        })
    }
    terminate(): void {
        this.terminateRunner.terminate()
    }
}

interface Post<S> {
    (state: S): S
}
