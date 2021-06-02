import { decodeBase64StringToUint8Array, encodeUint8ArrayToBase64String } from "../base64/helper"

interface ProtobufEncoder<T> {
    new (): T
    encode(message: T): { finish(): Uint8Array }
}
export function encodeProtobuf<T>(f: ProtobufEncoder<T>, build: { (message: T): void }): string {
    const message = new f()
    build(message)
    return encodeUint8ArrayToBase64String(f.encode(message).finish())
}

interface ProtobufDecoder<T> {
    decode(arr: Uint8Array): T
}
export function decodeProtobuf<T>(f: ProtobufDecoder<T>, text: string): T {
    return f.decode(decodeBase64StringToUint8Array(text))
}
