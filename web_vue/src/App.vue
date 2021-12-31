<script setup lang="ts">
import { provide, defineComponent, ref } from "vue";
import SqlFilter, { } from './components/SqlFilter.vue'
import MyInput from './components/MyInput.vue'
import InputArrEle from './components/InputArrEle.vue'
import InputArr from './components/InputArr.vue'
import SelectArr from './components/SelectArr.vue'
import { FilterNode2, FilterNode3, toComNode } from "./components/SqlFilter/SqlFilter";

// provide(keyFieldInfos, [
//   { name: "id", text: "ID", type: "Int", },
//   { name: "age", text: "年龄", type: "Int" },
//   { name: "name", text: "名称", type: "Str" },
//   // { name: "tel", text: "电话", type: "Str", nullable: true },
//   { name: "registerTime", text: "注册时间", type: "DateTime", nullable: true }
// ])

const infos = {
  id: { name: "id", text: "ID", type: "Int", operation: " Equal In " },
  age: { name: "age", text: "年龄", type: "Int", operation: " Equal Cmp In Nullable ", nullable: true },
  name: { name: "name", text: "名称", type: "Str", operation: " Equal In Like " },
  sex: { name: "sex", text: "性别", type: { "1": "男", "2": "女" }, operation: " Equal In " },
  // tel: {text: "电话", type: "Str", nullable: true },
  registerTime: { name: "registerTime", text: "注册时间", type: "DateTime", operation: " Equal Cmp In Nullable ", nullable: true }
} as const

const filterNode = ref<FilterNode3>()

function sqlFilterClear() {
  filterNode.value = undefined
}
function sqlFilterSetData() {
  filterNode.value = toComNode(["And", [
    ["IsNull", "age"],
    ["Eq", "name", { type: "Str", value: "sss0" }],
    ["Eq", "age", { type: "Int", value: 18 }],
  ]])
}
sqlFilterSetData()

// const valArr = ref(["1"])
// const valObj = ref([{ a: "aaa1" }])
// const selectArr = ref({ nodes: [{ n: 1 }, { n: 2 }, { n: 3 }] })


// const inputA = ref(new Map([["s", "2"]]))
</script>

<template>
  <!-- <img alt="Vue logo" src="./assets/logo.png" /> -->
  <button @click="sqlFilterSetData">SqlFilter 已有的数据</button>
  <button @click="sqlFilterClear">SqlFilter 初始空数据</button>
  <hr>
  <SqlFilter v-model="filterNode" :fieldInfos="infos"></SqlFilter>
  <!-- <MyInput v-model="inputA"></MyInput> -->
  <!-- <InputArrEle v-model="valObj[0].a" /> -->

  <!-- <div style="margin-top: 4em;"></div>
  <InputArr v-model="valArr" />
  <SelectArr v-model="selectArr"></SelectArr> -->
  <!-- <pre>
    {{JSON.stringify(filterNode, undefined, 2)}}
  </pre> -->
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  /* text-align: center; */
  color: #2c3e50;
  margin: 2em 1em;
}
</style>
