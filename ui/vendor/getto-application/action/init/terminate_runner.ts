import { ApplicationActionTerminateHook, ApplicationActionTerminateRunner } from "../infra"

export function initActionTerminateRunner(): ApplicationActionTerminateRunner {
    return new Runner()
}

class Runner implements ApplicationActionTerminateRunner {
    hooks: ApplicationActionTerminateHook[] = []

    register(hook: ApplicationActionTerminateHook): void {
        this.hooks = [...this.hooks, hook]
    }
    terminate(): void {
        this.hooks.forEach((hook) => hook())
    }
}
