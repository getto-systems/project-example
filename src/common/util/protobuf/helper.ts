interface ProtobufEncoder<T> {
    new (): T
    encode(message: T): { finish(): Uint8Array }
}
export function encodeProtobuf<T>(f: ProtobufEncoder<T>, build: { (message: T): void }): string {
    const message = new f()
    build(message)
    return encodeUint8ArrayToBase64String(f.encode(message).finish())
}
function encodeUint8ArrayToBase64String(arr: Uint8Array): string {
    return btoa(String.fromCharCode.apply(null, Array.from(arr)))
}

interface ProtobufDecoder<T> {
    decode(arr: Uint8Array): T
}
export function decodeProtobuf<T>(f: ProtobufDecoder<T>, text: string): T {
    return f.decode(decodeBase64StringToUint8Array(text))
}
function decodeBase64StringToUint8Array(raw: string): Uint8Array {
    return Uint8Array.from(atob(raw), (c) => c.charCodeAt(0))
}
