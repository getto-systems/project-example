export function decodeBase64StringToUint8Array(raw: string): Uint8Array {
    return Uint8Array.from(atob(raw), (c) => c.charCodeAt(0))
}
export function encodeUint8ArrayToBase64String(arr: Uint8Array): string {
    return btoa(String.fromCharCode.apply(null, Array.from(arr)))
}
