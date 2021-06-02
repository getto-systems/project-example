import { initActionIgniteRunner } from "./infra/ignite_runner"
import { initActionStatePubSub } from "./infra/state_pub_sub"
import { initActionTerminateRunner } from "./infra/terminate_runner"

import {
    ApplicationActionIgniteHook,
    ApplicationActionIgniteRunner,
    ApplicationActionTerminateHook,
    ApplicationActionTerminateRunner,
} from "./infra"

import { ApplicationActionStateSubscriber, ApplicationStateAction } from "./action"

export abstract class ApplicationAbstractStateAction<S> implements ApplicationStateAction<S> {
    abstract readonly initialState: S

    readonly subscriber: ApplicationActionStateSubscriber<S>

    // this.material.doSomething(this.post) できるようにプロパティとして提供
    readonly post: Post<S>

    readonly igniteRunner: ApplicationActionIgniteRunner<S>
    readonly terminateRunner: ApplicationActionTerminateRunner = initActionTerminateRunner()

    constructor(hook: ApplicationActionIgniteHook<S> = async () => this.initialState) {
        const { pub, sub } = initActionStatePubSub<S>()
        this.subscriber = sub
        this.post = (state: S) => pub.post(state)

        this.igniteRunner = initActionIgniteRunner(hook)
        this.terminateHook(() => {
            pub.terminate()
        })
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
