import { ticker } from "../timer/helper"

import { WaitTime } from "../config/infra"

import { RemotePod, RemoteSimulator } from "./infra"

export function mockRemotePod<M, V, R, E>(
    simulator: RemoteSimulator<M, R, E>,
    time: WaitTime,
): RemotePod<M, V, R, E> {
    return (converter) => async (message) => {
        if (time.wait_millisecond > 0) {
            await ticker(time, () => null)
        }
        const result = simulator(message)
        if (!result.success) {
            return result
        }
        return { success: true, value: converter(result.value) }
    }
}
