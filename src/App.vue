<template>
  <div class="common-layout">
    <el-container style="height: 100%;">
      <el-aside width="200px" style="height: 100vh;">
        <div v-for="list in  lists ">
          <span>
            {{ list }}</span>
        </div>
      </el-aside>
      <el-container>
        <el-header>
          <el-button type="primary" round @click="openAddConnDialog">新增连接</el-button>
        </el-header>
        <el-main>
          <div style="width: 100;height: 60%;border: solid 1px ;">
            <el-button type="primary" round @click="openAddConnDialog">新增连接</el-button>
          </div>
        </el-main>
        <el-footer>
          <div style="width: 100;height: 20%;border: solid 1px ;">
            <el-button type="primary" round @click="openAddConnDialog">新增连接</el-button>
          </div>
        </el-footer>
      </el-container>
    </el-container>
  </div>

  <el-dialog v-model="addConnDialog" title="添加链接" width="600px">
    <el-form label-position="right" label-width="100px" :model="connInfo" modal="false">
      <div style="display: flex;">
        <el-form-item label="ip">
          <el-input v-model="connInfo.host" />
        </el-form-item>
        <el-form-item label="port" style="width: 150px;">
          <el-input v-model="connInfo.port" />
        </el-form-item>
      </div>
      <div>
        <el-form-item label="用户名">
          <el-input v-model="connInfo.username" />
        </el-form-item>
        <el-form-item label="密码">
          <el-input type="password" show-password v-model="connInfo.password" />
        </el-form-item>
      </div>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="addConnDialog = false">取消</el-button>
        <el-button type="primary" @click="addConnection"> 确定</el-button>
      </span>
    </template>
  </el-dialog>
</template>


<script setup lang="ts">
import { reactive, ref, onMounted, onBeforeMount } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
const addConnDialog = ref(false)
let connInfo = reactive({
  host: '',
  port: 22,
  username: '',
  password: '',
})


const lists: String[] = reactive([])

const openAddConnDialog = () => {
  connInfo = reactive({
    host: '',
    port: 22,
    username: '',
    password: '',
  })
  addConnDialog.value = true
}

async function addConnection() {
  console.log(connInfo);
  let res = await invoke('add_connection', { conn: connInfo })
  if (res) {
    lists.push(connInfo.host)
  }
  addConnDialog.value = false
}
async function read_config() {
  let res: [] = await invoke('read_config')
  res.forEach((data: any) => {
    lists.push(data.host)
  })
}
onMounted(() => {
  read_config()
})

</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
