
// export type FilterNode =
//     | { and: FilterNode[] }
//     | { or: FilterNode[] }
//     | { isNull: FieldName }
//     | { isNotNull: FieldName }
//     | { eq: [FieldName, ValueNode] }
//     | { notEq: [FieldName, ValueNode] }
//     | { gt: [FieldName, ValueNode] }
//     | { gte: [FieldName, ValueNode] }
//     | { lt: [FieldName, ValueNode] }
//     | { lte: [FieldName, ValueNode] }
//     | { in: [FieldName, ListNode] }
//     | { notInt: [FieldName, ListNode] }
//    "startWith", "contains", "endWith"

export type FilterNode2 = LogicalNode | OperationNode

export type LogicalNode = [keyof typeof logicalObj, FilterNode2[]]
export type NullableNode = [keyof typeof nullableObj, FieldName]
export type EqualNode = [keyof typeof equalObj, FieldName, Value]
export type CmpNode = [keyof typeof cmpObj, FieldName, Value]
export type LikeNode = [keyof typeof likeObj, FieldName, string]
export type InNode = [keyof typeof inObj, FieldName, ListValue]
export type OperationNode = 
    | NullableNode
    | EqualNode
    | CmpNode
    | InNode
    | LikeNode

export type FilterNode3 = LogicalNodeCom | OperationNode

// export type LogicalNode3 = [keyof typeof logicalObj, { n: FilterNode3 }[]]
export type LogicalNodeCom = [keyof typeof logicalObj, { n: FilterNode3, id: number }[]]

export function isLogicalNode(node?: FilterNode2): node is LogicalNode
export function isLogicalNode(node?: FilterNode3): node is LogicalNodeCom
export function isLogicalNode(node?: FilterNode2 | FilterNode3) {
    return node?.[0]! in logicalObj
}

export function isCmpNode(node?: FilterNode2 | FilterNode3): node is CmpNode {
    return node?.[0]! in cmpObj
}
export function isNullableNode(node?: FilterNode2 | FilterNode3): node is NullableNode {
    return node?.[0]! in nullableObj
}
export function isInNode(node?: FilterNode2 | FilterNode3): node is InNode {
    return node?.[0]! in inObj
}
export function isLikeNode(node?: FilterNode2 | FilterNode3): node is LikeNode {
    return node?.[0]! in likeObj
}

export function toComNode(node: FilterNode2): FilterNode3 {
    if (isLogicalNode(node)) {
        const newNodes = node[1].map((n, id) => ({ n: toComNode(n), id }))
        return [node[0], newNodes]
    }
    return node
}

export function toFilterNode(node: FilterNode3): FilterNode2 {
    if (isLogicalNode(node)) {
        const newNodes = node[1].map(node => toFilterNode(node.n))
        return [node[0], newNodes]
    }
    return node
}

// export type NodeKey = FilterNode2[0]

// export type NullableKey = keyof typeof nullableObj
// export type EqKey = keyof typeof eqObj
// export type CmpKey = keyof typeof cmpObj
// export type LikeKey = keyof typeof likeObj
// export type InKey = keyof typeof inObj
// export const eqKey = { eq: "等于", notEq: "不等于" } as const
// export const cmpKey = { ...eqKey, gt: "大于", gte: "大于等于", lt: "小于", lte: "小于等于" } as const
export const logicalObj = { "And": "并且", "Or": "或者" } as const
export const nullableObj = { IsNull: "空的", IsNotNull: "非空" } as const
export const equalObj = { Eq: " = ", NotEq: "≠" } as const
export const cmpObj = { Gt: ">", Gte: "≥", Lt: "<", Lte: "≤" } as const
export const inObj = { In: "属于", NotIn: "不属于" } as const
export const likeObj = { StartWith: "开始为", Contains: "包含", EndWith: "结束为" } as const


type ObjStr = Record<string, string>

// 为了保持 operationObjs 顺序
export const operationObjs: [string, ObjStr][]
    = [[" Equal ", equalObj], [" Cmp ", cmpObj], [" Nullable ", nullableObj], [" Like ", likeObj], [" In ", inObj]]

export function createOperationNode(field: FieldInfo, operationObjs: ObjStr[]): OperationNode {
    const defOperator = Object.keys((operationObjs[0] || {}))[0] as OperationNode[0]
    if (!defOperator) { throw "operationObjs is empty" }

    const type = typeof field.type == "object" ? "Str" : field.type
    if (defOperator in nullableObj) {
        return [defOperator as NullableNode[0], field.name]
    } else if (defOperator in inObj) {
        return [defOperator as InNode[0], field.name, { type, value: [] }]
    } else if (defOperator in likeObj) {
        return [defOperator as LikeNode[0], field.name, "" as const]
    } else {
        const defValue = typeof field.type == "object" 
            ? { type: "Str" as const, value: Object.keys(field.type)[0] }
            : defalueValue[type]
        return [defOperator as (CmpNode | EqualNode)[0], field.name, defValue]
    }
}


// export const fieldOperator = {
//     Str: { ...cmpObj, ...likeObj },
//     Int: cmpObj,
//     Num: cmpObj,
//     DateTime: cmpObj
// } as const

// export const operatorEnum = { ...eqObj, ...inObj }

// type ObjAssign<Obj1, Obj2>
//     = Obj1 extends any
//     ? { [K in keyof Obj1 | keyof Obj2]: K extends keyof Obj2 ? Obj2[K] : Obj1[K & keyof Obj1] }
//     : never

// type ObjAssign2 = ObjAssign<{ type: "Str", value: string }, { value: 2 }>

// type ValueNode =
//     | { str: string }
//     | { int: number }
//     | { num: number }
//     | { dateTime: string }
//     | { unixTimestamp: number }
//     | "currentUserId"
//     | "currentTime"
//     | "currentDate"

type Value = typeof defalueValue[TypeKey]
    // | { type: "Int", value: number }
    // | { type: "Num", value: number }
    // | { type: "Str", value: string }
    // | { type: "DateTime", value: string }
    // | { type: "UnixTimestamp", value: number }
    // | { type: "CurrentUserId" | "CurrentTime" | "CurrentDate" }
    // | { [K in TypeKey]: { type: K, value: TypeMap[K] } }[keyof TypeMap]



export type TypeMap = typeof defalueValue[TypeKey]
// {
//     Int: number,
//     Num: number,
//     Str: string,
//     DateTime: string,
//     UnixTimestamp: number,
// }

export type TypeKey = keyof typeof defalueValue

// type ListValue = { [K in TypeKey]: { type: K, value: TypeMap[K][] } }[keyof TypeMap]
// type ListValue =
//     | { type: "Str", value: string[] }
//     | { type: "Int", value: number[] }
//     | { type: "Num", value: number[] }
//     | { type: "DateTime", value: string[] }
//     | { type: "UnixTimestamp", value: number[] }
type ListValue = _ListValue<Value>
type _ListValue<Val> = Val extends { type: infer T, value: infer V } ? { type: T, value: V[] } : never
// type ListValue = ObjAssign<Value, { value: 1}>

// type ValueListType<T extends FieldType> = TypeMap<T, ListValue>

// type TypeMap<T extends FieldType, Val>
//     = T extends object ? string
//     : Val extends { type: T, value: infer V } ? V
//     : never

type FieldName = string & {}

// type ListNode =
//     | { strList: string[] }
//     | { intList: number[] }
//     | { numList: number[] }

export type FieldType = TypeKey | object
export const defalueValue = {
    get Int() { return { type: "Int" as const, value: 0 } },
    get Num() { return { type: "Num" as const, value: 0 } },
    get Str() { return { type: "Str" as const, value: "" } },
    get DateTime() { return { type: "DateTime" as const, value: "" } },
    get UnixTimestamp() { return { type: "UnixTimestamp" as const, value: 0 } },
}
// export const inputType = {
//     Int: "number",
//     Num: "number",
//     Str: "text",
//     DateTime: "date",
//     UnixTimestamp: () { return { type: "UnixTimestamp" as const, value: 0 } },
// }

export interface FieldInfo {
    name: FieldName,
    text: string,
    type: FieldType,
    nullable?: true,
    operation: string,
}

// export type FieldInfos = Record<FieldName, FieldInfo>
export interface FieldInfos {
    [fieldName: FieldName]: FieldInfo
}


// function getColumnFilterConfig(key: string): Record<string, FieldType> { throw "" }

function getColumnFilter(key: string): { node?: FilterNode2, fieldInfo: FieldInfo } { throw "" }

function saveFilter(key: string, node: FilterNode2) { }

function main() {
    const { node, fieldInfo } = getColumnFilter("key")
    const initNode = [...Object.keys(logicalObj), ...Object.keys(fieldInfo)]
}

