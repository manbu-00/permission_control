<script lang="ts">
import { FieldInfo, FieldInfos, FilterNode3, LogicalNodeCom, OperationNode  } from "./SqlFilter/SqlFilter";
import { defalueValue, operationObjs, createOperationNode, toFilterNode } from "./SqlFilter/SqlFilter";

</script>

<script setup lang="ts">
import { inject, InjectionKey, ref, watch, onMounted, computed, reactive, watchEffect, onUpdated, onBeforeUpdate, toRefs } from "vue";
import { logicalObj, isLogicalNode, isInNode, isNullableNode, isLikeNode } from "./SqlFilter/SqlFilter"
import ListValueCom from "./SqlFilter/ListValueCom.vue"

// const fieldInfos = inject(keyFieldInfos)!
const { modelValue, fieldInfos } = defineProps<{
  modelValue?: FilterNode3
  fieldInfos: FieldInfos
}>()
// const props = defineProps<{
//   modelValue?: FilterNode3
//   fieldInfos: FieldInfos
// }>()

// const { modelValue, fieldInfos } = props
// const { modelValue: refModelValue, fieldInfos: refFieldInfos } = toRefs(props)


const emits = defineEmits<{
  (eventKey: 'update:modelValue', val?: FilterNode3): void
  (eventKey: 'removeNode'): void
}>()

type SelectNode = "-" | LogicalNodeCom[0] | FieldInfo
const selectNode = ref<SelectNode>(!modelValue ? "-" : isLogicalNode(modelValue) ? modelValue[0] : fieldInfos[modelValue[1]])
const selectAddNode = ref<SelectNode>("-")

// watchEffect(() => {
//   console.log("watchEffect", modelValue);
//   selectNode.value = !modelValue ? "-" : isLogicalNode(modelValue) ? modelValue[0] : fieldInfos[modelValue[1]]
// }, {flush: "post"})

watch(() => modelValue, () => {
  console.log("watch")
})
// onBeforeUpdate(() => {
//   console.log("onBeforeUpdate", refModelValue?.value);
// })
// onUpdated(() => {
//   console.log("onUpdated", refModelValue?.value);
//   selectNode.value = !modelValue ? "-" : isLogicalNode(modelValue) ? modelValue[0] : fieldInfos[modelValue[1]]
// })

function getOperators(selectNode: SelectNode) {
  if (typeof selectNode != "object") { return [] }

  const field = fieldInfos[selectNode.name]
  const opeArr = operationObjs.filter(([k, v]) => field.operation.includes(k)).map(([k, v]) => v)
  return opeArr
}

const operators = computed(() => getOperators(selectNode.value))

const fieldType = computed(() => typeof selectNode.value == "object" ? fieldInfos[selectNode.value.name].type : undefined)


function onChangeSelectNode() {
  const val = selectNode.value
  if (typeof val == "object") {
    emits("update:modelValue", createOperationNode(val, operators.value))
  } else if (val == "-") {
    if (isLogicalNode(modelValue)) {
      emits("update:modelValue", undefined)
    } else {
      emits("removeNode")
    }
  } else {
    emits("update:modelValue", [val, []])
  }
}

function onChangeSelect_AddNewNode(node: LogicalNodeCom) {
  const val = selectAddNode.value
  if (val == "-") { return }

  const newNode = typeof val == "object"
    ? createOperationNode(val, getOperators(val))
    : [val, []] as LogicalNodeCom

  node[1].push({ n: newNode, id: +new Date() })
  selectAddNode.value = "-"
}

function onChangeOperator(node: OperationNode) {
  if (isLikeNode(node)) {
    node[2] = ""
  } else if (isNullableNode(node)) {
    node.splice(2, 1)
  } else {
    const type = typeof fieldType.value == "object" ? "Str" : fieldType.value!
    if (isInNode(node)) {
      if (Array.isArray(node[2].value)) { return } // 保留上次的值

      node[2] = { type, value: [] }
    } else {
      if (!Array.isArray(node[2].value) && typeof node[2].value == "object") { return } // 保留上次的值

      node[2] = typeof fieldType.value == "object" 
        ? { type: "Str" as const, value: Object.keys(fieldType.value)[0] }
        : defalueValue[type] 
    }
  }
}

function removeNode(node: LogicalNodeCom, index: number) {
  node[1].splice(index, 1)
}

function clear() {
  selectAddNode.value = "-"
}

const log = (node?: FilterNode3) => {
  if (!node) { return }
  const newNode = toFilterNode(node)
  const strJson = JSON.stringify(newNode)
  console.log(strJson)
  console.log(JSON.parse(strJson))
}


</script>

<template>
  <div class="div_sql_filter">
    <button @click="log(modelValue)">log</button>
    <select v-model="selectNode" title="Filter" @change="onChangeSelectNode">
      <option style="color: #C33;" :value="'-'">&nbsp;&nbsp;─</option>
      <option v-for="(v, k) in logicalObj" :value="k" :key="k">{{ v }}</option>
      <option disabled></option>
      <option v-for="(v, k) in fieldInfos" :value="v" :key="k">{{ v.text }}</option>
    </select>
    <template v-if="modelValue == undefined"></template>
    <div v-else-if="isLogicalNode(modelValue)" class="div_logical_node">
      <SqlFilter
        v-for="(node, i) in modelValue[1]"
        :key="node.id"
        v-model="node.n"
        :fieldInfos="fieldInfos"
        @removeNode="removeNode(modelValue as any, i)"
      />
      <select v-model="selectAddNode" title="Filter" @change="onChangeSelect_AddNewNode(modelValue as any)">
        <option :value="'-'">&nbsp;&nbsp;＋</option>
        <option v-for="(v, k) in logicalObj" :value="k" :key="k">{{ v }}</option>
        <option disabled></option>
        <option v-for="(v, k) in fieldInfos" :value="v" :key="k">{{ v.text }}</option>
      </select>
    </div>
    <template v-else>
      <select v-model="modelValue[0]" title="operator" class="operator" @change="onChangeOperator(modelValue as any)">
        <template v-for="opeObj in operators">
          <option v-for="(v, k) in opeObj" :value="k" :key="k">{{ v }}</option>
        </template>
      </select>

      <template v-if="isNullableNode(modelValue)"></template>
      <input v-else-if="isLikeNode(modelValue)" :type="'text'" v-model="modelValue[2]">
      <ListValueCom v-else-if="isInNode(modelValue)" v-model="modelValue[2].value" :type="fieldType!" />
      <select v-else-if="typeof fieldType == 'object'" v-model="modelValue[2].value" class="select_enum_value">
          <option v-for="(v, k) in fieldType" :key="k" :value="k">{{ v }}</option>
      </select>
      <input v-else :type="typeof modelValue[2].value == 'string' ? 'text' : 'number'" v-model="modelValue[2].value">
    </template>
  </div>
</template>

<style scoped>
.div_sql_filter {
  display: flex;
  flex-wrap: wrap;
}
.div_logical_node {
  margin-left: 2em;
  border-left: solid 1px #aaa;
  width: 100%;
}
/* .div_operator {
  display: inline-flex;
} */
.operator {
  text-align: center;
  /* margin-right: -1em; */
}
.operator + input {
  /* flex: 1; */
  padding-left: 1em;
  /* margin-left: 1em; */
}
.select_enum_value {
  padding: 0 0.5em;
}
</style>

<style>
.div_sql_filter select {
  margin: 2px 0;
  border-top: none;
  border-left: none;
  border-right: none;
  border-radius: unset;
}
.div_sql_filter input {
  border-top: none;
  border-left: none;
  border-right: none;
  border-bottom: 1px #000 dotted;
  border-radius: unset;
  /* text-align: center; */
  margin-left: 2px;
  margin-bottom: 2.5px;
}
</style>