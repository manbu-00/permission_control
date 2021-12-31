#[derive(Debug)]
pub struct FnFlag {
    pub id: u32,
    /// 上个层级的功能标记
    pub parent_id: u32,
    /// 序号
    pub seq: u32,
    /// 功能标记位
    pub flag: u64,
    pub key: &'static str,
    /// 页面显示的文字
    pub text: &'static str,
    pub display: FnDisplay,
}

impl FnFlag {
    pub const fn new(
        id: u32,
        parent_id: u32,
        seq: u32,
        flag: u64,
        key: &'static str,
        text: &'static str,
        display: FnDisplay,
    ) -> Self {
        Self {
            id,
            seq,
            flag,
            parent_id,
            key,
            text,
            display: display,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FnDisplay {
    Show = 1 << 0,
    Disable = 1 << 1,
}

#[derive(Debug)]
pub struct ApiFlag {
    pub id: u32,
    /// [`FnFlag`] 节点的 id，可能不需要
    pub parent_id: u32,
    /// 序号
    pub seq: u32,
    /// 功能标记位
    pub flag: u64,
    /// 权限通俗易懂的名称
    pub name: &'static str,
    /// 对应后端的某个接口做鉴权
    pub api: &'static str,
}

impl ApiFlag {
    pub const fn new(
        id: u32,
        parent_id: u32,
        seq: u32,
        flag: u64,
        name: &'static str,
        api: &'static str,
    ) -> Self {
        Self {
            id,
            parent_id,
            seq,
            flag,
            api,
            name,
        }
    }
}

// #[derive(Debug)]
// pub struct FnApi {
//     pub fn_flag_id: u32,
//     pub api_flag_id: u32,
// }

#[derive(Debug)]
pub struct Role {
    pub id: u32,
    pub name: &'static str,
}

#[derive(Debug, Default)]
pub struct RoleFn {
    pub role_id: u32,
    pub seq: u32,
    /// 角色拥有功能的64位值
    pub value: u64,
}

impl RoleFn {
    pub fn set(&mut self, value: u64) -> &mut Self {
        self.value |= value;
        self
    }
    pub fn sets<const N: usize>(&mut self, values: [u64; N]) -> &mut Self {
        values.iter().for_each(|&v| self.value |= v);
        self
    }
}

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: &'static str,
}

#[derive(Debug)]
pub struct UserRole {
    pub user_id: u32,
    pub role_id: u32,
}

#[derive(Debug)]
pub struct UserPermission {
    fn_values: Vec<u64>,
}
impl UserPermission {
    pub fn authentication(&self, seq: u32, fn_flag: u64) -> bool {
        self.fn_values[seq as usize] & fn_flag == fn_flag
    }
}
