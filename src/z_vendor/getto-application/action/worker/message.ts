export type WorkerProxyCallID = number

export type WorkerProxyMessage<N, T> = Readonly<{
    id: WorkerProxyCallID
    name: N
    params: Readonly<T>
}>
export type WorkerProxyResponse<N, T> = Readonly<{ id: WorkerProxyCallID; name: N; data: T }>
