import { VNode } from "preact"

export type VNodeContent = VNodeEntry | readonly VNodeEntry[]
type VNodeEntry = string | number | VNode

export type VNodeKey = string | number
