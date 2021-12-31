
<script setup lang="ts">
import { computed, ref } from "vue";
import { FieldType, defalueValue } from "./SqlFilter";
const { modelValue, type } = defineProps<{
  modelValue: string[] | number[],
  type: FieldType
}>()

const selectAddVal = ref("-")
const inputAddVal = ref<number | string>()
// const inputType = computed(() => typeof type != "object" && typeof defalueValue[type] == "number" ? "number" : "text")

function onChangeSelectVal(index: number) {
  if (modelValue[index] != "-" ) { return }
  modelValue.splice(index, 1)
}

function onChangeSelect_AddVal() {
  if (selectAddVal.value == "-") { return }
  (modelValue as string[]).push(selectAddVal.value)
  selectAddVal.value = "-"
}

function onChangeInputVal(index: number) {
  if (inputAddVal.value) { return }
  modelValue.splice(index, 1)
}

function onChangeInput_AddVal() {
    
}

function onBlurInputVal(index: number) {
  if (modelValue[index]) { return }
  modelValue.splice(index, 1)
}

function onBlurInput_AddVal() {
  if (!inputAddVal.value) { return }
  (modelValue as (number | string)[]).push(inputAddVal.value)
  inputAddVal.value = undefined
}

</script>

<template>
  <div class="list_value">
    <template v-if="typeof type == 'object'">
      (<select v-for="(_, i) in modelValue" v-model="modelValue[i]" :key="i" @change="onChangeSelectVal(i)">
        <option style="color: #C33;" :value="'-'">─</option>
        <option v-for="(v, k) in type" :key="k" :value="k">{{ v }}</option>
      </select>
      <select v-model="selectAddVal" @change="onChangeSelect_AddVal">
        <option value="-">+</option>
        <option v-for="(v, k) in type" :key="k" :value="k">{{ v }}</option>
      </select>)
    </template>
    <!-- <template v-else-if="type == 'DateTime'"></template>
    <template v-else-if="type == 'UnixTimestamp'"></template> -->
    <template v-else>
      (<template v-for="(_, i) in modelValue" :key="i">
        <input v-model="modelValue[i]" :type="type == 'Str' ? 'text' : 'number'" @blur="onBlurInputVal(i)"/>,
      </template>
      <input :type="type == 'Str' ? 'text' : 'number'" v-model="inputAddVal" @blur="onBlurInput_AddVal"/>)
    </template>
  </div>
</template>

<style scoped>
.div_list_value {
  display: flex;
}

input {
  max-width: 3em;
}
</style>