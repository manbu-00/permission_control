
<script lang="ts">
import { defineComponent, DefineComponent, computed, toRefs, PropType } from "vue";
import SqlFilter from "../SqlFilter.vue";
import { FieldInfos, LogicalNodeCom } from "./SqlFilter";
interface Props {
    modelValue: LogicalNodeCom[1],
    fieldInfos: FieldInfos
}
// const def: DefineComponent<Props> = 
export default defineComponent({
    props: ["modelValue", "fieldInfos"],
    components: { SqlFilter },
    setup(props, ctx) {
        const { modelValue: mVal, fieldInfos: fInfos } = toRefs(props as Props)

        return { mVal, fInfos }
    }
})

// export default def

</script>


<template>
    <div class="div_logical_node">
        <SqlFilter
            v-for="(node, i) in modelValue"
            :key="node.id"
            v-model="node.n"
            :fieldInfos="fInfos"
            @removeNode="mVal.splice(i, 1)"
        />
        <SqlFilter :fieldInfos="fInfos" />
    </div>
</template>

<style scoped>
.div_logical_node {
    margin-left: 1em;
    border-left: solid 1px #aaa;
    width: 100%;
}
</style>