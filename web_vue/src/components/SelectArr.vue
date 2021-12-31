
<script setup lang="ts">
import { computed, ref, Ref, toRefs, onUpdated } from "vue";

const { modelValue } = defineProps<{
    modelValue: { nodes: { n: number }[] },
}>()

const emits = defineEmits<{
    (emit: 'update:modelValue', val: string): void
}>()

const selectOpt = ref("2")

function removeNode(index: number) {
    modelValue.nodes.splice(index, 1)
}
function change(index: number) {
    if (selectOpt.value == "-") {
        removeNode(index)
    }
}
</script>


<template>
    <div>
        <div v-for="(v, i) in modelValue.nodes" :key="i" >
            <select v-model="selectOpt" @change="change(i)">
                <option value="-">-</option>
                <option value="2">2</option>
            </select>
            <input type="number" v-model="v.n" />
            <button @click="removeNode(i)" >remove</button>
        </div>
    </div>
</template>